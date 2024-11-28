use crate::service::Server;
use common_errors::MessageProcessingError;
use std::error::Error;
use std::future::Future;
use tokio::pin;

impl Server {
    pub async fn run(
        self,
        signal: impl Future<Output=()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // // When call .await on a &mut _ reference, then pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        let mut consumer_guard = self.consumer().write().await;
        let consumer = consumer_guard.consumer_mut();

        tokio::select! {
                    // Wait for a signal that requests a graceful shutdown.
                    // Then break the loop and proceed with the shutdown.
                    _ = &mut signal_future => {break;}

            // Otherwise process messages.
            while let Some(message) = consumer.next().await {
                if let Ok(message) = message {
                    self.handle_message(message.message)
                        .await
                        .expect("Failed to handle message");
                } else if let Err(error) = message {
                    eprintln!("Error while handling message: {error}");
                    continue;
                }
            }
        }

        drop(consumer_guard);
        self.shutdown().await.expect("Failed to shutdown message service");

        Ok(())
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

        self.dbg_print("Cleaning up and shutting down consumer");
        let consumer = self.consumer().read().await;
        consumer
            .clean_up_and_shutdown()
            .await
            .expect("Failed to cleanup and shutdown consumer");

        self.dbg_print("Cleaning up and shutting down producer");
        self.producer()
            .clean_up_and_shutdown()
            .await
            .expect("Failed to cleanup and shutdown producer");

        // check if there are any remaining client producers; if so, shut them down.
        let client_producers = self.client_producers();
        if !client_producers.read().await.is_empty() {
            self.dbg_print("Cleaning up and shutting down client producers");
            for (client_id, producer) in client_producers.read().await.iter() {
                self.dbg_print("Cleaning up and shutting down client producer");
                self.dbg_print(&format!("Client ID: {}", client_id));
                producer
                    .clean_up_and_shutdown()
                    .await
                    .expect("Failed to cleanup and shutdown client producer");
            }
        }

        Ok(())
    }
}
