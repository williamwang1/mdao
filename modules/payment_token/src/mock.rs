//mocks for hashpower contract
#[cfg(test)]
use super::*;

use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};
//use randomness_collective_flip;

impl_outer_origin! {
	pub enum Origin for TestRuntime {}
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct TestRuntime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

pub type Balance = u64;

impl system::Trait for TestRuntime {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
}

impl Trait for TestRuntime {
	type Event = ();
	type Balance = Balance;
}

pub type PaymentToken = Module<TestRuntime>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.

pub fn build_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
	GenesisConfig::<TestRuntime> {
		token_owner: 1,
		total_supply: 21000000,
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}
