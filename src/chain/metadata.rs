#[cfg_attr(
    not(all(feature = "saved-metadata", not(feature = "live-metadata"))),
    cfg(target_family = "never")
)]
#[::subxt::subxt(runtime_metadata_path = "res/edu_chain_metadata.scale")]
pub mod edu_chain {}

#[cfg_attr(
    not(all(feature = "live-metadata", not(feature = "saved-metadata"))),
    cfg(target_family = "never")
)]
#[::subxt::subxt(runtime_metadata_url = "ws://127.0.0.1:9944")]
pub mod edu_chain {}
