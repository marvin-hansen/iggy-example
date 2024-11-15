use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ImsDataConfig {
    stream_user: String,
    stream_password: String,
    stream_id: String,
    topic_ids: String,
}

impl ImsDataConfig {
    pub fn new(
        stream_user: String,
        stream_password: String,
        stream_id: String,
        topic_ids: String,
    ) -> Self {
        Self {
            stream_user,
            stream_password,
            stream_id,
            topic_ids,
        }
    }
}

impl ImsDataConfig {
    pub fn stream_user(&self) -> &str {
        &self.stream_user
    }

    pub fn stream_id(&self) -> &str {
        &self.stream_id
    }

    pub fn topic_ids(&self) -> &str {
        &self.topic_ids
    }

    pub fn stream_password(&self) -> &str {
        &self.stream_password
    }
}

impl Display for ImsDataConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImsDataConfig {{ stream_user: {}, stream_password: {}, stream_id: {}, topic_ids: {} }}", self.stream_user, self.stream_password, self.stream_id, self.topic_ids)
    }
}
