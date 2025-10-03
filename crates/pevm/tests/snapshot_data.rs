//! Test if snapshotted mainnet data is correct

use alloy_primitives::B256;
use alloy_provider::{network::Ethereum, Provider, RootProvider};
use alloy_rpc_types_eth::BlockNumberOrTag;
use std::{collections::BTreeMap, fs::File, io::BufReader};

#[tokio::test]
async fn snapshotted_mainnet_block_hashes() {
    let file = File::open("../../data/block_hashes.bincode").unwrap();
    let mut reader = BufReader::new(file);
    let block_hashes: BTreeMap<u64, B256> =
        bincode::serde::decode_from_std_read(&mut reader, bincode::config::legacy()).unwrap();

    let rpc_url = match std::env::var("ETHEREUM_RPC_URL") {
        // The empty check is for GitHub Actions where the variable is set with an empty string when unset!?
        Ok(value) if !value.is_empty() => value.parse().unwrap(),
        _ => reqwest::Url::parse("https://eth-mainnet.public.blastapi.io").unwrap(),
    };

    let provider = RootProvider::<Ethereum>::connect(rpc_url.as_str())
        .await
        .unwrap();

    for (block_number, snapshotted_hash) in block_hashes {
        let block = provider
            .get_block_by_number(BlockNumberOrTag::Number(block_number))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(snapshotted_hash, B256::from(block.header.hash));
    }
}
