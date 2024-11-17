use crate::MessageProducer;
use bytes::Bytes;
use iggy::bytes_serializable::BytesSerializable;
use iggy::messages::send_messages::Message;
use message_shared::{SendMessage, SendMessageError};

impl SendMessage for MessageProducer {
    async fn send_one(&self, bytes: Vec<u8>) -> Result<(), SendMessageError> {
        // Convert a single byte blob into a message
        let message = Message::from_bytes(Bytes::from(bytes)).expect("Failed to create message");

        // Send the message
        match self.producer.send_one(message).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SendMessageError {
                message: e.to_string(),
            }),
        }
    }

    async fn send_batch(&self, bytes_batch: Vec<Vec<u8>>) -> Result<(), SendMessageError> {
        // Convert a byte array into a message batch
        let messages: Vec<Message> = bytes_batch
            .into_iter()
            .map(|bytes| Message::from_bytes(Bytes::from(bytes)).expect("Failed to create message"))
            .collect();

        // Send the message batch
        match self.producer.send(messages).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SendMessageError {
                message: e.to_string(),
            }),
        }
    }
}
