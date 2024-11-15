use std::error::Error;
use iggy::clients::consumer::ReceivedMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let consumer_name = "test-message-consumer";
    let config = message_shared::get_ims_data_config();
    let message_handler =message_handler;

    // build consumer
    let mut _consumer =
        message_consumer::MessageConsumer::from_config(&config, consumer_name, message_handler)
            .await
            .expect("Failed to create consumer");

    // Start consumer
    // _consumer.consume_messages().await.expect("Failed to consume messages");

    Ok(())
}

fn message_handler(message: &ReceivedMessage) -> Result<(), Box<dyn Error>> {
    println!("Received message: {:?}", &message.message);
    Ok(())
}
