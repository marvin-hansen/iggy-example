use crate::service::Service;
use common_errors::MessageProcessingError;
use iggy::client::Client;

impl Service {
    pub(crate) async fn shutdown(&self) -> Result<(), MessageProcessingError> {
        self.dbg_print("Shutting down");

        self.dbg_print("Shutting down consumer");
        self.consumer().shutdown().await.expect("Failed to shutdown");

        // Check if there is any active client left, and if so, logout and shutdown

        self.dbg_print("Shutting down producer");
        self.producer().shutdown().await.expect("Failed to shutdown");
        Ok(())
    }
}
