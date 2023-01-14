//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{account, benchmarks, whitelisted_caller, Vec, Zero};
use frame_support::sp_runtime::traits::Bounded;
use frame_system::RawOrigin;

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


	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
