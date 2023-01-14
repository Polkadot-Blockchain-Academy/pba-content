#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	sp_runtime::traits::Hash,
	traits::{Currency, ExistenceRequirement::AllowDeath},
};
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

		/// A configurable maximum number of users for this pallet.
		type MaxVoters: Get<u32>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	pub type MyValue<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type MyMap<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

	// A simple set of choices for a user vote.
	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Clone, Copy, Eq, PartialEq)]
	pub enum Vote {
		Abstain,
		Aye,
		Nay,
	}

	// A struct which connects a user to their vote.
	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	pub struct UserVote<AccountId, Vote> {
		who: AccountId,
		vote: Vote,
	}

	// The set of users allowed to make a vote, not sorted.
	#[pallet::storage]
	pub type Voters<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxVoters>, ValueQuery>;

	// The current set of votes by voters, not sorted.
	#[pallet::storage]
	pub type Votes<T: Config> =
		StorageValue<_, BoundedVec<UserVote<T::AccountId, Vote>, T::MaxVoters>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Outcome { aye: bool }
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AlreadyVoter,
		TooManyVoters,
		NotVoter,
		NotComplete,
	}

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
		pub fn branched_logic(origin: OriginFor<T>, branch: bool) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			if branch {
				// This branch uses a lot of computation.
				(0..1337).for_each(|x| {
					T::Hashing::hash(&x.encode());
				});
			} else {
				// This branch uses storage.
				MyValue::<T>::put(1337);
			}

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn register_voter(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			let mut voters = Voters::<T>::get();
			ensure!(!voters.contains(&who), Error::<T>::AlreadyVoter);
			voters.try_push(who).map_err(|_| Error::<T>::TooManyVoters)?;
			Voters::<T>::set(voters);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn make_vote(origin: OriginFor<T>, vote: Vote) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let voters = Voters::<T>::get();
			ensure!(voters.contains(&who), Error::<T>::NotVoter);
			let mut votes = Votes::<T>::get();

			let maybe_index = votes.iter().position(|v| { v.who == who });

			let user_vote = UserVote { who, vote };

			if let Some(index) = maybe_index {
				votes[index] = user_vote;
			} else {
				votes.try_push(user_vote).map_err(|_| Error::<T>::TooManyVoters)?;
			}
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn close_vote(origin: OriginFor<T>) -> DispatchResult {
			// Any user can attempt to close the vote.
			let _who = ensure_signed(origin)?;
			// Gets just the length of voters.
			let max_voters = Voters::<T>::decode_len().unwrap_or(0);
			let votes = Votes::<T>::get();
			let mut ayes = 0;
			let mut nays = 0;
			let not_voted = max_voters - votes.len();

			votes.iter().for_each(|v| {
				match v.vote {
					Vote::Aye => ayes += 1,
					Vote::Nay => nays += 1,
					// Nothing to do for abstainers.
					Vote::Abstain => {},
				}
			});

			if ayes > nays + not_voted {
				Self::deposit_event(Event::<T>::Outcome { aye: true });
			} else if nays >= ayes + not_voted {
				Self::deposit_event(Event::<T>::Outcome { aye: false });
			} else {
				return Err(Error::<T>::NotComplete.into());
			}

			Votes::<T>::kill();
			Ok(())
		}
	}
}
