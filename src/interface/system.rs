use super::RequestResult;
use crate::chain::{
    config::BlockNumber,
    metadata::edu_chain::{self},
    AccountId, EduchainOnlineClient, Hash,
};

pub type AccountInfo =
    crate::interface::system::edu_chain::runtime_types::frame_system::AccountInfo<
        crate::chain::Index,
        super::balances::AccountData,
    >;

#[async_trait::async_trait]
pub trait SystemCalls {
    async fn get_account_info(
        &self,
        account: AccountId,
        at: Option<Hash>,
    ) -> RequestResult<AccountInfo>;
    async fn latest_finalized_block(&self) -> RequestResult<BlockNumber>;
}

#[async_trait::async_trait]
impl SystemCalls for EduchainOnlineClient {
    async fn get_account_info(
        &self,
        account: AccountId,
        at: Option<Hash>,
    ) -> RequestResult<AccountInfo> {
        let account_info_key = edu_chain::storage().system().account(&account);
        self.storage().at(at).await?.fetch(&account_info_key).await
    }

    async fn latest_finalized_block(&self) -> RequestResult<BlockNumber> {
        todo!()
    }
}
