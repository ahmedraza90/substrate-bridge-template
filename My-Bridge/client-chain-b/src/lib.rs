//! Types used to connect to Chain B.

pub mod codegen_runtime;

// Similar structure but for Chain B
pub use codegen_runtime::api::runtime_types;

// Chain B specific types from generated runtime  
pub type RuntimeCall = runtime_types::chain_b_runtime::RuntimeCall;
pub type BridgeGrandpaCall = runtime_types::pallet_bridge_grandpa::pallet::Call;
pub type BridgeMessagesCall = runtime_types::pallet_bridge_messages::pallet::Call;

// Chain B client definition - core configuration only
#[derive(Clone)]
pub struct ChainB;

impl Chain for ChainB {
    const NAME: &'static str = "ChainB";
    const BEST_FINALIZED_HEADER_ID_METHOD: &'static str = "BridgeApi_best_finalized_header_id";
    const FREE_HEADERS_INTERVAL_METHOD: &'static str = "BridgeApi_free_headers_interval";
    const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_millis(6000);
    
    type SignedBlock = GenericSignedBlock<GenericBlock<
        GenericHeader<u32, BlakeTwo256>,
        OpaqueExtrinsic
    >>;
    // This will be replaced with generated runtime call type
    type Call = ChainBRuntimeCall;
}

impl ChainWithRuntimeVersion for ChainB {
    const RUNTIME_VERSION: Option<SimpleRuntimeVersion> = Some(SimpleRuntimeVersion {
        spec_version: 102,
        transaction_version: 1,
    });
}

impl ChainWithTransactions for ChainB {
    type AccountKeyPair = sp_core::sr25519::Pair;
    type SignedTransaction = OpaqueExtrinsic;

    fn sign_transaction(
        param: relay_substrate_client::SignParam<Self>,
        unsigned: relay_substrate_client::UnsignedTransaction<Self>,
    ) -> Result<Self::SignedTransaction, relay_substrate_client::Error> {
        relay_substrate_client::sign_transaction(param, unsigned)
    }
}

impl ChainWithGrandpa for ChainB {
    const SYNCED_HEADERS_GRANDPA_INFO_METHOD: &'static str = "BridgeGrandpaApi_synced_headers_grandpa_info";
    type KeyOwnerProof = sp_consensus_grandpa::OpaqueKeyOwnershipProof;
}

impl ChainWithMessages for ChainB {
    const TO_CHAIN_MESSAGE_DETAILS_METHOD: &'static str = "BridgeMessagesApi_outbound_message_details";
    const FROM_CHAIN_MESSAGE_DETAILS_METHOD: &'static str = "BridgeMessagesApi_inbound_message_details";
}