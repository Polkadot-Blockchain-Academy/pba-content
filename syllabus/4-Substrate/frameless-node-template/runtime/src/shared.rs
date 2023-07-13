use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::crypto::UncheckedFrom;
use sp_runtime::{generic, traits::BlakeTwo256};
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

pub trait Get<T> {
	fn get() -> T;
}
