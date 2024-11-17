use crate::error::SendMessageError;

// async fn in traits.
// https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html
#[allow(dead_code)]
#[trait_variant::make(SendMessage: Send)]
pub trait LocalSendMessage {
    /// Send a single byte message.
    ///
    /// The message is provided as a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent.
    async fn send_one_message(&self, bytes: Vec<u8>) -> Result<(), SendMessageError>;
    /// Send a batch of byte messages.
    ///
    /// The messages are provided as a `Vec` of `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the messages cannot be sent.
    async fn send_batch_messages(&self, bytes_batch: &[Vec<u8>]) -> Result<(), SendMessageError>;
}
