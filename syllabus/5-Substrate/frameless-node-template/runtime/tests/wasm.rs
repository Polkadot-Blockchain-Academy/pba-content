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

// TODO: in most tests, we use alice to mint in her own account, and then use her own account to
// transact. to separate the checks for nonce, it would be nice to separate this.

mod shared;

const LOG_TARGET: &'static str = "wasm-tests";

thread_local! {
	pub static CALLED_AUTHOR_AND_IMPORT: RefCell<bool> = RefCell::new(false);
}

fn balance_of(who: AccountId) -> Option<AccountBalance> {
	let key = [b"BalancesMap".as_ref(), who.encode().as_ref()].concat();
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
	.unwrap();

	// decode the bytes as a TransactionValidity.
	<TransactionValidity as Decode>::decode(&mut &*return_bytes).unwrap()
}

fn apply(ext: Extrinsic, state: &mut TestExternalities) -> ApplyExtrinsicResult {
	let return_bytes =
		executor_call(state, "BlockBuilder_apply_extrinsic", ext.encode().as_slice()).unwrap();

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

	log::info!(target: LOG_TARGET, "authoring a block with {:?} and .", exts.iter().map(|x| x.function.clone()).collect::<Vec<_>>());
	let mut extrinsics = vec![];
	let mut auth_state = TestExternalities::new_with_code(code.as_ref(), Default::default());

	executor_call(&mut auth_state, "Core_initialize_block", &header.encode()).unwrap();

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
			.unwrap();

	let block = Block { extrinsics, header };
	auth_state.commit_all().unwrap();
	assert_eq!(
		&auth_state.execute_with(|| sp_io_root()),
		auth_state.backend.root(),
		"something is wrong :/"
	);

	auth_state.execute_with(|| {
		// check the extrinsics key is set.
		let extrinsics = sp_io::storage::get(EXTRINSICS_KEY)
			.and_then(|bytes| <Vec<Vec<u8>> as Decode>::decode(&mut &*bytes).ok())
			.unwrap_or_default();
		assert_eq!(
			extrinsics.len(),
			block.extrinsics.len(),
			"incorrect extrinsics recorded in state"
		);

		// check the header key is not set.
		assert!(sp_io::storage::get(&HEADER_KEY).is_none());

		// check state root.
		assert_eq!(block.header.state_root, sp_io_root());

		// check extrinsics root.
		assert_eq!(
			block.header.extrinsics_root,
			BlakeTwo256::ordered_trie_root(extrinsics, sp_runtime::StateVersion::V0)
		);
	});
	drop(auth_state);

	// now we import the block into a fresh new state.
	executor_call(import_state, "Core_execute_block", &block.encode())
		.expect("execute block returned error; invalid block?");
	import_state.commit_all().unwrap();

	import_state.execute_with(|| {
		// check state root.
		assert_eq!(block.header.state_root, sp_io_root());

		// check extrinsics root.
		assert_eq!(
			block.header.extrinsics_root,
			BlakeTwo256::ordered_trie_root(
				block.extrinsics.into_iter().map(|e| e.encode()).collect::<Vec<_>>(),
				sp_runtime::StateVersion::V0
			)
		);
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
	res.map_err(|_| ())
}

fn new_test_ext() -> TestExternalities {
	sp_tracing::try_init_simple();
	let code_path =
		std::option_env!("WASM_FILE").unwrap_or("../target/debug/wbuild/runtime/runtime.wasm");
	log::info!(target: LOG_TARGET, "reading wasm file from {}", code_path);
	let code = std::fs::read(code_path).unwrap();
	let mut storage: sp_core::storage::Storage = Default::default();
	storage
		.top
		.insert(sp_core::storage::well_known_keys::CODE.to_vec(), code.to_vec());
	TestExternalities::new_with_code(&code, storage)
}

/*
- basic + block_builder + validate => 1
- currency + staking => 2
- tipping + nonce => 3 or 4

- if first 3 parts are failing, but the staking+currency is fine, we handle case by case.
- lenience will be acceptable.

TODO for Nuke: publish the solved mini_substrate.
*/

mod basics {
	use super::*;
	#[test]
	fn empty_block() {
		let mut state = new_test_ext();
		state.execute_with(|| assert!(sp_io::storage::get(VALUE_KEY).is_none()));
		author_and_import(&mut state, vec![], || {
			assert_eq!(balance_of(Alice.public()), None);
			assert_eq!(balance_of(Bob.public()), None);
		});
	}

	#[test]
	fn remark() {
		let exts =
			vec![signed(RuntimeCall::System(SystemCall::Remark { data: vec![42, 42] }), &Alice, 0)];
		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert!(sp_io::storage::get(VALUE_KEY).is_none());
		});
	}

	#[test]
	fn set_value() {
		let exts = vec![signed(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 0)];
		let mut state = new_test_ext();

		state.execute_with(|| assert!(sp_io::storage::get(VALUE_KEY).is_none()));
		author_and_import(&mut state, exts, || {
			assert_eq!(
				sp_io::storage::get(VALUE_KEY).and_then(|b| <u32 as Decode>::decode(&mut &*b).ok()),
				Some(42)
			);
		});
	}
}

mod block_builder {
	use super::*;

	#[test]
	fn apply_unsigned() {
		let ext = unsigned(RuntimeCall::System(SystemCall::Set { value: 42 }));
		let mut state = new_test_ext();

		assert_eq!(
			apply(ext, &mut state).unwrap_err(),
			TransactionValidityError::Invalid(InvalidTransaction::BadProof)
		);
	}

	#[test]
	fn apply_bad_signature() {
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
			apply(ext, &mut state).unwrap_err(),
			TransactionValidityError::Invalid(InvalidTransaction::BadProof)
		);
	}

	#[test]
	fn apply_with_error() {
		let mut state = new_test_ext();
		let ext = signed(RuntimeCall::System(SystemCall::SudoRemark { data: vec![42] }), &Bob, 0);
		assert!(matches!(apply(ext, &mut state), Ok(Err(_))));
	}

	#[test]
	fn apply_ok() {
		let mut state = new_test_ext();
		let ext = signed(RuntimeCall::System(SystemCall::SudoRemark { data: vec![42] }), &Alice, 0);
		assert!(matches!(apply(ext, &mut state), Ok(Ok(_))));
	}
}

mod validate_tx {
	use super::*;

	#[test]
	fn validate_bad_signature() {
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
			validity.unwrap_err(),
			TransactionValidityError::Invalid(InvalidTransaction::BadProof)
		);
	}

	#[test]
	fn validate_unsigned() {
		let ext = unsigned(RuntimeCall::System(SystemCall::Set { value: 42 }));
		let mut state = new_test_ext();

		let validity = validate(ext, &mut state);

		assert_eq!(
			validity.unwrap_err(),
			TransactionValidityError::Invalid(InvalidTransaction::BadProof)
		);
	}
}

mod currency {
	use super::*;

	#[test]
	fn mint_wrong_minter() {
		// bob account cannot mint.
		let mut state = new_test_ext();

		let exts = vec![signed(
			RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 20 }),
			&Bob,
			0,
		)];

		author_and_import(&mut state, exts, || {
			assert!(balance_of(Alice.public()).is_none());
			assert!(issuance().is_none());
		});
	}

	#[test]
	fn mint_success() {
		// can mint if alice
		let exts = vec![signed(
			RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 20 }),
			&Alice,
			0,
		)];

		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()), Some(20));
			assert_eq!(
				balance_of(Alice.public()).unwrap(),
				AccountBalance { free: 0, reserved: 0, nonce: 1 }
			);
			assert_eq!(issuance(), Some(20));
		});
	}

	#[test]
	fn multi_mint_success() {
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
		];

		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Alice.public()), Some(30));
			assert_eq!(free_of(Bob.public()), Some(20));
			assert_eq!(issuance(), Some(50));
		});
	}

	#[test]
	fn mixed_mint() {
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
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Charlie.public(), amount: 30 }),
				&Charlie,
				0,
			),
		];

		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0);
			assert_eq!(free_of(Charlie.public()).unwrap_or_default(), 0);
			assert_eq!(free_of(Bob.public()), Some(20));
			assert_eq!(issuance(), Some(20));
		});
	}

	#[test]
	fn mint_not_enough() {
		// cannot mint amount less than `MinimumBalance`
		let exts = vec![signed(
			RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 5 }),
			&Alice,
			0,
		)];

		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert_eq!(balance_of(Bob.public()), None);
			assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0);
			assert_eq!(issuance(), None);
		});
	}

	#[test]
	fn mint_not_enough_edge_1() {
		// still cannot mint amount less than `MinimumBalance`

		let exts = vec![signed(
			RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 9 }),
			&Alice,
			0,
		)];

		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()), None);
			assert_eq!(issuance(), None);
		});
	}

	#[test]
	fn mint_not_enough_edge_2() {
		let mut state = new_test_ext();

		// but 10 is ok.
		let exts = vec![signed(
			RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 10 }),
			&Alice,
			0,
		)];
		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()), Some(10));
			assert_eq!(issuance(), Some(10));
		});
	}

	#[test]
	fn transfer_success() {
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
				RuntimeCall::Currency(CurrencyCall::Transfer { dest: Alice.public(), amount: 20 }),
				&Bob,
				0,
			),
		];

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()), Some(80));
			assert_eq!(free_of(Alice.public()), Some(20));
			assert_eq!(issuance(), Some(100));
		});
	}

	#[test]
	fn transfer_more_than_you_can() {
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
			assert_eq!(free_of(Bob.public()), Some(100));
			assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0);
			assert_eq!(issuance(), Some(100));
		});
	}

	#[test]
	fn transfer_more_than_you_can_limit() {
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
			assert_eq!(free_of(Bob.public()), Some(10));
			assert_eq!(free_of(Alice.public()), Some(90));
			assert_eq!(issuance(), Some(100));
		});
	}

	#[test]
	fn transfer_all_1() {
		let mut state = new_test_ext();

		let exts = vec![
			signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
				&Alice,
				0,
			),
			signed(
				RuntimeCall::Currency(CurrencyCall::Transfer { dest: Alice.public(), amount: 100 }),
				&Bob,
				0,
			),
		];

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()).unwrap_or_default(), 0);
			assert_eq!(free_of(Alice.public()), Some(100));
			assert_eq!(issuance(), Some(100));
		});
	}

	#[test]
	fn transfer_all_2() {
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
			assert_eq!(free_of(Bob.public()).unwrap_or_default(), 0);
			assert_eq!(free_of(Alice.public()), Some(100));
			assert_eq!(issuance(), Some(100));
		});
	}

	#[test]
	fn transfer_all_account_is_destroyed() {
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
			// As opposed to storing something like `Some(0)`. In other tests we don't really care
			// about this, but we check it here.
			assert_eq!(balance_of(Bob.public()), None);
		});
	}

	#[test]
	fn transfer_all_without_funds() {
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
			// As opposed to storing something like `Some(0)`. In other tests we don't really care
			// about this, but we check it here.
			assert_eq!(
				balance_of(Alice.public()).unwrap(),
				AccountBalance { nonce: 2, ..Default::default() }
			);
		});
	}
}

mod staking {
	use super::*;

	// note: we test these, but they are basically same as testing bonding.
	#[test]
	fn bonding_success() {
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
				balance_of(Bob.public()),
				Some(AccountBalance { free: 80, reserved: 20, nonce: 1 })
			);
			assert_eq!(issuance(), Some(100));
		});
	}

	#[test]
	fn bonding_more_than_allowed() {
		todo!();
	}

	#[test]
	fn bonding_more_than_allowed_limit() {
		todo!();
	}

	#[test]
	fn bonding_and_tipping() {
		todo!();
	}

	#[test]
	fn transfer_all_when_bonded() {
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
			assert_eq!(free_of(Bob.public()).unwrap(), 80);
			assert_eq!(reserve_of(Bob.public()).unwrap(), 20);
			assert_eq!(balance_of(Charlie.public()), None);
		});
	}
}

mod nonce {
	#[test]
	fn test_it() {
		todo!();
	}
}

mod tipping {
	use super::*;
	use crate::author_and_import;
	use sp_runtime::transaction_validity::ValidTransaction;

	mod validate {
		use super::*;
		#[test]
		fn validate_sets_priority() {
			// account with some balance can get a higher priority.
			let mut state = new_test_ext();
			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 20 }),
				&Alice,
				0,
			)];

			// apply this to our state.
			author_and_import(&mut state, exts, || {});

			// now run validation on top of this state.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 5);
			let validity = validate(to_validate, &mut state);
			assert!(matches!(validity, Ok(ValidTransaction { priority: 5, .. })));
		}

		#[test]
		fn priority_overflow() {
			todo!();
		}

		#[test]
		fn cannot_tip_if_not_not_enough_balance() {
			// cannot tip and get a higher priority if not funded.
			let mut state = new_test_ext();
			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 20 }),
				&Alice,
				0,
			)];

			// apply this to our state.
			author_and_import(&mut state, exts, || {});

			// tip value is higher than what alice has.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 25);
			let validity = validate(to_validate, &mut state);
			assert!(matches!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::Payment))
			));
		}

		#[test]
		fn cannot_tip_if_below_ed() {
			// fund alice's account.
			let mut state = new_test_ext();
			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 20 }),
				&Alice,
				0,
			)];

			// apply this to our state.
			author_and_import(&mut state, exts, || {});

			// tip is 15, leaving only 5 for alice.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 15);
			let validity = validate(to_validate, &mut state);
			assert!(matches!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::Payment))
			));
		}

		#[test]
		fn can_kill_oneself_with_tip() {
			// fund alice's account.
			let mut state = new_test_ext();
			let exts = vec![signed(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 20 }),
				&Alice,
				0,
			)];

			// apply this to our state.
			author_and_import(&mut state, exts, || {});

			// tip is 15, leaving only 5 for alice.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 1, 20);
			let validity = validate(to_validate, &mut state);
			assert!(matches!(validity, Ok(ValidTransaction { priority: 20, .. })));
		}

		#[test]
		fn cannot_tip_if_not_funded() {
			// an account with no balance at all.
			let to_validate =
				tipped(RuntimeCall::System(SystemCall::Set { value: 42 }), &Alice, 0, 10);
			let validity = validate(to_validate, &mut new_test_ext());
			assert!(matches!(
				validity,
				Err(TransactionValidityError::Invalid(InvalidTransaction::Payment))
			));
		}
	}

	mod apply {
		use super::*;
		#[test]
		fn tip_unfunded_treasury() {
			let mut state = new_test_ext();
			state.execute_with(|| assert!(treasury().is_none()));

			let exts = vec![
				// alice gets 100.
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

			// apply this to our state.
			author_and_import(&mut state, exts, || {
				assert!(treasury().is_none());
				assert_eq!(free_of(Alice.public()).unwrap(), 75);
				assert_eq!(free_of(Bob.public()).unwrap(), 20);
				assert_eq!(issuance(), Some(95));
			});
		}

		#[test]
		fn success_dispatch_withdraw_fee() {
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

			// apply this to our state.
			author_and_import(&mut state, exts, || {
				assert_eq!(treasury().unwrap(), 10);
				assert_eq!(free_of(Alice.public()).unwrap(), 70);
				assert_eq!(free_of(Bob.public()).unwrap(), 20);
				assert_eq!(issuance(), Some(100));
			});
		}

		#[test]
		fn tip_and_transfer_below_ed() {
			let mut state = new_test_ext();
			let exts = vec![
				// Bob gets 20.
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
				assert!(treasury().is_none());
				assert_eq!(free_of(Bob.public()).unwrap(), 15);
				assert!(free_of(Charlie.public()).is_none());
				assert_eq!(issuance(), Some(15));
			});
		}

		#[test]
		fn tip_and_transfer_kill() {
			let mut state = new_test_ext();
			let exts = vec![
				// alice gets 20.
				signed(
					RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 20 }),
					&Alice,
					0,
				),
				// sends 10 to bob, while tipping 10. This will work, and alice wil die.
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
				assert_eq!(treasury(), Some(10));
				assert_eq!(free_of(Alice.public()).unwrap_or_default(), 0);
				assert_eq!(free_of(Bob.public()), Some(10));
				assert_eq!(issuance(), Some(20));
			});
		}
	}
}
