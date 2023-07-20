use self::storage::{StorageMap, StorageValue};
use crate::{
	shared::{
		AccountId, Balance, Block, CurrencyCall, Extrinsic, RuntimeCall, StakingCall, SystemCall,
		EXTRINSICS_KEY, MINIMUM_BALANCE, MINTER, TREASURY, VALUE_KEY,
	},
	Runtime, LOG_TARGET, VERSION,
};
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

#[allow(unused)]
impl Runtime {
	pub(crate) fn solution_apply_extrinsic(ext: Extrinsic) -> ApplyExtrinsicResult {
		let (sender, _tip, _nonce) = Self::solution_validate_transaction_inner(&ext)?;

		// execute it
		let dispatch_outcome = match ext.clone().function.call {
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

		log::debug!(target: LOG_TARGET, "dispatched {:?}, outcome = {:?}", ext, dispatch_outcome);

		Ok(dispatch_outcome)
	}

	pub(crate) fn solution_finalize_block(
		mut header: <Block as BlockT>::Header,
	) -> <Block as BlockT>::Header {
		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
		let state_root = sp_core::H256::decode(&mut &raw_state_root[..]).unwrap();

		let extrinsics = Self::get_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY).unwrap_or_default();
		let extrinsics_root =
			BlakeTwo256::ordered_trie_root(extrinsics, sp_runtime::StateVersion::V0);

		header.extrinsics_root = extrinsics_root;
		header.state_root = state_root;
		header
	}

	fn solution_validate_transaction_inner(
		ext: &<Block as BlockT>::Extrinsic,
	) -> Result<(AccountId, u64, u32), TransactionValidityError> {
		// first, check the signature.
		let signer = match &ext.signature {
			Some((signer, signature, extra_stuff)) => {
				let payload = (ext.function.clone(), extra_stuff);
				signature
					.verify(payload.encode().as_ref(), signer)
					.then(|| signer.clone())
					.ok_or(TransactionValidityError::Invalid(InvalidTransaction::BadProof))?
			},
			None => return Err(TransactionValidityError::Invalid(InvalidTransaction::BadProof)),
		};

		let mut balance = currency::BalancesMap::<Self>::get(signer.clone()).unwrap_or_default();
		let nonce = balance.nonce;

		// then check that their nonce is correct. Best to check this first, since it's cheap.
		match nonce.cmp(&ext.function.nonce) {
			sp_std::cmp::Ordering::Equal => {},
			sp_std::cmp::Ordering::Greater =>
				return Err(TransactionValidityError::Invalid(InvalidTransaction::Stale)),
			sp_std::cmp::Ordering::Less =>
				return Err(TransactionValidityError::Invalid(InvalidTransaction::Future)),
		}

		balance.nonce = balance.nonce.saturating_add(1);

		// then, check that they can pay for the fee.
		if let Some(tip) = ext.function.tip {
			let _ = balance
				.withdraw(tip)
				.map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::Payment))?;
			// We don't quite care of the treasury account is not funded and cannot handle the
			// incoming tip.
			// The writes in this code are redundant in `validate_transaction`, but we write it like
			// this so we can reuse it. FRAME does the same in certain places as well.

			// TODO: try and use our mutate api?
			let mut treasury =
				currency::BalancesMap::<Self>::get(AccountId::unchecked_from(TREASURY))
					.unwrap_or_default();
			match treasury.receive(tip) {
				Ok(_) => {
					currency::BalancesMap::<Self>::set(
						AccountId::unchecked_from(TREASURY),
						treasury,
					);
				},
				Err(_) => {
					let issuance = currency::TotalIssuance::<Self>::get();
					let mut issuance = issuance.expect("someone paid some fee that was withdrawn; issuance must already be non-zero");
					issuance -= tip;
					currency::TotalIssuance::<Self>::set(issuance);
				},
			}
		}

		// update tip and nonce.
		currency::BalancesMap::<Self>::set(signer.clone(), balance);

		let tip = ext.function.tip.map(|x| x as u64).unwrap_or_default();
		Ok((signer, tip, nonce))
	}

	pub(crate) fn solution_validate_transaction(
		_source: TransactionSource,
		ext: <Block as BlockT>::Extrinsic,
		_block_hash: <Block as BlockT>::Hash,
	) -> TransactionValidity {
		let (signer, tip, nonce) = Self::solution_validate_transaction_inner(&ext)?;
		let provides = vec![(signer.clone(), nonce).encode()];
		let requires = if nonce != 0 { vec![(signer.clone(), nonce - 1).encode()] } else { vec![] };
		Ok(ValidTransaction { requires, provides, priority: tip, ..Default::default() })
	}
}
