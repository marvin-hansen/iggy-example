use crate::service::Server;
use common_errors::MessageProcessingError;
use iggy::models::messages::PolledMessage;
use sbe_messages::{ClientLoginMessage, ClientLogoutMessage, MessageType};

impl Server {
    /// Handles a single message by processing it and sending it to the appropriate
    /// manager for further processing.
    ///
    /// This method takes a message payload, processes it by calling the `process_message`
    /// method, and sends it to the appropriate handler for further processing.
    ///
    /// # Parameters
    ///
    /// * `self` - The Server instance
    /// * `message` - The message payload to be processed
    ///
    /// # Returns
    /// * Ok on success,
    /// * Err on any processing error
    ///
    pub(crate) async fn handle_message(
        &self,
        polled_message: PolledMessage,
    ) -> Result<(), MessageProcessingError> {
        //
        let message = polled_message.payload.to_vec();
        let raw_message = message.as_slice();
        let message_type = MessageType::from(u16::from(raw_message[2]));

        match message_type {
            MessageType::ClientLogin => {
                let client_login_msg = ClientLoginMessage::from(raw_message);
                self.handle_client_login(&client_login_msg).await
            }
            MessageType::ClientLogout => {
                let client_logout_msg = ClientLogoutMessage::from(raw_message);
                self.handle_client_logout(&client_logout_msg).await
            }
            MessageType::StartData => {
                todo!()
            }

            MessageType::StopData => {
                todo!()
            }

            MessageType::StopAllData => {
                todo!()
            }

            _ => Err(MessageProcessingError(
                "[handle::handle_message]: Unknown message type. Abort processing".to_string(),
            )),
        }
    }
}
