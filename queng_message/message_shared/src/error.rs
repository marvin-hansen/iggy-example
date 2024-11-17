use std::fmt::Debug;

#[derive(Debug)]
pub struct SendMessageError {
    pub message: String,
}

impl std::fmt::Display for SendMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SendMessageError: {}", self.message)
    }
}

impl std::error::Error for SendMessageError {}
