#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath};
use frame_support::sp_runtime::traits::Hash;
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The currency used in the blockchain.
		type Currency: Currency<Self::AccountId>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	pub type MyValue<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type MyMap<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn store_value(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			MyValue::<T>::put(something);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn store_map(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			MyMap::<T>::insert(who, something);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			T::Currency::transfer(&from, &to, amount, AllowDeath)
		}

		#[pallet::weight(0)]
		pub fn branched_logic(
			origin: OriginFor<T>,
			branch: bool,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			if branch {
				// This branch uses a lot of computation.
				(0..1337).for_each(|x| { T::Hashing::hash(&x.encode()); });
			} else {
				// This branch uses storage.
				MyValue::<T>::put(1337);
			}

			Ok(())
		}
	}
}
