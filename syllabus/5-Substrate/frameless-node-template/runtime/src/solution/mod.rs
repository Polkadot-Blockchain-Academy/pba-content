use self::{
	currency::{BalancesMap, TotalIssuance},
	storage::{StorageMap, StorageValue},
};
use crate::{
	shared::{
		AccountId, Balance, Block, CurrencyCall, Extrinsic, RuntimeCall, StakingCall, SystemCall,
		EXTRINSICS_KEY, MINIMUM_BALANCE, SUDO, TREASURY, VALUE_KEY,
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
	ApplyExtrinsicResult, DispatchError, DispatchOutcome,
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
		AccountId::unchecked_from(SUDO)
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
		let sender = Self::inner_pre_dispatch(&ext)?;

		// execute it
		let dispatch_outcome = match ext.clone().function.call {
			RuntimeCall::System(SystemCall::Set { value }) => {
				sp_io::storage::set(VALUE_KEY, &value.encode());
				Ok(())
			},
			RuntimeCall::System(SystemCall::Remark { data: _ }) => Ok(()),
			RuntimeCall::System(SystemCall::SudoRemark { data: _ }) => {
				if sender == AccountId::unchecked_from(SUDO) {
					Ok(())
				} else {
					Err(DispatchError::BadOrigin)
				}
			},
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
		Self::consolidate_treasury();
		// note the extrinsic
		Self::mutate_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY, |current| {
			current.push(ext.encode());
		});

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

	fn check_signature(
		ext: &<Block as BlockT>::Extrinsic,
	) -> Result<AccountId, TransactionValidityError> {
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
		Ok(signer)
	}

	fn collect_tip(
		ext: &<Block as BlockT>::Extrinsic,
		signer: AccountId,
	) -> Result<(), TransactionValidityError> {
		if let Some(tip) = ext.function.tip {
			let allow_death = false; // The tip in itself is never capable of killing an account.
			let mut balance =
				currency::BalancesMap::<Self>::get(signer.clone()).unwrap_or_default();
			balance
				.withdraw(tip, allow_death)
				.map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::Payment))?;

			// The writes in this code are redundant in `validate_transaction`, but we write it like
			// this so we can reuse it. FRAME does the same in certain places as well.

			let treasury_acc = AccountId::unchecked_from(TREASURY);
			let mut treasury =
				currency::BalancesMap::<Self>::get(treasury_acc.clone()).unwrap_or_default();
			treasury
				.unchecked_receive(tip)
				.map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::Payment))?;
			currency::BalancesMap::<Self>::set(treasury_acc, treasury);

			currency::BalancesMap::<Self>::set(signer.clone(), balance);
		}

		Ok(())
	}

	fn consolidate_treasury() {
		let mut treasury =
			BalancesMap::<Self>::get(AccountId::unchecked_from(TREASURY)).unwrap_or_default();
		log::debug!(target: LOG_TARGET, "treasury: {:?}", treasury);
		if treasury.free < MINIMUM_BALANCE && treasury.free != 0 {
			let mut issuance = TotalIssuance::<Self>::get().unwrap_or_default();
			issuance -= treasury.free;
			log::warn!(target: LOG_TARGET, "burning {}", treasury.free);
			TotalIssuance::<Self>::set(issuance);
			drop(treasury);
			BalancesMap::<Self>::clear(AccountId::unchecked_from(TREASURY));
		}
	}

	fn check_nonce_pre_dispatch(
		ext: &<Block as BlockT>::Extrinsic,
		signer: AccountId,
	) -> Result<(), TransactionValidityError> {
		// This one, crucially, does not set the nonce, as it might need to be reverted.
		let mut balance = currency::BalancesMap::<Self>::get(signer.clone()).unwrap_or_default();

		match balance.nonce.cmp(&ext.function.nonce) {
			sp_std::cmp::Ordering::Equal => {},
			sp_std::cmp::Ordering::Greater =>
				return Err(TransactionValidityError::Invalid(InvalidTransaction::Stale)),
			sp_std::cmp::Ordering::Less =>
				return Err(TransactionValidityError::Invalid(InvalidTransaction::Future)),
		}

		Ok(())
	}

	fn check_nonce_validate(
		ext: &<Block as BlockT>::Extrinsic,
		signer: AccountId,
	) -> TransactionValidity {
		let balance = currency::BalancesMap::<Self>::get(signer.clone()).unwrap_or_default();

		let requires = match balance.nonce.cmp(&ext.function.nonce) {
			sp_std::cmp::Ordering::Equal => {
				vec![]
			},
			sp_std::cmp::Ordering::Greater =>
				return Err(TransactionValidityError::Invalid(InvalidTransaction::Stale)),
			sp_std::cmp::Ordering::Less => vec![(signer.clone(), ext.function.nonce - 1).encode()],
		};

		let provides = vec![(signer.clone(), ext.function.nonce).encode()];

		Ok(ValidTransaction {
			requires,
			provides,
			priority: ext.function.tip.unwrap_or_default().try_into().unwrap_or(u64::MAX),
			..Default::default()
		})
	}

	fn inner_pre_dispatch(
		ext: &<Block as BlockT>::Extrinsic,
	) -> Result<AccountId, TransactionValidityError> {
		let signer = Self::check_signature(ext)?;
		Self::check_nonce_pre_dispatch(ext, signer)?;
		Self::collect_tip(ext, signer)?;

		let mut balance = BalancesMap::<Runtime>::get(signer).unwrap_or_default();
		balance.nonce += 1;
		BalancesMap::<Runtime>::set(signer.clone(), balance);

		Ok(signer)
	}

	pub(crate) fn solution_validate_transaction(
		_source: TransactionSource,
		ext: <Block as BlockT>::Extrinsic,
		_block_hash: <Block as BlockT>::Hash,
	) -> TransactionValidity {
		let signer = Self::check_signature(&ext)?;
		let valid = Self::check_nonce_validate(&ext, signer)?;
		Self::collect_tip(&ext, signer)?;

		Ok(valid)
	}
}
