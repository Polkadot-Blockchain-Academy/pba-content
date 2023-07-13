//! Lesson ideas:
//!
//! ## Info points:
//!
//! - a few words about how to run this.
//! - a few words about how to add dependencies, and the whole /std thing. Could be part of the
//!   `substrate` crate docs.
//! - order of api calls recap.
//!
//! ## Exercise Steps:
//!
//! * Remove the code needed to set the right state root from the template, and ask students to
//! submit extrinsics and see it fail. They have to fix it.
//!
//! * Remove the parts needed to do a code upgrade. They have to add it again, and make sure it
//! works. Double check that they bump the spec version. Tell them how they can cause a version
//! mismatch if they don't.
//!
//! *: use the same to nuke your chain.
//!
//! * Re-implement `mini_substrate` here. Some of the same traits are provided here, and you can
//! reuse them.
//!
//! * Add signatures to your runtime.
//!
//! * Add a custom fee to each extrinsic, if it is signed. It must be fraud proof of course!
//!
//! * Make dispatchables transactional, such that if they return `Err()`, their state update is
//! reverted, but not the fee payment.
//!
//! ## Cheat sheet
//!
//! ```ignore
//! cargo remote run -- -- --dev --consensus manual-seal-1000
//! wscat -c ws://127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"author_submitExtrinsic", "params": ["0x14002a000000"]}'
//! wscat -c 127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"state_getStorage", "params": ["0x76616c7565"]}'
//! ```
//!
//! TODO: don't mix some types that are defined in both `shared` and `sp_runtime`.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

const LOG_TARGET: &'static str = "frameless";

mod currency;
mod shared;
mod staking;
mod storage;

use log::info;
use parity_scale_codec::{Decode, Encode};
use shared::{AccountId, Balance, Block, Extrinsic};

use sp_api::impl_runtime_apis;
use sp_runtime::{
	create_runtime_str,
	generic::{self},
	traits::{BlakeTwo256, Block as BlockT, Verify},
	transaction_validity::{TransactionSource, TransactionValidity, ValidTransaction},
	ApplyExtrinsicResult,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_storage::well_known_keys;

#[cfg(any(feature = "std", test))]
use sp_runtime::{BuildStorage, Storage};

use sp_core::{crypto::UncheckedFrom, hexdisplay::HexDisplay, OpaqueMetadata, H256};
use sp_runtime::traits::Hash;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use crate::shared::RuntimeCall;

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

/// Something that can be dispatched.
///
/// This is typically implemented for various `Call` enums.
pub trait Dispatchable {
	/// Dispatch self, assuming the given `sender`.
	fn dispatch(self, sender: AccountId) -> DispatchResult;
}

pub trait Get<T> {
	fn get() -> T;
}

/// Opaque types. This is what the lectures referred to as `ClientBlock`. Notice how
/// `OpaqueExtrinsic` is merely a `Vec<u8>`.
pub mod opaque {
	use super::*;
	type OpaqueExtrinsic = sp_runtime::OpaqueExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<shared::BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, OpaqueExtrinsic>;
}

pub mod export {
	pub use super::RuntimeGenesisConfig;
}

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("frameless-runtime"),
	impl_name: create_runtime_str!("frameless-runtime"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

/// The type that provides the genesis storage values for a new chain.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize, Default))]
pub struct RuntimeGenesisConfig;

#[cfg(feature = "std")]
impl BuildStorage for RuntimeGenesisConfig {
	fn assimilate_storage(&self, storage: &mut Storage) -> Result<(), String> {
		// make sure to not remove this, as our tests always expect the WASM blob to be in the
		// default state created by your `RuntimeGenesisConfig`.
		storage.top.insert(well_known_keys::CODE.into(), WASM_BINARY.unwrap().to_vec());

		// if you want more data in your genesis, add it here.
		Ok(())
	}
}

pub const HEADER_KEY: &[u8] = b"header"; // 686561646572
pub const EXTRINSICS_KEY: &[u8] = b"extrinsics"; // 65787472696e736963

/// The main struct in this module. In frame this comes from `construct_runtime!` macro.
#[derive(Debug, Encode, Decode, PartialEq, Eq, Clone)]
pub struct Runtime;

pub struct Minter;
impl Get<AccountId> for Minter {
	fn get() -> AccountId {
		AccountId::unchecked_from(shared::MINTER)
	}
}

pub struct MinimumBalance;
impl Get<Balance> for MinimumBalance {
	fn get() -> Balance {
		shared::MINIMUM_BALANCE
	}
}

impl currency::Config for Runtime {
	const MODULE_ID: &'static str = "CURRENCY";
	type Minter = Minter;
	type MinimumBalance = MinimumBalance;
	type Balance = shared::Balance;
}

impl From<shared::CurrencyCall> for currency::Call<Runtime> {
	fn from(value: shared::CurrencyCall) -> Self {
		match value {
			shared::CurrencyCall::Transfer { dest, amount } =>
				currency::Call::<Runtime>::Transfer { dest, amount },
			shared::CurrencyCall::Mint { dest, amount } =>
				currency::Call::<Runtime>::Mint { dest, amount },
			shared::CurrencyCall::TransferAll { dest } =>
				currency::Call::<Runtime>::TransferAll { dest },
		}
	}
}

impl staking::Config for Runtime {
	type Currency = currency::Module<Runtime>;
}

impl From<shared::StakingCall> for staking::Call<Runtime> {
	fn from(value: shared::StakingCall) -> Self {
		match value {
			shared::StakingCall::Bond { amount } => staking::Call::<Runtime>::Bond { amount },
		}
	}
}

impl Runtime {
	fn print_state() {
		let mut key = vec![];
		while let Some(next) = sp_io::storage::next_key(&key) {
			let val = sp_io::storage::get(&next).unwrap().to_vec();
			log::trace!(
				target: LOG_TARGET,
				"{} <=> {}",
				HexDisplay::from(&next),
				HexDisplay::from(&val)
			);
			key = next;
		}
	}

	fn get_state<T: Decode>(key: &[u8]) -> Option<T> {
		sp_io::storage::get(key).and_then(|d| T::decode(&mut &*d).ok())
	}

	fn mutate_state<T: Decode + Encode + Default>(key: &[u8], update: impl FnOnce(&mut T)) {
		let mut value = Self::get_state(key).unwrap_or_default();
		update(&mut value);
		sp_io::storage::set(key, &value.encode());
	}

	fn dispatch_extrinsic(ext: Extrinsic) -> DispatchResult {
		log::debug!(target: LOG_TARGET, "dispatching {:?}", ext);

		let sender =
			Self::do_check_signature(&ext).map_err(|_| DispatchError::Other("badSignature"))?;

		// TODO: handle ext.function.tip.
		// execute it
		match ext.function.call {
			RuntimeCall::System(shared::SystemCall::Set { value }) => {
				sp_io::storage::set(shared::VALUE_KEY, &value.encode());
			},
			RuntimeCall::System(shared::SystemCall::Remark { data: _ }) => {},
			RuntimeCall::System(shared::SystemCall::Upgrade { code }) => {
				sp_io::storage::set(sp_core::storage::well_known_keys::CODE, &code);
			},
			RuntimeCall::Currency(currency_call) => {
				let my_call: currency::Call<Runtime> = currency_call.into();
				my_call.dispatch(sender)?;
			},
			RuntimeCall::Staking(staking_call) => {
				let my_call: staking::Call<Runtime> = staking_call.into();
				my_call.dispatch(sender)?;
			},
		}

		Ok(())
	}

	fn apply_extrinsic_with_rollback(ext: Extrinsic) {
		// note the extrinsic:
		Self::mutate_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY, |current| {
			current.push(ext.encode());
		});

		sp_io::storage::start_transaction();

		let outcome = Self::dispatch_extrinsic(ext);

		if outcome.is_ok() {
			sp_io::storage::commit_transaction();
		} else {
			sp_io::storage::rollback_transaction();
		}
	}

	// NOTE: this header is different when you are importing a block, and when you are executing a
	// block. When you are importing a block, it is a full header. When you are building a block, it
	// is a raw, unfinished header. For example, its state root is not yet quite known.
	fn do_initialize_block(header: &<Block as BlockT>::Header) {
		info!(
			target: LOG_TARGET,
			"Entering initialize_block. header: {:?} / version: {:?}", header, VERSION.spec_version
		);
		sp_io::storage::set(&HEADER_KEY, &header.encode());
		sp_io::storage::clear(&EXTRINSICS_KEY);
	}

	// NOTE: this should now write anything to state, because it is discarded. Moreover, this is not
	// called in the import code path, so it MUST not write anything to state, otherwise it would
	// lead to a consensus error, i.e. block import and authoring leading to different results. It
	// should only return a header that is valid.
	fn do_finalize_block() -> <Block as BlockT>::Header {
		let mut header = Self::get_state::<<Block as BlockT>::Header>(HEADER_KEY)
			.expect("We initialized with header, it never got mutated, qed");

		// the header itself contains the state root, so it cannot be inside the state (circular
		// dependency..). Make sure in execute block path we have the same rule.
		sp_io::storage::clear(&HEADER_KEY);

		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];

		let extrinsics = Self::get_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY).unwrap_or_default();
		let extrinsics_root =
			BlakeTwo256::ordered_trie_root(extrinsics, sp_core::storage::StateVersion::V0);

		header.extrinsics_root = extrinsics_root;
		header.state_root = sp_core::H256::decode(&mut &raw_state_root[..]).unwrap();

		info!(target: LOG_TARGET, "finalizing block {:?}", header);
		Self::print_state();
		header
	}

	fn do_execute_block(block: Block) {
		info!(target: LOG_TARGET, "Entering execute_block. block: {:?}", block);
		sp_io::storage::clear(&EXTRINSICS_KEY);

		for extrinsic in block.clone().extrinsics {
			// block import cannot fail.
			Runtime::apply_extrinsic_with_rollback(extrinsic);
		}

		// check state root
		sp_io::storage::clear(&HEADER_KEY);

		// NOTE: if we forget to do this, how can you mess with the blockchain?
		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
		let state_root = H256::decode(&mut &raw_state_root[..]).unwrap();
		assert_eq!(block.header.state_root, state_root);

		// check extrinsics root
		let extrinsics = block.extrinsics.into_iter().map(|x| x.encode()).collect::<Vec<_>>();
		let extrinsics_root =
			BlakeTwo256::ordered_trie_root(extrinsics, sp_core::storage::StateVersion::V0);
		assert_eq!(block.header.extrinsics_root, extrinsics_root);

		info!(target: LOG_TARGET, "Finishing block import.");
		Self::print_state();
	}

	fn do_apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
		info!(target: LOG_TARGET, "Entering apply_extrinsic: {:?}", extrinsic);

		Self::apply_extrinsic_with_rollback(extrinsic);

		Ok(Ok(()))
	}

	fn do_check_signature(ext: &<Block as BlockT>::Extrinsic) -> Result<AccountId, ()> {
		match &ext.signature {
			Some((signer, signature, extra_stuff)) => {
				let payload = (ext.function.clone(), extra_stuff);
				signature
					.verify(payload.encode().as_ref(), signer)
					.then(|| signer.clone())
					.ok_or(())
			},
			None => Err(()),
		}
	}

	fn do_validate_transaction(
		source: TransactionSource,
		tx: <Block as BlockT>::Extrinsic,
		block_hash: <Block as BlockT>::Hash,
	) -> TransactionValidity {
		log::debug!(
			target: LOG_TARGET,
			"Entering validate_transaction. source: {:?}, tx: {:?}, block hash: {:?}",
			source,
			tx,
			block_hash
		);

		// we don't know how to validate this -- It should be fine??
		let data = tx.function;
		Ok(ValidTransaction { provides: vec![data.encode()], ..Default::default() })
	}
}

impl_runtime_apis! {
	// https://substrate.dev/rustdocs/master/sp_api/trait.Core.html
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Self::do_execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Self::do_initialize_block(header)
		}
	}

	// https://substrate.dev/rustdocs/master/sc_block_builder/trait.BlockBuilderApi.html
	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Self::do_apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Self::do_finalize_block()
		}

		fn inherent_extrinsics(_data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			Default::default()
		}

		fn check_inherents(
			_block: Block,
			_data: sp_inherents::InherentData
		) -> sp_inherents::CheckInherentsResult {
			Default::default()
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Self::do_validate_transaction(source, tx, block_hash)
		}
	}

	// Ignore everything after this.
	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Default::default())
		}

		fn metadata_at_version(_version: u32) -> Option<OpaqueMetadata> {
			Default::default()
		}

		fn metadata_versions() -> sp_std::vec::Vec<u32> {
			Default::default()
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(_header: &<Block as BlockT>::Header) {}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(_: Option<Vec<u8>>) -> Vec<u8> {
			Default::default()
		}

		fn decode_session_keys(
			_: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			Default::default()
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::shared::RuntimeCallWithTip;

	use super::*;
	use parity_scale_codec::Encode;
	use shared::{Extrinsic, RuntimeCall, VALUE_KEY};
	use sp_core::hexdisplay::HexDisplay;
	use sp_io::TestExternalities;
	use sp_runtime::traits::Extrinsic as _;

	fn set_value_call(value: u32) -> RuntimeCallWithTip {
		let call = RuntimeCall::System(shared::SystemCall::Set { value });
		RuntimeCallWithTip { call, tip: None }
	}

	#[test]
	fn host_function_call_works() {
		TestExternalities::new_empty().execute_with(|| {
			sp_io::storage::get(&HEADER_KEY);
		})
	}

	#[test]
	fn unsigned_set_value_does_not_work() {
		let ext = Extrinsic::new_unsigned(set_value_call(42));

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert!(Runtime::dispatch_extrinsic(ext).is_err());
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
		});
	}

	#[test]
	fn signed_set_value_works() {
		let signer = sp_keyring::AccountKeyring::Alice;
		let call = set_value_call(42);
		let payload = (call).encode();
		let signature = signer.sign(&payload);
		let ext = Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap();

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			Runtime::dispatch_extrinsic(ext).unwrap();
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), Some(42));
		});
	}

	#[test]
	fn bad_signature_fails() {
		let signer = sp_keyring::AccountKeyring::Alice;
		let call = set_value_call(42);
		let bad_call = set_value_call(43);
		let payload = (bad_call).encode();
		let signature = signer.sign(&payload);
		let ext = Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap();

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert!(Runtime::dispatch_extrinsic(ext).is_err());
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
		});
	}

	#[test]
	fn encode_examples() {
		let extrinsic = Extrinsic::new_unsigned(set_value_call(42));
		println!("ext {:?}", HexDisplay::from(&extrinsic.encode()));
	}

	#[test]
	fn roots_are_set() {}
}
