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

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
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

		type WeightInfo: weights::WeightInfo;
	}

	#[pallet::storage]
	pub type MyValue<T> = StorageValue<_, u8, ValueQuery>;

	#[pallet::storage]
	pub type MyMap<T: Config> = StorageMap<_, Blake2_128Concat, u8, T::AccountId>;

	// A simple set of choices for a user vote.
	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Clone, Copy, Eq, PartialEq)]
	pub enum Vote {
		Abstain,
		Aye,
		Nay,
	}

	// A struct which connects a user to their vote.
	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug)]
	pub struct UserVote<AccountId, Vote> {
		pub who: AccountId,
		pub vote: Vote,
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
		Outcome { aye: bool },
		NewVote { who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AlreadyVoter,
		TooManyVoters,
		NotVoter,
		NotComplete,
		NoVoters,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// An extrinsic which (inefficiently) counts to `amount` and stores it.
		#[pallet::call_index(0)]
		#[pallet::weight(u64::default())]
		pub fn counter(origin: OriginFor<T>, amount: u8) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			for i in 0..=amount {
				// Writing to storage each loop is not super smart... but illustrative.
				MyValue::<T>::put(i);
			}
			Ok(())
		}

		// An extrinsic which puts claims the indexes up to `amount` for a user.
		#[pallet::call_index(1)]
		#[pallet::weight(u64::default())]
		pub fn claimer(origin: OriginFor<T>, amount: u8) -> DispatchResult {
			let who = ensure_signed(origin)?;
			for i in 0..=amount {
				MyMap::<T>::insert(i, &who);
			}
			Ok(())
		}

		// An extrinsic which transfers the entire free balance of a user to another user.
		// Includes a call to `transfer`, which is implemented in a different pallet.
		#[pallet::call_index(2)]
		#[pallet::weight(u64::default())]
		pub fn transfer_all(origin: OriginFor<T>, to: T::AccountId) -> DispatchResult {
			let from = ensure_signed(origin)?;
			let amount = T::Currency::free_balance(&from);
			T::Currency::transfer(&from, &to, amount, AllowDeath)
		}

		// An extrinsic which has two very different logical paths.
		#[pallet::call_index(3)]
		#[pallet::weight(u64::default())]
		pub fn branched_logic(origin: OriginFor<T>, branch: bool) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;
			if branch {
				// This branch uses a lot of computation.
				(0..1337).for_each(|x| {
					T::Hashing::hash(&x.encode());
				});
				Ok(().into())
			} else {
				// This branch uses storage.
				MyValue::<T>::put(69);
				Ok(().into())
			}
		}

		// The following extrinsics form a simple voting system. Take into account worst case
		// scenarios.

		// Register a user which is allowed to be a voter. Only callable by the `Root` origin.
		#[pallet::call_index(4)]
		#[pallet::weight(u64::default())]
		pub fn register_voter(
			origin: OriginFor<T>,
			who: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let mut voters = Voters::<T>::get();
			ensure!(!voters.contains(&who), Error::<T>::AlreadyVoter);
			voters.try_push(who).map_err(|_| Error::<T>::TooManyVoters)?;
			Voters::<T>::set(voters);
			Ok(().into())
		}

		// Allow a registered voter to make or update their vote.
		#[pallet::call_index(5)]
		#[pallet::weight(u64::default())]
		pub fn make_vote(origin: OriginFor<T>, vote: Vote) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let voters = Voters::<T>::get();
			ensure!(voters.contains(&who), Error::<T>::NotVoter);
			let mut votes = Votes::<T>::get();

			let maybe_index = votes.iter().position(|v| v.who == who);

			let user_vote = UserVote { who: who.clone(), vote };

			if let Some(index) = maybe_index {
				votes[index] = user_vote;
			} else {
				votes.try_push(user_vote).map_err(|_| Error::<T>::TooManyVoters)?;
			}
			Votes::<T>::set(votes);
			Self::deposit_event(Event::<T>::NewVote { who });
			Ok(().into())
		}

		// Attempt to resolve a vote, which emits the outcome and resets the votes.
		#[pallet::call_index(6)]
		#[pallet::weight(u64::default())]
		pub fn close_vote(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// Any user can attempt to close the vote.
			let _who = ensure_signed(origin)?;
			// Gets just the length of voters.
			let max_voters = Voters::<T>::decode_len().unwrap_or(0);
			ensure!(max_voters > 0, Error::<T>::NoVoters);
			let votes = Votes::<T>::get();
			let votes_len = votes.len();
			let mut ayes = 0;
			let mut nays = 0;
			let not_voted = max_voters - votes_len;

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
				return Err(Error::<T>::NotComplete.into())
			}

			Votes::<T>::kill();
			Ok(().into())
		}

		// Extra Credit: Make a `remove_voter` function, and benchmark it.
		// You will need to think about how this function may affect other extrinsics above.
	}
}
