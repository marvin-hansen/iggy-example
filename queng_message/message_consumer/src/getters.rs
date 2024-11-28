use crate::MessageConsumer;
use iggy::clients::consumer::IggyConsumer;
use iggy::identifier::Identifier;

impl MessageConsumer {
    /// Returns a reference to the user identifier.
    #[must_use]
    pub const fn user_id(&self) -> &Identifier {
        &self.user_id
    }

    /// Returns a reference to the stream identifier.
    #[must_use]
    pub const fn stream_id(&self) -> &Identifier {
        &self.stream_id
    }

    /// Returns a reference to the topic identifier.
    #[must_use]
    pub const fn topic_id(&self) -> &Identifier {
        &self.topic_id
    }

    /// Returns a reference to the underlying consumer.
    #[must_use]
    pub const fn consumer(&self) -> &IggyConsumer {
        &self.consumer
    }

    /// Returns a mutable reference to the underlying consumer.
    #[must_use]
    pub fn consumer_mut(&mut self) -> &mut IggyConsumer {
        &mut self.consumer
    }
}
