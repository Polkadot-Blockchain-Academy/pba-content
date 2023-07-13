use crate::{
	shared::{
		AccountId, Balance, Block, CurrencyCall, Extrinsic, RuntimeCall, StakingCall, SystemCall,
		EXTRINSICS_KEY, MINIMUM_BALANCE, MINTER, VALUE_KEY,
	},
	Runtime, LOG_TARGET, VERSION,
};
use log::info;
use parity_scale_codec::{Decode, Encode};
use sp_api::{HashT, TransactionValidity};
use sp_core::crypto::UncheckedFrom;
use sp_runtime::{
	traits::{BlakeTwo256, Block as BlockT, Verify},
	transaction_validity::{
		InvalidTransaction, TransactionSource, TransactionValidityError, ValidTransaction,
	},
	ApplyExtrinsicResult, DispatchOutcome,
};
use sp_std::prelude::*;

mod currency;
mod staking;
mod storage;

pub trait Dispatchable {
	fn dispatch(self, sender: AccountId) -> DispatchOutcome;
}

pub trait Get<T> {
	fn get() -> T;
}

pub struct Minter;
impl Get<AccountId> for Minter {
	fn get() -> AccountId {
		AccountId::unchecked_from(MINTER)
	}
}

pub struct MinimumBalance;
impl Get<Balance> for MinimumBalance {
	fn get() -> Balance {
		MINIMUM_BALANCE
	}
}

impl currency::Config for Runtime {
	const MODULE_ID: &'static str = "CURRENCY";
	type Minter = Minter;
	type MinimumBalance = MinimumBalance;
	type Balance = Balance;
}

impl From<CurrencyCall> for currency::Call<Runtime> {
	fn from(value: CurrencyCall) -> Self {
		match value {
			CurrencyCall::Transfer { dest, amount } =>
				currency::Call::<Runtime>::Transfer { dest, amount },
			CurrencyCall::Mint { dest, amount } => currency::Call::<Runtime>::Mint { dest, amount },
			CurrencyCall::TransferAll { dest } => currency::Call::<Runtime>::TransferAll { dest },
		}
	}
}

impl staking::Config for Runtime {
	type Currency = currency::Module<Runtime>;
}

impl From<StakingCall> for staking::Call<Runtime> {
	fn from(value: StakingCall) -> Self {
		match value {
			StakingCall::Bond { amount } => staking::Call::<Runtime>::Bond { amount },
		}
	}
}

impl Runtime {
	pub(crate) fn solution_apply_extrinsic(ext: Extrinsic) -> ApplyExtrinsicResult {
		log::debug!(target: LOG_TARGET, "dispatching {:?}", ext);

		let sender =
			Self::solution_check_signature(&ext).map_err(|_| InvalidTransaction::BadProof)?;

		// TODO: handle ext.function.tip.

		// execute it
		let dispatch_outcome = match ext.function.call {
			RuntimeCall::System(SystemCall::Set { value }) => {
				sp_io::storage::set(VALUE_KEY, &value.encode());
				Ok(())
			},
			RuntimeCall::System(SystemCall::Remark { data: _ }) => Ok(()),
			RuntimeCall::System(SystemCall::Upgrade { code }) => {
				sp_io::storage::set(sp_core::storage::well_known_keys::CODE, &code);
				Ok(())
			},
			RuntimeCall::Currency(currency_call) => {
				let my_call: currency::Call<Runtime> = currency_call.into();
				my_call.dispatch(sender)
			},
			RuntimeCall::Staking(staking_call) => {
				let my_call: staking::Call<Runtime> = staking_call.into();
				my_call.dispatch(sender)
			},
		};

		Ok(dispatch_outcome)
	}

	pub(crate) fn solution_finalize_block(header: &mut <Block as BlockT>::Header) {
		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
		let state_root = sp_core::H256::decode(&mut &raw_state_root[..]).unwrap();

		let extrinsics = Self::get_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY).unwrap_or_default();
		let extrinsics_root = BlakeTwo256::ordered_trie_root(extrinsics, Default::default());

		header.extrinsics_root = extrinsics_root;
		header.state_root = state_root;
	}

	pub(crate) fn solution_check_signature(
		ext: &<Block as BlockT>::Extrinsic,
	) -> Result<AccountId, ()> {
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

	pub(crate) fn solution_validate_transaction(
		_source: TransactionSource,
		ext: <Block as BlockT>::Extrinsic,
		_block_hash: <Block as BlockT>::Hash,
	) -> TransactionValidity {
		log::debug!(
			target: LOG_TARGET,
			"Entering validate_transaction. tx: {:?}",
			ext,
		);

		let _ = Self::solution_check_signature(&ext)
			.map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::BadProof))?;
		Ok(ValidTransaction { provides: vec![ext.function.encode()], ..Default::default() })
	}
}
