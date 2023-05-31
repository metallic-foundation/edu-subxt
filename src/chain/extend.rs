use subxt::client::OnlineClientT;
use subxt::config::Config;
use subxt::error::Error;
use subxt::storage::Storage;
use subxt::storage::StorageClient;

#[async_trait::async_trait]
pub trait ExtendStorageClient<T, Client>
where
    T: Config,
{
    /// Get storage at given block hash or at latest if None
    async fn at_or_latest(&self, at: Option<T::Hash>) -> Result<Storage<T, Client>, Error>;
}

#[async_trait::async_trait]
impl<T, Client> ExtendStorageClient<T, Client> for StorageClient<T, Client>
where
    T: Config + Sync,
    Client: OnlineClientT<T>,
{
    async fn at_or_latest(&self, at: Option<T::Hash>) -> Result<Storage<T, Client>, Error> {
        match at {
            Some(hash) => Ok(self.at(hash)),
            None => self.at_latest().await,
        }
    }
}
