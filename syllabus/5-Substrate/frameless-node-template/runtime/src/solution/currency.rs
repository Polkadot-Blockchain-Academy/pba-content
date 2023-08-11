use super::{
	storage::{StorageMap, StorageValue},
	Dispatchable, Get,
};
use crate::shared::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{CheckedAdd, CheckedSub, Zero},
	DispatchOutcome,
};
use sp_std::prelude::*;

pub trait Config {
	const MODULE_ID: &'static str;
	type Minter: Get<AccountId>;
	type MinimumBalance: Get<Self::Balance>;
	type Balance: BalanceT;
}

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
	Mint { dest: AccountId, amount: T::Balance },
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
	Transfer { dest: AccountId, amount: T::Balance },
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
	TransferAll { dest: AccountId },
}

pub enum Error<T: Config> {
	DoesNotExist,
	NotAllowed,
	InsufficientFunds,
	Overflow,
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

impl<T: Config> From<Error<T>> for sp_runtime::DispatchError {
	fn from(_: Error<T>) -> Self {
		Self::Other("SomeOtherErrorWeDontCareAbout")
	}
}

// We can store this because it has the same encoding as `shared::AccountBalance`.
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct AccountBalance<T: Config> {
	pub free: T::Balance,
	pub reserved: T::Balance,
	pub nonce: u32,
}

impl<T: Config> Default for AccountBalance<T> {
	fn default() -> Self {
		Self { free: T::Balance::default(), reserved: T::Balance::default(), nonce: 0 }
	}
}

impl<T: Config> AccountBalance<T> {
	pub(crate) fn reserve(&mut self, amount: T::Balance) -> DispatchOutcome {
		match self.free.checked_sub(&amount) {
			Some(leftover) if leftover >= T::MinimumBalance::get() => {
				self.free = leftover;
				self.reserved = self.reserved.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
				Ok(())
			},
			_ => Err(Error::<T>::InsufficientFunds)?,
		}
	}

	pub(crate) fn can_reserve(&self, amount: T::Balance) -> DispatchOutcome {
		match self.free.checked_sub(&amount) {
			Some(leftover) if leftover >= T::MinimumBalance::get() => Ok(()),
			_ => Err(Error::<T>::InsufficientFunds)?,
		}
	}

	#[allow(unused)]
	pub(crate) fn unreserve(&mut self, amount: T::Balance) -> DispatchOutcome {
		match self.reserved.checked_sub(&amount) {
			Some(leftover) => {
				self.reserved = leftover;
				self.free = self.free.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
				Ok(())
			},
			_ => Err(Error::<T>::InsufficientFunds)?,
		}
	}

	pub(crate) fn can_withdraw(&self, amount: T::Balance, allow_zero: bool) -> DispatchOutcome {
		match self.free.checked_sub(&amount) {
			Some(leftover)
				if leftover >= T::MinimumBalance::get() || (leftover.is_zero() && allow_zero) =>
				Ok(()),
			_ => Err(Error::<T>::InsufficientFunds)?,
		}
	}

	pub(crate) fn withdraw(&mut self, amount: T::Balance, allow_zero: bool) -> DispatchOutcome {
		match self.free.checked_sub(&amount) {
			Some(leftover)
				if leftover >= T::MinimumBalance::get() || (leftover.is_zero() && allow_zero) =>
			{
				self.free = leftover;
				Ok(())
			},
			_ => Err(Error::<T>::InsufficientFunds)?,
		}
	}

	pub(crate) fn can_receive(&self, amount: T::Balance) -> DispatchOutcome {
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

	pub(crate) fn receive(&mut self, amount: T::Balance) -> DispatchOutcome {
		self.free = self.free.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
		if self.free < T::MinimumBalance::get() {
			Err(Error::<T>::InsufficientFunds)?
		}
		Ok(())
	}

	pub(crate) fn unchecked_receive(&mut self, amount: T::Balance) -> DispatchOutcome {
		self.free = self.free.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
		Ok(())
	}
}

pub struct BalancesMap<T: Config>(sp_std::marker::PhantomData<T>);
impl<T: Config> StorageMap for BalancesMap<T> {
	type Key = AccountId;
	type Value = AccountBalance<T>;
	fn raw_storage_key(key: Self::Key) -> Vec<u8> {
		let mut final_key = b"BalancesMap".to_vec();
		final_key.extend(key.encode());
		final_key
	}
}

pub struct TotalIssuance<T: Config>(sp_std::marker::PhantomData<T>);
impl<T: Config> StorageValue for TotalIssuance<T> {
	type Value = T::Balance;
	fn raw_storage_key() -> Vec<u8> {
		b"TotalIssuance".to_vec()
	}
}

pub struct Module<T: Config>(sp_std::marker::PhantomData<T>);
impl<T: Config> Module<T> {
	fn transfer(
		sender: AccountId,
		dest: AccountId,
		amount: T::Balance,
		allow_zero: bool,
	) -> DispatchOutcome {
		let mut sender_balance = BalancesMap::<T>::get(sender).ok_or(Error::<T>::DoesNotExist)?;
		let mut dest_balance = BalancesMap::<T>::get(dest).unwrap_or_default();

		sender_balance.can_withdraw(amount, allow_zero)?;
		dest_balance.can_receive(amount)?;

		sender_balance.withdraw(amount, allow_zero).expect("checked above");
		dest_balance.receive(amount).expect("checked above");

		if sender_balance.free.is_zero() && sender_balance.reserved.is_zero() {
			BalancesMap::<T>::clear(sender);
		} else {
			BalancesMap::<T>::set(sender, sender_balance);
		}
		BalancesMap::<T>::set(dest, dest_balance);
		Ok(())
	}

	fn transfer_all(sender: AccountId, dest: AccountId) -> DispatchOutcome {
		let balance = BalancesMap::<T>::get(sender).ok_or(Error::<T>::DoesNotExist)?;
		if !balance.reserved.is_zero() {
			Err(Error::<T>::NotAllowed)?;
		}
		let amount = balance.free;
		Self::transfer(sender, dest, amount, true)
	}

	fn mint(sender: AccountId, who: AccountId, amount: T::Balance) -> DispatchOutcome {
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
	pub fn reserve(from: AccountId, amount: T::Balance) -> DispatchOutcome {
		let mut balance = BalancesMap::<T>::get(from).ok_or(Error::<T>::DoesNotExist)?;

		balance.can_reserve(amount)?;
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
	#[allow(unused)]
	pub fn unreserve(from: AccountId, amount: T::Balance) -> DispatchOutcome {
		let mut balance = BalancesMap::<T>::get(from).ok_or(Error::<T>::DoesNotExist)?;

		balance.unreserve(amount)?;

		BalancesMap::<T>::set(from, balance);
		Ok(())
	}
}

impl<T: Config> Dispatchable for Call<T> {
	fn dispatch(self, sender: AccountId) -> DispatchOutcome {
		match self {
			Call::Mint { dest, amount } => Module::<T>::mint(sender, dest, amount),
			Call::Transfer { dest, amount } => Module::<T>::transfer(sender, dest, amount, false),
			Call::TransferAll { dest } => Module::<T>::transfer_all(sender, dest),
		}
	}
}

impl<T: Config> CryptoCurrency for Module<T> {
	type Balance = T::Balance;

	fn transfer(from: AccountId, to: AccountId, amount: Self::Balance) -> DispatchOutcome {
		Module::<T>::transfer(from, to, amount, false)
	}

	fn reserve(from: AccountId, amount: Self::Balance) -> DispatchOutcome {
		Module::<T>::reserve(from, amount)
	}

	fn free_balance(of: AccountId) -> Option<Self::Balance> {
		BalancesMap::<T>::get(of).map(|b| b.free)
	}

	fn reserved_balance(of: AccountId) -> Option<Self::Balance> {
		BalancesMap::<T>::get(of).map(|b| b.reserved)
	}
}

pub trait BalanceT:
	Copy
	+ Clone
	+ Default
	+ Encode
	+ Decode
	+ CheckedSub
	+ CheckedAdd
	+ Zero
	+ Ord
	+ PartialOrd
	+ Eq
	+ PartialEq
	+ sp_std::fmt::Debug
{
}
impl<
		T: Copy
			+ Clone
			+ Default
			+ Encode
			+ Decode
			+ CheckedSub
			+ CheckedAdd
			+ Ord
			+ Zero
			+ PartialOrd
			+ Eq
			+ PartialEq
			+ sp_std::fmt::Debug,
	> BalanceT for T
{
}

/// A trait to represent basic functionality of a crypto-currency.
///
/// This should be implemented by `currency_module::Module`.
pub trait CryptoCurrency {
	/// The numeric type used to represent balances.
	type Balance: BalanceT;

	/// Transfer `amount` from `from` to `to`.
	fn transfer(from: AccountId, to: AccountId, amount: Self::Balance) -> DispatchOutcome;

	/// Reserve exactly `amount` from `from`.
	fn reserve(from: AccountId, amount: Self::Balance) -> DispatchOutcome;

	/// Get the free balance of a given account, `None` if not existent.
	fn free_balance(of: AccountId) -> Option<Self::Balance>;

	/// Get the reserved balance of a given account, `None` if non-existent.
	fn reserved_balance(of: AccountId) -> Option<Self::Balance>;
}
