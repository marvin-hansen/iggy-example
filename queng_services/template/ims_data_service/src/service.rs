use common_iggy::IggyConfig;
use common_ims::IntegrationConfig;
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use std::collections::HashMap;
use std::error::Error;

type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

/// A server that handles IMS (Integration Management Service) data processing.
pub struct Service {
    dbg: bool,
    consumer_client: IggyClient,
    producer_client: IggyClient,
    iggy_config: IggyConfig,
    integration_config: IntegrationConfig,
    client_configs: Guarded<HashMap<u16, IggyConfig>>,
    client_producers: Guarded<HashMap<u16, IggyClient>>,
}

impl Service {
    /// Creates a new IMS data service server with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `integration_config` - Configuration for integration endpoints and channels
    /// * `iggy_config` -  Configuration for the iggy messaging system
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
        iggy_config: IggyConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(false, integration_config, iggy_config).await
    }

    /// Creates a new IMS data service server with debug mode enabled.
    ///
    /// # Arguments
    ///
    /// * `integration_config` - Configuration for integration endpoints and channels
    /// * `iggy_config` -  Configuration for the iggy messaging system
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
        iggy_config: IggyConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(true, integration_config, iggy_config).await
    }
}

impl Service {
    async fn build(
        dbg: bool,
        integration_config: IntegrationConfig,
        iggy_config: IggyConfig,
    ) -> Result<Self, Box<dyn Error>> {
        dbg!("Configure iggy producer");

        let stream_id = integration_config.control_channel();
        let topic_id = integration_config.control_channel();

        dbg!("Construct iggy producer");
        let producer = message_shared::utils::build_client(stream_id.clone(), topic_id.clone())
            .await
            .expect("Failed to build client");

        dbg!("Connecting producer");
        producer.connect().await.expect("Failed to connect");

        dbg!("Login producer");
        producer
            .login_user(&iggy_config.user().username(), &iggy_config.user().password())
            .await
            .expect("Failed to login user");

        dbg!("Construct iggy consumer");
        let consumer = message_shared::utils::build_client(stream_id.clone(), topic_id.clone())
            .await
            .expect("Failed to build client");

        dbg!("Connecting consumer");
        consumer.connect().await.expect("Failed to connect");

        dbg!("Login consumer");
        consumer
            .login_user(&iggy_config.user().username(), &iggy_config.user().password())
            .await
            .expect("Failed to login user");

        let client_configs = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));
        let client_producers = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        Ok(Self {
            dbg,
            consumer_client: consumer,
            producer_client: producer,
            iggy_config,
            integration_config,
            client_configs,
            client_producers,
        })
    }
}

// Getters
impl Service {
    pub fn dbg(&self) -> bool {
        self.dbg
    }

    pub fn consumer(&self) -> &IggyClient {
        &self.consumer_client
    }

    pub fn producer(&self) -> &IggyClient {
        &self.producer_client
    }

    pub fn iggy_config(&self) -> &IggyConfig {
        &self.iggy_config
    }

    pub fn integration_config(&self) -> &IntegrationConfig {
        &self.integration_config
    }

    pub fn client_configs(&self) -> &Guarded<HashMap<u16, IggyConfig>> {
        &self.client_configs
    }

    pub fn client_producers(&self) -> &Guarded<HashMap<u16, IggyClient>> {
        &self.client_producers
    }
}

impl Service {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[IMSData/Service]: {msg}");
        }
    }
}
