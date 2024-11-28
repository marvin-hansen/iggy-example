use common_iggy::IggyConfig;
use common_ims::IntegrationConfig;
use iggy::clients::client::IggyClient;
use std::collections::HashMap;
use std::error::Error;

type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

/// A server that handles IMS (Integration Management Service) data processing.
pub struct Server {
    dbg: bool,
    consumer: IggyClient,
    producer: IggyClient,
    iggy_config: IggyConfig,
    integration_config: IntegrationConfig,
    client_configs: Guarded<HashMap<u16, IggyConfig>>,
    client_producers: Guarded<HashMap<u16, IggyClient>>,
}

impl Server {
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

impl Server {
    async fn build(
        dbg: bool,
        integration_config: IntegrationConfig,
        iggy_config: IggyConfig,
    ) -> Result<Self, Box<dyn Error>> {
        if dbg {
            println!("Configure iggy producer")
        }

        let stream_id = integration_config.control_channel();
        let topic_id = integration_config.control_channel();

        if dbg {
            println!("Construct iggy producer")
        }
        let producer = message_shared::utils::build_client(stream_id.clone(), topic_id.clone())
            .await
            .expect("Failed to build client");

        if dbg {
            println!("Configure iggy consumer")
        }

        if dbg {
            println!("Construct iggy consumer")
        }
        let consumer = message_shared::utils::build_client(stream_id.clone(), topic_id.clone())
            .await
            .expect("Failed to build client");

        let client_configs = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));
        let client_producers = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        Ok(Self {
            dbg,
            consumer,
            producer,
            iggy_config,
            integration_config,
            client_configs,
            client_producers,
        })
    }
}

// Getters
impl Server {
    pub fn dbg(&self) -> bool {
        self.dbg
    }

    pub fn consumer(&self) -> &IggyClient {
        &self.consumer
    }

    pub fn producer(&self) -> &IggyClient {
        &self.producer
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

impl Server {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[IMSData/Server]: {msg}");
        }
    }
}
