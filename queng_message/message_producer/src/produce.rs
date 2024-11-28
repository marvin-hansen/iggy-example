use crate::MessageProducer;
use bytes::Bytes;
use iggy::messages::send_messages::Message;
use message_shared::{SendMessage, SendMessageError};

impl SendMessage for MessageProducer {
    /// Send a single byte message.
    ///
    /// The message is provided as a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent.
    ///
    async fn send_one_message(&self, bytes: Vec<u8>) -> Result<(), SendMessageError> {
        // SBE messages serialize into Vec<u8> bytes.
        // Most SBE messages are smaller than 24 bytes and therefore
        // cannot be casted directly into an iggy messages.

        // Convert the SBE bytes into a new message with auto-generated ID, payload and no headers.
        // The SBE headers from the paylod are used instead.
        let message = Message::new(None, Bytes::from(bytes), None);

        // Send the message
        match self.producer.send_one(message).await {
            Ok(()) => Ok(()),
            Err(e) => Err(SendMessageError {
                message: e.to_string(),
            }),
        }
    }

    /// Send a collection of byte messages.
    ///
    /// The messages are provided as a `Vec` of `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the messages cannot be sent.
    ///
    async fn send_batch_messages(&self, bytes_batch: &[Vec<u8>]) -> Result<(), SendMessageError> {
        // SBE messages serialize into Vec<u8> bytes; hence a vector of Vec<u8>
        // represents a collection of SBE messages

        // Convert a byte array into a vector of messages
        let messages: Vec<Message> = bytes_batch
            .iter()
            // Convert the SBE bytes into a new message with auto-generated ID, payload, and no headers.
            .map(|bytes| Message::new(None, Bytes::from(bytes.to_owned()), None))
            .collect();

        // Send the message batch
        match self.producer.send(messages).await {
            Ok(()) => Ok(()),
            Err(e) => Err(SendMessageError {
                message: e.to_string(),
            }),
        }
    }
}
