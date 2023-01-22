---
title: FRAME Origin
description: Deep dive into FRAME Origins
duration: 1 hour
instructors: ["Shawn Tabrizi"]
---


# Origin

---


## Origin

This presentation will cover the use of Origin in FRAME, and how you can customize and extend this abstractions.

---

## What is Origin?

All dispatchable calls have an `Origin` that describes where the call originates from.

```rust
/// Make some on-chain remark.
#[pallet::weight(T::SystemWeightInfo::remark(_remark.len() as u32))]
pub fn remark(origin: OriginFor<T>, _remark: Vec<u8>) -> DispatchResultWithPostInfo {
	ensure_signed_or_root(origin)?;
	Ok(().into())
}
```

---

## FRAME System `RawOrigin`

These are origins which are included with FRAME by default.

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

---

## How is it used?

The Runtime Origin is used by dispatchable functions to check where a call has come from.

This is similar to `msg.sender` in Solidity, but FRAME is more powerful, and so is `Origin`.

---

## Origin Checks

```rust
/// Ensure that the origin `o` represents the root. Returns `Ok` or an `Err` otherwise.
pub fn ensure_root<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<(), BadOrigin>
```

```rust
/// Ensure that the origin `o` represents a signed extrinsic (i.e. transaction).
/// Returns `Ok` with the account that signed the extrinsic or an `Err` otherwise.
pub fn ensure_signed<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<AccountId, BadOrigin>
```

```rust
/// Ensure that the origin `o` represents an unsigned extrinsic. Returns `Ok` or an `Err` otherwise.
pub fn ensure_none<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<(), BadOrigin>
```

```rust
/// Ensure that the origin `o` represents either a signed extrinsic (i.e. transaction) or the root.
/// Returns `Ok` with the account that signed the extrinsic, `None` if it was root,  or an `Err`
/// otherwise.
pub fn ensure_signed_or_root<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<Option<AccountId>, BadOrigin>
```

---

## Examples: Signed Origin

A Simple Balance Transfer.

```rust
#[pallet::call_index(0)]
#[pallet::weight(T::WeightInfo::transfer())]
pub fn transfer(
	origin: OriginFor<T>,
	dest: AccountIdLookupOf<T>,
	#[pallet::compact] value: T::Balance,
) -> DispatchResultWithPostInfo {
	let transactor = ensure_signed(origin)?;
	// -- snip --
}
```

Most extrinsics use a `Signed` origin.

---

## Examples: Root Origin

The extrinsic to upgrade a chain.

```rust
/// Set the new runtime code.
#[pallet::call_index(2)]
#[pallet::weight((T::BlockWeights::get().max_block, DispatchClass::Operational))]
pub fn set_code(origin: OriginFor<T>, code: Vec<u8>) -> DispatchResultWithPostInfo {
	ensure_root(origin)?;
	Self::can_set_code(&code)?;
	T::OnSetCode::set_code(code)?;
	Ok(().into())
}
```

`Root` has access to many functions which can directly modify your blockchain. Assume Root access can do anything.

---

## Examples: None Origin

Setting the timestamp of the block.

```rust
		/// Set the current time.
		#[pallet::call_index(0)]
		#[pallet::weight((T::WeightInfo::set(), DispatchClass::Mandatory))]
		pub fn set(origin: OriginFor<T>, #[pallet::compact] now: T::Moment) -> DispatchResult {
			ensure_none(origin)?;
			// -- snip --
		}
	}
```

`None` origin is not very straight forward. More details next...

---

## None for Inherents

`None` origin can be used to represents extrinsics which are specifically included by the block author, also known as an inherent.

In those cases, it includes inherent checking logic with `ProvideInherent`.

```rust
#[pallet::inherent]
impl<T: Config> ProvideInherent for Pallet<T> {
	type Call = Call<T>;
	type Error = InherentError;
	const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

	// -- snip --
```

---

## None for Unsigned

`None` can also be used to represent "unsigned extrinsics", which are intended to be submitted by anyone without a key.

In those cases, it includes unsigned validation logic with `ValidateUnsigned`.


```rust
#[pallet::validate_unsigned]
impl<T: Config> ValidateUnsigned for Pallet<T> {
	type Call = Call<T>;
	fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		Self::validate_unsigned(source, call)
	}

	fn pre_dispatch(call: &Self::Call) -> Result<(), TransactionValidityError> {
		Self::pre_dispatch(call)
	}
}
```

---

## Custom Origins



---

## Fees

Fees are usually handled by some pallet like the Transaction Payments Pallet.

However, if there is no `Signed` origin, you can't really take a fee.

You should assume any transaction which is not from the `Signed` origin is feeless, unless you explicitly write code to handle it.

---


    filtering
	re-dispatch
    custom origins (system)
    collective origin
    example
