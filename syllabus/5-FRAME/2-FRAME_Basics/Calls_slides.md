---
title: FRAME Calls
description: FRAME Calls for Web3 engineers.
duration: 1 hour
instructors: ["Shawn Tabrizi, Kian Paimani"]
---

# FRAME Calls

---

## Overview

Calls allow users to interact with your state transition function.

In this lecture, you will learn how to create calls for your Pallet with FRAME.

---

## Terminology

The term "call", "extrinsic", and "dispatchable" all get mixed together.

Here is a sentence which should help clarify their relationship, and why they are such similar terms:

> Users submit an **extrinsic** to the blockchain, which is **dispatched** to a Pallet **call**.

---

## Call Definition

Here is a simple pallet call. Let's break it down.

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
	#[pallet::call_index(0)]
	#[pallet::weight(T::WeightInfo::transfer())]
	pub fn transfer(
		origin: OriginFor<T>,
		dest: AccountIdLookupOf<T>,
		#[pallet::compact] value: T::Balance,
	) -> DispatchResult {
		let transactor = ensure_signed(origin)?;
		let dest = T::Lookup::lookup(dest)?;
		<Self as Currency<_>>::transfer( &transactor, &dest, value, ExistenceRequirement::AllowDeath)?;
		Ok(())
	}
}
```

---

## Call Implementation

Calls are just functions which are implemented on top of the `Pallet` struct.

You can do this with any kind of function, however, "FRAME magic" turns these into dispatchable calls through the `#[pallet::call]` macro.

---

## Call Origin

Every pallet call must have an `origin` parameter, which uses the `OriginFor<T>` type, which comes from `frame_system`.

```rust
/// Type alias for the `Origin` associated type of system config.
pub type OriginFor<T> = <T as crate::Config>::RuntimeOrigin;
```

---

## Origin

The basic origins available in frame are:

```rust
/// Origin for the System pallet.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum RawOrigin<AccountId> {
	/// The system itself ordained this dispatch to happen: this is the highest privilege level.
	Root,
	/// It is signed by some public key and we provide the `AccountId`.
	Signed(AccountId),
	/// It is signed by nobody, can be either:
	/// * included and agreed upon by the validators anyway,
	/// * or unsigned transaction validated by a pallet.
	None,
}
```

We will have another presentation diving deeper into Origins.

---

## Call Parameters

Pallet calls can have additional parameters beyond `origin` allowing you to submit relevant data about what the caller would like to do.

All call parameters need to satisfy the `Parameter` trait:

```rust
/// A type that can be used as a parameter in a dispatchable function.
pub trait Parameter: Codec + EncodeLike + Clone + Eq + fmt::Debug + scale_info::TypeInfo {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + fmt::Debug + scale_info::TypeInfo {}
```

---



## Weight

---

## Call Index

---

## Compact Parameters



---

## Call Return
