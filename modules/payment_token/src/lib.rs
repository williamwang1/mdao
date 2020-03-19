#![cfg_attr(not(feature = "std"), no_std)]

///payment token module to handle
/// token issue
/// token transfer
/// token reserve and unserve
/// enquery and set account balance and son on
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult,
	Parameter, ensure,
};
use sp_runtime::{
	traits::{
		Member, MaybeSerializeDeserialize, Zero
	}
};
use sp_runtime::traits::{CheckedAdd, CheckedSub, CheckedDiv, CheckedMul, Hash};
use sp_arithmetic::traits::SimpleArithmetic;
use codec::{Codec, Encode, Decode};
use system::ensure_signed;

#[cfg(all(feature = "std", test))]
mod tests;

#[cfg(all(feature = "std", test))]
mod mock;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Balance: Parameter + Member + Codec + Default + Copy + MaybeSerializeDeserialize + PartialOrd + SimpleArithmetic;
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as PaymentToken {
		Owner get(token_owner) config(): T::AccountId;
		TotalSupply get(total_supply) config(): T::Balance;
		Balance get(free_balance_of): map hasher(blake2_256) T::AccountId => T::Balance;
		Reserved get (reserved_balance_of): map hasher(blake2_256) T::AccountId => T::Balance;
	}
}

// The pallet's dispatchable functions.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		
		fn issue_token(origin) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(sender == Self::token_owner(), Error::<T>::OnlyOwnerCanOperate);
			
			let total_suppy = Self::total_supply();
			<Balance<T>>::insert(&sender, total_suppy);

			Self::deposit_event(RawEvent::TokenIssued(sender.clone(), total_suppy));

			Ok(())
		}

		//Note that this will not maintain the original total supply of tokens
		fn set_balance(origin, account: T::AccountId, new_free: T::Balance, new_reserve: T::Balance) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(sender == Self::token_owner(), Error::<T>::OnlyOwnerCanOperate);

			let old_total = Self::free_balance_of(&account) + Self::reserved_balance_of(&account);
			let new_total = new_free + new_reserve;
			if old_total <= new_total {
				<TotalSupply<T>>::mutate(|v| *v+= new_total - old_total);
			} else {
				<TotalSupply<T>>::mutate(|v| *v-= old_total - new_total);
			}

			<Balance<T>>::insert(&account, new_free);
			<Reserved<T>>::insert(&account, new_reserve);

			Self::deposit_event(RawEvent::BalanceSet(account, new_free, new_reserve));
			
			Ok(())
		}

		fn transfer_from(origin, to: T::AccountId, amount: T::Balance)  -> DispatchResult {
			let sender = ensure_signed(origin)?;
			if sender == to || amount.is_zero() {
				return Ok(());
			};

			Self::transfer(sender, to, amount)?;
			Ok(())
		}

		fn reserve_token(origin, account: T::AccountId, amount: T::Balance) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(sender == Self::token_owner(), Error::<T>::OnlyOwnerCanOperate);

			if amount.is_zero() {return Ok(())};
			let free_old = Self::free_balance_of(&account);
			ensure!(free_old >= amount, Error::<T>::AmountTooLow);
			let free_new = free_old.checked_sub(&amount).ok_or(Error::<T>::UnderFlowHappens)?;
			let reserved_old = Self::reserved_balance_of(&account);
			let reserved_new = reserved_old.checked_add(&amount).ok_or(Error::<T>::OverFlowHappens)?;

			<Balance<T>>::insert(&account, free_new);
			<Reserved<T>>::insert(&account, reserved_new);

			Ok(())
		}

		fn unreserve_token(origin, account: T::AccountId, amount: T::Balance) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(sender == Self::token_owner(), Error::<T>::OnlyOwnerCanOperate);

			if amount.is_zero() {return Ok(())};
			let reserved_old = Self::reserved_balance_of(&account);
			ensure!(reserved_old >= amount, Error::<T>::AmountTooLow);
			let reserved_new = reserved_old.checked_sub(&amount).ok_or(Error::<T>::UnderFlowHappens)?;
			let free_old = Self::free_balance_of(&account);
			let free_new = free_old.checked_add(&amount).ok_or(Error::<T>::OverFlowHappens)?;

			<Balance<T>>::insert(&account, free_new);
			<Reserved<T>>::insert(&account, reserved_new);

			Ok(())
		}

	}
}

impl<T: Trait> Module<T> {
	fn transfer(from: T::AccountId, to: T::AccountId, amount: T::Balance) ->DispatchResult {
		ensure!(<Balance<T>>::exists(&from), Error::<T>::AccountNotExist);

		let free_balance = Self::free_balance_of(&from);
		ensure!(free_balance >= amount, Error::<T>::AmountTooLow);
		
		if <Balance<T>>::exists(&to) {
			let old_balance = Self::free_balance_of(&to);
			let new_balance = old_balance.checked_add(&amount).ok_or(Error::<T>::OverFlowHappens)?;
			<Balance<T>>::insert(&to, new_balance);
		} else {
			<Balance<T>>::insert(&to, amount);
		}
		
		let new_balance_from = free_balance.checked_sub(&amount).ok_or(Error::<T>::UnderFlowHappens)?;
		<Balance<T>>::insert(&from, new_balance_from);

		Self::deposit_event(RawEvent::TokenTransferred(from, to, amount));
		Ok(())
	}

	fn free_balance(account: T::AccountId) -> T::Balance {
		Self::free_balance_of(&account)
	}

	fn reserved_balance(account: T::AccountId) -> T::Balance {
		Self::reserved_balance_of(&account)
	}
} 

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
		Balance = <T as Trait>::Balance,
	{
		TokenIssued(AccountId, Balance),
		BalanceSet(AccountId, Balance, Balance),
		TokenTransferred(AccountId, AccountId, Balance),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		AmountTooLow,
		OnlyOwnerCanOperate,
		AccountNotExist,
		OverFlowHappens,
		UnderFlowHappens,
	}
}
