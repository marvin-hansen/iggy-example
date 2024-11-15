use ahash::AHashMap;

use iggy::client::{StreamClient, UserClient};
use iggy::clients::client::IggyClient;
use iggy::error::IggyError;
use iggy::models::permissions::{Permissions, StreamPermissions};
use iggy::models::user_status::UserStatus;

pub(crate) async fn create_stream_and_user(
    stream_name: &str,
    username: &str,
    userpassword: &str,
    client: &IggyClient,
) -> Result<(), IggyError> {
    let stream = client.create_stream(stream_name, None).await?;
    let mut streams_permissions = AHashMap::new();

    streams_permissions.insert(
        stream.id,
        StreamPermissions {
            read_stream: true,
            manage_topics: true,
            ..Default::default()
        },
    );

    let permissions = Permissions {
        streams: Some(streams_permissions),
        ..Default::default()
    };

    client
        .create_user(
            username,
            userpassword,
            UserStatus::Active,
            Some(permissions),
        )
        .await
        .expect("Failed to create user");

    Ok(())
}
