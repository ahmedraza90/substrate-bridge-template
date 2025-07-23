use bp_header_chain::ChainWithGrandpa;
use bp_messages::{ChainWithMessages, MessageNonce};
use bp_runtime::{Chain, ChainId};
use frame_support::sp_runtime::StateVersion;
use sp_runtime::{traits::BlakeTwo256, AccountId32, MultiSignature};

// Chain A defines ONLY itself
pub const CHAIN_A_ID: ChainId = *b"chna";

pub struct ChainA;

impl Chain for ChainA {
    const ID: ChainId = CHAIN_A_ID;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<Self::BlockNumber, Self::Hasher>;
    type AccountId = AccountId32;
    type Balance = u128;
    type Nonce = u32;
    type Signature = MultiSignature;

    const STATE_VERSION: StateVersion = StateVersion::V1;

    fn max_extrinsic_size() -> u32 {
        4 * 1024 * 1024
    }

    fn max_extrinsic_weight() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(2_000_000_000_000, 64 * 1024)
    }
}

impl ChainWithGrandpa for ChainA {
    // Chain A tracks Chain B's finality via "BridgeGrandpaChainB" pallet
    const WITH_CHAIN_GRANDPA_PALLET_NAME: &'static str = "BridgeGrandpaChainB";
    const MAX_AUTHORITIES_COUNT: u32 = 1024;
    const REASONABLE_HEADERS_IN_JUSTIFICATION_ANCESTRY: u32 = 8;
    const MAX_MANDATORY_HEADER_SIZE: u32 = 256;
    const AVERAGE_HEADER_SIZE: u32 = 64;
}

impl ChainWithMessages for ChainA {
    const WITH_CHAIN_MESSAGES_PALLET_NAME: &'static str = "BridgeMessages";
    const MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX: MessageNonce = 1024;
    const MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX: MessageNonce = 1024;
}

// ❌ DO NOT define ChainB here! It belongs in Chain B's config.
