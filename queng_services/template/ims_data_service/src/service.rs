use common_ims::IntegrationConfig;
use common_message::StreamUser;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use std::collections::HashMap;
use std::error::Error;

type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

/// A server that handles IMS (Integration Management Service) data processing.
///
/// The server manages message consumption and production for both control and data channels,
/// maintaining thread-safe access to shared resources using Tokio's async-aware locks.
pub struct Server {
    dbg: bool,
    // data_integration: Box<dyn DataIntegration>,
    integration_config: IntegrationConfig,
    consumer: Guarded<MessageConsumer>,
    producer: MessageProducer,
    client_stream_user: StreamUser,
    client_producers: Guarded<HashMap<u16, MessageProducer>>,
}

impl Server {
    /// Creates a new IMS data service server with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `integration_config` - Configuration for integration endpoints and channels
    /// * `stream_user` - Custom stream user shared between producer and consumer
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `Server` instance if successful, or a boxed error if initialization fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Failed to create the message consumer
    /// * Failed to create the message producer
    /// * Failed to initialize communication channels
    pub async fn new(
        integration_config: IntegrationConfig,
        stream_user: StreamUser,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(false, integration_config, stream_user).await
    }

    /// Creates a new IMS data service server with debug mode enabled.
    ///
    /// # Arguments
    ///
    /// * `integration_config` - Configuration for integration endpoints and channels
    /// * `stream_user` - Custom stream user shared between producer and consumer
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `Server` instance if successful, or a boxed error if initialization fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Failed to create the message consumer
    /// * Failed to create the message producer
    /// * Failed to initialize communication channels
    pub async fn with_debug(
        integration_config: IntegrationConfig,
        stream_user: StreamUser,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(true, integration_config, stream_user).await
    }
}

impl Server {
    /// Builds a new server instance with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `dbg` - Whether to enable debug mode
    /// * `integration_config` - Configuration for integration endpoints and channels
    /// * `stream_user` - Custom stream user shared between producer and consumer
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `Server` instance if successful, or a boxed error if initialization fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Failed to create the message consumer with the specified credentials
    /// * Failed to create the message producer with the specified credentials
    /// * Failed to initialize the control or data channels
    /// * Failed to set up client producers
    async fn build(
        dbg: bool,
        integration_config: IntegrationConfig,
        client_stream_user: StreamUser,
    ) -> Result<Self, Box<dyn Error>> {
        if dbg {
            println!("Configure iggy producer")
        }

        let stream_id = integration_config.control_channel();
        let topic_id = integration_config.control_channel();

        if dbg {
            println!("Construct iggy producer")
        }
        let producer = MessageProducer::new(stream_id, topic_id, &client_stream_user)
            .await
            .expect("Failed to create producer");

        if dbg {
            println!("Configure iggy consumer")
        }
        let consumer_name = "ims-data-binance-control";
        let stream_id = integration_config.control_channel();
        let topic_id = integration_config.control_channel();

        if dbg {
            println!("Construct iggy consumer")
        }
        let consumer =
            MessageConsumer::new(consumer_name, stream_id, topic_id, &client_stream_user)
                .await
                .expect("Failed to create consumer");

        // Create a new HashMap to store data producers for each client
        let client_producers = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        Ok(Self {
            dbg,
            integration_config,
            consumer: std::sync::Arc::new(tokio::sync::RwLock::new(consumer)),
            producer,
            client_producers,
            client_stream_user,
        })
    }

    /// Returns a reference to the thread-safe map of client producers.
    ///
    /// The client producers are stored in a thread-safe container using `Arc<RwLock>`,
    /// allowing for concurrent access from multiple tasks.
    ///
    /// # Returns
    ///
    /// A reference to the guarded HashMap containing client IDs mapped to their respective `MessageProducer`s.
    pub fn client_producers(&self) -> &Guarded<HashMap<u16, MessageProducer>> {
        &self.client_producers
    }

    /// Returns a reference to the thread-safe message consumer.
    ///
    /// The consumer is stored in a thread-safe container using `Arc<RwLock>`,
    /// allowing for concurrent access from multiple tasks.
    ///
    /// # Returns
    ///
    /// A reference to the guarded `MessageConsumer`.
    pub fn consumer(&self) -> &Guarded<MessageConsumer> {
        &self.consumer
    }

    /// Returns a reference to the message producer.
    ///
    /// # Returns
    ///
    /// A reference to the `MessageProducer` used for sending messages.
    pub fn producer(&self) -> &MessageProducer {
        &self.producer
    }

    /// Returns a reference to the custom stream user used for client connections.
    ///
    /// The custom stream user is used to authenticate clients connecting to the server.
    ///
    /// # Returns
    ///
    /// A reference to the `StreamUser` instance used for client authentication.
    pub fn client_stream_user(&self) -> &StreamUser {
        &self.client_stream_user
    }

    /// Returns a reference to the integration configuration.
    ///
    /// The integration configuration contains information about the server's
    /// integration endpoints, channels, and version.
    ///
    /// # Returns
    ///
    /// A reference to the `IntegrationConfig` instance.
    pub fn integration_config(&self) -> &IntegrationConfig {
        &self.integration_config
    }
}

impl Server {
    /// Shuts down the message service.
    ///
    /// This function will shut down the consumer and producer for the message service.
    /// It will clean up any resources used by the consumer and producer and then shut them down.
    ///
    /// Errors:
    ///
    /// * If the consumer or producer fails to clean up and/or shut down.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping a `Box<dyn Error>` or `Ok(())` if the message service was shutdown successfully.
    pub(super) async fn shutdown(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("Shutting down message service");


        Ok(())
    }
}


impl Server {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[IMSData/Server]: {msg}");
        }
    }
}
