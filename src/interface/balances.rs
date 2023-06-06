use subxt::tx::TxPayload;

use super::CurrentPairSigner;
use super::ExtrinsicResult;
use crate::chain::metadata::edu_chain;
use crate::chain::metadata::edu_chain::runtime_types;
use crate::chain::AccountId;
use crate::chain::Balance;
use crate::chain::EduchainOnlineClient;

pub type AccountData = runtime_types::pallet_balances::AccountData<Balance>;

#[async_trait::async_trait]
pub trait BalancesCall {
    async fn transfer_balance(
        &self,
        from: &CurrentPairSigner,
        to: AccountId,
        amount: Balance,
    ) -> ExtrinsicResult;
}

#[async_trait::async_trait]
impl BalancesCall for EduchainOnlineClient {
    async fn transfer_balance(
        &self,
        from: &CurrentPairSigner,
        to: AccountId,
        amount: Balance,
    ) -> ExtrinsicResult {
        let tx = edu_chain::tx().balances().transfer(to.into(), amount);
        println!("{:?}", tx);
        self.tx()
            .sign_and_submit_then_watch_default(&tx, from)
            .await?
            .wait_for_in_block()
            .await
    }
}
