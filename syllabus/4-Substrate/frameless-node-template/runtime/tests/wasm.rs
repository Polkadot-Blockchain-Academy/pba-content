use parity_scale_codec::{Decode, Encode};
use runtime::export::RuntimeGenesisConfig;
use shared::{
	AccountBalance, AccountId, Balance, Block, CurrencyCall, Extrinsic, Header, RuntimeCall,
	RuntimeCallWithTip, StakingCall, SystemCall, VALUE_KEY,
};
use sp_core::traits::{CallContext, CodeExecutor, Externalities};
use sp_io::TestExternalities;
use sp_keyring::AccountKeyring::*;
use sp_runtime::{traits::Extrinsic as _, BuildStorage};

mod shared;

const LOG_TARGET: &'static str = "wasm-tests";

fn balance_of(who: AccountId) -> Option<AccountBalance> {
	let key = [b"BalancesMap".as_ref(), who.encode().as_ref()].concat();
	sp_io::storage::get(&key).and_then(|bytes| AccountBalance::decode(&mut &bytes[..]).ok())
}

fn free_of(who: AccountId) -> Option<Balance> {
	balance_of(who).map(|b| b.free)
}

#[allow(unused)]
fn reserve_of(who: AccountId) -> Option<Balance> {
	balance_of(who).map(|b| b.reserved)
}

fn issuance() -> Option<Balance> {
	let key = b"TotalIssuance".as_ref();
	sp_io::storage::get(&key).and_then(|bytes| Balance::decode(&mut &bytes[..]).ok())
}

fn sign(call: RuntimeCall, signer: &sp_keyring::AccountKeyring) -> Extrinsic {
	let call_with_tip = RuntimeCallWithTip { tip: None, call };
	let payload = (call_with_tip).encode();
	let signature = signer.sign(&payload);
	Extrinsic::new(call_with_tip, Some((signer.public(), signature, ()))).unwrap()
}

fn author_and_import(
	state: &mut TestExternalities,
	exts: Vec<Extrinsic>,
	post: impl FnOnce() -> (),
) {
	// ensure ext has some code in it, otherwise something is wrong.
	let code = state
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
		executor_call(&mut auth_state, "BlockBuilder_apply_extrinsic", &ext.encode()).unwrap();
		extrinsics.push(ext);
	}

	let header: Header =
		executor_call(&mut auth_state, "BlockBuilder_finalize_block", Default::default())
			.map(|data| <Header as Decode>::decode(&mut &*data).unwrap())
			.unwrap();

	let block = Block { extrinsics, header };
	log::debug!(target: LOG_TARGET, "authored a block with state root {:?}, importing now", block.header.state_root);

	// now we import the block into a fresh new state.
	executor_call(state, "Core_execute_block", &block.encode()).unwrap();
	state.commit_all().unwrap();
	assert_eq!(&block.header.state_root, state.backend.root());
	// TODO: check extrinsic root is set.

	log::debug!(target: LOG_TARGET, "all good; running post checks");
	state.execute_with(|| post());
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
	let storage = RuntimeGenesisConfig::default().build_storage().unwrap();
	let code_storage = storage.clone();
	let code = code_storage.top.get(sp_core::storage::well_known_keys::CODE).unwrap();
	TestExternalities::new_with_code(code, storage)
}
#[test]
fn empty_block() {
	let mut state = new_test_ext();
	state.execute_with(|| assert!(sp_io::storage::get(VALUE_KEY).is_none()));
	author_and_import(&mut state, vec![], || {});
}

#[test]
fn remark() {
	let call = RuntimeCall::System(SystemCall::Remark { data: vec![42, 42] });
	let exts = vec![sign(call, &Alice)];

	let mut state = new_test_ext();

	author_and_import(&mut state, exts, || assert!(sp_io::storage::get(VALUE_KEY).is_none()));
}

#[test]
fn basic_setup_works() {
	let call = RuntimeCall::System(SystemCall::Set { value: 42 });
	let exts = vec![sign(call, &Alice)];

	let mut state = new_test_ext();

	state.execute_with(|| assert!(sp_io::storage::get(VALUE_KEY).is_none()));
	author_and_import(&mut state, exts, || assert!(sp_io::storage::get(VALUE_KEY).is_some()));
}

mod currency {
	use super::*;
	#[test]
	fn mint_wrong_minter() {
		// bob account cannot mint.
		let mut state = new_test_ext();

		let call = RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 20 });
		let exts = vec![sign(call, &Bob)];

		author_and_import(&mut state, exts, || {
			assert!(balance_of(Alice.public()).is_none());
			assert!(issuance().is_none());
		});
	}

	#[test]
	fn mint_success() {
		// can mint if alice
		let call = RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 20 });
		let exts = vec![sign(call, &Alice)];

		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()), Some(20));
			assert!(free_of(Alice.public()).is_none());
			assert_eq!(issuance(), Some(20));
		});
	}

	#[test]
	fn multi_mint_success() {
		// can mint multiple times if alice
		let exts = vec![
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 20 }),
				&Alice,
			),
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 30 }),
				&Alice,
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
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 20 }),
				&Alice,
			),
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Alice.public(), amount: 30 }),
				&Bob,
			),
		];

		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Alice.public()), None);
			assert_eq!(free_of(Bob.public()), Some(20));
			assert_eq!(issuance(), Some(20));
		});
	}

	#[test]
	fn mint_not_enough() {
		// cannot mint amount less than `MinimumBalance`

		let call = RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 5 });
		let exts = vec![sign(call, &Alice)];

		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert_eq!(balance_of(Bob.public()), None);
			assert_eq!(issuance(), None);
		});
	}

	#[test]
	fn mint_not_enough_edge() {
		// still cannot mint amount less than `MinimumBalance`

		let call = RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 9 });
		let exts = vec![sign(call, &Alice)];

		let mut state = new_test_ext();

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()), None);
			assert_eq!(issuance(), None);
		});

		// but 10 is ok.
		let call = RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 10 });
		let exts = vec![sign(call, &Alice)];
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
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
				&Alice,
			),
			// transfer 20 to alice.
			sign(
				RuntimeCall::Currency(CurrencyCall::Transfer { dest: Alice.public(), amount: 20 }),
				&Bob,
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
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
				&Alice,
			),
			sign(
				RuntimeCall::Currency(CurrencyCall::Transfer {
					dest: Alice.public(),
					amount: spendable + 1,
				}),
				&Bob,
			),
		];

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()), Some(100));
			assert_eq!(free_of(Alice.public()), None);
			assert_eq!(issuance(), Some(100));
		});
	}

	#[test]
	fn transfer_more_than_you_can_limit() {
		let mut state = new_test_ext();
		let spendable = 100 - 10;

		let exts = vec![
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
				&Alice,
			),
			sign(
				RuntimeCall::Currency(CurrencyCall::Transfer {
					dest: Alice.public(),
					amount: spendable,
				}),
				&Bob,
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
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
				&Alice,
			),
			sign(
				RuntimeCall::Currency(CurrencyCall::Transfer { dest: Alice.public(), amount: 100 }),
				&Bob,
			),
		];

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()), Some(0)); // TODO: None should also be acceptable here.
			assert_eq!(free_of(Alice.public()), Some(100));
			assert_eq!(issuance(), Some(100));
		});
	}

	#[test]
	fn transfer_all_2() {
		let mut state = new_test_ext();

		let exts = vec![
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
				&Alice,
			),
			sign(RuntimeCall::Currency(CurrencyCall::TransferAll { dest: Alice.public() }), &Bob),
		];

		author_and_import(&mut state, exts, || {
			assert_eq!(free_of(Bob.public()), Some(0));
			assert_eq!(free_of(Alice.public()), Some(100));
			assert_eq!(issuance(), Some(100));
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
			sign(
				RuntimeCall::Currency(CurrencyCall::Mint { dest: Bob.public(), amount: 100 }),
				&Alice,
			),
			sign(RuntimeCall::Staking(StakingCall::Bond { amount: 20 }), &Bob),
		];

		author_and_import(&mut state, exts, || {
			assert_eq!(balance_of(Bob.public()), Some(AccountBalance { free: 80, reserved: 20 }));
			assert_eq!(issuance(), Some(100));
		});
	}

	#[test]
	fn bonding_more_than_allowed() {}

	#[test]
	fn bonding_more_than_allowed_limit() {}
}
