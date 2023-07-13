use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::crypto::UncheckedFrom;
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, CheckedAdd, CheckedSub, Zero},
};
use sp_std::prelude::*;

pub type Balance = u128;
pub type SignedExtra = ();
pub type BlockNumber = u32;
pub type Signature = sp_core::sr25519::Signature;
pub type AccountId = sp_core::sr25519::Public;
pub type Extrinsic = generic::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, SignedExtra>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, Extrinsic>;

pub struct Minter;
impl Get<AccountId> for Minter {
	fn get() -> AccountId {
		// TODO: make sp-keyring available in no-std
		let bytes =
			hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
		AccountId::unchecked_from(bytes)
	}
}

pub struct MinimumBalance;
impl Get<Balance> for MinimumBalance {
	fn get() -> Balance {
		10
	}
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum StakingCall {
	Bond { amount: Balance },
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum SystemCall {
	Remark { data: sp_std::prelude::Vec<u8> },
	Set { value: u32 },
	Upgrade { code: sp_std::prelude::Vec<u8> },
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum CurrencyCall {
	Mint { dest: AccountId, amount: Balance },
	Transfer { dest: AccountId, amount: Balance },
	TransferAll { dest: AccountId },
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum RuntimeCall {
	Currency(CurrencyCall),
	Staking(StakingCall),
	System(SystemCall),
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct AccountBalance {
	pub free: Balance,
	pub reserved: Balance,
}

/// The errors that can occur during dispatch.
#[derive(Debug, PartialEq, Clone)]
pub enum DispatchError {
	/// An error happening in `module_id`.
	Module { module_id: &'static str },
	/// All other errors, with some explanatory string.
	Other(&'static str),
}

/// Final return type of all dispatch functions.
pub type DispatchResult = Result<(), DispatchError>;

pub trait Get<T> {
	fn get() -> T;
}

/// Something that can be dispatched.
///
/// This is typically implemented for various `Call` enums.
pub trait Dispatchable {
	/// Dispatch self, assuming the given `sender`.
	fn dispatch(self, sender: AccountId) -> DispatchResult;
}

/// This is just a marker trait that wraps a bunch of other traits. It is meant to represent a
/// numeric type, like a balance, e.g. `u32`.
///
/// It helps us not repeat the long list of traits multiple times, and instead just have `type:
/// BalanceT`.
///
/// The blanket implementation for such marker traits is interesting and a common pattern.
///
/// Note the usage of `CheckedSub` and `CheckedAdd`, this is how we perform "overflow-safe"
/// arithmetic.
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
	+ sp_std::fmt::Debug // TODO: RuntimeDebug?
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
	fn transfer(from: AccountId, to: AccountId, amount: Self::Balance) -> DispatchResult;

	/// Reserve exactly `amount` from `from`.
	fn reserve(from: AccountId, amount: Self::Balance) -> DispatchResult;

	/// Get the free balance of a given account, `None` if not existent.
	fn free_balance(of: AccountId) -> Option<Self::Balance>;

	/// Get the reserved balance of a given account, `None` if non-existent.
	fn reserved_balance(of: AccountId) -> Option<Self::Balance>;
}
