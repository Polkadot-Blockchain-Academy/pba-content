//! Shared types used in your solution, and our grading tool.

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{generic, traits::BlakeTwo256};
use sp_std::prelude::*;

/// The Balance type. You should probably not change this.
pub type Balance = u128;
/// The block number type.
pub type BlockNumber = u32;
/// Signature type. We use `sr25519` crypto.
pub type Signature = sp_core::sr25519::Signature;
/// Account id type is the public key. We use `sr25519` crypto.
///
/// be aware of using the right crypto type when using `sp-keyring` crate.
pub type AccountId = sp_core::sr25519::Public;

/// The account id who's allowed to mint. This is the sr25519 representation of `Alice` in
/// `sp-keyring`.
pub const MINTER: [u8; 32] =
	hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
/// The treasury account to which tips should be deposited.
pub const TREASURY: [u8; 32] =
	hex_literal::hex!["ff3593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
/// Minimum account balanced. No account balance should fall below this, unless if the account is
/// being destroyed.
pub const MINIMUM_BALANCE: Balance = 10;
/// The key to which [`SystemCall::Set`] will write the value.
///
/// Hex: 0x76616c7565
pub const VALUE_KEY: &[u8] = b"value";
/// Temporary key used to store the header. This should always be clear at the end of the block.
///
/// Hex: 0x686561646572
pub const HEADER_KEY: &[u8] = b"header";
/// Key used to store all extrinsics in a block. Should always remain at the end of the block, and
/// be cleared at the beginning of the next block.
pub const EXTRINSICS_KEY: &[u8] = b"extrinsics";

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum StakingCall {
	/// Bond `amount` form the sender, if they have enough free balance.
	///
	/// This results in `amount` being moved from their free balance to their reserved balance. See
	/// [`AccountBalance`].
	Bond { amount: Balance },
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum SystemCall {
	/// Do nothing.
	///
	/// This will only ensure that `data` is remarked as the block data.
	Remark { data: sp_std::prelude::Vec<u8> },
	/// Set the value under [`VALUE_KEY`] to `value`.
	Set { value: u32 },
	/// Upgrade the runtime to the given code.
	Upgrade { code: sp_std::prelude::Vec<u8> },
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum CurrencyCall {
	/// Mint `amount` of tokens to `dest`. This will increase the total issuance of the system.
	///
	/// If `dest` exists, its balance is increased. Else, it is created, if possible.
	///
	/// ### Errors
	///
	/// * If any type of arithmetic operation overflows.
	/// * If the `dest`'s free balance will not be enough to pass the bar of [`MINIMUM_BALANCE`].
	/// * If the sender is not [`MINTER`].
	Mint { dest: AccountId, amount: Balance },
	/// Transfer `amount` to `dest`.
	///
	/// The `sender` must exist in ANY CASE, but the `dest` might be created in the process.
	///
	/// Both `sender` and `dest` must finish the operation with equal or more free balance than
	/// [`MINIMUM_BALANCE`].
	///
	/// ### Errors
	///
	/// * If any type of arithmetic operation overflows.
	/// * If the sender does not exist.
	/// * If either `sender` or `dest` finish without [`MINIMUM_BALANCE`] of free balance left.
	Transfer { dest: AccountId, amount: Balance },
	/// Transfer all of sender's free balance to `dest`. This is equal to "destroying" the
	/// sender account.
	///
	/// If `sender` has some reserved balance, operation should not be allowed.
	///
	/// ### Errors
	///
	/// * If any type of arithmetic operation overflows.
	/// * If the sender has any free balance left.
	///
	/// Since the sender is a valid account, with more than [`MINIMUM_BALANCE`], the recipient
	/// is also guaranteed to have at least [`MINIMUM_BALANCE`].
	TransferAll { dest: AccountId },
}

/// The outer runtime call.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum RuntimeCall {
	Currency(CurrencyCall),
	Staking(StakingCall),
	System(SystemCall),
}

/// The final runtime call, which is a callable operation, and an optional tip.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub struct RuntimeCallWithTip {
	pub tip: Option<Balance>,
	pub call: RuntimeCall,
}

/// Final extrinsic type of the runtime.
///
/// Our tests will use the given types to interact with your runtime. Note that you can use any
/// other type on your runtime type, as long as you can convert it to/from these types.
pub type Extrinsic = generic::UncheckedExtrinsic<AccountId, RuntimeCallWithTip, Signature, ()>;

/// The header type of the runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// The block type of the runtime.
pub type Block = generic::Block<Header, Extrinsic>;

/// The account balance struct that we expect to find under `BalancesMap ++ account_id`.
///
/// The free balance of an account is the subset of the account balance that can be transferred
/// out of the account. As noted elsewhere, the free balance of ALL accounts at ALL TIMES mut be
/// equal or more than that of [`MINIMUM_BALANCE`].
///
/// Conversely, the reserved part of an account is a subset that CANNOT be transferred out,
/// unless if explicitly unreserved.
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct AccountBalance {
	/// The free balance that they have. This can be transferred.
	pub free: Balance,
	/// The reserved balance that they have. This CANNOT be transferred.
	pub reserved: Balance,
}
