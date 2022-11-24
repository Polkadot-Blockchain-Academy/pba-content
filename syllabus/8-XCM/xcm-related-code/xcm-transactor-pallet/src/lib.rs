// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;

pub use pallet::*;

#[pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::{boxed::Box, convert::TryFrom, vec, vec::Vec};
	use xcm::{latest::prelude::*, VersionedMultiLocation};
	use xcm_executor::traits::{Convert, InvertLocation};

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Convert `Origin` to `MultiLocation`.
		type OriginConverter: Convert<Self::RuntimeOrigin, MultiLocation>;

		/// Means of inverting a location.
		type LocationInverter: InvertLocation;

		// The origin that is allowed to dispatch calls from the sovereign account directly
		type SovereignAccountDispatcherOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// XCM sender.
		type XcmSender: SendXcm;
	}

	/// Enum defining the way to express a Currency.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, scale_info::TypeInfo)]
	pub enum Currency {
		// Express the Currency as its MultiLOcation
		AsMultiLocation(Box<VersionedMultiLocation>),
	}

	impl Default for Currency {
		fn default() -> Currency {
			Currency::AsMultiLocation(Box::new(MultiLocation::default().into()))
		}
	}

	#[derive(Default, Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, scale_info::TypeInfo)]
	/// Struct that defines how to express the payment in a particular currency
	/// currency is defined by the Currency enum, which can be expressed as:
	/// - MultiLocation
	pub struct CurrencyPayment {
		// the currency in which we want to express our payment
		pub currency: Currency,
		// indicates the fee amount to be used
		pub fee_amount: u128,
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebug, PartialEq, scale_info::TypeInfo)]
	/// Struct tindicating information about transact weights
	/// It allows to specify:
	/// - transact_required_weight_at_most: the amount of weight the Transact instruction should
	///   consume at most
	/// - overall_weight: the overall weight to be used for the whole XCM message execution.
	pub struct TransactWeights {
		// the amount of weight the Transact instruction should consume at most
		pub transact_required_weight_at_most: xcm::latest::Weight,
		// the overall weight to be used for the whole XCM message execution.
		pub overall_weight: xcm::latest::Weight,
	}

	/// An error that can occur while executing the mapping pallet's logic.
	#[pallet::error]
	pub enum Error<T> {
		IndexAlreadyClaimed,
		UnclaimedIndex,
		NotOwner,
		UnweighableMessage,
		CannotReanchor,
		AssetHasNoReserve,
		InvalidDest,
		NotCrossChainTransfer,
		AssetIsNotReserveInDestination,
		DestinationNotInvertible,
		ErrorSending,
		DispatchWeightBiggerThanTotalWeight,
		WeightOverflow,
		AmountOverflow,
		TransactorInfoNotSet,
		NotCrossChainTransferableCurrency,
		XcmExecuteError,
		BadVersion,
		MaxWeightTransactReached,
		UnableToWithdrawAsset,
		FeePerSecondNotSet,
		SignedTransactNotAllowedForDestination,
		FailedMultiLocationToJunction,
		FailedOriginToMultiLocationConversion,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Transacted the call through the sovereign account in a destination chain.
		TransactedSovereign { dest: MultiLocation, call: Vec<u8> },
		/// Transacted the call through a signed account in a destination chain.
		TransactedSigned { caller: MultiLocation, dest: MultiLocation, call: Vec<u8> },
		/// Transact failed
		TransactFailed { error: XcmError },
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transact the call through the sovereign account in a destination chain,
		///
		/// SovereignAccountDispatcherOrigin callable only
		#[pallet::weight(0)]
		pub fn transact_through_sovereign(
			origin: OriginFor<T>,
			// destination to which the message should be sent
			dest: Box<VersionedMultiLocation>,
			// fee to be used
			fee: CurrencyPayment,
			// call to be executed in destination
			call: Vec<u8>,
			// origin kind to be used
			origin_kind: OriginKind,
			// weight information to be used
			weight_info: TransactWeights,
		) -> DispatchResult {
			T::SovereignAccountDispatcherOrigin::ensure_origin(origin)?;

			let fee_location = Self::currency_to_multilocation(fee.currency)
				.ok_or(Error::<T>::NotCrossChainTransferableCurrency)?;

			let dest = MultiLocation::try_from(*dest).map_err(|()| Error::<T>::BadVersion)?;
			// Grab the destination
			Self::transact_in_dest_chain_asset_non_signed(
				dest.clone(),
				fee_location,
				call.clone(),
				origin_kind,
				fee.fee_amount,
				weight_info,
			)?;

			// Deposit event
			Self::deposit_event(Event::<T>::TransactedSovereign { dest, call });

			Ok(())
		}

		/// Transact the call through the a signed origin in this chain
		/// that should be converted to a transaction dispatch account in the destination chain
		/// by any method implemented in the destination chains runtime
		///
		/// This time we are giving the currency as a currencyId instead of multilocation
		#[pallet::weight(0)]
		pub fn transact_through_descended_sovereign(
			origin: OriginFor<T>,
			// destination to which the message should be sent
			dest: Box<VersionedMultiLocation>,
			// fee to be used
			fee: CurrencyPayment,
			// call to be executed in destination
			call: Vec<u8>,
			// weight information to be used
			weight_info: TransactWeights,
		) -> DispatchResult {
			// Convert origin to multilocation
			let caller = T::OriginConverter::convert(origin)
				.map_err(|_| Error::<T>::FailedMultiLocationToJunction)?;

			let dest = MultiLocation::try_from(*dest).map_err(|()| Error::<T>::BadVersion)?;

			let fee_location = Self::currency_to_multilocation(fee.currency)
				.ok_or(Error::<T>::NotCrossChainTransferableCurrency)?;

			// Grab the destination
			Self::transact_in_dest_chain_asset_signed(
				dest.clone(),
				caller.clone(),
				fee_location,
				call.clone(),
				OriginKind::SovereignAccount,
				fee.fee_amount,
				weight_info,
			)?;

			// Deposit event
			Self::deposit_event(Event::<T>::TransactedSigned { caller, dest, call });

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn transact_in_dest_chain_asset_non_signed(
			dest: MultiLocation,
			fee_location: MultiLocation,
			call: Vec<u8>,
			origin_kind: OriginKind,
			fee_amount: u128,
			weight_info: TransactWeights,
		) -> DispatchResult {
			let fee = MultiAsset { id: Concrete(fee_location), fun: Fungible(fee_amount) };

			// Construct the transact message. This is composed of WithdrawAsset||BuyExecution||
			// Transact.
			// WithdrawAsset: Withdraws "amount" from the sovereign account. These tokens will be
			// used to pay fees
			// BuyExecution: Buys "execution power" in the destination chain
			// Transact: Issues the transaction
			let transact_message: Xcm<()> = Self::transact_message(
				dest.clone(),
				fee,
				weight_info.overall_weight,
				call,
				weight_info.transact_required_weight_at_most,
				origin_kind,
			)?;

			// Send to destination
			T::XcmSender::send_xcm(dest, transact_message).map_err(|_| Error::<T>::ErrorSending)?;

			Ok(())
		}

		fn transact_in_dest_chain_asset_signed(
			dest: MultiLocation,
			caller: MultiLocation,
			fee_location: MultiLocation,
			call: Vec<u8>,
			origin_kind: OriginKind,
			fee_amount: u128,
			weight_info: TransactWeights,
		) -> DispatchResult {
			// Construct the transact message. This is composed of WithdrawAsset||BuyExecution||
			// Transact.
			// WithdrawAsset: Withdraws "amount" from the sovereign account. These tokens will be
			// used to pay fees
			// BuyExecution: Buys "execution power" in the destination chain
			// Transact: Issues the transaction
			let mut transact_message: Xcm<()> = Self::transact_message(
				dest.clone(),
				MultiAsset { id: Concrete(fee_location), fun: Fungible(fee_amount) },
				weight_info.overall_weight,
				call,
				weight_info.transact_required_weight_at_most,
				origin_kind,
			)?;

			// We append DescendOrigin as the first instruction in the message
			// The new message looks like DescendOrigin||WithdrawAsset||BuyExecution||
			// Transact.
			let interior: Junctions = caller
				.clone()
				.try_into()
				.map_err(|_| Error::<T>::FailedMultiLocationToJunction)?;
			transact_message.0.insert(0, DescendOrigin(interior));

			// Send to destination chain
			T::XcmSender::send_xcm(dest, transact_message).map_err(|_| Error::<T>::ErrorSending)?;

			Ok(())
		}

		/// Construct the transact xcm message with the provided parameters
		fn transact_message(
			dest: MultiLocation,
			asset: MultiAsset,
			dest_weight: xcm::latest::Weight,
			call: Vec<u8>,
			dispatch_weight: xcm::latest::Weight,
			origin_kind: OriginKind,
		) -> Result<Xcm<()>, DispatchError> {
			Ok(Xcm(vec![
				Self::withdraw_instruction(asset.clone(), &dest)?,
				Self::buy_execution(asset, &dest, dest_weight)?,
				Transact {
					origin_type: origin_kind,
					require_weight_at_most: dispatch_weight,
					call: call.into(),
				},
			]))
		}

		/// Construct a buy execution xcm order with the provided parameters
		fn buy_execution(
			asset: MultiAsset,
			at: &MultiLocation,
			weight: u64,
		) -> Result<Instruction<()>, DispatchError> {
			let ancestry = T::LocationInverter::ancestry();
			let fees = asset.reanchored(at, &ancestry).map_err(|_| Error::<T>::CannotReanchor)?;

			Ok(BuyExecution { fees, weight_limit: WeightLimit::Limited(weight) })
		}

		/// Construct a withdraw instruction from a sovereign account
		fn withdraw_instruction(
			asset: MultiAsset,
			at: &MultiLocation,
		) -> Result<Instruction<()>, DispatchError> {
			let ancestry = T::LocationInverter::ancestry();
			let fees = asset.reanchored(at, &ancestry).map_err(|_| Error::<T>::CannotReanchor)?;

			Ok(WithdrawAsset(fees.into()))
		}

		/// Converts Currency to multilocation
		pub fn currency_to_multilocation(currency: Currency) -> Option<MultiLocation> {
			match currency {
				Currency::AsMultiLocation(multiloc) => MultiLocation::try_from(*multiloc).ok(),
			}
		}
	}
}
