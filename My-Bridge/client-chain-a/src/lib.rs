//! Types used to connect to Chain A.

pub mod codegen_runtime;

use std::time::Duration;
use codec::Encode;
use relay_substrate_client::{
    Chain, ChainWithTransactions, ChainWithMessages, ChainWithGrandpa,
    ChainWithRuntimeVersion, SignParam, SimpleRuntimeVersion, 
    UnderlyingChainProvider, UnsignedTransaction, Error as SubstrateError
};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{
    generic,
    traits::{FakeDispatchable, IdentifyAccount},
};

// Import only Chain A generated types
pub use codegen_runtime::api::runtime_types;

// Chain A specific types from generated runtime
pub type RuntimeCall = runtime_types::chain_a_runtime::RuntimeCall;
pub type BridgeGrandpaCall = runtime_types::pallet_bridge_grandpa::pallet::Call;
pub type BridgeMessagesCall = runtime_types::pallet_bridge_messages::pallet::Call;

// Chain A client definition - core configuration only
#[derive(Clone)]
pub struct ChainA;

impl UnderlyingChainProvider for ChainA {
	type Chain = bp_bridge_hub_rococo::BridgeHubRococo;
}

impl Chain for ChainA {
    const NAME: &'static str = "ChainA";
    const BEST_FINALIZED_HEADER_ID_METHOD: &'static str = "BridgeApi_best_finalized_header_id";
    const FREE_HEADERS_INTERVAL_METHOD: &'static str = "BridgeApi_free_headers_interval";
    const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_millis(6000);
    
    pub type SignedBlock = generic::SignedBlock<Block>;
    // This will be replaced with generated runtime call type
    type Call = ChainARuntimeCall;
}

impl ChainWithRuntimeVersion for ChainA {
    const RUNTIME_VERSION: Option<SimpleRuntimeVersion> = Some(SimpleRuntimeVersion {
        spec_version: 102,
        transaction_version: 1,
    });
}

impl ChainWithTransactions for ChainA {
    type AccountKeyPair = sp_core::sr25519::Pair;
    type SignedTransaction = OpaqueExtrinsic;

    fn sign_transaction(
        param: relay_substrate_client::SignParam<Self>,
        unsigned: relay_substrate_client::UnsignedTransaction<Self>,
    ) -> Result<Self::SignedTransaction, relay_substrate_client::Error> {
        relay_substrate_client::sign_transaction(param, unsigned)
    }
}

impl ChainWithGrandpa for ChainA {
    const SYNCED_HEADERS_GRANDPA_INFO_METHOD: &'static str = "BridgeGrandpaApi_synced_headers_grandpa_info";
    type KeyOwnerProof = sp_consensus_grandpa::OpaqueKeyOwnershipProof;
}

impl ChainWithMessages for ChainA {
    const TO_CHAIN_MESSAGE_DETAILS_METHOD: &'static str = "BridgeMessagesApi_outbound_message_details";
    const FROM_CHAIN_MESSAGE_DETAILS_METHOD: &'static str = "BridgeMessagesApi_inbound_message_details";
}
