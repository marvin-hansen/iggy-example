use crate::service::Server;
use common_errors::MessageProcessingError;
use futures_util::StreamExt;
use std::error::Error;

impl Server {
    pub async fn run(
        self,
        // _signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // // When call .await on a &mut _ reference, then pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        // let signal_future = signal;
        // pin!(signal_future);

        // Keep the write guard alive for the duration of message processing
        let mut consumer_guard = self.consumer().write().await;
        let consumer = consumer_guard.consumer_mut();

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

        self.shutdown()
            .await
            .expect("Failed to stop message service");

        drop(consumer_guard);

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
        // Shutdown consumer
        let consumer = self.consumer().read().await;

        consumer
            .clean_up_and_shutdown()
            .await
            .expect("Failed to cleanup and shutdown consumer");

        // Shutdown producer
        self.producer()
            .clean_up_and_shutdown()
            .await
            .expect("Failed to cleanup and shutdown producer");

        Ok(())
    }
}
