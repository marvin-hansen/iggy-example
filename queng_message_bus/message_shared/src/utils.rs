use crate::Args;
use iggy::client_provider;
use iggy::client_provider::ClientProviderConfig;
use iggy::clients::client::IggyClient;
use iggy::error::IggyError;
use std::sync::Arc;

use ahash::AHashMap;

use iggy::client::{StreamClient, UserClient};
use iggy::models::permissions::{Permissions, StreamPermissions};
use iggy::models::user_status::UserStatus;

/// Builds an Iggy client using the provided `Args`.
///
/// # Arguments
///
/// * `args` - The `Args` to use to build the client.
///
/// # Returns
///
/// A `Result` wrapping the `IggyClient` instance or an `IggyError`.
///
pub async fn build_client(args: &Args) -> Result<IggyClient, IggyError> {
    // Build client provider configuration
    let client_provider_config = Arc::new(
        ClientProviderConfig::from_args(args.to_sdk_args())
            .expect("Failed to create client provider config"),
    );

    // Build client_provider
    let client = client_provider::get_raw_client(client_provider_config, false)
        .await
        .expect("Failed to create client");

    // Build client
    let client = IggyClient::builder()
        .with_client(client)
        .build()
        .expect("Failed to create client");

    Ok(client)
}

/// Creates a stream and a user in the Iggy cluster.
///
/// # Arguments
///
/// * `stream_name` - The name of the stream to create.
/// * `username` - The username of the user to create.
/// * `userpassword` - The password of the user to create.
/// * `client` - The client to use to create the stream and user.
///
/// # Returns
///
/// A `Result` with an `IggyError` if the stream or user creation fails.
///
pub async fn create_stream_and_user(
    stream_name: &str,
    username: &str,
    userpassword: &str,
    client: &IggyClient,
) -> Result<(), IggyError> {
    let stream = client
        .create_stream(stream_name, None)
        .await
        .expect("Failed to create stream");
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
