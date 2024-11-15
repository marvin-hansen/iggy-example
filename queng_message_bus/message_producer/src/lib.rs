mod send;
mod utils;

use iggy::client::Client;
use iggy::client_provider;
use iggy::client_provider::ClientProviderConfig;
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::messages::send_messages::Partitioning;
use iggy::utils::duration::IggyDuration;
use message_shared::Args;
use std::str::FromStr;
use std::sync::Arc;

pub struct MessageProducer {
    producer: IggyProducer,
}

impl MessageProducer {
    pub async fn default() -> Result<Self, IggyError> {
        Self::build(Args::default()).await
    }
}

impl MessageProducer {
    async fn build(args: Args) -> Result<Self, IggyError> {
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
            .build();

        producer.init().await.expect("Failed to init producer");

        Ok(MessageProducer { producer })
    }
}

impl MessageProducer {
    pub async fn shutdown() {}
}
