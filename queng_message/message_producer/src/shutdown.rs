use crate::MessageProducer;
use iggy::client::{Client, StreamClient, TopicClient, UserClient};
use iggy::error::IggyError;

impl MessageProducer {
    /// Cleans up the stream, topic, and user created by this `MessageProducer`.
    ///
    /// # Errors
    ///
    /// Returns an `IggyError` if the stream, topic, user deletion, or client shutdown fails.
    ///
    pub async fn clean_up(&self) -> Result<(), IggyError> {
        // Connect client
        self.client.connect().await.expect("Failed to connect");

        // Delete the topic
        self.client
            .delete_topic(&self.stream_id, &self.topic_id)
            .await
            .expect("Failed to delete topic");

        // Delete the stream
        self.client
            .delete_stream(&self.stream_id)
            .await
            .expect("Failed to delete stream");

        // Delete the user
        self.client
            .delete_user(&self.user_id)
            .await
            .expect("Failed to delete user");

        Ok(())
    }

    /// Shuts down the underlying client.
    ///
    /// # Errors
    ///
    /// Returns an `IggyError` if the client shutdown fails.
    ///
    pub async fn shutdown(&self) -> Result<(), IggyError> {
        // Connect client
        self.client.connect().await.expect("Failed to connect");

        // Shutdown
        self.client.shutdown().await
    }

    /// Cleans up the stream, topic, and user created by this `MessageProducer` and shuts down the underlying client.
    ///
    /// # Errors
    ///
    /// Returns an `IggyError` if the stream, topic, user deletion, or client shutdown fails.
    ///
    pub async fn clean_up_and_shutdown(&self) -> Result<(), IggyError> {
        // Clean up
        self.clean_up().await.expect("Failed to clean up");

        // Shutdown
        self.client.shutdown().await.expect("Failed to shutdown");

        Ok(())
    }
}
