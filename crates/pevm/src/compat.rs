// TODO: Support custom chains like OP & RISE
// Ideally REVM & Alloy would provide all these.

use alloy_rpc_types_eth::Header;
use revm::context::BlockEnv;
use revm::context_interface::block::BlobExcessGasAndPrice;
use revm::primitives::{
    eip4844::{BLOB_BASE_FEE_UPDATE_FRACTION_CANCUN, BLOB_BASE_FEE_UPDATE_FRACTION_PRAGUE},
    hardfork::SpecId,
    U256,
};

/// Get the REVM block env of an Alloy block.
// https://github.com/paradigmxyz/reth/blob/280aaaedc4699c14a5b6e88f25d929fe22642fa3/crates/primitives/src/revm/env.rs#L23-L48
// TODO: Better error handling & properly test this, especially
// [blob_excess_gas_and_price].
pub(crate) fn get_block_env(header: &Header, spec_id: SpecId) -> BlockEnv {
    BlockEnv {
        number: U256::from(header.number),
        beneficiary: header.beneficiary,
        timestamp: U256::from(header.timestamp),
        gas_limit: header.gas_limit,
        basefee: header.base_fee_per_gas.unwrap_or_default(),
        difficulty: header.difficulty,
        prevrandao: Some(header.mix_hash),
        blob_excess_gas_and_price: header.excess_blob_gas.map(|excess_blob_gas| {
            let base_fee_update_fraction = if spec_id.is_enabled_in(SpecId::OSAKA) {
                BLOB_BASE_FEE_UPDATE_FRACTION_PRAGUE
            } else {
                BLOB_BASE_FEE_UPDATE_FRACTION_CANCUN
            };
            BlobExcessGasAndPrice::new(excess_blob_gas, base_fee_update_fraction)
        }),
    }
}
