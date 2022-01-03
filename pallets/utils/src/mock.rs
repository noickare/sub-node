use super::*;

use sp_core::H256;
use sp_std::collections::btree_set::BTreeSet;
use sp_io::TestExternalities;

use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use frame_support::{parameter_types, dispatch::DispatchError};
use frame_system as system;

use crate as utils;
use crate::{DEFAULT_MIN_HANDLE_LEN, DEFAULT_MAX_HANDLE_LEN};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Utils: utils::{Pallet, Storage, Event<T>, Config<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
  pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
}

parameter_types! {
    pub const MinHandleLen: u32 = DEFAULT_MIN_HANDLE_LEN;
    pub const MaxHandleLen: u32 = DEFAULT_MAX_HANDLE_LEN;
  }
impl Config for Test {
    type Event = Event;
    type Currency = Balances;
    type MinHandleLen = MinHandleLen;
    type MaxHandleLen = MaxHandleLen;
}

pub type AccountId = u64;
pub(crate) type UsersSet = BTreeSet<User<AccountId>>;

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build() -> TestExternalities {
        let storage = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let mut ext = TestExternalities::from(storage);
        ext.execute_with(|| System::set_block_number(1));

        ext
    }
}


pub(crate) const USER1: User<AccountId> = User::Account(1);
pub(crate) const USER2: User<AccountId> = User::Account(2);
pub(crate) const USER3: User<AccountId> = User::Account(3);

pub(crate) fn _convert_users_vec_to_btree_set(
    users_vec: Vec<User<AccountId>>
) -> Result<UsersSet, DispatchError> {
    Utils::convert_users_vec_to_btree_set(users_vec)
}
