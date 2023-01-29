---
title: FRAME Pallet Coupling
description: A look into how multiple pallets interact.
duration: 1 hour
instructors: ["Shawn Tabrizi"]
---

# Pallet Coupling

---

## Overview

Substrate believes in building modular and composable blockchain runtimes.

The building blocks of a FRAME runtime are Pallets.

Pallet coupling will teach you how to configure multiple pallets to interact with each other.

---

## Types of Coupling

- Tightly Coupled Pallets

  - Pallets which are directly connected to one another.
  - You must construct a runtime using exactly the pallets which are tightly coupled.

- Loosely Coupled Pallets

  - Pallets which are connected "loosely" with a trait / interface.
  - You can construct a runtime using any pallets which satisfy the required interfaces.

---

## Tightly Coupled Pallets

Tightly coupling is often an easier, but less flexible way to have two pallets interact with each other.

It looks like this:

```rust [2]
#[pallet::config]
pub trait Config: frame_system::Config + pallet_treasury::Config {
	// -- snip --
}
```

Note that everything is tightly coupled to `frame_system`!

---

## What Does It Mean?

If Pallet A is tightly coupled to Pallet B, then it basically means:

> This Pallet A requires a runtime which is also configured with Pallet B.

You do not necessarily need Pallet A to use Pallet B, but you will always need Pallet B if you use Pallet A.

---

## Example: Treasury Pallet

The Treasury Pallet is a standalone pallet which controls a pot of funds that can be distributed by the governance of the chain.

There are two other pallets which are tightly coupled with the Treasury Pallet: Tips and Bounties.

You can think of these like "Pallet Extensions".

---

## Treasury, Tips, Bounties

`pallet_treasury`

```rust
#[pallet::config]
pub trait Config<I: 'static = ()>: frame_system::Config { ... }
```

`pallet_tips` & `pallet_bounties`

```rust
#[pallet::config]
pub trait Config<I: 'static = ()>: frame_system::Config + pallet_treasury::Config<I> { ... }
```

---

## Tight Coupling Error

Here is the kind of error you will see when you try to use a tightly coupled pallet without the appropriate pallet dependencies configured:

```rust
error[E0277]: the trait bound `Test: pallet_treasury::Config` is not satisfied
   --> frame/sudo/src/mock.rs:149:17
    |
149 | impl Config for Test {
    |                 ^^^^ the trait `pallet_treasury::Config` is not implemented for `Test`
    |
n o t e: required by a bound in `pallet::Config`
   --> frame/sudo/src/lib.rs:125:43
    |
125 |     pub trait Config: frame_system::Config + pallet_treasury::Config{
    |                                              ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Config`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `pallet-sudo` due to previous error
warning: build failed, waiting for other jobs to finish...
```

---

## Advantage of Tight Coupling

With tight coupling, you have direct access to all public functions and interfaces of another pallet. Just like directly using a crate / module.

Examples:

```rust
// Get the block number from `frame_system`
frame_system::Pallet::<T>::block_number()
```

```rust
// Use type configurations defined in another pallets.
let who: T::AccountId = ensure_signed(origin)?;
```

```rust
// Dispatch an error defined in another pallet.
ensure!(
	bounty.value <= max_amount,
	pallet_treasury::Error::<T, I>::InsufficientPermission
);
```

---

## When To Use Tight Coupling

Tight coupling can make a lot of sense when trying to break apart a single "large" pallet into smaller, yet fully dependant pieces.

As mentioned before, you can think of these as "extensions".

Since there is less flexibility in how you can configure tightly coupled pallets, there is also less chance for error in configuring them.

---

## Loosely Coupled Pallets

Loose coupling is the "preferred" way to build Pallets, as it emphasizes the modular nature of Pallet development.

It looks like this:

```rust [3]
#[pallet::config]
pub trait Config<I: 'static = ()>: frame_system::Config {
	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

	// -- snip --
}
```

Here you can see that this pallet requires some associated type `Currency` to be configured which implements some traits `Currency` and `ReservableCurrency`, however there is no requirements on how or where that type is configured.

---

## Trait Definition

To begin loose coupling, you need to define a trait / interface that can be provided and depended on. A very common example is the `Currency` trait, which most often is implemented by `pallet_balances`.

```rust
/// Abstraction over a fungible assets system.
pub trait Currency<AccountId> {
	/// The balance of an account.
	type Balance: Balance + MaybeSerializeDeserialize + Debug + MaxEncodedLen + FixedPointOperand;

	/// The 'free' balance of a given account.
	/// This is the only balance that matters in terms of most operations on tokens.
	fn free_balance(who: &AccountId) -> Self::Balance;

	/// Transfer some liquid free balance to another user.
	fn transfer(
		source: &AccountId,
		dest: &AccountId,
		value: Self::Balance,
		existence_requirement: ExistenceRequirement,
	) -> DispatchResult;

	// -- snip --
}
```

---

## Trait Implementation

This trait can then be implemented by a Pallet, for example `pallet_balances`.

```rust
impl<T: Config<I>, I: 'static> Currency<T::AccountId> for Pallet<T, I>
where
	T::Balance: MaybeSerializeDeserialize + Debug,
{
	type Balance = T::Balance;

	fn free_balance(who: &T::AccountId) -> Self::Balance {
		Self::account(who).free
	}

	// -- snip --
}
```

Any pallet, even one you write, could implement this trait.

---

## Trait Dependency

Another pallet can then, separately, depend on this trait.

```rust [3]
#[pallet::config]
pub trait Config: frame_system::Config {
	type Currency: Currency<Self::AccountId>;
}
```

And can use this trait throughout their pallet:

```rust [4-5]
#[pallet::weight(0)]
pub fn transfer_all(origin: OriginFor<T>, to: T::AccountId) -> DispatchResult {
	let from = ensure_signed(origin)?;
	let amount = T::Currency::free_balance(&from);
	T::Currency::transfer(&from, &to, amount, AllowDeath)
}
```

---

## Runtime Implementation

Finally, in the runtime configuration, we concretely define which pallet implements the trait.

```rust
/// Configuration of a pallet using the `Currency` trait.
impl pallet_voting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = pallet_balances::Pallet<Runtime>;
}
```

This is the place where things are no longer "loosely" defined.

---

## Challenges of Loose Coupling

Loose coupling is more difficult because you need to think ahead of time about developing a flexible API that makes sense for potentially multiple implementations.

You need to try to not let implementation details affect the API, providing maximum flexibility to users and providers of those traits.

When done right, it can be very powerful; like the ERC20 token format.

---

## Challenges of Generic Types

Many new pallet developers also find loose coupling challenging because associated types are not concretely defined... on purpose.

For example, note that the `Currency` trait has a generic `Balances` type.

This allows pallet developers can configure most unsigned integers types (`u32`, `u64`, `u128`) as the `Balance` type for their chain, however, this also means that you need to be more clever when doing math or other operations with those generic types.

---

# Questions?

Next we will look over common pallets and traits, and will see many of the pallet coupling patterns first hand.
