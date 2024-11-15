use crate::MessageProducer;
use iggy::messages::send_messages::Message;

impl MessageProducer {
    pub async fn send_one(&self, message: Message) {
        self.producer
            .send_one(message)
            .await
            .expect("Failed to send message");
    }

    pub async fn send_batch(&self, messages: Vec<Message>) {
        self.producer
            .send(messages)
            .await
            .expect("Failed to send batch of messages");
    }
}
