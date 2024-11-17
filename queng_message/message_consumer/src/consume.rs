use crate::MessageConsumer;
use futures_util::StreamExt;
use std::error::Error;

impl MessageConsumer {
    /// Consume messages from the topic.
    ///
    /// This function will loop indefinitely consuming messages from the topic and
    /// passing them to the `message_handler` function. If the `message_handler`
    /// function returns an error, the error will be logged and the loop will
    /// continue. If the consumer errors while consuming messages, the error will
    /// be logged and the loop will continue.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with an error if the consumer errors while consuming
    /// messages.
    pub async fn consume_messages(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(message) = self.consumer.next().await {
            if let Ok(message) = message {
                (self.message_handler)(&message).expect("Failed to handle message");
            } else if let Err(error) = message {
                eprintln!("Error while handling message: {error}");
                continue;
            }
        }

        Ok(())
    }
}
