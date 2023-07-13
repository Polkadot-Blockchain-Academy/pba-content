use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{generic, traits::BlakeTwo256};
use sp_std::prelude::*;

/// Primitive types used in the runtime.
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Signature = sp_core::sr25519::Signature;
pub type AccountId = sp_core::sr25519::Public;

/// The account id who's allowed to mint. This is the sr25519 representation of `Alice`.
pub const MINTER: [u8; 32] =
	hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
/// Minimum account balanced.
pub const MINIMUM_BALANCE: Balance = 10;
/// The key to which [`SystemCall::Set`] will write the value.
pub const VALUE_KEY: &[u8] = b"value"; // 76616c7565
/// Temporary key used to store the header. This should always be clear at the end of the block.
pub const HEADER_KEY: &[u8] = b"header"; // 686561646572
/// Key used to store all extrinsics in a block. Should always remain at the end of the block, and
/// be cleared at the beginning of the next block.
pub const EXTRINSICS_KEY: &[u8] = b"extrinsics"; // 65787472696e736963

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

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub struct RuntimeCallWithTip {
	pub tip: Option<Balance>,
	pub call: RuntimeCall,
}

/// Our tests will use the given types to interact with your runtime. Note that you can use any
/// other type on your runtime type, as long as you can convert it to/from these types.
pub type Extrinsic = generic::UncheckedExtrinsic<AccountId, RuntimeCallWithTip, Signature, ()>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, Extrinsic>;

/// The account balance struct that we expect to find under `BalancesMap ++ account_id`.
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct AccountBalance {
	pub free: Balance,
	pub reserved: Balance,
}
