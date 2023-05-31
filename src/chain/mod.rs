pub mod config;
/// extend functionality of foreign types
pub mod extend;
pub mod metadata;

use config::EduchainConfig;

pub type EduchainOnlineClient = subxt::OnlineClient<EduchainConfig>;
pub type EduchainBlocksClient = subxt::blocks::BlocksClient<EduchainConfig, EduchainOnlineClient>;
pub type EduchainStorageClient =
    subxt::storage::StorageClient<EduchainConfig, EduchainOnlineClient>;
pub type EduchainRpcClient = subxt::rpc::RpcClient;

pub type AccountId = <EduchainConfig as subxt::Config>::AccountId;
pub type Balance = config::Balance;
pub type Index = <EduchainConfig as subxt::Config>::Index;
pub type Hash = <EduchainConfig as subxt::Config>::Hash;
