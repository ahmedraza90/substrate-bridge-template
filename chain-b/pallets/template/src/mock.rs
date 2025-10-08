use crate as pallet_template;
use frame_support::derive_impl;
use pallet_timestamp;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

#[frame_support::runtime]
mod runtime {
    // The main runtime
    #[runtime::runtime]
    // Runtime Types to be generated
    #[runtime::derive(RuntimeCall, RuntimeEvent, RuntimeError, RuntimeOrigin, RuntimeTask)]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type Template = pallet_template::Pallet<Test>;

    #[runtime::pallet_index(2)]
    pub type Timestamp = pallet_timestamp::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
    type WeightInfo = ();
}

// impl pallet_template::Config for Test {}
impl pallet_template::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type TimeProvider = Timestamp;
    type MaxHistoryPerFarm = frame_support::traits::ConstU32<100>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
