use iggy::client::Client;
use iggy::client_provider;
use iggy::client_provider::ClientProviderConfig;
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::messages::send_messages::{Message, Partitioning};
use iggy::utils::duration::IggyDuration;
use iggy::utils::expiry::IggyExpiry;
use iggy::utils::topic_size::MaxTopicSize;
use message_shared::Args;
use std::str::FromStr;
use std::sync::Arc;


pub struct MessageProducer {
    producer: IggyProducer,
}

impl MessageProducer {
    pub async fn default() -> Self {
        Self::build(Args::default()).await
    }
}

impl MessageProducer {
    async fn build(args: Args) -> Self {
        let client_provider_config = Arc::new(
            ClientProviderConfig::from_args(args.to_sdk_args())
                .expect("Failed to create client provider config"),
        );

        let client = client_provider::get_raw_client(client_provider_config, false)
            .await
            .expect("Failed to create client");

        let client = IggyClient::builder()
            .with_client(client)
            .build()
            .expect("Failed to create client");

        client.connect().await.expect("Failed to connect");

        let mut producer = client
            .producer(&args.stream_id, &args.topic_id)
            .expect("Failed to create producer")
            .batch_size(args.messages_per_batch)
            .send_interval(IggyDuration::from_str(&args.interval).expect("Invalid interval format"))
            .partitioning(Partitioning::balanced())

            .create_topic_if_not_exists(
                3,
                None,
                IggyExpiry::ServerDefault,
                MaxTopicSize::ServerDefault,
            )
            .build();

        producer.init().await.expect("Failed to init producer");

        MessageProducer { producer }
    }
}

impl MessageProducer {
    pub async fn send_one(&mut self, message: Message) {
        self.producer
            .send_one(message)
            .await
            .expect("Failed to send message");
    }

    pub async fn send_batch(&mut self, messages: Vec<Message>) {
        self.producer
            .send(messages)
            .await
            .expect("Failed to send batch of messages");
    }
}
