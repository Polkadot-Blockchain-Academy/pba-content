//! Benchmarking setup for pallet-template

#[allow(unused)]
use super::{Pallet as Template, *};
use frame_benchmarking::{account, benchmarks, vec, whitelisted_caller, Vec, Zero};
use frame_support::sp_runtime::traits::{Bounded, Get};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

const SEED: u32 = 0;

fn assert_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

fn set_voters<T: Config>(caller: Option<T::AccountId>) -> usize {
	let l = 1..T::MaxVoters::get() - 1;

	let mut voters = vec![];
	for i in l.into_iter() {
		let voter = account::<T::AccountId>("voter", i + 1, SEED);
		voters.push(voter);
	}

	if let Some(caller) = caller {
		voters.push(caller);
	}
	let bounded_voters: BoundedVec<T::AccountId, T::MaxVoters> =
		BoundedVec::try_from(voters).unwrap();
	Voters::<T>::set(bounded_voters.clone());
	bounded_voters.len()
}

fn set_votes<T: Config>() {
	let l = 1..T::MaxVoters::get() - 1;

	let mut votes: Vec<UserVote<T::AccountId, Vote>> = vec![];
	for i in l.clone().into_iter() {
		let who = account::<T::AccountId>("voter", i, SEED);
		let vote = match i % 3 {
			0 => Vote::Abstain,
			1 | 2 => Vote::Aye,
			_ => Vote::Nay,
		};
		let user_vote = UserVote { who, vote };
		votes.push(user_vote);
	}

	let bounded_votes: BoundedVec<UserVote<T::AccountId, Vote>, T::MaxVoters> =
		BoundedVec::try_from(votes).unwrap();
	Votes::<T>::set(bounded_votes);
}

benchmarks! {

	// Write a benchmark for how long it takes to add all values between 0 and 1_000_000.
	add { let mut total = 0u64; } : { (0..1_000_000).for_each(|x| { total += x; }) } verify { assert!(total > 0); }

	// Write the same benchmark, but use `checked_add` instead.
	checked_add { let mut total = 0u64; }: { (0..1_000_000).for_each(|x| { total = total.checked_add(x).unwrap(); }) } verify { assert!(total > 0); }

	// Write a benchmark which writes multiple times to the same storage.
	multi_write {
		let i in 1 .. 100u32;
		let user: T::AccountId = account("user", 0, 0);
	}: { (0..=i).for_each(|j| { MyMap::<T>::insert(&user, j) }) }
	verify {
		assert!(MyMap::<T>::get(user) == Some(i))
	}

	// Write a benchmark which writes values to multiples keys in the same map.
	multi_map {
		let i in 1 .. 100u32;
		let mut users: Vec<T::AccountId> = Vec::new();
		for j in 0 .. i {
			users.push(account("user", j, 0))
		}
	}: { users.iter().for_each(|who| { MyMap::<T>::insert(who, i) }) }
	verify {
		//assert!(MyMap::<T>::iter() == Some(i))
	}

	// Write a benchmark for the transfer function.
	transfer {
		let caller = whitelisted_caller();
		let recipient: T::AccountId = account("recipient", 0, 0);
		let balance = BalanceOf::<T>::max_value();
		T::Currency::make_free_balance_be(&caller, balance);
		let transfer_amount = balance / 2u32.into();
	}: _(RawOrigin::Signed(caller.clone()), recipient.clone(), transfer_amount)
	verify {
		assert!(!transfer_amount.is_zero());
		assert_eq!(T::Currency::free_balance(&caller), balance - transfer_amount);
		assert_eq!(T::Currency::free_balance(&recipient), transfer_amount);
	}

	// Write both benchmarks needed for the branching function.
	branch_true {
		let caller: T::AccountId = whitelisted_caller();
	}: branched_logic(RawOrigin::Signed(caller.clone()), true)
	verify {
		assert_eq!(MyValue::<T>::get(), None)
	}

	// Write both benchmarks needed for the branching function.
	branch_false {
		let caller: T::AccountId = whitelisted_caller();
	}: branched_logic(RawOrigin::Signed(caller.clone()), false)
	verify {
		assert_eq!(MyValue::<T>::get(), Some(1337))
	}

	// Write benchmark for register_vote function
	register_voter {
		let new_voter = account::<T::AccountId>("voter", 0, SEED);
		let l = set_voters::<T>(None);
	}: register_voter(RawOrigin::Root, new_voter)
	verify {
		assert_eq!(Voters::<T>::get().len(), l + 1)
	}

	// Write benchmark for make_vote function
	make_vote {
		let caller = account::<T::AccountId>("voter", 0, SEED);
		let origin = RawOrigin::Signed(caller.clone());
		let vote = Vote::Aye;
		set_voters::<T>(Some(caller));
		set_votes::<T>();

	}: make_vote(origin, vote)
	verify
	{
		assert_event::<T>(Event::NewVote.into());
	}

	close_vote {
		let caller = account::<T::AccountId>("voter", 0, SEED);
		let origin = RawOrigin::Signed(caller.clone());
		let vote = Vote::Aye;
		set_voters::<T>(Some(caller));
		set_votes::<T>();

	}: close_vote(origin)
	verify
	{
		assert_eq!(Votes::<T>::get().len(),0);
		assert_event::<T>(Event::Outcome { aye: true }.into());
	}



	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
