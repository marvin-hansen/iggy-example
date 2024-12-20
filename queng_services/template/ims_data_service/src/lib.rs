use crate::service::Service;
use common_iggy::IggyConfig;
use common_ims::IntegrationConfig;
use common_service::{print_utils, shutdown_utils};
use tokio::time::Instant;

// mod handle;
mod run;
mod service;
mod shutdown;
// mod utils;

pub async fn start(
    dbg: bool,
    service_name: &str,
    integration_config: IntegrationConfig,
    iggy_config: IggyConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let dbg_print = |msg: &str| {
        if dbg {
            println!("[{service_name}]: {msg}");
        }
    };
    let start = Instant::now();
    let stream_id = integration_config.control_channel();

    dbg_print("Configuring server");
    //Creates a new server
    let server = if dbg {
        Service::with_debug(integration_config, iggy_config)
            .await
            .expect("Failed to build new service")
    } else {
        Service::new(integration_config, iggy_config)
            .await
            .expect("Failed to build new service")
    };

    dbg_print("Run service");
    let signal = shutdown_utils::signal_handler("message server signal handler");
    let service_handle = tokio::spawn(server.run(signal));

    dbg_print("Set integration online");
    //

    // Print service start header
    print_utils::print_duration("Starting service took:", &start.elapsed());
    print_utils::print_start_header_message_service(service_name, &stream_id);

    //Start server.
    match tokio::try_join!(service_handle) {
        Ok(_) => {}
        Err(e) => {
            println!("[{service_name}]/main: Failed to start Message service: {e:?}");
        }
    }
    //
    dbg_print("Set integration offline");
    //

    Ok(())
}
