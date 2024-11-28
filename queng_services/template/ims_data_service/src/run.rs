use crate::service::Server;
use common_errors::MessageProcessingError;
use std::future::Future;
use tokio::pin;

impl Server {
    pub async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // When call .await on a &mut _ reference, then pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
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
        self.shutdown()
            .await
            .expect("Failed to shutdown message service");

        Ok(())
    }
}

