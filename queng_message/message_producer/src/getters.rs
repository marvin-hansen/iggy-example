use crate::MessageProducer;
use iggy::clients::producer::IggyProducer;
use iggy::identifier::Identifier;

impl MessageProducer {
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

    /// Returns a reference to the `IggyProducer`.
    #[must_use]
    pub const fn producer(&self) -> &IggyProducer {
        &self.producer
    }
}
