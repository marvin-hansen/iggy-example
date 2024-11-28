use crate::messages::client_login::ClientLoginMessage;
use crate::MessageType;

impl ClientLoginMessage {
    #[must_use]
    pub const fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    #[must_use]
    pub const fn client_id(&self) -> u16 {
        self.client_id
    }
}
