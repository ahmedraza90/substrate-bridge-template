#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod apis;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;
mod bridge_config;
pub mod configs;
use bridge_config::*;

// Bridge-related imports
use pallet_bridge_grandpa::Instance1;
use pallet_bridge_messages::Instance1 as MessagesInstance1;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use bp_messages::{target_chain::ForbidInboundMessages, HashedLaneId};
use frame_support::{
    derive_impl, parameter_types,
    traits::fungible::Mutate,
    weights::{ConstantMultiplier, IdentityFee, RuntimeDbWeight, Weight},
};
use sp_runtime::generic;
use sp_runtime::{
    impl_opaque_keys,
    traits::{BlakeTwo256, ConstU128, ConstU32, IdentifyAccount, IdentityLookup, Verify},
    AccountId32, MultiAddress, MultiSignature, StateVersion,
};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// use polkadot_runtime_parachains::{
// 	assigner_coretime as parachains_assigner_coretime, configuration as parachains_configuration,
// 	configuration::ActiveConfigHrmpChannelSizeAndCapacityRatio,
// 	coretime, disputes as parachains_disputes,
// 	disputes::slashing as parachains_slashing,
// 	dmp as parachains_dmp, hrmp as parachains_hrmp, inclusion as parachains_inclusion,
// 	inclusion::{AggregateMessageOrigin, UmpQueueId},
// 	initializer as parachains_initializer, on_demand as parachains_on_demand,
// 	origin as parachains_origin, paras as parachains_paras,
// 	paras_inherent as parachains_paras_inherent,
// 	runtime_api_impl::{
// 		v11 as parachains_runtime_api_impl, vstaging as parachains_staging_runtime_api_impl,
// 	},
// 	scheduler as parachains_scheduler, session_info as parachains_session_info,
// 	shared as parachains_shared,
// };

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
pub mod genesis_config_presets;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    use sp_runtime::{
        generic,
        traits::{BlakeTwo256, Hash as HashT},
    };

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    /// Opaque block hash type.
    pub type Hash = <BlakeTwo256 as HashT>::Output;
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
        pub grandpa: Grandpa,
    }
}

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: alloc::borrow::Cow::Borrowed("solochain-template-runtime"),
    impl_name: alloc::borrow::Cow::Borrowed("solochain-template-runtime"),
    authoring_version: 1,
    // The version of the runtime specification. A full node will not attempt to use its native
    //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    //   `spec_version`, and `authoring_version` are the same between Wasm and native.
    // This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
    //   the compatible custom types.
    spec_version: 102,
    impl_version: 1,
    apis: apis::RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

mod block_times {
    /// This determines the average expected block time that we are targeting. Blocks will be
    /// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
    /// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
    /// slot_duration()`.
    ///
    /// Change this to adjust the block time.
    pub const MILLI_SECS_PER_BLOCK: u64 = 6000;

    // NOTE: Currently it is not possible to change the slot duration after the chain has started.
    // Attempting to do so will brick block production.
    pub const SLOT_DURATION: u64 = MILLI_SECS_PER_BLOCK;
}
pub use block_times::*;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLI_SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub const BLOCK_HASH_COUNT: BlockNumber = 2400;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLI_UNIT: Balance = 1_000_000_000;
pub const MICRO_UNIT: Balance = 1_000_000;

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLI_UNIT;

// Session constants - ADD THESE
pub const SESSION_PERIOD: u32 = 10; // 10 blocks per session
pub const SESSION_OFFSET: u32 = 0;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The `TransactionExtension` to the basic transaction logic.
pub type TxExtension = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
    frame_metadata_hash_extension::CheckMetadataHash<Runtime>,
    frame_system::WeightReclaim<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;

/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, TxExtension>;

/// All migrations of the runtime, aside from the ones declared in the pallets.
///
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = ();

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,
>;

pub type TestLaneIdType = HashedLaneId;

// Create the runtime by composing the FRAME pallets that were previously configured.
#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask,
        RuntimeViewFunction
    )]
    pub struct Runtime;

    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp;

    #[runtime::pallet_index(2)]
    pub type Aura = pallet_aura;

    #[runtime::pallet_index(3)]
    pub type Grandpa = pallet_grandpa;

    #[runtime::pallet_index(4)]
    pub type Balances = pallet_balances;

    #[runtime::pallet_index(5)]
    pub type TransactionPayment = pallet_transaction_payment;

    #[runtime::pallet_index(6)]
    pub type Sudo = pallet_sudo;

    #[runtime::pallet_index(7)]
    pub type Template = pallet_template;

    #[runtime::pallet_index(8)]
    pub type Session = pallet_session;

    #[runtime::pallet_index(9)]
    pub type BridgeGrandpaChainB = pallet_bridge_grandpa<Instance1>;

    #[runtime::pallet_index(10)]
    pub type BridgeMessages = pallet_bridge_messages<Instance1>;
}

// ADD THIS CONFIGURATION BLOCK

// Bridge GRANDPA Configuration - Chain A tracks Chain B's finality
impl pallet_bridge_grandpa::Config<pallet_bridge_messages::Instance1> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type BridgedChain = ChainB;
    type MaxFreeHeadersPerBlock = ConstU32<4>;
    type FreeHeadersInterval = ConstU32<1>;
    type HeadersToKeep = ConstU32<1024>;
    type WeightInfo = ();
}

// 💡 What is a "free header"?
// A free header means: submitting a block header from the source blockchain to the target blockchain (via the relayer) **without paying transaction fees on the target chain.
// 🧠 Key Clarification:
// The relayer submits a transaction on the destination chain (not on the source chain).
// A free header just means:
// A block header that can be submitted without paying transaction fees.
// It means:
// "Allow 1 free header submission every 1 block."
// In simple words:
// Every block, one header can be submitted without fees.

// Bridge Messages Configuration - Chain A sends/receives messages to/from Chain B

/// Dummy message dispatcher.
// pub struct DummyMessageDispatch;

// impl DummyMessageDispatch {
//     pub fn deactivate(lane: TestLaneIdType) {
//         frame_support::storage::unhashed::put(&(b"inactive", lane).encode()[..], &false);
//     }
// }

// impl MessageDispatch for DummyMessageDispatch {
//     type DispatchPayload = Vec<u8>;
//     type DispatchLevelResult = ();
//     type LaneId = TestLaneIdType;

//     fn is_active(lane: Self::LaneId) -> bool {
//         frame_support::storage::unhashed::take::<bool>(&(b"inactive", lane).encode()[..])
//             != Some(false)
//     }

//     fn dispatch_weight(
//         _message: &mut DispatchMessage<Self::DispatchPayload, Self::LaneId>,
//     ) -> Weight {
//         Weight::zero()
//     }

//     fn dispatch(
//         _: DispatchMessage<Self::DispatchPayload, Self::LaneId>,
//     ) -> MessageDispatchResult<Self::DispatchLevelResult> {
//         MessageDispatchResult {
//             unspent_weight: Weight::zero(),
//             dispatch_level_result: (),
//         }
//     }
// }

impl pallet_bridge_messages::Config<pallet_bridge_messages::Instance1> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = (); // Use default weights for now

    // Use the correct interface from your example
    type ThisChain = bridge_config::ChainA;
    type BridgedChain = bridge_config::ChainB;
    type BridgedHeaderChain =
        pallet_bridge_grandpa::GrandpaChainHeaders<Self, pallet_bridge_grandpa::Instance1>;

    type OutboundPayload = sp_std::vec::Vec<u8>; // Simplified payload
    type InboundPayload = sp_std::vec::Vec<u8>; // Simplified payload
    type LaneId = TestLaneIdType; // Required LaneId type

    type DeliveryPayments = ();
    type DeliveryConfirmationPayments = ();

    // type MessageDispatch = DummyMessageDispatch
    type MessageDispatch = ForbidInboundMessages<Vec<u8>, Self::LaneId>;
    type OnMessagesDelivered = ();
}
