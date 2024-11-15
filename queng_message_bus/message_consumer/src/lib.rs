mod consume;

use common_message_bus::prelude::ImsDataConfig;
use iggy::client::Client;
use iggy::clients::consumer::{AutoCommit, AutoCommitWhen, IggyConsumer, ReceivedMessage};
use iggy::consumer::ConsumerKind;
use iggy::error::IggyError;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::utils::duration::IggyDuration;
use message_shared::utils as shared_utils;
use message_shared::Args;
use std::error::Error;
use std::str::FromStr;

pub struct MessageConsumer {
    consumer: IggyConsumer,
    message_handler: fn(&ReceivedMessage) -> Result<(), Box<dyn Error>>,
}

impl MessageConsumer {
    /// Creates a new `MessageConsumer` instance using the provided credentials and identifiers.
    ///
    /// # Arguments
    ///
    /// * `consumer_name` - The name of the consumer.
    /// * `username` - The username for stream authentication.
    /// * `password` - The password for stream authentication.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    /// * `tcp_server_address` - The tcp server address i.e. "127.0.0.1:8090"
    /// * `message_handler` - A callback function that will be called for each received message.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn new(
        consumer_name: &str,
        username: String,
        password: String,
        stream_id: String,
        topic_id: String,
        tcp_server_address: String,
        message_handler: fn(&ReceivedMessage) -> Result<(), Box<dyn Error>>,
    ) -> Result<Self, IggyError> {
        Self::build(
            Args::new(username, password, stream_id, topic_id, tcp_server_address),
            consumer_name,
            message_handler,
        )
        .await
    }

    /// Creates a new `MessageConsumer` instance using the provided `ImsDataConfig`.
    ///
    /// # Arguments
    ///
    /// * `config` - The `ImsDataConfig` to build the `MessageConsumer` instance from.
    /// * `consumer_name` - The name of the consumer.
    /// * `message_handler` - A callback function that will be called for each received message.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn from_config(
        config: &ImsDataConfig,
        consumer_name: &str,
        message_handler: fn(&ReceivedMessage) -> Result<(), Box<dyn Error>>,
    ) -> Result<Self, IggyError> {
        Self::build(
            Args::from_ims_data_config(config),
            consumer_name,
            message_handler,
        )
        .await
    }

    /// Creates a new `MessageConsumer` instance using the default configuration.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn default() -> Result<Self, IggyError> {
        let consumer_name = "default-message-consumer";
        let message_handler = Self::default_message_handler;
        Self::build(Args::default(), consumer_name, message_handler).await
    }

    fn default_message_handler(message: &ReceivedMessage) -> Result<(), Box<dyn Error>> {
        println!("Received message: {:?}", &message.message.id);
        Ok(())
    }
}

impl MessageConsumer {
    async fn build(
        args: Args,
        consumer_name: &str,
        message_handler: fn(&ReceivedMessage) -> Result<(), Box<dyn Error>>,
    ) -> Result<Self, IggyError> {
        // Build client
        let client = shared_utils::build_client(&args)
            .await
            .expect("Failed to create client");

        // Connect client
        client.connect().await.expect("Failed to connect");

        // Build consumer
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

        consumer
            .init()
            .await
            .expect("Failed to initialize consumer");

        Ok(Self {
            consumer,
            message_handler,
        })
    }
}
