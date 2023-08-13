use std::cell::RefCell;

use crate::shared::HEADER_KEY;
use parity_scale_codec::{Decode, Encode};
use runtime::shared::TREASURY;
use shared::{
	AccountBalance, AccountId, Balance, Block, CurrencyCall, Extrinsic, Header, RuntimeCall,
	RuntimeCallExt, StakingCall, SystemCall, EXTRINSICS_KEY, VALUE_KEY,
};
use sp_api::{HashT, TransactionValidity};
use sp_core::{
	crypto::UncheckedFrom,
	traits::{CallContext, CodeExecutor, Externalities},
	H256,
};
use sp_io::TestExternalities;
use sp_keyring::AccountKeyring::*;
use sp_runtime::{
	traits::{BlakeTwo256, Block as BlockT, Extrinsic as _},
	transaction_validity::{InvalidTransaction, TransactionSource, TransactionValidityError},
	ApplyExtrinsicResult,
};

// FOR NEXT TIME:
// * The requirement to leave noted extrinsics in the state is not super clear. This one
// tests will check it. Nonetheless, the `import_and_author_equal` will check it. We will only
// check it here, but not in other tests.
// * The specification is not super clear about validation of future transactions not being a
//   failure.
// * Clear distinction between apply and dispatch.
// * SudoSet vs. SudoRemark
// * add sp_tracing boilerplate everywhere.
// * the only way to kill should be TransferAll, not Transfer.

mod shared;

const LOG_TARGET: &'static str = "grading";

thread_local! {
	pub static CALLED_AUTHOR_AND_IMPORT: RefCell<bool> = RefCell::new(false);
}

fn balance_of(who: AccountId) -> Option<AccountBalance> {
	let key = [b"BalancesMap".as_ref(), who.as_ref()].concat();
	sp_io::storage::get(&key).and_then(|bytes| AccountBalance::decode(&mut &bytes[..]).ok())
}

fn free_of(who: AccountId) -> Option<Balance> {
	balance_of(who).map(|b| b.free)
}

fn nonce_of(who: AccountId) -> Option<u32> {
	balance_of(who).map(|b| b.nonce)
}

fn treasury() -> Option<Balance> {
	free_of(AccountId::unchecked_from(TREASURY))
}

#[allow(unused)]
fn reserve_of(who: AccountId) -> Option<Balance> {
	balance_of(who).map(|b| b.reserved)
}

fn issuance() -> Option<Balance> {
	let key = b"TotalIssuance".as_ref();
	sp_io::storage::get(&key).and_then(|bytes| Balance::decode(&mut &bytes[..]).ok())
}

fn sp_io_root() -> H256 {
	H256::decode(&mut &sp_io::storage::root(Default::default())[..][..]).unwrap()
}

fn signed(call: RuntimeCall, signer: &sp_keyring::AccountKeyring, nonce: u32) -> Extrinsic {
	let call_with_tip = RuntimeCallExt { tip: None, call, nonce };
	let payload = (call_with_tip).encode();
	let signature = signer.sign(&payload);
	Extrinsic::new(call_with_tip, Some((signer.public(), signature, ()))).unwrap()
}

fn tipped(
	call: RuntimeCall,
	signer: &sp_keyring::AccountKeyring,
	nonce: u32,
	tip: Balance,
) -> Extrinsic {
	let call_with_tip = RuntimeCallExt { tip: Some(tip), call, nonce };
	let payload = (call_with_tip).encode();
	let signature = signer.sign(&payload);
	Extrinsic::new(call_with_tip, Some((signer.public(), signature, ()))).unwrap()
}

fn unsigned(call: RuntimeCall) -> Extrinsic {
	let call_with_tip = RuntimeCallExt { tip: None, call, nonce: 0 };
	Extrinsic::new(call_with_tip, None).unwrap()
}

fn validate(to_validate: Extrinsic, state: &mut TestExternalities) -> TransactionValidity {
	let return_bytes = executor_call(
		state,
		"TaggedTransactionQueue_validate_transaction",
		(TransactionSource::External, to_validate, <Block as BlockT>::Hash::default())
			.encode()
			.as_slice(),
	)
	.expect(
		"calling into TaggedTransactionQueue_validate_transaction failed, panic must have happened in the runtime",
	);

	// decode the bytes as a TransactionValidity.
	<TransactionValidity as Decode>::decode(&mut &*return_bytes).unwrap()
}

fn apply(ext: Extrinsic, state: &mut TestExternalities) -> ApplyExtrinsicResult {
	let return_bytes = executor_call(
		state,
		"BlockBuilder_apply_extrinsic",
		ext.encode().as_slice(),
	)
	.expect(
		"calling into BlockBuilder_apply_extrinsic failed, panic must have happened in the runtime",
	);

	// decode the bytes as a ApplyExtrinsicResult.
	<ApplyExtrinsicResult as Decode>::decode(&mut &*return_bytes).unwrap()
}

fn author_and_import(
	import_state: &mut TestExternalities,
	exts: Vec<Extrinsic>,
	post: impl FnOnce() -> (),
) {
	CALLED_AUTHOR_AND_IMPORT.with(|c| {
		assert!(!*c.borrow(), "author_and_import called already in a test thread?");
		*c.borrow_mut() = true
	});

	// ensure ext has some code in it, otherwise something is wrong.
	let code = import_state
		.execute_with(|| sp_io::storage::get(&sp_core::storage::well_known_keys::CODE).unwrap());
	assert!(code.len() > 0);

	let header = Header {
		parent_hash: Default::default(),
		number: 0,
		state_root: Default::default(),
		extrinsics_root: Default::default(),
		digest: Default::default(),
	};

	log::info!(target: LOG_TARGET, "authoring a block with {:?}.", exts.iter().map(|x| x.function.clone()).collect::<Vec<_>>());
	let mut extrinsics = vec![];
	let mut auth_state = TestExternalities::new_with_code(code.as_ref(), Default::default());

	executor_call(&mut auth_state, "Core_initialize_block", &header.encode())
		.expect("Core_initialize_block failed; panic happened in runtime");

	for ext in exts {
		let apply_outcome = apply(ext.clone(), &mut auth_state);
		if apply_outcome.is_ok() {
			extrinsics.push(ext);
		} else {
			log::error!(target: LOG_TARGET, "extrinsic {:?} failed to apply: {:?}", ext, apply_outcome);
		}
	}

	let header: Header =
		executor_call(&mut auth_state, "BlockBuilder_finalize_block", Default::default())
			.map(|data| <Header as Decode>::decode(&mut &*data).unwrap())
			.expect("could not decode header returned from BlockBuilder_finalize_block, diverged from shared.rs?");

	let block = Block { extrinsics, header };
	auth_state.commit_all().unwrap();
	assert_eq!(
		&auth_state.execute_with(|| sp_io_root()),
		auth_state.backend.root(),
		"something is wrong :/"
	);

	// These checks are too greedy and explicit. A correct `execute_block` should already check
	// them, therefore we skip it for now.
	let sanity_checks = false;
	if sanity_checks {
		auth_state.execute_with(|| {
			// check the extrinsics key is set.
			let noted_extrinsics = sp_io::storage::get(EXTRINSICS_KEY)
				.and_then(|bytes| <Vec<Vec<u8>> as Decode>::decode(&mut &*bytes).ok())
				.unwrap_or_default();

			assert_eq!(
				noted_extrinsics.len(),
				block.extrinsics.len(),
				"incorrect extrinsics recorded in state, in block {:?}, noted in `EXTRINSICS_KEY` {:?}",
				block.extrinsics,
				noted_extrinsics
			);

			// check the header key is not set.
			assert!(sp_io::storage::get(&HEADER_KEY).is_none(), "Header key left in storage");

			// check state root.
			assert_eq!(block.header.state_root, sp_io_root(), "incorrect state root");

			// check extrinsics root.
			assert_eq!(
				block.header.extrinsics_root,
				BlakeTwo256::ordered_trie_root(noted_extrinsics, sp_runtime::StateVersion::V0),
				"incorrect extrinsics root",
			);
		});
	}
	drop(auth_state);
	log::debug!(target: LOG_TARGET, "authored the block.");

	// now we import the block into a fresh new state.
	executor_call(import_state, "Core_execute_block", &block.encode())
		.expect("Core_execute_block failed; panic happened in runtime");
	import_state.commit_all().unwrap();

	import_state.execute_with(|| {
		// TODO: these checks ensure that even if both block import and authoring are faulty, a
		// reasonable root was set in storage. Consider what if a student has removed their state
		// root check from everywhere?

		// check state root.
		assert_eq!(
			block.header.state_root,
			sp_io_root(),
			"incorrect state root in authored block after importing"
		);

		// check extrinsics root.
		let expected_extrinsics_root = BlakeTwo256::ordered_trie_root(
			block.extrinsics.into_iter().map(|e| e.encode()).collect::<Vec<_>>(),
			sp_runtime::StateVersion::V0,
		);
		if expected_extrinsics_root != block.header.extrinsics_root {
			panic!(
				"incorrect extrinsics root in authored block after importing: got {:?}, expected
			{:?}",
				block.header.extrinsics_root, expected_extrinsics_root,
			);
			// log::error!(
			// 	target: LOG_TARGET,
			// 	"incorrect extrinsics root in authored block: got {:?}, expected {:?}",
			// 	block.header.extrinsics_root,
			// 	expected_extrinsics_root,
			// );
		}
	});

	log::debug!(target: LOG_TARGET, "all good; running post checks");
	import_state.execute_with(|| post());
}

fn executor_call(t: &mut TestExternalities, method: &str, data: &[u8]) -> Result<Vec<u8>, ()> {
	let mut t = t.ext();

	let code = t.storage(sp_core::storage::well_known_keys::CODE).unwrap();
	let heap_pages = t.storage(sp_core::storage::well_known_keys::HEAP_PAGES);
	let runtime_code = sp_core::traits::RuntimeCode {
		code_fetcher: &sp_core::traits::WrappedRuntimeCode(code.as_slice().into()),
		hash: sp_core::blake2_256(&code).to_vec(),
		heap_pages: heap_pages.and_then(|hp| Decode::decode(&mut &hp[..]).ok()),
	};

	let executor = sc_executor::WasmExecutor::<sp_io::SubstrateHostFunctions>::builder().build();

	let (res, was_native) =
		executor.call(&mut t, &runtime_code, method, data, false, CallContext::Onchain);
	assert!(!was_native);
	if let Err(e) = &res {
		log::error!(target: LOG_TARGET, "executor call into runtime API: {} failed, Err:{:?}", method, e);
	}

	res.map_err(|_| ())
}

fn new_test_ext() -> TestExternalities {
	sp_tracing::try_init_simple();
	let code_path = std::option_env!("WASM_FILE").unwrap_or(if cfg!(debug_assertions) {
		"../target/debug/wbuild/runtime/runtime.wasm"
	} else {
		"../target/release/wbuild/runtime/runtime.wasm"
	});
	let code = std::fs::read(code_path).expect("code should be present");
	log::info!(target: LOG_TARGET, "reading code from {}", code_path);
	let mut storage: sp_core::storage::Storage = Default::default();
	storage
		.top
		.insert(sp_core::storage::well_known_keys::CODE.to_vec(), code.to_vec());
	TestExternalities::new_with_code(&code, storage)
}

mod basics {
	use super::*;

	mod fundamentals {
		use super::*;

		#[test]
		fn empty_block() {
			let mut state = new_test_ext();
			state.execute_with(|| assert!(sp_io::storage::get(VALUE_KEY).is_none()));
			author_and_import(&mut state, vec![], || {
				assert_eq!(balance_of(Alice.public()), None, "empty block has state");
				assert_eq!(balance_of(Bob.public()), None, "empty block has state");
			});
		}

		#[test]
		fn remark() {
			let exts = vec![signed(
				RuntimeCall::System(SystemCall::Remark { data: vec![42, 42] }),
				&Alice,
				0,
			)];
			let mut state = new_test_ext();

			author_and_import(&mut state, exts, || {
				assert!(sp_io::storage::get(VALUE_KEY).is_none(), "remark should not change state");
			});
		}

		#[test]
		fn set_value() {
			let exts = vec![signed(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 0)];
			let mut state = new_test_ext();

			state.execute_with(|| assert!(sp_io::storage::get(VALUE_KEY).is_none()));
			author_and_import(&mut state, exts, || {
				let value_key_value = sp_io::storage::get(VALUE_KEY)
					.and_then(|b| <u32 as Decode>::decode(&mut &*b).ok());
				assert_eq!(
					value_key_value,
					Some(42),
					"set should change state under VALUE_KEY, expected {:?}, got {:?}",
					Some(42),
					value_key_value
				);
			});
		}

		#[test]
		fn apply_unsigned_set_fails() {
			let ext = unsigned(RuntimeCall::System(SystemCall::Set { value: 42 }));
			let mut state = new_test_ext();

			assert_eq!(
				apply(ext, &mut state),
				Err(TransactionValidityError::Invalid(InvalidTransaction::BadProof)),
				"unsigned extrinsic return Err(BadProof) in apply"
			);
		}

		#[test]
		fn apply_bad_signature_fails() {
			let mut ext = signed(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 0);
			let other_sig = {
				signed(RuntimeCall::System(SystemCall::Set { value: 43 }), &Alice, 0)
					.signature
					.unwrap()
					.1
			};
			ext.signature.as_mut().unwrap().1 = other_sig;

			let mut state = new_test_ext();
			assert_eq!(
				apply(ext, &mut state),
				Err(TransactionValidityError::Invalid(InvalidTransaction::BadProof)),
				"bad signature extrinsic return Err(BadProof) in apply"
			);
		}

		#[test]
		fn validate_signed_set_value_okay() {
			let ext = signed(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 0);
			let mut state = new_test_ext();
			let validity = validate(ext, &mut state);

			// For now, we just check that this is ok. We don't check anything else.
			assert!(validity.is_ok(), "signed set value should be Ok(_)");
		}

		#[test]
		fn validate_bad_signature_fails() {
			let mut ext = signed(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 0);
			let other_sig = {
				signed(RuntimeCall::System(SystemCall::Set { value: 43 }), &Alice, 0)
					.signature
					.unwrap()
					.1
			};
			ext.signature.as_mut().unwrap().1 = other_sig;

			let mut state = new_test_ext();
			let validity = validate(ext, &mut state);

			assert_eq!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::BadProof)),
				"bad signature extrinsic return Err(BadProof) in validate_transaction"
			);
		}

		#[test]
		fn validate_unsigned() {
			let ext = unsigned(RuntimeCall::System(SystemCall::Set { value: 42 }));
			let mut state = new_test_ext();

			let validity = validate(ext, &mut state);

			assert_eq!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::BadProof)),
				"unsigned extrinsic return Err(BadProof) in validate_transaction"
			);
		}
	}

	mod optional {}

	mod challenging {
		use super::*;

		#[test]
		fn apply_sudo_remark_by_bob_fails() {
			let mut state = new_test_ext();
			let ext =
				signed(RuntimeCall::System(SystemCall::SudoRemark { data: vec![42] }), &Bob, 0);
			assert!(
				matches!(apply(ext, &mut state), Ok(Err(_))),
				"Bob cannot sudo remark, apply should return Ok(Err(_))"
			);
		}

		#[test]
		fn apply_remark_okay() {
			let mut state = new_test_ext();
			let ext =
				signed(RuntimeCall::System(SystemCall::SudoRemark { data: vec![42] }), &Alice, 0);
			assert!(matches!(apply(ext, &mut state), Ok(Ok(_))));

			let ext = signed(RuntimeCall::System(SystemCall::Remark { data: vec![42] }), &Alice, 1);
			assert!(matches!(apply(ext, &mut state), Ok(Ok(_))));
		}

		#[test]
		fn validate_sudo_remark_by_bob() {
			// Bob won't be able to dispatch this, but we should not need to care about this.
			let ext =
				signed(RuntimeCall::System(SystemCall::SudoRemark { data: vec![42] }), &Bob, 0);
			let mut state = new_test_ext();
			let validity = validate(ext, &mut state);

			// For now, we just check that this is ok. We don't check anything else.
			assert!(
				validity.is_ok(),
				"A sudo remark by bob should still be Ok(_) in validate_transaction"
			);
		}
	}
}

mod currency {
	use super::*;

	mod fundamentals {
		use super::*;
		#[test]
		fn bob_cannot_mint_to_alice() {
			// bob account cannot mint.
			let mut state = new_test_ext();

			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 20 }),
				&Bob,
				0,
			)];

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0, "alice should have 0");
				assert_eq!(issuance().unwrap_or_default(), 0, "issuance should be 0");
			});
		}

		#[test]
		fn alice_can_mint_to_bob() {
			// can mint if alice
			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 20 }),
				&Alice,
				0,
			)];

			let mut state = new_test_ext();

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 20, "bob should have 20");
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0, "alice should have 0");
				assert_eq!(issuance().unwrap_or_default(), 20, "issuance should be 20");
			});
		}

		#[test]
		fn alice_mints_10_to_bob() {
			let mut state = new_test_ext();
			// but 10 is ok.
			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 10 }),
				&Alice,
				0,
			)];

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0, "alice should have 0");
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 10, "bob should have 10");
				assert_eq!(issuance().unwrap_or_default(), 10, "issuance should be 10");
			});
		}

		#[test]
		fn alice_mints_100_to_bob_bob_transfers_20_to_alice() {
			let mut state = new_test_ext();

			let exts = vec![
				// mint 100 for bob.
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				// transfer 20 to alice.
				signed(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: Alice.public(),
						amount: 20,
					}),
					&Bob,
					0,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 80, "bob should have 80");
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 20, "alice should have 20");
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn alice_mints_100_to_bob_bob_transfers_91_to_alice() {
			let mut state = new_test_ext();
			// min balance is 10.
			let spendable = 100 - 10;

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: Alice.public(),
						amount: spendable + 1,
					}),
					&Bob,
					0,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 100, "bob should have 100");
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0, "alice should have 0");
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}
	}

	mod challenging {
		use super::*;
		#[test]
		fn multiple_mints_in_single_block_success_and_failure() {
			// can mint multiple times if alice
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 20 }),
					&Alice,
					0,
				),
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 30 }),
					&Bob,
					0,
				),
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint {
						dest: Charlie.public(),
						amount: 30,
					}),
					&Alice,
					1,
				),
			];

			let mut state = new_test_ext();

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0, "alice should have 0");
				assert_eq!(
					free_of(Charlie.public()).unwrap_or_default(),
					30,
					"charlie should have 30"
				);
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 20, "bob should have 20");
				assert_eq!(issuance().unwrap_or_default(), 50, "issuance should be 50");
			});
		}

		#[test]
		fn alice_mints_100_to_bob_bob_transfers_90_to_alice() {
			let mut state = new_test_ext();
			let spendable = 100 - 10;

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: Alice.public(),
						amount: spendable,
					}),
					&Bob,
					0,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 10, "bob should have 10");
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 90, "alice should have 90");
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn alice_mints_100_to_bob_bob_transfers_100_to_alice() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: Alice.public(),
						amount: 100,
					}),
					&Bob,
					0,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 100, "bob should have 100");
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0, "alice should have 0");
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn alice_mints_100_to_bob_bob_transfers_all_to_alice() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(
					RuntimeCall::Currency(CurrencyCall::TransferAll { dest: Alice.public() }),
					&Bob,
					0,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 0, "bob should have 0");
				assert_eq!(
					free_of(Alice.public()).unwrap_or_default(),
					100,
					"alice should have 100"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");

				// As opposed to storing something like `Some(0)`. In other tests we don't really
				// care about this, but we check it here.
				assert_eq!(
					balance_of(Bob.public()),
					None,
					"bob's account should be REMOVED, not set to 0"
				);
			});
		}

		#[test]
		fn alice_mints_5_to_bob() {
			// cannot mint amount less than `MinimumBalance`
			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 5 }),
				&Alice,
				0,
			)];

			let mut state = new_test_ext();

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 0, "bob should have 0");
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0, "alice should have 0");
				assert_eq!(issuance().unwrap_or_default(), 0, "issuance should be 0");
			});
		}

		#[test]
		fn alice_mints_9_to_bob() {
			// still cannot mint amount less than `MinimumBalance`
			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 9 }),
				&Alice,
				0,
			)];

			let mut state = new_test_ext();

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 0, "bob should have 0");
				assert_eq!(issuance().unwrap_or_default(), 0, "issuance should be 0");
			});
		}

		#[test]
		fn multiple_mints_in_single_block() {
			// can mint multiple times if alice
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 20 }),
					&Alice,
					0,
				),
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 30 }),
					&Alice,
					1,
				),
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint {
						dest: Charlie.public(),
						amount: 50,
					}),
					&Alice,
					2,
				),
			];

			let mut state = new_test_ext();

			author_and_import(&mut state, exts, || {
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 30, "alice should have 30");
				assert_eq!(free_of(Bob.public()).unwrap_or_default(), 20, "bob should have 20");
				assert_eq!(
					free_of(Charlie.public()).unwrap_or_default(),
					50,
					"charlie should have 50"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}
	}

	mod optional {
		use super::*;

		#[test]
		fn alice_remarks_then_transfers_all() {
			// edge-case: 0 to 0 is a successful transferAll, but it should not trigger a kill,
			// because it is from zero to zero
			let mut state = new_test_ext();

			let exts = vec![
				// alice incs her nonce
				signed(RuntimeCall::System(SystemCall::Remark { data: vec![42] }), &Alice, 0),
				// then alice does a transfer_all
				signed(
					RuntimeCall::Currency(CurrencyCall::TransferAll { dest: Bob.public() }),
					&Alice,
					1,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(
					balance_of(Alice.public()),
					Some(AccountBalance { nonce: 2, ..Default::default() }),
					"alice's account should not be removed in this case"
				);
			});
		}
	}
}

mod staking {
	use super::*;

	mod fundamentals {
		use super::*;

		#[test]
		fn bob_with_100_stakes_20() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(RuntimeCall::Staking(StakingCall::Bond { amount: 20 }), &Bob, 0),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					80,
					"bob's free should be 80"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					20,
					"bob's reserve should be 20"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn bob_with_100_stakes_120() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(RuntimeCall::Staking(StakingCall::Bond { amount: 120 }), &Bob, 0),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					100,
					"bob's free should be 100"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					0,
					"bob's reserve should be 0"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}
	}

	mod challenging {
		use super::*;

		#[test]
		fn bob_with_100_stakes_90() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(RuntimeCall::Staking(StakingCall::Bond { amount: 90 }), &Bob, 0),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					10,
					"bob's free should be 10"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					90,
					"bob's reserve should be 90"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn bob_with_100_stakes_100() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(RuntimeCall::Staking(StakingCall::Bond { amount: 100 }), &Bob, 0),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					100,
					"bob's free should be 100"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					0,
					"bob's reserve should be 0"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn bob_with_100_stakes_20_then_transfers_all() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(RuntimeCall::Staking(StakingCall::Bond { amount: 20 }), &Bob, 0),
				// transfer all from bob
				signed(
					RuntimeCall::Currency(CurrencyCall::TransferAll { dest: Charlie.public() }),
					&Bob,
					1,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					80,
					"bob's free should be 80"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					20,
					"bob's reserve should be 20"
				);
				assert_eq!(balance_of(Charlie.public()), None, "charlie's balance should be none");
			});
		}

		#[test]
		fn bob_with_100_stakes_95() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				signed(RuntimeCall::Staking(StakingCall::Bond { amount: 95 }), &Bob, 0),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					100,
					"bob's free should be 100"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					0,
					"bob's reserve should be 0"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}
	}

	mod optional {}
}

mod tipping {
	use super::*;
	use crate::author_and_import;
	use sp_runtime::transaction_validity::ValidTransaction;

	fn setup_alice() -> TestExternalities {
		let mut state = new_test_ext();
		let exts = vec![signed(
			RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 100 }),
			&Alice,
			0,
		)];

		author_and_import(&mut state, exts, || {});
		state
	}

	mod fundamentals {
		use super::*;

		#[test]
		fn bob_with_100_stakes_50_and_tips_10() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				tipped(RuntimeCall::Staking(StakingCall::Bond { amount: 50 }), &Bob, 0, 10),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 10, "treasury should be 10");
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					40,
					"bob's free should be 40"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					50,
					"bob's reserve should be 50"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn alice_with_100_transfers_20_to_bob_with_10_tip() {
			// treasury is empty.
			let mut state = new_test_ext();
			state.execute_with(|| assert!(treasury().is_none()));

			let exts = vec![
				// alice gets 100.
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 100 }),
					&Alice,
					0,
				),
				// sends 20 to bob, while tipping 10 of it. This creates treasury.
				tipped(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: Bob.public(),
						amount: 20,
					}),
					&Alice,
					1,
					10,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(
					treasury().unwrap_or_default(),
					10,
					"treasury account should exist and have 10"
				);
				assert_eq!(
					free_of(Alice.public()).unwrap_or_default(),
					70,
					"alice's free balance should be 70"
				);
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					20,
					"bob's free balance should be 20"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn alice_with_100_transfers_all_to_bob_and_tips_10() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 100 }),
					&Alice,
					0,
				),
				tipped(
					RuntimeCall::Currency(CurrencyCall::TransferAll { dest: Bob.public() }),
					&Alice,
					1,
					10,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 10, "treasury account should exist");
				assert_eq!(
					free_of(Alice.public()).unwrap_or_default(),
					0,
					"alice's free balance should be 0"
				);
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					90,
					"bob's free balance should be 90"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn validate_tx_alice_with_100_tips_5() {
			let mut state = setup_alice();

			// now run validation on top of this state.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 5);
			let validity = validate(to_validate, &mut state);
			assert!(matches!(validity, Ok(ValidTransaction { priority: 5, .. })));
		}

		#[test]
		fn validate_tx_alice_with_100_tips_15() {
			let mut state = setup_alice();

			// now run validation on top of this state.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 15);
			let validity = validate(to_validate, &mut state);
			assert!(matches!(validity, Ok(ValidTransaction { priority: 15, .. })));
		}

		#[test]
		fn validate_tx_alice_with_100_tips_95() {
			// cannot tip to an amount that I don't even have.
			let mut state = setup_alice();

			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 95);
			let validity = validate(to_validate, &mut state);
			assert_eq!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::Payment))
			);
		}

		#[test]
		fn validate_tx_alice_with_100_tips_105() {
			// cannot tip to an amount that I don't even have.
			let mut state = setup_alice();

			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 105);
			let validity = validate(to_validate, &mut state);
			assert_eq!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::Payment))
			);
		}

		#[test]
		fn alice_with_u128_max_div2_tips_u128_max_div4() {
			let mut state = new_test_ext();
			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint {
					dest: Alice.public(),
					amount: u128::MAX / 2,
				}),
				&Alice,
				0,
			)];

			// apply this to our state.
			author_and_import(&mut state, exts, || {});

			// now run validation on top of this state.
			let to_validate = tipped(
				RuntimeCall::System(SystemCall::Set { value: 42 }),
				&Alice,
				1,
				u128::MAX / 4,
			);
			let validity = validate(to_validate, &mut state);
			assert!(matches!(validity, Ok(ValidTransaction { priority: u64::MAX, .. })));
		}
	}

	mod challenging {
		use super::*;

		#[test]
		fn bob_with_100_stakes_85_and_tip_10() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				tipped(RuntimeCall::Staking(StakingCall::Bond { amount: 85 }), &Bob, 0, 10),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 10, "treasury should be 10");
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					90,
					"bob's free should be 90"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					0,
					"bob's reserve should be 0"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn bob_with_100_stakes_89_and_tip_5() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				tipped(RuntimeCall::Staking(StakingCall::Bond { amount: 89 }), &Bob, 0, 5),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 0, "treasury should be 0");
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					95,
					"bob's free should be 95"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					0,
					"bob's reserve should be 0"
				);
				assert_eq!(issuance().unwrap_or_default(), 95);
			});
		}

		#[test]
		fn alice_with_100_transfers_all_to_bob_with_tip_5() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 100 }),
					&Alice,
					0,
				),
				tipped(
					RuntimeCall::Currency(CurrencyCall::TransferAll { dest: Bob.public() }),
					&Alice,
					1,
					5,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 0, "treasury should be 0");
				assert_eq!(
					free_of(Alice.public()).unwrap_or_default(),
					0,
					"alice's free balance should be 0"
				);
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					95,
					"bob's free balance should be 95"
				);
				assert_eq!(issuance().unwrap_or_default(), 95, "issuance should be 95");
			});
		}

		#[test]
		fn alice_with_100_transfers_20_to_bob_with_5_tip() {
			let mut state = new_test_ext();
			state.execute_with(|| assert!(treasury().is_none()));

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 100 }),
					&Alice,
					0,
				),
				// sends 20 to bob, while tipping 5 of it. This will not create the treasury, and
				// is an edge case where the total issuance needs to be updated.
				tipped(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: Bob.public(),
						amount: 20,
					}),
					&Alice,
					1,
					5,
				),
			];

			author_and_import(&mut state, exts, || {
				assert!(treasury().is_none(), "treasury account should not exist");
				assert_eq!(
					free_of(Alice.public()).unwrap_or_default(),
					75,
					"alice's free balance should be 75"
				);
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					20,
					"bob's free balance should be 20"
				);
				assert_eq!(issuance().unwrap_or_default(), 95, "issuance should be 95");
			});
		}

		#[test]
		fn bob_with_100_stakes_90_and_tip_10() {
			let mut state = new_test_ext();

			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
					&Alice,
					0,
				),
				tipped(RuntimeCall::Staking(StakingCall::Bond { amount: 90 }), &Bob, 0, 10),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 10, "treasury should be 10");
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					90,
					"bob's free should be 90"
				);
				assert_eq!(
					reserve_of(Bob.public()).unwrap_or_default(),
					0,
					"bob's reserve should be 0"
				);
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance should be 100");
			});
		}

		#[test]
		fn bob_with_20_sends_10_to_charlie_and_tip_5() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 20 }),
					&Alice,
					0,
				),
				// bob sends 10 to charlie, while tipping 5. This will fail, but the tip will go
				// though.
				tipped(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: Charlie.public(),
						amount: 10,
					}),
					&Bob,
					0,
					5,
				),
			];

			author_and_import(&mut state, exts, || {
				assert!(treasury().is_none(), "treasury account should not exist");
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					15,
					"bob's free balance should be 15"
				);
				assert_eq!(
					free_of(Charlie.public()).unwrap_or_default(),
					0,
					"charlie's free balance should be 0"
				);
				assert_eq!(issuance().unwrap_or_default(), 15, "issuance should be 15");
			});
		}

		#[test]
		fn alice_with_20_transfers_10_to_bob_and_tips_10() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 20 }),
					&Alice,
					0,
				),
				tipped(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: Bob.public(),
						amount: 10,
					}),
					&Alice,
					1,
					10,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 10, "treasury account should exist");
				assert_eq!(
					free_of(Alice.public()).unwrap_or_default(),
					10,
					"alice's free balance should be 10"
				);
				assert_eq!(
					free_of(Bob.public()).unwrap_or_default(),
					0,
					"bob's free balance should be 0"
				);
				assert_eq!(issuance().unwrap_or_default(), 20, "issuance should be 20");
			});
		}

		#[test]
		fn multi_tip_in_single_block() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 100 }),
					&Alice,
					0,
				),
				// tip not enough to create the treasury, burnt.
				tipped(
					RuntimeCall::System(SystemCall::Remark { data: Default::default() }),
					&Alice,
					1,
					5,
				),
				// tip creates the treasury now
				tipped(
					RuntimeCall::System(SystemCall::Remark { data: Default::default() }),
					&Alice,
					2,
					10,
				),
				// 5 more is tipped.
				tipped(
					RuntimeCall::System(SystemCall::Remark { data: Default::default() }),
					&Alice,
					3,
					5,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 15, "treasury must be 15");
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 80, "alice must have 80");
				assert_eq!(issuance().unwrap_or_default(), 95, "issuance must be 95");
			})
		}

		#[test]
		fn validate_tx_alice_with_zero_tips_10() {
			// an account with no balance at all.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 0, 10);
			let validity = validate(to_validate, &mut new_test_ext());
			assert_eq!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::Payment))
			);
		}

		#[test]
		fn validate_tx_alice_with_100_tips_zero() {
			let mut state = setup_alice();

			// now run validation on top of this state.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 0);
			let validity = validate(to_validate, &mut state);
			assert!(matches!(validity, Ok(ValidTransaction { priority: 0, .. })));
		}

		#[test]
		fn validate_tx_alice_with_100_tips_100() {
			let mut state = setup_alice();

			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 100);
			let validity = validate(to_validate, &mut state);
			assert_eq!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::Payment))
			);
		}
	}

	mod optional {
		use super::*;

		#[test]
		fn alice_with_100_transfers_all_to_bob_and_tips_95() {
			// TODO: this is unclear for me as well, needs to be revised for next round's spec.
		}

		#[test]
		fn alice_with_100_transfer_10_to_treasury_with_tip_1() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 100 }),
					&Alice,
					0,
				),
				// tip not enough to create the treasury, burnt.
				tipped(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: AccountId::unchecked_from(TREASURY),
						amount: 10,
					}),
					&Alice,
					1,
					1,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 11, "treasury should be 11");
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 89, "alice must have 89");
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance must be 100");
			})
		}

		#[test]
		fn alice_with_100_transfer_5_to_treasury_with_tip_5() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 100 }),
					&Alice,
					0,
				),
				tipped(
					RuntimeCall::Currency(CurrencyCall::Transfer {
						dest: AccountId::unchecked_from(TREASURY),
						amount: 5,
					}),
					&Alice,
					1,
					5,
				),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap_or_default(), 10, "treasury should be 10");
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 90, "alice must have 90");
				assert_eq!(issuance().unwrap_or_default(), 100, "issuance must be 100");
			})
		}

		#[test]
		fn validate_tx_alice_with_zero_tips_zero() {
			let mut state = new_test_ext();
			// now run validation on top of this state.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 0);
			let validity = validate(to_validate, &mut state);
			assert_eq!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::Payment))
			);
		}
	}
}

mod nonce {
	use super::*;
	use sp_runtime::transaction_validity::ValidTransaction;

	const CALL: RuntimeCall = RuntimeCall::System(SystemCall::Set { value: 42 });

	// create alice and set her nonce to 1.
	fn setup_alice() -> TestExternalities {
		let mut state = new_test_ext();
		let exts = vec![signed(CALL, &Alice, 0), signed(CALL, &Alice, 1), signed(CALL, &Alice, 2)];
		author_and_import(&mut state, exts, || {});
		state
	}

	mod fundamentals {
		use super::*;

		#[test]
		fn apply_stale() {
			let mut state = setup_alice();

			let ext = signed(CALL, &Alice, 2);
			let apply_result = apply(ext, &mut state);
			assert_eq!(apply_result.unwrap_err(), InvalidTransaction::Stale.into());
		}

		#[test]
		fn apply_future() {
			let mut state = setup_alice();

			let ext = signed(CALL, &Alice, 4);
			let apply_result = apply(ext, &mut state);
			assert_eq!(apply_result.unwrap_err(), InvalidTransaction::Future.into());
		}

		#[test]
		fn apply_ready() {
			let mut state = setup_alice();

			let ext = signed(CALL, &Alice, 3);
			let apply_result = apply(ext, &mut state);
			assert!(apply_result.is_ok());
		}

		#[test]
		fn nonce_is_set_after_successful_apply() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(CALL, &Alice, 0),
				signed(CALL, &Alice, 1),
				signed(CALL, &Alice, 2),
				signed(CALL, &Alice, 3),
			];
			author_and_import(&mut state, exts, || {
				assert_eq!(nonce_of(Alice.public()).unwrap_or_default(), 4);
			})
		}

		#[test]
		fn chain_nonce_failures() {
			let mut state = new_test_ext();
			let exts = vec![
				signed(CALL, &Alice, 0),
				signed(CALL, &Alice, 1),
				unsigned(CALL),
				signed(CALL, &Alice, 3),
				signed(CALL, &Alice, 4),
			];

			author_and_import(&mut state, exts, || {
				assert_eq!(nonce_of(Alice.public()).unwrap_or_default(), 2);
			})
		}
	}

	mod optional {
		use super::*;

		#[test]
		fn validate_ready() {
			let mut state = setup_alice();

			let ext = signed(CALL, &Alice, 3);
			let validity = validate(ext, &mut state);
			let expected_requires: Vec<Vec<u8>> = vec![];
			let expected_provides = vec![(&Alice.public(), 3).encode()];
			match validity {
				Ok(ValidTransaction { requires, provides, .. }) => {
					assert_eq!(
						requires, expected_requires,
						"requires should be {:?}, got {:?}",
						&expected_requires, requires
					);
					assert_eq!(
						provides, expected_provides,
						"provides should be {:?}, got {:?}",
						&expected_provides, provides
					);
				},
				other @ _ => {
					panic!(
						"Expected Ok(_) with requires {:?} and provides {:?}, got {:?}",
						expected_requires, expected_provides, other
					);
				},
			}
		}

		#[test]
		fn validate_future() {
			let mut state = setup_alice();

			let ext = signed(CALL, &Alice, 5);
			let validity = validate(ext, &mut state);
			let expected_requires = vec![(&Alice.public(), 4).encode()];
			let expected_provides = vec![(&Alice.public(), 5).encode()];
			match validity {
				Ok(ValidTransaction { requires, provides, .. }) => {
					assert_eq!(
						requires, expected_requires,
						"requires should be {:?}, got {:?}",
						&expected_requires, requires
					);
					assert_eq!(
						provides, expected_provides,
						"provides should be {:?}, got {:?}",
						&expected_provides, provides
					);
				},
				other @ _ => {
					panic!(
						"Expected Ok(_) with requires {:?} and provides {:?}, got {:?}",
						expected_requires, expected_provides, other
					);
				},
			}
		}

		#[test]
		fn validate_stale() {
			let mut state = setup_alice();

			let ext = signed(CALL, &Alice, 2);
			let validity = validate(ext, &mut state);
			assert_eq!(validity.unwrap_err(), InvalidTransaction::Stale.into());
		}
	}
}
