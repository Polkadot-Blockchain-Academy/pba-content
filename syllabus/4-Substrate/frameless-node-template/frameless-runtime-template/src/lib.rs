#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use parity_scale_codec::{Decode, Encode};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

use log::info;

use sp_api::{impl_runtime_apis, HashT};
use sp_runtime::{
	create_runtime_str,
	generic::{self},
	impl_opaque_keys,
	traits::{BlakeTwo256, Block as BlockT, Extrinsic},
	transaction_validity::{
		InvalidTransaction, TransactionSource, TransactionValidity, TransactionValidityError,
		ValidTransaction,
	},
	ApplyExtrinsicResult, BoundToRuntimeAppPublic,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_storage::well_known_keys;

#[cfg(any(feature = "std", test))]
use sp_runtime::{BuildStorage, Storage};

use sp_core::{hexdisplay::HexDisplay, OpaqueMetadata, H256};

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datas-tructures.
pub mod opaque {
	use super::*;
	type OpaqueExtrinsic = BasicExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, OpaqueExtrinsic>;

	// This part is necessary for generating session keys in the runtime
	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: AuraAppPublic,
			pub grandpa: GrandpaAppPublic,
		}
	}

	// Typically these are not implemented manually, but rather for the pallet associated with the
	// keys. Here we are not using the pallets, and these implementations are trivial, so we just
	// re-write them.
	pub struct AuraAppPublic;
	impl BoundToRuntimeAppPublic for AuraAppPublic {
		type Public = AuraId;
	}

	pub struct GrandpaAppPublic;
	impl BoundToRuntimeAppPublic for GrandpaAppPublic {
		type Public = sp_finality_grandpa::AuthorityId;
	}
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("frameless-runtime-template"),
	impl_name: create_runtime_str!("frameless-runtime-template"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

/// The type that provides the genesis storage values for a new chain
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Default))]
pub struct GenesisConfig;

#[cfg(feature = "std")]
impl BuildStorage for GenesisConfig {
	fn assimilate_storage(&self, storage: &mut Storage) -> Result<(), String> {
		// we have nothing to put into storage in genesis, except this:
		storage.top.insert(well_known_keys::CODE.into(), WASM_BINARY.unwrap().to_vec());

		Ok(())
	}
}

pub type BlockNumber = u32;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, BasicExtrinsic>;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, parity_util_mem::MallocSizeOf))]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub enum Call {
	Foo,
}

// this extrinsic type does nothing other than fulfill the compiler.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, parity_util_mem::MallocSizeOf))]
#[derive(Debug, Encode, Decode, PartialEq, Eq, Clone)]
// #[derive(Encode, Decode)]
pub struct BasicExtrinsic(Call);

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

impl sp_runtime::traits::GetNodeBlockType for Runtime {
	type NodeBlock = opaque::Block;
}

impl sp_runtime::traits::GetRuntimeBlockType for Runtime {
	type RuntimeBlock = Block;
}

const LOG_TARGET: &'static str = "frameless";
const BLOCK_TIME: u64 = 3000;

pub const HEADER_KEY: &[u8] = b"header"; // 686561646572

// just FYI:
// :code => 3a636f6465

/// The main struct in this module. In frame this comes from `construct_runtime!`
pub struct Runtime;

type DispatchResult = Result<(), ()>;

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

	fn dispatch_extrinsic(ext: BasicExtrinsic) -> DispatchResult {
		log::debug!(target: LOG_TARGET, "dispatching {:?}", ext);

		// execute it
		// TODO..

		Ok(())
	}

	fn do_initialize_block(header: &<Block as BlockT>::Header) {
		info!(
			target: LOG_TARGET,
			"Entering initialize_block. header: {:?} / version: {:?}", header, VERSION.spec_version
		);
		sp_io::storage::set(&HEADER_KEY, &header.encode());
	}

	fn do_finalize_block() -> <Block as BlockT>::Header {
		let mut header = Self::get_state::<<Block as BlockT>::Header>(HEADER_KEY)
			.expect("We initialized with header, it never got mutated, qed");

		// the header itself contains the state root, so it cannot be inside the state (circular
		// dependency..). Make sure in execute block path we have the same rule.
		sp_io::storage::clear(&HEADER_KEY);

		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];

		// header.extrinsics_root = ???;
		header.state_root = sp_core::H256::decode(&mut &raw_state_root[..]).unwrap();

		info!(target: LOG_TARGET, "finalizing block {:?}", header);
		header
	}

	fn do_execute_block(block: Block) {
		info!(target: LOG_TARGET, "Entering execute_block. block: {:?}", block);

		for extrinsic in block.clone().extrinsics {
			// block import cannot fail.
			Runtime::dispatch_extrinsic(extrinsic).unwrap();
		}

		// check state root
		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
		let state_root = H256::decode(&mut &raw_state_root[..]).unwrap();
		Self::print_state();
		assert_eq!(block.header.state_root, state_root);

		// check extrinsics root.
		let extrinsics =
			block.extrinsics.into_iter().map(|x| x.encode()).collect::<Vec<_>>();
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

	fn do_inherent_extrinsics(_: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
		log::debug!(target: LOG_TARGET, "Entering do_inherent_extrinsics");
		Default::default()
	}

	fn do_check_inherents(
		_: Block,
		_: sp_inherents::InherentData,
	) -> sp_inherents::CheckInherentsResult {
		log::debug!(target: LOG_TARGET, "Entering do_check_inherents");
		Default::default()
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

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			Self::do_inherent_extrinsics(data)
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData
		) -> sp_inherents::CheckInherentsResult {
			Self::do_check_inherents(block, data)
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
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(_header: &<Block as BlockT>::Header) {
			// we do not do anything.
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			info!(target: "frameless", "🖼️ Entering generate_session_keys. seed: {:?}", seed);
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(BLOCK_TIME)
		}

		fn authorities() -> Vec<AuraId> {
			// The only authority is Alice. This makes things work nicely in `--dev` mode
			use sp_application_crypto::ByteArray;

			vec![
				AuraId::from_slice(
					&hex_literal::hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").to_vec()
				).unwrap()
			]
		}
	}

	impl sp_finality_grandpa::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> sp_finality_grandpa::AuthorityList {
			use sp_application_crypto::ByteArray;
			vec![
				(
					sp_finality_grandpa::AuthorityId::from_slice(
						&hex_literal::hex!("88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee").to_vec()
					).unwrap(),
					1
				)
			]
		}

		fn current_set_id() -> sp_finality_grandpa::SetId {
			0u64
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: sp_finality_grandpa::EquivocationProof<
				<Block as BlockT>::Hash,
				sp_runtime::traits::NumberFor<Block>,
			>,
			_key_owner_proof: sp_finality_grandpa::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: sp_finality_grandpa::SetId,
			_authority_id: sp_finality_grandpa::AuthorityId,
		) -> Option<sp_finality_grandpa::OpaqueKeyOwnershipProof> {
			None
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
	fn encode_examples() {
		let extrinsic = BasicExtrinsic::new_unsigned(Call::Foo);
		println!("ext {:?}", HexDisplay::from(&extrinsic.encode()));
	}
}
