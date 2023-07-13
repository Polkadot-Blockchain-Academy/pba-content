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

use shared::Dispatchable;
use sp_api::impl_runtime_apis;
use sp_runtime::{
	create_runtime_str,
	generic::{self},
	traits::{BlakeTwo256, Block as BlockT, Verify},
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

use sp_core::{crypto::UncheckedFrom, hexdisplay::HexDisplay, OpaqueMetadata, H256};
use sp_runtime::traits::Hash;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use crate::shared::{DispatchError, RuntimeCall};

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
	pub use super::Runtime;
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
		storage.top.insert(well_known_keys::CODE.into(), WASM_BINARY.unwrap().to_vec());

		// if you want more data in your genesis, add it here.
		Ok(())
	}
}

pub const VALUE_KEY: &[u8] = b"value"; // 76616c7565
pub const HEADER_KEY: &[u8] = b"header"; // 686561646572
pub const EXTRINSICS_KEY: &[u8] = b"extrinsics"; // 65787472696e736963

/// The main struct in this module. In frame this comes from `construct_runtime!` macro.
#[derive(Debug, Encode, Decode, PartialEq, Eq, Clone)]
pub struct Runtime;

pub struct Minter;
impl shared::Get<AccountId> for Minter {
	fn get() -> AccountId {
		// TODO: make sp-keyring available in no-std
		let bytes =
			hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
		AccountId::unchecked_from(bytes)
	}
}

pub struct MinimumBalance;
impl shared::Get<Balance> for MinimumBalance {
	fn get() -> Balance {
		10
	}
}

impl currency::Config for Runtime {
	const MODULE_ID: &'static str = "CURRENCY";
	type Minter = Minter;
	type MinimumBalance = MinimumBalance;
	type Balance = shared::Balance;
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

	fn dispatch_extrinsic(ext: Extrinsic) -> shared::DispatchResult {
		log::debug!(target: LOG_TARGET, "dispatching {:?}", ext);

		let sender =
			Self::do_check_signature(&ext).map_err(|_| DispatchError::Other("badSignature"))?;

		// execute it
		match ext.function {
			RuntimeCall::System(shared::SystemCall::Set { value }) => {
				sp_io::storage::set(VALUE_KEY, &value.encode());
			},
			RuntimeCall::System(shared::SystemCall::Remark { data: _ }) => {},
			RuntimeCall::System(shared::SystemCall::Upgrade { code }) => {
				sp_io::storage::set(sp_core::storage::well_known_keys::CODE, &code);
			},
			RuntimeCall::Currency(currency_call) => {
				currency_call.dispatch(sender)?;
			},
			_ => unimplemented!(),
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
	use super::*;
	use parity_scale_codec::Encode;
	use shared::{Extrinsic, RuntimeCall};
	use sp_core::hexdisplay::HexDisplay;
	use sp_io::TestExternalities;
	use sp_runtime::traits::Extrinsic as _;

	fn set_value_call(value: u32) -> RuntimeCall {
		RuntimeCall::System(shared::SystemCall::Set { value })
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
}

#[cfg(test)]
mod integration_tests {
	use crate::{
		shared::{AccountId, Balance, SystemCall},
		VALUE_KEY,
	};

	use super::{export as submission, shared};
	use parity_scale_codec::{Decode, Encode};
	use shared::{Block, Extrinsic, Header, RuntimeCall};
	use sp_api::runtime_decl_for_core::CoreV4;
	use sp_block_builder::runtime_decl_for_block_builder::BlockBuilderV6;
	use sp_io::TestExternalities;
	use sp_runtime::traits::Extrinsic as _;

	trait RuntimeT: CoreV4<Block> + BlockBuilderV6<Block> {}
	impl<T> RuntimeT for T where T: CoreV4<Block> + BlockBuilderV6<Block> {}

	fn author<R: RuntimeT>(exts: Vec<Extrinsic>) -> Block {
		let header = Header {
			parent_hash: Default::default(),
			number: 0,
			state_root: Default::default(),
			extrinsics_root: Default::default(),
			digest: Default::default(),
		};
		let mut extrinsics = vec![];

		let mut auth_state = TestExternalities::new_empty();

		auth_state.execute_with(|| {
			R::initialize_block(&header);
		});

		for ext in exts {
			auth_state.execute_with(|| {
				let _dispatch_result = R::apply_extrinsic(ext.clone());
			});
			extrinsics.push(ext);
		}

		let header = auth_state.execute_with(|| R::finalize_block());
		let block = Block { extrinsics, header };

		block
	}

	fn author_and_import<R: RuntimeT>(
		state: &mut TestExternalities,
		exts: Vec<Extrinsic>,
		post: impl FnOnce() -> (),
	) {
		let block = author::<R>(exts);
		state.execute_with(|| {
			R::execute_block(block.clone());
		});

		state.commit_all().unwrap();
		assert_eq!(&block.header.state_root, state.backend.root());

		state.execute_with(|| post());
	}

	fn balance_of(who: AccountId) -> Option<Balance> {
		let key = [b"BalancesMap".as_ref(), who.encode().as_ref()].concat();
		sp_io::storage::get(&key).and_then(|bytes| Balance::decode(&mut &bytes[..]).ok())
	}

	fn issuance() -> Option<Balance> {
		let key = b"TotalIssuance".as_ref();
		sp_io::storage::get(&key).and_then(|bytes| Balance::decode(&mut &bytes[..]).ok())
	}

	fn sign(call: RuntimeCall, signer: &sp_keyring::AccountKeyring) -> Extrinsic {
		let payload = (call).encode();
		let signature = signer.sign(&payload);
		Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap()
	}

	#[test]
	fn empty_block() {
		let mut state = TestExternalities::new_empty();
		state.execute_with(|| assert!(sp_io::storage::get(crate::VALUE_KEY).is_none()));
		author_and_import::<submission::Runtime>(&mut state, vec![], || {});
	}

	#[test]
	fn remark() {
		sp_tracing::try_init_simple();

		let call = RuntimeCall::System(SystemCall::Remark { data: vec![42, 42] });
		let exts = vec![sign(call, &sp_keyring::AccountKeyring::Alice)];

		let mut state = TestExternalities::new_empty();

		author_and_import::<submission::Runtime>(&mut state, exts, || {
			assert!(sp_io::storage::get(VALUE_KEY).is_none())
		});
	}

	#[test]
	fn basic_setup_works() {
		sp_tracing::try_init_simple();

		let call = RuntimeCall::System(super::shared::SystemCall::Set { value: 42 });
		let exts = vec![sign(call, &sp_keyring::AccountKeyring::Alice)];

		let mut state = TestExternalities::new_empty();

		state.execute_with(|| assert!(sp_io::storage::get(crate::VALUE_KEY).is_none()));
		author_and_import::<submission::Runtime>(&mut state, exts, || {
			assert!(sp_io::storage::get(crate::VALUE_KEY).is_some())
		});
	}

	mod currency_tests {
		use super::*;
		use crate::{currency, Runtime};
		use sp_keyring::AccountKeyring::*;

		#[test]
		fn mint_wrong_minter() {
			sp_tracing::try_init_simple();

			// bob account cannot mint.
			let mut state = TestExternalities::new_empty();

			let call = RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
				dest: Alice.public(),
				amount: 20,
			});
			let exts = vec![sign(call, &Bob)];

			author_and_import::<submission::Runtime>(&mut state, exts, || {
				assert!(balance_of(Alice.public()).is_none());
				assert!(issuance().is_none());
			});
		}

		#[test]
		fn mint_success() {
			sp_tracing::try_init_simple();

			// can mint if alice
			let call = RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
				dest: Bob.public(),
				amount: 20,
			});
			let exts = vec![sign(call, &Alice)];

			let mut state = TestExternalities::new_empty();

			author_and_import::<submission::Runtime>(&mut state, exts, || {
				assert_eq!(balance_of(Bob.public()), Some(20));
				assert!(balance_of(Alice.public()).is_none());
				assert_eq!(issuance(), Some(20));
			});
		}

		#[test]
		fn multi_mint_success() {
			sp_tracing::try_init_simple();

			// can mint multiple times if alice
			let exts = vec![
				sign(
					RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
						dest: Bob.public(),
						amount: 20,
					}),
					&Alice,
				),
				sign(
					RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
						dest: Alice.public(),
						amount: 30,
					}),
					&Alice,
				),
			];

			let mut state = TestExternalities::new_empty();

			author_and_import::<submission::Runtime>(&mut state, exts, || {
				assert_eq!(balance_of(Alice.public()), Some(30));
				assert_eq!(balance_of(Bob.public()), Some(20));
				assert_eq!(issuance(), Some(50));
			});
		}

		#[test]
		fn mixed_mint() {
			sp_tracing::try_init_simple();

			// can mint multiple times if alice
			let exts = vec![
				sign(
					RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
						dest: Bob.public(),
						amount: 20,
					}),
					&Alice,
				),
				sign(
					RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
						dest: Alice.public(),
						amount: 30,
					}),
					&Bob,
				),
			];

			let mut state = TestExternalities::new_empty();

			author_and_import::<submission::Runtime>(&mut state, exts, || {
				assert_eq!(balance_of(Alice.public()), None);
				assert_eq!(balance_of(Bob.public()), Some(20));
				assert_eq!(issuance(), Some(20));
			});
		}

		#[test]
		fn mint_not_enough() {
			// cannot mint amount less than `MinimumBalance`
			sp_tracing::try_init_simple();

			let call = RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
				dest: Bob.public(),
				amount: 5,
			});
			let exts = vec![sign(call, &Alice)];

			let mut state = TestExternalities::new_empty();

			author_and_import::<submission::Runtime>(&mut state, exts, || {
				assert_eq!(balance_of(Bob.public()), None);
				assert_eq!(issuance(), None);
			});
		}

		#[test]
		fn mint_not_enough_edge() {
			// still cannot mint amount less than `MinimumBalance`
			sp_tracing::try_init_simple();

			let call = RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
				dest: Bob.public(),
				amount: 9,
			});
			let exts = vec![sign(call, &Alice)];

			let mut state = TestExternalities::new_empty();

			author_and_import::<submission::Runtime>(&mut state, exts, || {
				assert_eq!(balance_of(Bob.public()), None);
				assert_eq!(issuance(), None);
			});

			// but 10 is ok.
			let call = RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
				dest: Bob.public(),
				amount: 10,
			});
			let exts = vec![sign(call, &Alice)];
			author_and_import::<submission::Runtime>(&mut state, exts, || {
				assert_eq!(balance_of(Bob.public()), Some(10));
				assert_eq!(issuance(), Some(10));
			});
		}

		#[test]
		fn transfer_success() {
			let mut state = TestExternalities::new_empty();

			let exts = vec![
				// mint 100 for bob.
				sign(
					RuntimeCall::Currency(currency::Call::<Runtime>::Mint {
						dest: Bob.public(),
						amount: 100,
					}),
					&Alice,
				),
				// transfer 20 to alice.
				sign(
					RuntimeCall::Currency(currency::Call::<Runtime>::Transfer {
						dest: Alice.public(),
						amount: 20,
					}),
					&Bob,
				),
			];

			author_and_import::<submission::Runtime>(&mut state, exts, || {
				assert_eq!(balance_of(Bob.public()), Some(80));
				assert_eq!(balance_of(Alice.public()), Some(20));
				assert_eq!(issuance(), Some(100));
			});
		}
	}
}

// 	#[test]
// 	fn transfer_more_than_you_can() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(issuance(), 100);

// 		let spendable = free(7) - MinimumBalance::get();

// 		assert_eq!(
// 			Call::<TRutnime>::Transfer { dest: acc(10), amount: spendable + 1 }
// 				.dispatch(acc(7))
// 				.unwrap_err(),
// 			DispatchError::Module {
// 				module_id: "MOD_CURRENCY_TEST",
// 				reason: "InsufficientFunds".to_string()
// 			}
// 		);

// 		assert_eq!(free(7), 100);
// 		assert_eq!(issuance(), 100);
// 	}

// 	#[test]
// 	fn transfer_more_than_you_can_limit() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(issuance(), 100);

// 		let spendable = free(7) - MinimumBalance::get();

// 		assert!(Call::<TRutnime>::Transfer { dest: acc(10), amount: spendable }
// 			.dispatch(acc(7))
// 			.is_ok());

// 		assert_eq!(free(7), 5);
// 		assert_eq!(free(10), 95);
// 		assert_eq!(issuance(), 100);
// 	}

// 	#[test]
// 	fn transfer_all_1() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(issuance(), 100);

// 		assert!(dbg!(
// 			Call::<TRutnime>::Transfer { dest: acc(10), amount: 100 }.dispatch(acc(7))
// 		)
// 		.is_ok());

// 		assert_eq!(free(7), 0);
// 		assert_eq!(free(10), 100);
// 		assert_eq!(issuance(), 100);
// 	}

// 	#[test]
// 	fn transfer_all_2() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(issuance(), 100);

// 		assert!(Call::<TRutnime>::TransferAll { dest: acc(10) }.dispatch(acc(7)).is_ok());

// 		assert_eq!(free(7), 0);
// 		assert_eq!(free(10), 100);
// 		assert_eq!(issuance(), 100);
// 	}

// 	type CurrencyImpl = <TRutnime as staking_module::Config>::Currency;

// 	#[test]
// 	fn reserve_success() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(reserved(7), 0);
// 		assert_eq!(issuance(), 100);

// 		assert!(CurrencyImpl::reserve(acc(7), 20).is_ok());

// 		assert_eq!(free(7), 80);
// 		assert_eq!(reserved(7), 20);
// 		assert_eq!(issuance(), 100);
// 	}

// 	#[test]
// 	fn reserve_more_than_allowed() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(reserved(7), 0);
// 		assert_eq!(issuance(), 100);

// 		assert_eq!(
// 			CurrencyImpl::reserve(acc(7), 96).unwrap_err(),
// 			DispatchError::Module {
// 				module_id: "MOD_CURRENCY_TEST",
// 				reason: "InsufficientFunds".to_string()
// 			}
// 		);

// 		assert_eq!(free(7), 100);
// 		assert_eq!(reserved(7), 0);
// 		assert_eq!(issuance(), 100);
// 	}

// 	#[test]
// 	fn reserve_more_than_allowed_limit() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(reserved(7), 0);
// 		assert_eq!(issuance(), 100);

// 		assert!(CurrencyImpl::reserve(acc(7), 95).is_ok());

// 		assert_eq!(free(7), 5);
// 		assert_eq!(reserved(7), 95);
// 		assert_eq!(issuance(), 100);
// 	}
// }

// mod staking_tests {
// 	use super::*;

// 	// note: we test these, but they are basically same as testing bonding.
// 	#[test]
// 	fn bonding_success() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(reserved(7), 0);
// 		assert_eq!(issuance(), 100);

// 		assert!(staking_module::Call::<TRutnime>::Bond { amount: 20 }.dispatch(acc(7)).is_ok());

// 		assert_eq!(free(7), 80);
// 		assert_eq!(reserved(7), 20);
// 		assert_eq!(issuance(), 100);
// 	}

// 	#[test]
// 	fn bonding_more_than_allowed() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(reserved(7), 0);
// 		assert_eq!(issuance(), 100);

// 		assert_eq!(
// 			staking_module::Call::<TRutnime>::Bond { amount: 96 }
// 				.dispatch(acc(7))
// 				.unwrap_err(),
// 			DispatchError::Module {
// 				module_id: "MOD_CURRENCY_TEST",
// 				reason: "InsufficientFunds".to_string()
// 			}
// 		);

// 		assert_eq!(free(7), 100);
// 		assert_eq!(reserved(7), 0);
// 		assert_eq!(issuance(), 100);
// 	}

// 	#[test]
// 	fn bonding_more_than_allowed_limit() {
// 		setup();
// 		assert_eq!(free(7), 100);
// 		assert_eq!(reserved(7), 0);
// 		assert_eq!(issuance(), 100);

// 		assert!(staking_module::Call::<TRutnime>::Bond { amount: 95 }.dispatch(acc(7)).is_ok());

// 		assert_eq!(free(7), 5);
// 		assert_eq!(reserved(7), 95);
// 		assert_eq!(issuance(), 100);
// 	}
// }
