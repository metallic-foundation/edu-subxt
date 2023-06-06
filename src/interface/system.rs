use super::RequestResult;
use crate::chain::extend::ExtendStorageClient;
use crate::chain::{
    config::BlockNumber, metadata::edu_chain, AccountId, EduchainOnlineClient, Hash,
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
        account: &AccountId,
        at: Option<Hash>,
    ) -> RequestResult<AccountInfo>;
    async fn latest_finalized_block(&self) -> RequestResult<BlockNumber>;
}

#[async_trait::async_trait]
impl SystemCalls for EduchainOnlineClient {
    async fn get_account_info(
        &self,
        account: &AccountId,
        at: Option<Hash>,
    ) -> RequestResult<AccountInfo> {
        let account_info_key = edu_chain::storage().system().account(account);
        self.storage()
            .at_or_latest(at)
            .await?
            .fetch(&account_info_key)
            .await
    }

    async fn latest_finalized_block(&self) -> RequestResult<BlockNumber> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::balances::BalancesCall;
    use crate::interface::PairSigner;
    use crate::testing::*;
    use sp_keyring::AccountKeyring;

    #[tokio::test]
    async fn fetch_account_info() {
        let api = make_local_client().await;

        let alice = PairSigner::new(AccountKeyring::Alice.pair());
        let bob = PairSigner::new(AccountKeyring::Bob.pair());

        let alice_info = api
            .get_account_info(alice.account_id(), None)
            .await
            .expect("Fetch error")
            .expect("Alice does not exists");
        assert_eq!(alice_info.data.free, 1_152_921_504_606_846_976);

        let bob_info = api
            .get_account_info(bob.account_id(), None)
            .await
            .expect("Fetch error")
            .expect("Bob does not exists");
        assert_eq!(bob_info.data.free, 1_152_921_504_606_846_976);
    }
}
