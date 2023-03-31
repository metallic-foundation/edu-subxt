pub mod config;
pub mod metadata;

use config::DatahighwayConfig;

pub type DatahighwayOnlineClient = subxt::OnlineClient<DatahighwayConfig>;
pub type DatahighwayBlocksClient =
    subxt::blocks::BlocksClient<DatahighwayConfig, DatahighwayOnlineClient>;
pub type DatahighwayStorageClient =
    subxt::storage::StorageClient<DatahighwayConfig, DatahighwayOnlineClient>;
pub type DatahighwayRpcClient = subxt::rpc::RpcClient;

pub type AccountId = <DatahighwayConfig as subxt::Config>::AccountId;
pub type Balance = config::Balance;
pub type Index = <DatahighwayConfig as subxt::Config>::Index;
pub type Hash = <DatahighwayConfig as subxt::Config>::Hash;
