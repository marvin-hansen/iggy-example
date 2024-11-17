use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = message_shared::get_ims_data_config();

    let _producer = message_producer::MessageProducer::from_config(&config)
        .await
        .expect("Failed to create producer");

    // Generate some messages

    // Send messages
    // producer.send_one().await
    //     .expect("Failed to send message");

    Ok(())
}
