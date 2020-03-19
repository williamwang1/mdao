#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::traits::Randomness;
/// rockx pallet hashpower contract
/// define contract structure as HashPowerContract
/// method for
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure};
//use sp_runtime::traits::{CheckedAdd, CheckedDiv, CheckedMul, Hash};
use sp_runtime::traits::Hash;
use sp_runtime::RuntimeDebug;
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Randomness: Randomness<<Self as system::Trait>::Hash>;
}

#[cfg(all(feature = "std", test))]
mod tests;

#[cfg(all(feature = "std", test))]
mod mock;

pub type ValueHashPower = u64;

//#[cfg_attr(feature = "std", derive())]
#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct HashPowerContract<Hash, BlockNumber, AccountId> {
	id: Hash,
	hash_power: ValueHashPower,
	start_date: BlockNumber,
	end_date: BlockNumber,
	service_provider: AccountId,
}

decl_storage! {
	trait Store for Module<T: Trait> as HashPowerContract {
		Contracts get(contract): map T::Hash => HashPowerContract<T::Hash, T::BlockNumber, T::AccountId>;
		ContractOwner get(owner_of):  map T::Hash => Option<T::AccountId>;

		AllContractsArray get(contract_at_index): map u64 => T::Hash;
		AllContractsCount get(contracts_count): u64;
		AllContractsIndex get(index_of): map T::Hash => u64;

		OwnedContractsArray get(contracts_of): map (T::AccountId, u64) => T::Hash;
		OwnedContractsCount get(owned_contract_count): map T::AccountId => u64;
		OwnedContractsIndex get(owned_index_of): map T::Hash => u64;

		Nonce get(nonce): u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn create_contract(
			origin,
			hash_power: ValueHashPower,
			start_date: T::BlockNumber,
			end_date: T::BlockNumber,
			service_provider: T::AccountId
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			//@@XM TODO: add some sanity check of contract parameters? not now
			let nonce = Self::nonce();
			//let random_hash = (&sender,nonce).using_encoded(<T as system::Trait>::Hashing::hash);
			let random_hash = Self::random_number(nonce);

			let new_contract  = HashPowerContract {
				id: random_hash,
				hash_power: hash_power,
				start_date: start_date,
				end_date: end_date,
				service_provider: service_provider,
			};
			//mint a contract
			Self::mint_contract(&sender, &random_hash, &new_contract)?;

			<Nonce>::mutate(|nonce| *nonce += 1);
			Ok(())
		}

		pub fn transfer_contract(origin, to: T::AccountId, contract_id: T::Hash) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//make sure sender is the owner
			ensure!(Self::owner_of(contract_id) == Some(sender.clone()), "can only transfer your own contract");

			Self::transfer_from(&sender, &to, &contract_id)?;
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
		Hash = <T as system::Trait>::Hash,
	{
		ContractCreated(AccountId, Hash),
		ContractTransfered(AccountId, AccountId, Hash),
	}
);

impl<T: Trait> Module<T> {
	fn random_number(nonce: u64) -> T::Hash {
		let payload = (
			T::Randomness::random(b"funny_random"),
			nonce,
			<system::Module<T>>::extrinsic_index(),
			<system::Module<T>>::block_number(),
		);
		payload.using_encoded(<T as system::Trait>::Hashing::hash)
	}

	fn mint_contract(
		owner: &T::AccountId,
		contract_id: &T::Hash,
		contract: &HashPowerContract<T::Hash, T::BlockNumber, T::AccountId>,
	) -> DispatchResult {
		//check whether this contract exist or not
		ensure!(!<ContractOwner<T>>::exists(contract_id), "contract already exits");

		//add contract and contract owner
		<Contracts<T>>::insert(contract_id, contract);
		<ContractOwner<T>>::insert(contract_id, owner);

		//update all contracts array
		let old_all_count = Self::contracts_count();
		let new_all_count = old_all_count
			.checked_add(1)
			.ok_or("overflow happens when contracts counts increase")?;
		<AllContractsArray<T>>::insert(old_all_count, contract_id);
		<AllContractsCount>::put(new_all_count);
		<AllContractsIndex<T>>::insert(contract_id, old_all_count);

		//update owned contract
		let old_owned_count = Self::owned_contract_count(owner);
		let new_owned_count = old_owned_count
			.checked_add(1)
			.ok_or("overflow happends when owned contract counts increate")?;
		<OwnedContractsArray<T>>::insert((owner, old_owned_count), contract_id);
		<OwnedContractsCount<T>>::insert(owner, new_owned_count);
		<OwnedContractsIndex<T>>::insert(contract_id, old_owned_count);

		//deposit a event
		Self::deposit_event(RawEvent::ContractCreated(owner.clone(), contract_id.clone()));

		Ok(())
	}

	fn transfer_from(from: &T::AccountId, to: &T::AccountId, contract_id: &T::Hash) -> DispatchResult {
		//make sure sender and to are different owners
		ensure!(from != to, "can't transfer contract to yourself");
		//make sure contract exists
		ensure!(<Contracts<T>>::exists(contract_id), "this contract doesn't exist");

		//change owner
		<ContractOwner<T>>::insert(contract_id, to);

		//update owned contracts array of original owner
		let old_owned_index_from = Self::owned_index_of(contract_id);
		let old_owned_count_from = Self::owned_contract_count(from);
		let new_owned_count_from = old_owned_count_from
			.checked_sub(1)
			.ok_or("underflow happens when owned count decrease")?;

		<OwnedContractsCount<T>>::insert(from, new_owned_count_from);
		Self::update_owned_array(from, old_owned_index_from, new_owned_count_from)?; //index of last contract in array is new_owned_count
		<OwnedContractsArray<T>>::remove((from, new_owned_count_from));

		//update owned contrats array of new owner
		let old_owned_count_to = Self::owned_contract_count(to);
		let new_owned_count_to = old_owned_count_to
			.checked_add(1)
			.ok_or("overflow happends when owned count increase")?;
		<OwnedContractsCount<T>>::insert(to, new_owned_count_to);
		<OwnedContractsArray<T>>::insert((to, old_owned_count_to), contract_id);

		<OwnedContractsIndex<T>>::insert(contract_id, old_owned_count_to);

		Self::deposit_event(RawEvent::ContractTransfered(
			from.clone(),
			to.clone(),
			contract_id.clone(),
		));

		Ok(())
	}

	fn update_owned_array(owner: &T::AccountId, index_gone: u64, index_last: u64) -> DispatchResult {
		if index_gone != index_last {
			let last_owned_contract = Self::contracts_of((owner, index_last));
			<OwnedContractsArray<T>>::insert((owner, index_gone), last_owned_contract);
			<OwnedContractsIndex<T>>::insert(last_owned_contract, index_gone);
		}
		Ok(())
	}
}
