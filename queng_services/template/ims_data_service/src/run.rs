use crate::service::Server;
use common_errors::MessageProcessingError;
use std::future::Future;
use tokio::{pin, select};

impl Server {
    pub async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // When call .await on a &mut _ reference, then pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        loop {
            select! {
                    _ = &mut signal_future => {break;}
                //
                // polled_messages = self.consumer().pi(self.poll_command()) => {
                //     match polled_messages {
                //         Ok(polled_messages) => {
                //             for polled_message in polled_messages.messages {
                //                 self.handle_message(polled_message.payload.as_ref())
                //                    .await.expect("Failed to process message");
                //             }
                //         },
                //         Err(e) => {
                //             println!("[QDGW/run]: Error polling messages from iggy message bus: {}", e);
                //             break;
                //         }
                //     }
                // } // end match polled messages
            } // end select
        } // end loop

        self.shutdown().await.expect("Failed to shutdown iggy");

        Ok(())
    }
}

impl Server {
    async fn shutdown(&self) -> Result<(), MessageProcessingError> {
        Ok(())
    }
}
