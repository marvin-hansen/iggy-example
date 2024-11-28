use common_iggy::{IggyConfig, IggyUser};
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};

pub fn ims_data_integration_config() -> IntegrationConfig {
    IntegrationConfig::new(
        "sample-ims-data".to_string(),
        1,
        ImsIntegrationType::Data,
        IntegrationMessageConfig::new(1, 1),
    )
}

pub fn ims_data_iggy_config() -> IggyConfig {
    IggyConfig::new(IggyUser::default(), "127.0.0.1:8090", 1, 1, 1, 1, true)
}
