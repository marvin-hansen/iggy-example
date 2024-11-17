use crate::MessageProducer;
use iggy::error::IggyError;
use iggy::messages::send_messages::Message;

impl MessageProducer {
    pub async fn send_one(&self, message: Message) -> Result<(), IggyError> {
        self.producer.send_one(message).await
    }

    pub async fn send_batch(&self, messages: Vec<Message>) -> Result<(), IggyError> {
        self.producer.send(messages).await
    }
}
