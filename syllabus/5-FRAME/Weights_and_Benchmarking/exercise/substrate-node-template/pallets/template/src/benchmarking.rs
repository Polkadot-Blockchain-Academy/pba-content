//! Benchmarking setup for pallet-template

#[allow(unused)]
use super::{Pallet as Template, *};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{
	ensure,
	pallet_prelude::DispatchResultWithPostInfo,
	sp_runtime::traits::{Bounded, Get},
};
use frame_system::RawOrigin;

const SEED: u32 = 0;

fn assert_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

// Call `register_voter` n times.
// The `whitelisted_caller` is always used as the final voter.
fn set_voters<T: Config>(n: u32) -> DispatchResultWithPostInfo {
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

	Ok(().into())
}

// Set the votes of the voters in the pallet by number of ayes, nays, and abstains.
// If the total number of votes exceeds the number of voters, we will return an error.
fn set_votes<T: Config>(ayes: u32, nays: u32, abstain: u32) -> DispatchResultWithPostInfo {
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

	Ok(().into())
}

// - Complete the following benchmarks.
// - Ensure they test for the worst case scenario.
// - Where it makes sense, use components to get parameterized weight outputs.
// - Add a verification function to each benchmark to make sure the expected code executed.
// - Don't forget to use the `whitelisted_caller` for the `Origin`.
// - You can iteratively test your benchmarks with:
// 		`cargo test -p pallet-template --features runtime-benchmarks`
benchmarks! {
	// Write a benchmark for `i` hashes.
	hashing {
		let i in 0 .. 1_000;
	}: {
		(0..i).for_each(|x| {
			// Just add some kind of hashing here!
			// Hint: Look at the pallet code for some copyable code!
		})
	}

	// Write a benchmark for the `counter` extrinsic.

	// Write a benchmark for the `claimer` extrinsic.

	// Write a benchmark for the `transfer_all` extrinsic.
	// Hint: This is a valid line of code:
	// `T::Currency::make_free_balance_be(&caller, balance);`

	// Write **both** benchmarks needed for the branching function.

	// For the next benchmarks, feel free to use the provided helper functions in this file.

	// Write benchmark for register_vote function.
	// How can you verify that things executed as expected?
	// Extra: Consider what would be needed to support Weight refunds.

	// Write benchmark for make_vote function in the worst case scenario.
	// How can you verify that things executed as expected?
	// Extra: Consider what would be needed to support Weight refunds.

	// Write a benchmark for the close_vote function in the worst case scenario.
	// How can you verify that things executed as expected?
	// Extra: Consider what would be needed to support Weight refunds.

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
