//! Benchmarking setup for pallet-template

#[allow(unused)]
use super::{Pallet as Template, *};
use frame_benchmarking::{account, benchmarks, whitelisted_caller, Zero};
use frame_support::{
	ensure,
	pallet_prelude::DispatchResult,
	sp_runtime::traits::{Bounded, Get},
};
use frame_system::RawOrigin;

const SEED: u32 = 0;

fn assert_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

// Call `register_voter` n times.
// The `whitelisted_caller` is always used as the final voter.
fn set_voters<T: Config>(n: u32) -> DispatchResult {
	if n > 0 {
		// Starting at 1 to leave room for the whitelisted caller.
		for i in 1..n {
			// Add random voters.
			let voter = account::<T::AccountId>("voter", i, SEED);
			Pallet::<T>::register_voter(RawOrigin::Root.into(), voter)?;
		}

		// Add the whitelisted caller at the end.
		let caller = whitelisted_caller();
		Pallet::<T>::register_voter(RawOrigin::Root.into(), caller)?;
	}

	Ok(())
}

// Set the votes of the voters in the pallet by number of ayes, nays, and abstains.
// If the total number of votes exceeds the number of voters, we will return an error.
fn set_votes<T: Config>(ayes: u32, nays: u32, abstain: u32) -> DispatchResult {
	let voters = Voters::<T>::get();
	let total_votes = ayes + nays + abstain;
	ensure!(voters.len() as u32 >= total_votes, "Too many votes for voters.");

	// Distribute votes to voters. Order of votes should not matter.
	for (i, voter) in voters.into_iter().enumerate() {
		if (i as u32) < ayes {
			Pallet::<T>::make_vote(RawOrigin::Signed(voter).into(), Vote::Aye)?;
		} else if (i as u32) < ayes + nays {
			Pallet::<T>::make_vote(RawOrigin::Signed(voter).into(), Vote::Nay)?;
		} else if (i as u32) < ayes + nays + abstain {
			Pallet::<T>::make_vote(RawOrigin::Signed(voter).into(), Vote::Abstain)?;
		} else {
			break
		}
	}

	Ok(())
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

	// Write benchmark for register_vote function.
	// Extra: Consider what would be needed to support Weight refunds.
	register_voter {
		// Need to leave room for at least one more voter to join.
		let v in 0 .. T::MaxVoters::get() - 1;
		// Some random account. Doesn't matter.
		let new_voter = account::<T::AccountId>("new_voter", 0, SEED);
		set_voters::<T>(v)?;
	}: register_voter(RawOrigin::Root, new_voter)
	verify {
		assert_eq!(Voters::<T>::get().len() as u32, v + 1)
	}

	// Write benchmark for make_vote function.
	// Extra: Consider what would be needed to support Weight refunds.
	make_vote {
		// Need to leave room for at least one more vote.
		let s in 0 .. T::MaxVoters::get() - 1;

		// At least 1 voter needed.
		set_voters::<T>(s + 1)?;
		set_votes::<T>(0, 0, s)?;

		let caller: T::AccountId = whitelisted_caller();
		let vote = Vote::Aye;
	}: make_vote(RawOrigin::Signed(caller.clone()), vote)
	verify {
		assert_event::<T>(Event::NewVote { who: caller }.into());
	}

	// Write a benchmark for the close_vote function.
	// Extra: Consider what would be needed to support Weight refunds.
	close_vote {
		// At least 1 voter/vote needed.
		let s in 1 .. T::MaxVoters::get();

		set_voters::<T>(s)?;
		set_votes::<T>(s, 0, 0)?;

		let caller = account::<T::AccountId>("closer", 0, SEED);
	}: close_vote(RawOrigin::Signed(caller))
	verify
	{
		assert_eq!(Votes::<T>::get().len(),0);
		assert_event::<T>(Event::Outcome { aye: true }.into());
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
