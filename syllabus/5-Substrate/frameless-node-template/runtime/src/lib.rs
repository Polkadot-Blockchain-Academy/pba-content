//! Welcome to the `Frameless` exercise, the third edition.
//!
//! > This assignment is based on Joshy's experiment, years ago, to explore building a Substrate
//! > runtime using pure Rust. If you learn something new in this exercise, attribute it to his
//! > work. We hope you to also explore new possibilities, and share it with other for education.
//!
//! > This assignment builds on top of the `mini_substrate` section of the pre-course material. It
//! > is highly recommended to re-familiarize yourself with that first.
//!
//! ## Context
//!
//! As the name suggest, this is Frameless runtime. It is a fully substrate-compatible runtime,
//! which you can easily run with companion `node`, without using `frame`.
//!
//! To run the `node`, execute `cargo run -- --dev`, possibly with `--release`. `--dev` will ensure
//! that a new database is created each time, and your chain starts afresh.
//!
//! While you are welcome to explore the `node` folder, it is not part of this assignment, and you
//! can leave it as-is.
//!
//! This node uses a testing block-authoring/consensus scheme in which a block is produced at fixed
//! intervals. See `--consensus` cli option.
//!
//! ## Warm Up
//!
//! First, study the runtime code with the help of your instructor. You will soon realize that it is
//! missing some key components. Most notably, the logic to:
//!
//! 1. apply extrinsics
//! 2. validate extrinsics for the tx-pool
//! 3. finalize a block upon authoring
//!
//! are not complete. Your first task is to complete them to some extent, and make sure all local
//! tests are passing. To do so, you won't need to alter tx-pool api yet, but you should provide a
//! simple `apply_extrinsic`, and finish `finalize_block`.
//!
//! The third objective above will ensure that when you author blocks, you set the correct state
//! root and extrinsics root in the block header. This makes the block be a valid import-able block.
//! This test demonstrates this property:
#![doc = docify::embed!("src/lib.rs", import_and_author_equal)]
//!
//! Also, if you run your chain with two nodes, you will be able to test this property. See Hints
//! section below.
//!
//! As a part of both, you will realize that you are asked to implement proper signature checking.
//! In this assignment, we are using the types from [`sp_runtime::generic`] (see [`shared`]).
//! Studying these types and how the handle signatures within themselves should help you implement
//! proper signature checking.
//!
//! All in all, your runtime should only work with signed extrinsics, and
//! instantly reject unsigned (or badly signed) extrinsics. See the following tests:
#![doc = docify::embed!("src/lib.rs", bad_signature_fails)]
//!
#![doc = docify::embed!("src/lib.rs", unsigned_set_value_does_not_work)]
//!
//! By the end of this section, you should fix the aforementioned 3 parts of your runtime, implement
//! proper dispatch logic for [`shared::SystemCall`], and this should enable you yo pass all tests
//! in this file. Moreover, your should be able to play with your blockchain, through running a
//! node, and interacting with it via `curl`, `wscat` or a similar tool. See `encode_examples` test.
//!
//! > Most of [`shared::SystemCall`] instances are for you to use for learning. Try and upgrade your
//! > chain using [`shared::SystemCall::Upgrade`]!
//!
//! TODO: prepare a diff for everything thus far to hand over at some point, or live code it. TODO:
//! clear graph somewhere of the flow of import and authoring.
//!
//! ## Main Task
//!
//! Once you are done with the above, you can start your main objective, which is to re-implement
//! everything you have done for `mini_substrate` here. That is, implement a simple currency system,
//! with abilities to mint, transfer and reserve, and a staking system on top of it.
//!
//! ## Specification, Grading
//!
//! ### Grading
//!
//! Unlike `mini_substrate`, this assignment is primarily graded through a wasm executor, not by
//! looking at the internals of your runtime. Manual grading is a small part. This means you should
//! be very careful about adhering to the rules and specifications.
//!
//! Automatic Wasm grading means:
//!
//! * we do not care about the internal of your runtime, other than the standard set of runtime
//!   apis.
//! * but we do care about your storage layout being exactly as described in `mini_substrate`.
//! * we do care about the extrinsic format being exactly as described in [`shared`].
//!
//! While we can't force you not to change [`shared`] module, we use an exact copy of this file to
//! craft extrinsics/blocks to interact with your runtime, and we expect to find the types mentioned
//! in there (eg. [`shared::AccountBalance`]) to be what we decode from your storage.
//!
//! That being said, you can use types that are equivalent to their encoding to the ones mentioned
//! in [`shared`].
//!
//! > Example: You are still free to use your more generic currency module from `mini_substrate`,
//! > and as long as you correctly configure it to use [`shared::Balance`] as its balance type,
//! > everything should work just fine. Remember that two _different types_ can have the _same
//! > encoding_.
//!
//! ### Dispatchables
//!
//! Similar to `mini_substrate`, the exact expectation of behavior for each instance of
//! [`shared::RuntimeCallExt`] is document in its own documentation. This should be identical to
//! what you had to code for `mini_substrate`.
//!
//! > As noted above, whether you want to use a trait like `Dispatchable` or not is up to you.
//!
//! ### Storage
//!
//! Exact same expectation as `mini_substrate`.
//!
//! * mapping [`shared::AccountId`] to [`shared::AccountBalance`] kept at `BalancesMap +
//!   encode(account)`.
//! * value of type [`shared::Balance`] for total issuance kept at `TotalIssuance`.
//!
//! > Again, you are free to use the same storage abstractions as in `mini_substrate` or not. We
//! > highly advice you do ;)
//!
//! ### Additional Logic
//!
//! Notable new desired behavior, compared to `mini_substrate`:
//!
//! #### 1. Tipping
//!
//! The final [`shared::Extrinsic`]'s `Call`, namely [`shared::RuntimeCallExt`] contains `tip`
//! field. As the name suggests, this is some additional funds that are sent by the user to chain.
//! Other than this optional tip, all other extrinsics are free.
//!
//! Tipped extrinsics are prioritized over non-tipped ones through the virtue of a higher
//! `priority`. This is further explained in `validate_transaction` section below.
//!
//! Deducting the tip from the sender must happen prior to executing anything else in the extrinsic.
//! Failure to pay for fees is always a full failure of the extrinsic (similar to a failed signature
//! check).
//!
//! Once the tip is deducted, it is added to the an account id specified by [`shared::TREASURY`].
//! The same rules about account creation apply to this account as well.
//!
//! Total issuance must be kept up to date in all of the cases above.
//!
//! #### 2. Account Creation/Deletion
//!
//! An account that is destroyed should not be kept in storage with a value like
//! `Default::default()`. Instead, it should be removed from storage. This is crucial to save space
//! in your blockchain.
//!
//! As a rule of thumb, you should avoid storing `Some(Default::default())` in your storage.
//! Instead, simply remove them.
//!
//! #### Nonce
//!
//! You should implement a nonce system, as explained as a part of the tx-pool lecture. In short,
//! the validation of each transaction should `require` nonce `(sender, n-1)` and provide `(sender,
//! n)`. See `TaggedTransactionQueue` below for more information.
//!
//! Note that your nonce should be checked as a part of transaction pool api, which means it should
//! be implemented as efficiently as possibly, next to other checks that need to happen.
//!
//! ### `BlockBuilder::apply_extrinsic`
//!
//! One of your objectives is to implement the logic for `apply_extrinsic`. Here, we describe what
//! return value we expect from it.
//!
//! Recall that [`ApplyExtrinsicResult`] is essentially a nested `Result`. The outer one says
//! whether _applying_ the extrinsic to the block was fine, and the inner one says whether the
//! extrinsic itself was *dispatched* fine.
//!
//! For example, a failed transfer will still reside in a block, and is *applied* successfully, but
//! it is not *dispatched* successfully. So such an extrinsic should something like `Ok(Err(_))` in
//! its `apply_extrinsic`.
//!
//! Your `apply_extrinsic` should:
//!
//! * Return `Err` with [`sp_runtime::transaction_validity::TransactionValidityError::Invalid`] and
//!   [`sp_runtime::transaction_validity::InvalidTransaction::BadProof`] if the extrinsic has an
//!   invalid signature.
//! * Return `Err` with [`sp_runtime::transaction_validity::TransactionValidityError::Invalid`] and
//!   [`sp_runtime::transaction_validity::InvalidTransaction::Payment`] if the extrinsic cannot pay
//!   for its declared tip.
//! * Return `Err` with [`sp_runtime::transaction_validity::TransactionValidityError::Future`] or
//!   `Stale` if the nonce is too high or too low.
//!
//! In all other cases, outer `Result` is `Ok`.
//!
//! * If the inner dispatch is failing, your return value should look like `Ok(Err(_))`, and we
//!   don't care which variant of `DispatchError` you return.
//!
//! ### `TaggedTransactionQueue::validate_transaction`
//!
//! Recall that the return type of `validate_transaction` is
//! [`sp_runtime::transaction_validity::TransactionValidity`] which is simply a `Result`. Similar to
//! the above, your `validate_transaction` implementation must:
//!
//! * Return `Err` with [`sp_runtime::transaction_validity::TransactionValidityError::Invalid`] and
//!   [`sp_runtime::transaction_validity::InvalidTransaction::BadProof`] if the extrinsic has an
//!   invalid signature.
//! * Return `Err` with [`sp_runtime::transaction_validity::TransactionValidityError::Invalid`] and
//!   [`sp_runtime::transaction_validity::InvalidTransaction::Payment`] if the extrinsic cannot pay
//!   for its declared tip.
//! * Return `Err` with [`sp_runtime::transaction_validity::TransactionValidityError::Future`] or
//!   `Stale` if the nonce is too high or too low.
//!
//! Moreover, if the signature and tip are valid, the
//! [`sp_runtime::transaction_validity::ValidTransaction::priority`] must be set to the tip value
//! (use a simple saturated conversion if needed).
//!
//! ### `Core_execute_block`
//!
//! The `execute_block` expects a valid block in which all transactions will get included. That is,
//! it will expect all `ApplyExtrinsicResult` to be `Ok(_)`. Note that a failed dispatch is
//! acceptable, like `Ok(Err(_))`.
//!
//! You should not need to change this API, but studying it will be fruitful.
//!
//! ## Hints
//!
//! ### Block Authoring vs. Import
//!
//! Recall that the api call flow in block authoring is:
//!
//! ```ignore
//! Core::initialize_block(raw_header);
//! loop {
//!     BlockBuilder::apply_extrinsic
//! }
//! BlockBuilder::finalize_block() -> final_header
//! ```
//!
//! The client builds a raw header that has `number` and a few other fields set, but no roots yet,
//! and passes it to the runtime in `initialize_block`. The runtime stored this raw header at this
//! point, and intends to return its final version in `finalize_block`.
//!
//! When importing a block, the api call flow is:
//!
//! ```ignore
//! Core::initialize_block(final_header);
//! Core::import_block(block);
//! ```
//!
//! End of the day, you must ensure that the above two code paths come to the same state root, and
//! record it in the block header, along with the root of all extrinsics.
//!
//! ### Logging
//!
//! Logging can be enabled by setting the `RUST_LOG` environment variable, as such:
//!
//! ```ignore
//! RUST_LOG=frameless=debug cargo run
//! ```
//!
//! Or equally:
//!
//! ```ignore
//! cargo run -- --dev -l frameless=debug
//! ```
//!
//! ### Running Two Nodes
//!
//! In order to run two nodes, execute the following commands in two different terminals.
//!
//! ```ignore
//! cargo run -- --dev --alice -l frameless=debug
//! cargo r -- --dev --bob -l frameless=debug --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<node-id-of-alice>
//! ```
//!
//! If you let the former `--alice` node progress for a bit, you will see that `--bob` will start
//! syncing from alice.
//!
//! ### Extra: `SignedExtensions`
//!
//! What we have implemented in this extra as added fields to our [`shared::RuntimeCallExt`] should
//! have ideally been implemented as a signed extension. In a separate branch, explore this, and ask
//! for our feedback. If make progress on this front, DO NOT submit it for grading, as our grading
//! will work with the simpler `RuntimeCallExt` model.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

const LOG_TARGET: &'static str = "frameless";

pub mod shared;
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
		// make sure to not remove this, as it might break the node code.
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
		let header = Self::get_state::<<Block as BlockT>::Header>(HEADER_KEY)
			.expect("We initialized with header, it never got mutated, qed");
		// and make sure to _remove_ it.
		sp_io::storage::clear(&HEADER_KEY);

		// TODO: set correct state root and extrinsics root, as described in the corresponding test
		// case.
		let header = Self::solution_finalize_block(header);
		header
	}

	/// Your code path to execute a block that has been previously authored.
	///
	/// Study this carefully, but you probably don't need to change it, other than providing a
	/// proper `do_apply_extrinsic`.
	fn do_execute_block(block: Block) {
		info!(target: LOG_TARGET, "Entering execute_block. block: {:?}", block);
		sp_io::storage::clear(&EXTRINSICS_KEY);

		for extrinsic in block.clone().extrinsics {
			let _outcome = Runtime::do_apply_extrinsic(extrinsic)
				.expect("A block author has provided us with an invalid block; bailing; qed");
		}

		// check state root. Clean the state prior to asking for the root.
		sp_io::storage::clear(&HEADER_KEY);

		// NOTE: if we forget to do this, how can you mess with the blockchain?
		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
		let state_root = H256::decode(&mut &raw_state_root[..]).unwrap();
		assert_eq!(block.header.state_root, state_root);

		// check extrinsics root
		let extrinsics = block.extrinsics.into_iter().map(|x| x.encode()).collect::<Vec<_>>();
		let extrinsics_root =
			BlakeTwo256::ordered_trie_root(extrinsics, sp_runtime::StateVersion::V0);
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

		// TODO: we don't have a means of dispatch, implement it! You probably want to match on
		// `ext.call.function`, and start implementing different arms one at a time.
		Self::solution_apply_extrinsic(ext.clone()).map(|outcome| {
			// note the extrinsic
			Self::mutate_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY, |current| {
				current.push(ext.encode());
			});
			outcome
		})
		// Ok(Ok(()))
	}

	fn do_validate_transaction(
		_source: TransactionSource,
		ext: <Block as BlockT>::Extrinsic,
		_block_hash: <Block as BlockT>::Hash,
	) -> TransactionValidity {
		log::debug!(target: LOG_TARGET,"Entering validate_transaction. tx: {:?}", ext);
		// TODO: we don't have a means of validating, implement it!
		Self::solution_validate_transaction(_source, ext, _block_hash)
		// Ok(Default::default())
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
	use crate::shared::RuntimeCallExt;
	use parity_scale_codec::Encode;
	use shared::{Extrinsic, RuntimeCall, VALUE_KEY};
	use sp_core::hexdisplay::HexDisplay;
	use sp_io::TestExternalities;
	use sp_runtime::{
		traits::Extrinsic as _,
		transaction_validity::{InvalidTransaction, TransactionValidityError},
	};

	fn set_value_call(value: u32, nonce: u32) -> RuntimeCallExt {
		RuntimeCallExt {
			call: RuntimeCall::System(shared::SystemCall::Set { value }),
			tip: None,
			nonce,
		}
	}

	fn unsigned_set_value(value: u32) -> Extrinsic {
		let call = RuntimeCallExt {
			call: RuntimeCall::System(shared::SystemCall::Set { value }),
			tip: None,
			nonce: 0,
		};
		Extrinsic::new(call, None).unwrap()
	}

	fn signed_set_value(value: u32, nonce: u32) -> Extrinsic {
		let call = set_value_call(value, nonce);
		let signer = sp_keyring::AccountKeyring::Alice;
		let payload = (call).encode();
		let signature = signer.sign(&payload);
		Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap()
	}

	#[test]
	#[docify::export]
	fn host_function_call_works() {
		// this is just to demonstrate to you that you should always wrap any code containing host
		// functions in `TestExternalities`.
		TestExternalities::new_empty().execute_with(|| {
			sp_io::storage::get(&VALUE_KEY);
		})
	}

	#[test]
	#[docify::export]
	fn encode_examples() {
		// demonstrate some basic encodings. Example usage:
		//
		// ```
		// wscat -c 127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"state_getStorage", "params": ["0x123"]}'
		// wscat -c ws://127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"author_submitExtrinsic", "params": ["0x123"]}'
		// ```
		let unsigned = Extrinsic::new_unsigned(set_value_call(42, 0));

		let signer = sp_keyring::AccountKeyring::Alice;
		let call = set_value_call(42, 0);
		let payload = (call).encode();
		let signature = signer.sign(&payload);
		let signed = Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap();

		println!("unsigned = {:?} {:?}", unsigned, HexDisplay::from(&unsigned.encode()));
		println!("signed {:?} {:?}", signed, HexDisplay::from(&signed.encode()));
		println!("value key = {:?}", HexDisplay::from(&VALUE_KEY));
	}

	#[test]
	#[docify::export]
	fn signed_set_value_works() {
		// A signed `Set` works.
		let ext = signed_set_value(42, 0);
		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			Runtime::do_apply_extrinsic(ext).unwrap().unwrap();
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), Some(42));
		});
	}

	#[test]
	#[docify::export]
	fn bad_signature_fails() {
		// A poorly signed extrinsic must fail.
		let signer = sp_keyring::AccountKeyring::Alice;
		let call = set_value_call(42, 0);
		let bad_call = set_value_call(43, 0);
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
	#[docify::export]
	fn unsigned_set_value_does_not_work() {
		// An unsigned `Set` must fail as well.
		let ext = unsigned_set_value(42);

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(
				Runtime::do_apply_extrinsic(ext).unwrap_err(),
				TransactionValidityError::Invalid(InvalidTransaction::BadProof)
			);
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			todo!("assert here that this extrinsic has not been noted");
		});
	}

	#[test]
	#[docify::export]
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
	#[docify::export]
	fn import_and_author_equal() {
		let ext1 = signed_set_value(42, 0);
		let ext2 = signed_set_value(43, 1);
		let ext3 = signed_set_value(44, 2);

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
				BlakeTwo256::ordered_trie_root(extrinsics, sp_runtime::StateVersion::V0);

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
		});
	}
}
