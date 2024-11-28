mod config;

use std::error::Error;

const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let service_name: &str = "sample_data_service";
    let ims_data_integration_config = config::ims_data_integration_config();
    let ims_data_iggy_config = config::ims_data_iggy_config();

    ims_data_service::start(
        DBG,
        service_name,
        ims_data_integration_config,
        ims_data_iggy_config,
    )
    .await
    .expect("Failed to start server");

    Ok(())
}
