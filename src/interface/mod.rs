pub mod balances;
pub mod common;
pub mod error;
pub mod intake;
pub mod system;
pub mod university;
use crate::chain::config::EduchainConfig;
use crate::chain::EduchainOnlineClient;
use subxt::tx::TxInBlock;
use subxt::ext::sp_core as subxt_sp_core;

/// general type to be returned for (most) storage fetch request
pub type RequestResult<T> = Result<Option<T>, subxt::Error>;
/// general type to be returned for (most) extrinsic calls made
pub type ExtrinsicResult = Result<TxInBlock<EduchainConfig, EduchainOnlineClient>, subxt::Error>;

/// Block number used in chains
pub type BlockNumber = crate::chain::config::BlockNumber;

/// Signer
pub type PairSigner<Pair> = subxt::tx::PairSigner<crate::chain::config::EduchainConfig, Pair>;
pub type EcdsaPairSigner = PairSigner<subxt_sp_core::ecdsa::Pair>;
pub type Sr25519PairSigner = PairSigner<subxt_sp_core::sr25519::Pair>;
pub type Ed25519PairSigner = PairSigner<subxt_sp_core::ed25519::Pair>;
pub type DummyPairSigner = PairSigner<subxt_sp_core::crypto::Dummy>;
/// Signer used currently in Config
pub type CurrentPairSigner = Sr25519PairSigner;
