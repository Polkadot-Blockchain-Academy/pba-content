//! Lesson ideas:
//!
//! ## Info points:
//!
//! - a few words about how to run this.
//! - a few words about how to add dependencies, and the whole /std thing. Could be part of the
//!   `substrate` crate docs.
//!
//! ## Exercise Steps:
//!
//! 1. Remove the code needed to set the right state root from the template, and ask students to
//! submit extrinsics and see it fail. They have to fix it.
//!
//! 2. Remove the parts needed to do a code upgrade. They have to add it again, and make sure it
//! works. Double check that they bump the spec version. Tell them how they can cause a version
//! mismatch if they don't.
//!
//! 3.1: use the same to nuke your chain.
//!
//! 3. Re-implement `mini_substrate` here. Some of the same traits are provided here, and you can
//! reuse them.
//!
//! 4. Add signatures to your runtime.
//!
//! 5. Add a custom tip to each extrinsic, if it is signed. It must be fraud proof of course!
//!
//! ## Cheat sheet
//!
//! ```
//! cargo remote run -- -- --dev --consensus manual-seal-1000
//! wscat -c ws://127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"author_submitExtrinsic", "params": ["0x14002a000000"]}'
//! wscat -c 127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"state_getStorage", "params": ["0x76616c7565"]}'
//! ```

// This instructs
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

const LOG_TARGET: &'static str = "frameless";

use log::info;
use parity_scale_codec::{Compact, Decode, Encode};
use scale_info::TypeInfo;

use sp_api::impl_runtime_apis;
use sp_runtime::{
	create_runtime_str,
	generic::{self},
	traits::{BlakeTwo256, Block as BlockT, Extrinsic},
	transaction_validity::{
		InvalidTransaction, TransactionSource, TransactionValidity, TransactionValidityError,
		ValidTransaction,
	},
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

/// Opaque types. This is what the lectures referred to as `ClientBlock`. Notice how
/// `OpaqueExtrinsic` is merely a `Vec<u8>`.
pub mod opaque {
	use super::*;
	type OpaqueExtrinsic = sp_runtime::OpaqueExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, OpaqueExtrinsic>;
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
	state_version: 0,
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
		storage.top.insert(well_known_keys::CODE.into(), WASM_BINARY.unwrap().to_vec());

		// if you want more data in your genesis, add it here.

		Ok(())
	}
}

pub type BlockNumber = u32;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, BasicExtrinsic>;

/// Our call type. This enum should encode the different extrinsics that you can call into.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum Call {
	/// Set the value under [`VALUE_KEY`]
	Set(u32),
	/// Upgrade the chain's code to the given bytes.
	Upgrade(Vec<u8>),
}

/// A simple extrinsic type.
///
/// For now, given that we don't have any signatures or anything, it doesn't have anything other
/// than the inner `Call`.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, TypeInfo)]
pub struct BasicExtrinsic(Call);

// This custom is a very nuanced detail, see if you can make sense out of it, and else ask your
// instructor/TA about it.
//
// Hint: it is related to `opaque::OpaqueExtrinsic`.
impl Encode for BasicExtrinsic {
	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		let call = self.0.encode();
		let size = Compact::<u32>(call.len() as u32).encode();
		dest.write(&size);
		dest.write(&call);
	}
}

impl Decode for BasicExtrinsic {
	fn decode<I: parity_scale_codec::Input>(
		input: &mut I,
	) -> Result<Self, parity_scale_codec::Error> {
		// we don't really need th length when we decode this to its concrete type.
		let _expected_length: Compact<u32> = Decode::decode(input)?;
		let call: Call = Decode::decode(input)?;
		Ok(BasicExtrinsic(call))
	}
}

#[cfg(test)]
impl BasicExtrinsic {
	fn new_unsigned(call: Call) -> Self {
		<Self as Extrinsic>::new(call, None).unwrap()
	}
}

impl Extrinsic for BasicExtrinsic {
	type Call = Call;
	type SignaturePayload = ();

	fn new(data: Self::Call, _: Option<Self::SignaturePayload>) -> Option<Self> {
		Some(Self(data))
	}
}

pub const VALUE_KEY: &[u8] = b"value"; // 76616c7565
pub const HEADER_KEY: &[u8] = b"header"; // 686561646572
pub const EXTRINSICS_KEY: &[u8] = b"extrinsic"; // 65787472696e736963

/// The main struct in this module. In frame this comes from `construct_runtime!` macro.
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

	fn dispatch_extrinsic(ext: BasicExtrinsic) -> Result<(), ()> {
		log::debug!(target: LOG_TARGET, "dispatching {:?}", ext);
		// note the extrinsic:
		Self::mutate_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY, |current| {
			current.push(ext.encode());
		});

		// execute it
		match ext.0 {
			Call::Set(val) => sp_io::storage::set(VALUE_KEY, &val.encode()),
			Call::Upgrade(code) =>
				sp_io::storage::set(sp_core::storage::well_known_keys::CODE, &code),
		}

		Ok(())
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

		Self::print_state();
		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];

		let extrinsics = Self::get_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY).unwrap_or_default();
		let extrinsics_root =
			BlakeTwo256::ordered_trie_root(extrinsics, sp_core::storage::StateVersion::V0);

		header.extrinsics_root = extrinsics_root;
		header.state_root = sp_core::H256::decode(&mut &raw_state_root[..]).unwrap();

		info!(target: LOG_TARGET, "finalizing block {:?}", header);
		header
	}

	fn do_execute_block(block: Block) {
		info!(target: LOG_TARGET, "Entering execute_block. block: {:?}", block);
		sp_io::storage::clear(&EXTRINSICS_KEY);

		for extrinsic in block.clone().extrinsics {
			// block import cannot fail.
			Runtime::dispatch_extrinsic(extrinsic).unwrap();
		}

		// check state root
		// NOTE: if we forget to do this, how can you mess with the blockchain?
		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
		let state_root = H256::decode(&mut &raw_state_root[..]).unwrap();
		Self::print_state();
		assert_eq!(block.header.state_root, state_root);

		// check extrinsics root
		let extrinsics = block.extrinsics.into_iter().map(|x| x.encode()).collect::<Vec<_>>();
		let extrinsics_root =
			BlakeTwo256::ordered_trie_root(extrinsics, sp_core::storage::StateVersion::V0);
		assert_eq!(block.header.extrinsics_root, extrinsics_root);
	}

	fn do_apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
		info!(target: LOG_TARGET, "Entering apply_extrinsic: {:?}", extrinsic);

		Self::dispatch_extrinsic(extrinsic)
			.map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::Custom(0)))?;

		Ok(Ok(()))
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

		let data = tx.0;
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
		fn offchain_worker(_header: &<Block as BlockT>::Header) {
			// we do not do anything.
		}
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
	use parity_scale_codec::Encode;
	use sp_core::hexdisplay::HexDisplay;

	#[test]
	fn host_function_call_works() {
		sp_io::TestExternalities::new_empty().execute_with(|| {
			sp_io::storage::get(&HEADER_KEY);
		})
	}

	#[test]
	fn opaque_encoding_works() {
		let ext = BasicExtrinsic::new_unsigned(Call::Set(42));
		let encoded = ext.encode();
		let concrete_decoded: BasicExtrinsic = Decode::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(concrete_decoded, ext);
		let opaque_decoded: Vec<u8> = Decode::decode(&mut encoded.as_slice()).unwrap();
		let concrete_call_from_opaque: Call =
			Decode::decode(&mut opaque_decoded.as_slice()).unwrap();
		assert_eq!(concrete_call_from_opaque, Call::Set(42))
	}

	#[test]
	fn upgrade_call() {
		// use std::io::Write;
		// let wasm = include_bytes!("../../frameless_runtime.wasm");
		// let call = Call::Upgrade(wasm.to_vec());
		// let ext = BasicExtrinsic::new(call, None).unwrap();
		// let mut file = std::fs::File::create("../payload.json").unwrap();
		// let payload = format!(
		// 	r#"{{
		// 		"jsonrpc":"2.0",
		// 		"id":1, "method":"author_submitExtrinsic",
		// 		"params": ["0x{:?}"]
		// 	}}"#,
		// 	HexDisplay::from(&ext.encode())
		// );
		// file.write_all(payload.as_bytes()).unwrap();
	}

	#[test]
	fn encode_examples() {
		let extrinsic = BasicExtrinsic::new_unsigned(Call::Set(42));
		println!("ext {:?}", HexDisplay::from(&extrinsic.encode()));
	}
}
