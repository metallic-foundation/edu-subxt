use crate::chain::EduchainOnlineClient;

/// create a new local client that connects to 127.0.0.1:9944
pub async fn make_local_client() -> EduchainOnlineClient {
    EduchainOnlineClient::new()
        .await
        .expect("Cannot create online client")
}
