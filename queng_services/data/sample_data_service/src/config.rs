use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use common_message::StreamUser;

pub fn ims_data_integration_config() -> IntegrationConfig {
    IntegrationConfig::new(
        "sample-ims-data".to_string(),
        1,
        ImsIntegrationType::Data,
        IntegrationMessageConfig::new(1, 1),
    )
}

pub fn stream_user() -> StreamUser {
    StreamUser::new("sample-data-user", "secret")
}
