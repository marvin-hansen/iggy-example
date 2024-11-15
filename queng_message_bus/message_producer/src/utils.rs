use ahash::AHashMap;

use iggy::client::{StreamClient, UserClient};
use iggy::clients::client::IggyClient;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::models::permissions::{Permissions, StreamPermissions};
use iggy::models::user_status::UserStatus;

async fn create_stream_and_user(
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

async fn ensure_stream_access(
    client: &IggyClient,
    available_stream: &str,
    unavailable_streams: &[&str],
) -> Result<(), IggyError> {
    client
        .get_stream(&available_stream.try_into()?)
        .await?
        .unwrap_or_else(|| panic!("No access to stream: {available_stream}"));
    for stream in unavailable_streams {
        if client
            .get_stream(&Identifier::named(stream)?)
            .await?
            .is_none()
        {
        } else {
            panic!("Access to stream: {stream} should not be allowed");
        }
    }
    Ok(())
}
