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

pub(crate) mod shared;
mod solution;

use log::info;
use parity_scale_codec::{Decode, Encode};
use shared::Block;

use sp_api::impl_runtime_apis;
use sp_runtime::{
	create_runtime_str,
	generic::{self},
	traits::{BlakeTwo256, Block as BlockT},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_storage::well_known_keys;

#[cfg(any(feature = "std", test))]
use sp_runtime::{BuildStorage, Storage};

use sp_core::{hexdisplay::HexDisplay, OpaqueMetadata, H256};
use sp_runtime::traits::Hash;

#[cfg(feature = "std")]
use sp_version::NativeVersion;

use sp_version::RuntimeVersion;

use crate::shared::{EXTRINSICS_KEY, HEADER_KEY};

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

/// The main struct in this module. In frame this comes from `construct_runtime!` macro.
#[derive(Debug, Encode, Decode, PartialEq, Eq, Clone)]
pub struct Runtime;

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

	fn do_initialize_block(header: &<Block as BlockT>::Header) {
		info!(
			target: LOG_TARGET,
			"Entering initialize_block. header: {:?} / version: {:?}", header, VERSION.spec_version
		);
		sp_io::storage::set(&HEADER_KEY, &header.encode());
		sp_io::storage::clear(&EXTRINSICS_KEY);
	}

	fn do_finalize_block() -> <Block as BlockT>::Header {
		// fetch the header that was given to us at the beginning of the block.
		let mut header = Self::get_state::<<Block as BlockT>::Header>(HEADER_KEY)
			.expect("We initialized with header, it never got mutated, qed");
		// and make sure to _remove_ it.
		sp_io::storage::clear(&HEADER_KEY);

		// TODO: set correct state root and extrinsics root, as described in the corresponding test
		// case.
		Self::solution_finalize_block(&mut header);
		header
	}

	fn do_execute_block(block: Block) {
		info!(target: LOG_TARGET, "Entering execute_block. block: {:?}", block);
		sp_io::storage::clear(&EXTRINSICS_KEY);

		for extrinsic in block.clone().extrinsics {
			let _outcome = Runtime::do_apply_extrinsic(extrinsic)
				.expect("the only possible failure is signature check, which must have already been checked by `validate_transaction`; cannot fail; qed");
		}

		// check state root
		sp_io::storage::clear(&HEADER_KEY);

		// NOTE: if we forget to do this, how can you mess with the blockchain?
		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
		let state_root = H256::decode(&mut &raw_state_root[..]).unwrap();
		assert_eq!(block.header.state_root, state_root);

		// check extrinsics root
		let extrinsics = block.extrinsics.into_iter().map(|x| x.encode()).collect::<Vec<_>>();
		let extrinsics_root = BlakeTwo256::ordered_trie_root(extrinsics, Default::default());
		assert_eq!(block.header.extrinsics_root, extrinsics_root);

		info!(target: LOG_TARGET, "Finishing block import.");
		Self::print_state();
	}

	/// Apply a single extrinsic.
	///
	/// If an internal error occurs during the dispatch, such as "insufficient funds" etc, we don't
	/// care about which variant of `DispatchError` you return. But, if a bad signature is provided,
	/// then `Err(InvalidTransaction::BadProof)` must be returned.
	fn do_apply_extrinsic(ext: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
		info!(target: LOG_TARGET, "Entering apply_extrinsic: {:?}", ext);

		// note the extrinsic
		Self::mutate_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY, |current| {
			current.push(ext.encode());
		});

		// TODO: we don't have a means of dispatch, implement it!
		// let outcome = Ok(());
		Runtime::solution_apply_extrinsic(ext)
	}

	fn do_validate_transaction(
		source: TransactionSource,
		ext: <Block as BlockT>::Extrinsic,
		block_hash: <Block as BlockT>::Hash,
	) -> TransactionValidity {
		log::debug!(target: LOG_TARGET,"Entering validate_transaction. tx: {:?}", ext);
		// TODO: we don't have a means of validating, implement it!
		// Ok(Default::default())
		Runtime::solution_validate_transaction(source, ext, block_hash)
	}
}

impl_runtime_apis! {
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
	use super::*;
	use crate::shared::RuntimeCallWithTip;
	use parity_scale_codec::Encode;
	use shared::{Extrinsic, RuntimeCall, VALUE_KEY};
	use sp_core::hexdisplay::HexDisplay;
	use sp_io::TestExternalities;
	use sp_runtime::{
		traits::Extrinsic as _,
		transaction_validity::{InvalidTransaction, TransactionValidityError},
	};

	fn set_value_call(value: u32) -> RuntimeCallWithTip {
		RuntimeCallWithTip {
			call: RuntimeCall::System(shared::SystemCall::Set { value }),
			tip: None,
		}
	}

	fn unsigned_set_value(value: u32) -> Extrinsic {
		let call = RuntimeCallWithTip {
			call: RuntimeCall::System(shared::SystemCall::Set { value }),
			tip: None,
		};
		Extrinsic::new(call, None).unwrap()
	}

	fn signed_set_value(value: u32) -> Extrinsic {
		let call = set_value_call(value);
		let signer = sp_keyring::AccountKeyring::Alice;
		let payload = (call).encode();
		let signature = signer.sign(&payload);
		Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap()
	}

	#[test]
	fn encode_examples() {
		// demonstrate some basic encodings. Example usage:
		//
		// ```
		// wscat -c 127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"state_getStorage", "params": ["0x123"]}'
		// wscat -c ws://127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"author_submitExtrinsic", "params": ["0x123"]}'
		// ```
		let unsigned = Extrinsic::new_unsigned(set_value_call(42));
		println!("unsigned = {:?} {:?}", unsigned, HexDisplay::from(&unsigned.encode()));

		let signer = sp_keyring::AccountKeyring::Alice;
		let call = set_value_call(42);
		let payload = (call).encode();
		let signature = signer.sign(&payload);
		let signed = Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap();
		println!("signed {:?} {:?}", signed, HexDisplay::from(&signed.encode()));

		println!("value key = {:?}", HexDisplay::from(&VALUE_KEY));
	}

	#[test]
	fn host_function_call_works() {
		// this is just to demonstrate to you that you should always wrap any code containing host
		// functions in `TestExternalities`.
		TestExternalities::new_empty().execute_with(|| {
			sp_io::storage::get(&VALUE_KEY);
		})
	}

	#[test]
	fn signed_set_value_works() {
		// A signed `Set` works.
		let ext = signed_set_value(42);
		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			Runtime::do_apply_extrinsic(ext).unwrap().unwrap();
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), Some(42));
		});
	}

	#[test]
	fn bad_signature_fails() {
		// A poorly signed extrinsic must fail.
		let signer = sp_keyring::AccountKeyring::Alice;
		let call = set_value_call(42);
		let bad_call = set_value_call(43);
		let payload = (bad_call).encode();
		let signature = signer.sign(&payload);
		let ext = Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap();

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(
				Runtime::do_apply_extrinsic(ext).unwrap_err(),
				TransactionValidityError::Invalid(InvalidTransaction::BadProof)
			);
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
		});
	}

	#[test]
	fn unsigned_set_value_does_not_work() {
		// An unsigned `Set` must fail.
		let ext = unsigned_set_value(42);

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(
				Runtime::do_apply_extrinsic(ext).unwrap_err(),
				TransactionValidityError::Invalid(InvalidTransaction::BadProof)
			);
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
		});
	}

	#[test]
	fn validate_works() {
		// An unsigned `Set` cannot be validated. Same should go for one with a bad signature.
		let ext = unsigned_set_value(42);

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(
				Runtime::do_validate_transaction(
					TransactionSource::External,
					ext,
					Default::default()
				)
				.unwrap_err(),
				TransactionValidityError::Invalid(InvalidTransaction::BadProof)
			);
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
		});
	}

	#[test]
	fn import_and_author_equal() {
		let ext1 = signed_set_value(42);
		let ext2 = signed_set_value(43);
		let ext3 = signed_set_value(44);

		let header = shared::Header {
			digest: Default::default(),
			extrinsics_root: Default::default(),
			parent_hash: Default::default(),
			number: 0,
			state_root: Default::default(),
		};

		let block = TestExternalities::new_empty().execute_with(|| {
			Runtime::do_initialize_block(&header);
			drop(header);

			Runtime::do_apply_extrinsic(ext1.clone()).unwrap().unwrap();
			Runtime::do_apply_extrinsic(ext2.clone()).unwrap().unwrap();
			Runtime::do_apply_extrinsic(ext3.clone()).unwrap().unwrap();

			let header = Runtime::do_finalize_block();

			assert!(
				sp_io::storage::get(HEADER_KEY).is_none(),
				"header must have been cleared from storage"
			);
			let extrinsics = sp_io::storage::get(EXTRINSICS_KEY)
				.and_then(|bytes| <Vec<Vec<u8>> as Decode>::decode(&mut &*bytes).ok())
				.unwrap();
			assert_eq!(extrinsics.len(), 3, "incorrect extrinsics recorded in state");

			let expected_state_root = {
				let raw_state_root = &sp_io::storage::root(Default::default())[..];
				H256::decode(&mut &raw_state_root[..]).unwrap()
			};
			let expected_extrinsics_root =
				BlakeTwo256::ordered_trie_root(extrinsics, Default::default());

			assert_eq!(
				header.state_root, expected_state_root,
				"block finalization should set correct state root in header"
			);
			assert_eq!(
				header.extrinsics_root, expected_extrinsics_root,
				"block finalization should set correct extrinsics root in header"
			);

			Block { extrinsics: vec![ext1, ext2, ext3], header }
		});

		TestExternalities::new_empty().execute_with(|| {
			// This should internally check state/extrinsics root. If it does not panic, then we are
			// gucci.
			Runtime::do_execute_block(block.clone());
		})
	}
}
