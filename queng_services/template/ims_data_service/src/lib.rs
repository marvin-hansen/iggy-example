use crate::service::Server;
use common_ims::IntegrationConfig;
use common_message::StreamUser;
use common_service::print_utils;
use tokio::time::Instant;

mod handle;
mod run;
mod service;
mod utils;

pub async fn start(
    dbg: bool,
    service_name: &str,
    service_addr: &str,
    integration_config: IntegrationConfig,
    stream_user: StreamUser,
) -> Result<(), Box<dyn std::error::Error>> {
    let dbg_print = |msg: &str| {
        if dbg {
            println!("[{service_name}]: {msg}");
        }
    };
    let start = Instant::now();


    dbg_print("Configuring server and signal handler");
    //Creates a new server
    let server = if dbg{
        Server::with_debug(integration_config, stream_user)
            .await
            .expect("Failed to build new service")

     } else {
        Server::new(integration_config, stream_user)
            .await
            .expect("Failed to build new service")
    };

    // let signal = shutdown_utils::signal_handler("gRPC server");

    dbg_print("Run service");
    let service_handle = tokio::spawn(server.run());

    dbg_print("Set integration online");

    // Print service start header
    print_utils::print_duration("Starting service took:", &start.elapsed());
    print_utils::print_start_header_simple(service_name, service_addr);

    //Start server.
    match tokio::try_join!(service_handle) {
        Ok(_) => {}
        Err(e) => {
            println!("[{service_name}]/main: Failed to start Message service: {e:?}");
        }
    }
    //
    dbg_print("Set integration offline");

    Ok(())
}
