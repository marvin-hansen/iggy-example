use crate::error::SendMessageError;

// https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html
#[allow(dead_code)]
#[trait_variant::make(SendMessage: Send)]
pub trait LocalSendMessage {
    async fn send_one(&self, bytes: Vec<u8>) -> Result<(), SendMessageError>;
    async fn send_batch(&self, bytes_batch: Vec<Vec<u8>>) -> Result<(), SendMessageError>;
}
