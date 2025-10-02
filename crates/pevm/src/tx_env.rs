use alloy_eip2930::AccessList;
use alloy_eip7702::SignedAuthorization;
use alloy_primitives::{Address, Bytes, B256, U256};
use revm::context::TxEnv as RevmTxEnv;
#[cfg(feature = "optimism")]
use revm::primitives::OptimismFields;
use revm::primitives::{eip4844::GAS_PER_BLOB, TxKind};

/// Destination selector for an Ethereum transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactTo {
    /// Call an existing account.
    Call(Address),
    /// Create a new contract.
    Create,
}

impl TransactTo {
    /// Returns the callee address when this is a `Call` transaction.
    pub fn to(&self) -> Option<&Address> {
        match self {
            Self::Call(address) => Some(address),
            Self::Create => None,
        }
    }

    /// Converts this destination into the REVM `TxKind`.
    pub fn kind(&self) -> TxKind {
        match self {
            Self::Call(address) => TxKind::Call(*address),
            Self::Create => TxKind::Create,
        }
    }
}

impl Default for TransactTo {
    fn default() -> Self {
        TransactTo::Create
    }
}

/// Transaction environment used by PEVM and REVM.
#[derive(Clone, Debug, Default)]
pub struct TxEnv {
    /// EIP-2718 transaction type tag.
    pub tx_type: u8,
    /// Transaction sender.
    pub caller: Address,
    /// Gas limit supplied by the sender.
    pub gas_limit: u64,
    /// Gas price used for legacy style transactions.
    pub gas_price: u128,
    /// Optional max priority fee for EIP-1559 style transactions.
    pub gas_priority_fee: Option<u128>,
    /// Call or create destination.
    pub transact_to: TransactTo,
    /// Amount of ETH transferred alongside the call.
    pub value: U256,
    /// Calldata / initcode payload.
    pub data: Bytes,
    /// Optional nonce override. `None` skips nonce validation.
    pub nonce: Option<u64>,
    /// Optional chain id override.
    pub chain_id: Option<u64>,
    /// Access list for EIP-2930 style transactions.
    pub access_list: AccessList,
    /// Blob versioned hashes for EIP-4844 transactions.
    pub blob_versioned_hashes: Vec<B256>,
    /// Max data fee per blob for EIP-4844 transactions.
    pub max_fee_per_blob_gas: Option<u128>,
    /// Optional EIP-7702 authorizations.
    pub authorization_list: Vec<SignedAuthorization>,
    #[cfg(feature = "optimism")]
    pub optimism: OptimismFields,
}

impl TxEnv {
    /// Returns the REVM transaction kind.
    pub fn kind(&self) -> TxKind {
        self.transact_to.kind()
    }

    /// Total blob gas consumed by this transaction.
    pub fn get_total_blob_gas(&self) -> u64 {
        GAS_PER_BLOB * self.blob_versioned_hashes.len() as u64
    }

    /// Calculates the maximum EIP-4844 data fee for this transaction.
    pub fn calc_max_data_fee(&self) -> U256 {
        let blob_gas = U256::from(self.get_total_blob_gas());
        let max_fee = U256::from(self.max_fee_per_blob_gas.unwrap_or_default());
        max_fee.saturating_mul(blob_gas)
    }

    /// Builds a REVM transaction environment from this instance.
    pub fn as_revm(&self) -> RevmTxEnv {
        RevmTxEnv::from(self)
    }
}

impl From<&TxEnv> for RevmTxEnv {
    fn from(env: &TxEnv) -> Self {
        let mut tx = RevmTxEnv::default();
        tx.tx_type = env.tx_type;
        tx.caller = env.caller;
        tx.gas_limit = env.gas_limit;
        tx.gas_price = env.gas_price;
        tx.gas_priority_fee = env.gas_priority_fee;
        tx.kind = env.transact_to.kind();
        tx.value = env.value;
        tx.data = env.data.clone();
        tx.nonce = env.nonce.unwrap_or_default();
        tx.chain_id = env.chain_id;
        tx.access_list = env.access_list.clone();
        tx.blob_hashes = env.blob_versioned_hashes.clone();
        tx.max_fee_per_blob_gas = env.max_fee_per_blob_gas.unwrap_or_default();
        tx.authorization_list = env.authorization_list.clone();
        #[cfg(feature = "optimism")]
        {
            tx.optimism = env.optimism.clone();
        }
        tx
    }
}
