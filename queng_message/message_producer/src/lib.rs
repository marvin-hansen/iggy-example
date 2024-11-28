mod getters;
mod send;
mod shutdown;

use ahash::AHashMap;
use common_message::StreamUser;
use iggy::client::{Client, StreamClient, UserClient};
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::send_messages::Partitioning;
use iggy::models::permissions::{Permissions, StreamPermissions};
use iggy::models::user_status::UserStatus;
use iggy::utils::duration::IggyDuration;
use message_shared::utils as shared_utils;
use message_shared::Args;
use std::str::FromStr;

pub struct MessageProducer {
    user_id: Identifier,
    stream_id: Identifier,
    topic_id: Identifier,
    client: IggyClient,
    producer: IggyProducer,
}

impl MessageProducer {
    /// Creates a new `MessageProducer` instance using the provided credentials and identifiers.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for stream authentication.
    /// * `password` - The password for stream authentication.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    /// * `tcp_server_address` - The tcp server address i.e. "127.0.0.1:8090"
    ///
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    /// Creates a new `MessageProducer` instance using the provided credentials and identifiers.
    ///
    /// # Arguments
    ///
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    /// * `stream_user` - The `StreamUser` containing the username and password for stream authentication.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn new(
        stream_id: String,
        topic_id: String,
        stream_user: &StreamUser,
    ) -> Result<Self, IggyError> {
        let args = Args::new(stream_id, topic_id);
        Self::build(args, None, stream_user).await
    }

    /// Creates a new `MessageProducer` instance using the provided `IggyClient` and identifiers.
    ///
    /// # Arguments
    ///
    /// * `client` - The `IggyClient` to use for authentication and communication.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    /// * `stream_user` - The `StreamUser` containing the username and password for stream authentication.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn with_client(
        client: IggyClient,
        stream_id: String,
        topic_id: String,
        stream_user: &StreamUser,
    ) -> Result<Self, IggyError> {
        let args = Args::new(stream_id, topic_id);
        Self::build(args, Some(client), stream_user).await
    }

    /// Creates a default `MessageProducer` instance using the default `Args` and `StreamUser`.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn default() -> Result<Self, IggyError> {
        Self::build(Args::default(), None, &StreamUser::default()).await
    }
}

impl MessageProducer {
    async fn build(
        args: Args,
        client: Option<IggyClient>,
        stream_user: &StreamUser,
    ) -> Result<Self, IggyError> {
        // Create identifiers for stream, topic, and user.
        dbg!("Creating identifiers");
        let stream_id = Identifier::from_str_value(&args.stream_id).expect("Invalid stream id");
        let topic_id = Identifier::from_str_value(&args.topic_id).expect("Invalid topic id");
        let user_id = Identifier::from_str_value(&args.username).expect("Invalid user id");

        dbg!("Building client");
        let client = if client.is_some() {
            // Unwrap client from option
            client.unwrap()
        } else {
            // Build new client
            shared_utils::build_client(args.to_sdk_args())
                .await
                .expect("Failed to create client")
        };

        dbg!("Connecting client");
        client.connect().await.expect("Failed to connect");

        dbg!("Login admin user to stream");
        client
            .login_user(&args.username, &args.password)
            .await
            .expect("Failed to login user");

        dbg!("Creating producer");
        let mut producer = client
            .producer(&args.stream_id, &args.topic_id)
            .expect("Failed to create producer")
            .batch_size(args.messages_per_batch)
            .send_interval(IggyDuration::from_str(&args.interval).expect("Invalid interval format"))
            .partitioning(Partitioning::balanced())
            .build();

        // Create stream
        dbg!("Creating stream");
        let res = client.create_stream(&args.stream_id, None).await;
        dbg!(&res);

        let stream = if res.is_err() {
            let code = res.as_ref().unwrap_err().as_code() as usize;
            if code == IggyError::StreamIdAlreadyExists as usize {
                // Stream already exists
                dbg!("Stream already exists");
                client
                    .get_stream(&Identifier::from_str_value(&args.stream_id)?)
                    .await
                    .expect("Failed to get stream")
                    .unwrap()
            } else {
                dbg!("Error creating stream");
                return Err(res.unwrap_err());
            }
        } else {
            res?
        };

        // Configure stream permissions
        dbg!("Configuring stream permissions");
        let mut streams_permissions = AHashMap::new();
        streams_permissions.insert(
            stream.id,
            StreamPermissions {
                read_stream: true,
                read_topics: true,
                ..Default::default()
            },
        );

        let permissions = Permissions {
            streams: Some(streams_permissions),
            ..Default::default()
        };

        // Create custom stream user
        dbg!("Creating custom stream user");
        match client
            .create_user(
                stream_user.username(),
                stream_user.password(),
                UserStatus::Active,
                Some(permissions),
            )
            .await
        {
            Ok(_) => {
                // user crated
                dbg!("User created");
            }
            Err(e) => {
                // Error code 46 means user already exists; so we will not create it again.
                if e.as_code() == 304 {
                    // Do nothing
                    dbg!("User already exists");
                } else {
                    return Err(e);
                }
            }
        }

        // Init producer
        dbg!("Initializing producer");
        producer.init().await.expect("Failed to init producer");

        Ok(Self {
            user_id,
            stream_id,
            topic_id,
            client,
            producer,
        })
    }
}
