use subxt::Config;
use subxt::SubstrateConfig;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct EduchainConfig;

impl subxt::config::Config for EduchainConfig {
    type Index = u32;
    type Hash = subxt::utils::H256;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type AccountId = subxt::utils::AccountId32;
    type Address = subxt::utils::MultiAddress<Self::AccountId, Self::Index>;
    type Header = subxt::config::substrate::SubstrateHeader<BlockNumber, Self::Hasher>;
    type Signature = subxt::utils::MultiSignature;
    type ExtrinsicParams = <SubstrateConfig as Config>::ExtrinsicParams;
}

pub type BlockNumber = u32;
pub type Balance = u128;
