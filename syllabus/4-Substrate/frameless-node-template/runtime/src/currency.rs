//! The crypto-currency module.
//!
//! It contains:
//!
//! 1. [`currency_module::Config`]: a wrapper for configurations of this module that should come
//!        from the over-arching runtime.
//! 2. [`currency_module::Module`]: a struct that will contain all the implementations, including
//!    the transactions, and the [`shared::CryptoCurrency`] trait.
//! 3. [`currency_module::Call`]: The `Call` type for this module.
//!
//! Among other things.
//!
//! This module contains two storage items:
//!
//! 1. [`currency_module::TotalIssuance`]: a `StorageValue` containing the sum of all balances in
//!    the system.
//! 2. [`currency_module::BalancesMap`]: a `StorageMap` that maps from an account ID to their
//!    balance.

use crate::{
	shared::{self, DispatchResult, Get},
	storage::{StorageMap, StorageValue},
};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::traits::{CheckedAdd, CheckedSub, Zero};
use sp_std::prelude::*;

/// Configurations of this module, coming from the outer world/runtime.
///
/// These are basically all the things that we don't want to make a concrete decision about, so
/// we let the outer world decide.
///
/// Within this module, we keep all implementation blocks generic over a `<T: Config>`, and use
/// the associated items as `T::MinimumBalance`, `T::Balance` etc.
///
/// This is a very common pattern in Substrate!
pub trait Config {
	/// The identifier of this module.
	const MODULE_ID: &'static str;

	/// The account that is allowed to mint new tokens. Think: the admin.
	type Minter: Get<shared::AccountId>;

	/// The minimum *free* balances (as explained in [`AccountBalance::free`]) that should be
	/// held by ANY account at ANY POINT IN TIME.
	///
	/// An account with free balance less than this amount is considered a logical error.
	type MinimumBalance: shared::Get<Self::Balance>;

	/// The numeric type that we use to store balances, e.g. `u64`.
	type Balance: shared::BalanceT;
}

/// This module's `Call` enum.
///
/// Contains all of the operations, and possible arguments (except `sender`, of course).
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
#[scale_info(skip_type_params(T))]
pub enum Call<T: Config> {
	/// Mint `amount` of tokens to `dest`. This will increase the total issuance of the system.
	///
	/// If `dest` exists, its balance is increased. Else, it is created.
	///
	/// ### Dispatch Errors
	///
	/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
	/// * [`Error::InsufficientFunds`] if the `dest`'s free balance will not be enough to pass
	/// the bar of `T::MinimumBalance`.
	/// * [`Error::NotAllowed`] if the sender is not allowed to mint.
	Mint { dest: shared::AccountId, amount: T::Balance },
	/// Transfer `amount` to `dest`.
	///
	/// The `sender` must exist in ANY CASE, but the `dest` might be created in the process.
	///
	/// Both `sender` and `dest` must finish the operation with equal or more free balance than
	/// `T::MinimumBalance`.
	///
	/// ### Dispatch Errors
	///
	/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
	/// * [`Error::DoesNotExist`] if the sender does not exist.
	/// * [`Error::InsufficientFunds`] if either `sender` or `dest` finish without
	///   `T::MinimumBalance` of free balance left.
	Transfer { dest: shared::AccountId, amount: T::Balance },
	/// Transfer all of sender's free balance to `dest`. This is equal to "destroying" the
	/// sender account.
	///
	/// If `sender` has some reserved balance, operation should not be allowed.
	///
	/// ### Dispatch Errors
	///
	/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
	/// * [`Error::NotAllowed`] If the sender has any free balance left.
	///
	/// Since the sender is a valid account, with more than `T::MinimumBalance`, the recipient
	/// is also guaranteed to have at least `T::MinimumBalance`.
	TransferAll { dest: shared::AccountId },
}

/// The error type of this module.
///
/// We will provide a conversion `From<Error> for shared::DispatchError`. This will allow us to
/// easily convert the error of this particular module into the error of
/// [`shared::DispatchResult`]. Moreover, it allows for the `?` operator to be easily used.
pub enum Error<T: Config> {
	/// The account of choice does not exist.
	DoesNotExist,
	/// Given operation is not allowed.
	NotAllowed,
	/// The account of choice does exist, but the amount that is being used is not enough to
	/// cover the requested operation.
	InsufficientFunds,
	/// Some arithmetic operation overflowed.
	Overflow,
	/// We use T in a PhantomData so that `Error` is parameterized over `T`, allowing access to
	/// Config items like `T::MODULE_ID` when we use `Error` later.
	#[allow(non_camel_case_types)]
	__marker(sp_std::marker::PhantomData<T>),
}

impl<T: Config> sp_std::fmt::Debug for Error<T> {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		match self {
			Error::DoesNotExist => write!(f, "DoesNotExist"),
			Error::NotAllowed => write!(f, "NotAllowed"),
			Error::InsufficientFunds => write!(f, "InsufficientFunds"),
			Error::Overflow => write!(f, "Overflow"),
			Error::__marker(_) => unreachable!("__marker should never be printed"),
		}
	}
}

impl<T: Config> From<Error<T>> for shared::DispatchError {
	fn from(_: Error<T>) -> Self {
		Self::Module { module_id: T::MODULE_ID }
	}
}

/// A wrapper for the balance of a user/account.
///
/// The free balance of an account is the subset of the account balance that can be transferred
/// out of the account. As noted elsewhere, the free balance of ALL accounts at ALL TIMES mut be
/// equal or more than that of `T::MinimumBalance`.
///
/// Conversely, the reserved part of an account is a subset that CANNOT be transferred out,
/// unless if explicitly unreserved.
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct AccountBalance<T: Config> {
	/// The free balance that they have. This can be transferred.
	pub free: T::Balance,
	/// The reserved balance that they have. This CANNOT be transferred.
	pub reserved: T::Balance,
}

// NOTE: why can't we `derive` this? explore!
impl<T: Config> Default for AccountBalance<T> {
	fn default() -> Self {
		Self { free: T::Balance::default(), reserved: T::Balance::default() }
	}
}

// NOTE: make sure to return correct [`Error`] types based on [`Call`] specifications.
impl<T: Config> AccountBalance<T> {
	/// Reserve `amount`, if possible.
	fn reserve(&mut self, amount: T::Balance) -> shared::DispatchResult {
		match self.free.checked_sub(&amount) {
			Some(leftover) if leftover >= T::MinimumBalance::get() => {
				self.free = leftover;
				self.reserved = self.reserved.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
				Ok(())
			},
			_ => Err(Error::<T>::InsufficientFunds)?,
		}
	}

	/// Unreserve `amount`, if possible.
	fn unreserve(&mut self, amount: T::Balance) -> shared::DispatchResult {
		match self.reserved.checked_sub(&amount) {
			Some(leftover) => {
				self.reserved = leftover;
				self.free = self.free.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
				Ok(())
			},
			_ => Err(Error::<T>::InsufficientFunds)?,
		}
	}

	/// Returns true if we have enough free balance to transfer `amount`.
	fn can_transfer(&self, amount: T::Balance) -> DispatchResult {
		match self.free.checked_sub(&amount) {
			Some(leftover) if leftover >= T::MinimumBalance::get() || leftover.is_zero() => Ok(()),
			_ => Err(Error::<T>::InsufficientFunds)?,
		}
	}

	/// Send/transfer `amount` from the free balance.
	fn transfer(&mut self, amount: T::Balance) -> shared::DispatchResult {
		match self.free.checked_sub(&amount) {
			Some(leftover) if leftover >= T::MinimumBalance::get() || leftover.is_zero() => {
				self.free = leftover;
				Ok(())
			},
			_ => Err(Error::<T>::InsufficientFunds)?,
		}
	}

	/// Returns true if this amount can be received
	fn can_receive(&self, amount: T::Balance) -> DispatchResult {
		self.free
			.checked_add(&amount)
			.ok_or(Error::<T>::Overflow)
			.and_then(|n| {
				(n >= T::MinimumBalance::get())
					.then_some(())
					.ok_or(Error::<T>::InsufficientFunds)
			})
			.map_err(Into::into)
	}

	/// Add `amount` to the free balance, if possible.
	fn receive(&mut self, amount: T::Balance) -> DispatchResult {
		self.free = self.free.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
		if self.free < T::MinimumBalance::get() {
			Err(Error::<T>::InsufficientFunds)?
		}
		Ok(())
	}
}

/// A map from `AccountId` -> `AccountBalance`.
///
/// This is where the balance of each user should be stored.
pub struct BalancesMap<T: Config>(sp_std::marker::PhantomData<T>);
impl<T: Config> StorageMap for BalancesMap<T> {
	type Key = shared::AccountId;
	type Value = AccountBalance<T>;
	fn raw_storage_key(key: Self::Key) -> Vec<u8> {
		let mut final_key = b"BalancesMap".to_vec();
		final_key.extend(key.encode());
		final_key
	}
}

/// The total issuance. This should track be the sum of **free and reserved** balance of all
/// accounts, at all times.
pub struct TotalIssuance<T: Config>(sp_std::marker::PhantomData<T>);
impl<T: Config> StorageValue for TotalIssuance<T> {
	type Value = T::Balance;
	fn raw_storage_key() -> Vec<u8> {
		b"TotalIssuance".to_vec()
	}
}

/// Just a wrapper for this module's implementations.
///
/// Note that this struct is itself public, but the internal implementations are not. The public
/// interface of each module is its `Call` (followed by calling `dispatch` on it), not `Module`.
pub struct Module<T: Config>(sp_std::marker::PhantomData<T>);
impl<T: Config> Module<T> {
	// NOTE: better not repeat yourself in documentation ;).

	/// See [`Call::Transfer`].
	fn transfer(
		sender: shared::AccountId,
		dest: shared::AccountId,
		amount: T::Balance,
	) -> shared::DispatchResult {
		let mut sender_balance = BalancesMap::<T>::get(sender).ok_or(Error::<T>::DoesNotExist)?;
		let mut dest_balance = BalancesMap::<T>::get(dest).unwrap_or_default();

		sender_balance.can_transfer(amount)?;
		dest_balance.can_receive(amount)?;

		sender_balance.transfer(amount).expect("checked above");
		dest_balance.receive(amount).expect("checked above");

		BalancesMap::<T>::set(sender, sender_balance);
		BalancesMap::<T>::set(dest, dest_balance);
		Ok(())
	}

	/// See [`Call::TransferAll`].
	fn transfer_all(sender: shared::AccountId, dest: shared::AccountId) -> shared::DispatchResult {
		let balance = BalancesMap::<T>::get(sender).ok_or(Error::<T>::DoesNotExist)?;
		if !balance.reserved.is_zero() {
			Err(Error::<T>::NotAllowed)?;
		}
		let amount = balance.free;
		Self::transfer(sender, dest, amount)
	}

	/// See [`Call::Mint`].
	fn mint(
		sender: shared::AccountId,
		who: shared::AccountId,
		amount: T::Balance,
	) -> shared::DispatchResult {
		if sender != T::Minter::get() {
			Err(Error::<T>::NotAllowed)?;
		}

		let mut balance = BalancesMap::<T>::get(who).unwrap_or_default();
		let issuance = TotalIssuance::<T>::get().unwrap_or_else(Zero::zero);

		let issuance = issuance.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
		balance.receive(amount)?;

		BalancesMap::<T>::set(who, balance);
		TotalIssuance::<T>::set(issuance);

		Ok(())
	}

	// NOTE: This is not reflected in [`Call`], so we document it here.

	/// Reserve exactly `amount` from `from`'s free balance.
	///
	/// ### Errors
	///
	/// * [`Error::DoesNotExist`] if the `from` account does not currently exist
	/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
	/// * [`Error::InsufficientFunds`] if the account does not have enough free funds to preform
	///   this operation. Recall that an accounts free balance must always remain equal or above
	///   `T::MinimumBalance`.
	pub fn reserve(from: shared::AccountId, amount: T::Balance) -> shared::DispatchResult {
		let mut balance = BalancesMap::<T>::get(from).ok_or(Error::<T>::DoesNotExist)?;

		balance.can_transfer(amount)?;
		balance.reserve(amount).expect("checked above");

		BalancesMap::<T>::set(from, balance);
		Ok(())
	}

	/// Unreserve exactly `amount` from `from`'s reserved balance, returning it back
	/// to the free balance
	///
	/// ### Errors
	///
	/// * [`Error::DoesNotExist`] if the `from` account does not currently exist
	/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
	/// * [`Error::InsufficientFunds`] if the account does not have enough reserved funds to preform
	///   this operation.
	pub fn unreserve(from: shared::AccountId, amount: T::Balance) -> shared::DispatchResult {
		let mut balance = BalancesMap::<T>::get(from).ok_or(Error::<T>::DoesNotExist)?;

		balance.unreserve(amount)?;

		BalancesMap::<T>::set(from, balance);
		Ok(())
	}
}

impl<T: Config> shared::Dispatchable for Call<T> {
	fn dispatch(self, sender: shared::AccountId) -> shared::DispatchResult {
		match self {
			Call::Mint { dest, amount } => Module::<T>::mint(sender, dest, amount),
			Call::Transfer { dest, amount } => Module::<T>::transfer(sender, dest, amount),
			Call::TransferAll { dest } => Module::<T>::transfer_all(sender, dest),
		}
	}
}

impl<T: Config> shared::CryptoCurrency for Module<T> {
	type Balance = T::Balance;

	fn transfer(
		from: shared::AccountId,
		to: shared::AccountId,
		amount: Self::Balance,
	) -> shared::DispatchResult {
		Module::<T>::transfer(from, to, amount)
	}

	fn reserve(from: shared::AccountId, amount: Self::Balance) -> shared::DispatchResult {
		Module::<T>::reserve(from, amount)
	}

	fn free_balance(of: shared::AccountId) -> Option<Self::Balance> {
		BalancesMap::<T>::get(of).map(|b| b.free)
	}

	fn reserved_balance(of: shared::AccountId) -> Option<Self::Balance> {
		BalancesMap::<T>::get(of).map(|b| b.reserved)
	}
}
