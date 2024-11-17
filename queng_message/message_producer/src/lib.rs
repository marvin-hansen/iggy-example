mod produce;

use common_message;
use iggy::client::Client;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::messages::send_messages::Partitioning;
use iggy::utils::duration::IggyDuration;
use message_shared::utils as shared_utils;
use message_shared::Args;
use std::str::FromStr;

pub struct MessageProducer {
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
    pub async fn new(
        username: String,
        password: String,
        stream_id: String,
        topic_id: String,
        tcp_server_address: String,
    ) -> Result<Self, IggyError> {
        Self::build(Args::new(
            username,
            password,
            stream_id,
            topic_id,
            tcp_server_address,
        ))
        .await
    }

    /// Creates a new `MessageProducer` instance using the provided `ImsDataConfig`.
    ///
    /// # Arguments
    ///
    /// * `config` - The `ImsDataConfig` to build the `MessageProducer` instance from.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn from_config(config: &common_message::ImsDataConfig) -> Result<Self, IggyError> {
        Self::build(Args::from_ims_data_config(config)).await
    }

    /// Creates a new `MessageProducer` instance using the default configuration.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn default() -> Result<Self, IggyError> {
        Self::build(Args::default()).await
    }
}

impl MessageProducer {
    async fn build(args: Args) -> Result<Self, IggyError> {
        // Build client
        let client = shared_utils::build_client(&args)
            .await
            .expect("Failed to create client");

        // Connect client
        client.connect().await.expect("Failed to connect");

        // Create producer
        let mut producer = client
            .producer(&args.stream_id, &args.topic_id)
            .expect("Failed to create producer")
            .batch_size(args.messages_per_batch)
            .send_interval(IggyDuration::from_str(&args.interval).expect("Invalid interval format"))
            .partitioning(Partitioning::balanced())
            .build();

        // Create stream and user
        shared_utils::create_stream_and_user(
            &args.stream_id,
            &args.username,
            &args.password,
            &client,
        )
        .await
        .expect("Failed to create stream and user");

        // Init producer
        producer.init().await.expect("Failed to init producer");

        Ok(Self { producer })
    }
}

impl MessageProducer {
    pub async fn shutdown() {}
}
