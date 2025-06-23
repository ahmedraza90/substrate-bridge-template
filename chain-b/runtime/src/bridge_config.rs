use bp_header_chain::{ChainWithGrandpa, StoredHeaderData};
use bp_messages::{ChainWithMessages, MessageNonce};
use bp_runtime::{
    decl_bridge_finality_runtime_apis, decl_bridge_messages_runtime_apis, Chain, ChainId, Parachain,
};
use frame_support::{parameter_types, sp_runtime::StateVersion, weights::Weight};
use sp_runtime::{
    traits::{BlakeTwo256, ConstU128, ConstU32, IdentifyAccount, IdentityLookup, Verify},
    AccountId32, MultiAddress, MultiSignature,
};

// Chain identifiers
pub const CHAIN_A_ID: ChainId = *b"chna";
pub const CHAIN_B_ID: ChainId = *b"chnb";

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
    const WITH_CHAIN_GRANDPA_PALLET_NAME: &'static str = "BridgeGrandpaChainA";
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

// Chain B specification for Chain A's bridge
pub struct ChainB;

impl Chain for ChainB {
    const ID: ChainId = CHAIN_B_ID;
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

impl ChainWithGrandpa for ChainB {
    const WITH_CHAIN_GRANDPA_PALLET_NAME: &'static str = "BridgeGrandpaChainB";
    const MAX_AUTHORITIES_COUNT: u32 = 1024;
    const REASONABLE_HEADERS_IN_JUSTIFICATION_ANCESTRY: u32 = 8;
    const MAX_MANDATORY_HEADER_SIZE: u32 = 256;
    const AVERAGE_HEADER_SIZE: u32 = 64;
}

impl ChainWithMessages for ChainB {
    const WITH_CHAIN_MESSAGES_PALLET_NAME: &'static str = "BridgeMessages";
    const MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX: MessageNonce = 1024;
    const MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX: MessageNonce = 1024;
}
