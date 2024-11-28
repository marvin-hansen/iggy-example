use common_message::StreamUser;
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::{AutoCommit, AutoCommitWhen, IggyConsumer};
use iggy::consumer::ConsumerKind;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::utils::duration::IggyDuration;
use message_shared::utils as shared_utils;
use message_shared::Args;
use std::str::FromStr;

mod getters;
mod shutdown;

pub struct MessageConsumer {
    user_id: Identifier,
    stream_id: Identifier,
    topic_id: Identifier,
    client: IggyClient,
    consumer: IggyConsumer,
}

impl MessageConsumer {
    /// Creates a new `MessageConsumer` instance with the specified arguments.
    ///
    /// # Arguments
    ///
    /// * `consumer_name` - The name of the consumer.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    /// * `stream_user` - The stream user for authentication.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn new(
        consumer_name: &str,
        stream_id: String,
        topic_id: String,
        stream_user: &StreamUser,
    ) -> Result<Self, IggyError> {
        let args = Args::new(stream_id, topic_id);
        Self::build(args, None, consumer_name, stream_user).await
    }

    /// Creates a `MessageConsumer` instance using the provided `IggyClient` and configuration.
    ///
    /// # Arguments
    ///
    /// * `client` - The `IggyClient` to use for creating the consumer.
    /// * `consumer_name` - The name of the consumer.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    /// * `stream_user` - The custom stream user to use for authentication.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn with_client(
        client: IggyClient,
        consumer_name: &str,
        stream_id: String,
        topic_id: String,
        stream_user: &StreamUser,
    ) -> Result<Self, IggyError> {
        let args = Args::new(stream_id, topic_id);
        Self::build(args, Some(client), consumer_name, stream_user).await
    }

    /// Creates a default `MessageConsumer` instance with default arguments.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn default() -> Result<Self, IggyError> {
        let consumer_name = "default-message-consumer";
        Self::build(Args::default(), None, consumer_name, &StreamUser::default()).await
    }
}

impl MessageConsumer {
    async fn build(
        args: Args,
        client: Option<IggyClient>,
        consumer_name: &str,
        stream_user: &StreamUser,
    ) -> Result<Self, IggyError> {
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
            .login_user(stream_user.username(), stream_user.password())
            .await
            .expect("Failed to login user");

        dbg!("Building consumer");
        let mut consumer =
            match ConsumerKind::from_code(args.consumer_kind).expect("Invalid consumer kind") {
                ConsumerKind::Consumer => client
                    .consumer(
                        consumer_name,
                        &args.stream_id,
                        &args.topic_id,
                        args.partition_id,
                    )
                    .expect("Failed to create consumer"),
                ConsumerKind::ConsumerGroup => client
                    .consumer_group(consumer_name, &args.stream_id, &args.topic_id)
                    .expect("Failed to create consumer group"),
            }
            .auto_commit(AutoCommit::When(AutoCommitWhen::PollingMessages))
            .create_consumer_group_if_not_exists()
            .auto_join_consumer_group()
            .polling_strategy(PollingStrategy::next())
            .poll_interval(IggyDuration::from_str(&args.interval).expect("Invalid interval format"))
            .batch_size(args.messages_per_batch)
            .build();

        dbg!("Initializing consumer");
        consumer
            .init()
            .await
            .expect("Failed to initialize consumer");

        Ok(Self {
            user_id,
            stream_id,
            topic_id,
            client,
            consumer,
        })
    }
}
