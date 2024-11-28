use common_errors::MessageProcessingError;
use message_shared::SendMessage;
use sbe_messages::ClientErrorType;

use crate::service::Server;

impl Server {
    /// Sends a `ClientError` message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `client_id` - The id of the client the error is for
    /// * `client_error` - The `ClientErrorType` to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_client_error(
        &self,
        client_id: u16,
        client_error: ClientErrorType,
    ) -> Result<(), MessageProcessingError> {
        // Encode message as SBE binary
        let message = sbe_utils::encode_client_error(client_id, client_error)
            .expect("Failed to encode client error message");

        // Send message
        self.send_error(message)
            .await
            .expect("Failed to send client error message");

        Ok(())
    }

    // /// Sends a `DataError` message to the given producer.
    // ///
    // /// # Parameters
    // ///
    // /// * `producer` - The topic producer to send the message on
    // /// * `client_id` - The id of the client the error is for
    // /// * `data_error` - The `DataErrorType` to send
    // ///
    // /// # Returns
    // ///
    // /// Returns a `Result` with `()` if successful, otherwise returns a
    // /// `MessageProcessingError` on failure to send.
    // ///
    // pub(crate) async fn send_data_error(
    //     &self,
    //     client_id: u16,
    //     data_error: DataErrorType,
    // ) -> Result<(), MessageProcessingError> {
    //     // Encode message as SBE binary
    //     let message = sbe_utils::encode_data_error(client_id, data_error)
    //         .expect("Failed to encode data error message");
    //
    //     // Send message
    //     self.send_error(message)
    //         .await
    //         .expect("Failed to send error message");
    //
    //     Ok(())
    // }

    pub(crate) async fn send_error(&self, bytes: Vec<u8>) -> Result<(), MessageProcessingError> {
        // Send message
        self.producer()
            .send_one_message(bytes)
            .await
            .expect("Failed to send error message");

        Ok(())
    }
}
