use crate::service::Server;
use common_errors::MessageProcessingError;

impl Server {
    /// Checks if a client with the specified ID is logged in.
    ///
    /// This method checks the client producers map to verify if a client with the given ID
    /// has an active producer, indicating they are logged in.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The unique identifier of the client to check
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// * `Ok(true)` if the client is logged in
    /// * `Ok(false)` if the client is not logged in
    ///
    /// # Errors
    ///
    /// Returns a `MessageProcessingError` if:
    /// * Failed to acquire read lock on client producers map
    /// * Lock is poisoned due to a panic in another thread
    pub(crate) async fn check_client_login(
        &self,
        client_id: u16,
    ) -> Result<bool, MessageProcessingError> {
        let client_db = self.client_producers().read().await;

        Ok(client_db.contains_key(&client_id))
    }
}
