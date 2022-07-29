---
title: FRAME Dispatchables
description: FRAME Dispatchables for Web3 engineers.
duration: 1 hour
instructors: ["Shawn Tabrizi, Kian Paimani"]
---

# FRAME: Dispatchables

---v

### Dispatchables

> Core of a blockchain is **state transition**, and dispatchables are one of the main common ways to
> do that.

---v

### Dispatchables: Recap on Taxonomy

> Extrinsic: Signed / Unsigned / Inherent

In this lecture, we care more about the concept of `Call`.

Call is a piece of runtime logic, denoted by some arguments, that can be dispatched. This denoted by
a trait called `Dispatchable`, that we talk about later.

---

### Dispatchables

```rust
#[derive(Decode)]
struct Foo {
  x: Vec<u32>
  y: Option<String>
}

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::call_index(12)]
  #[pallet::weight(128_000_000)]
  fn dispatch(
    origin: OriginFor<T>,
    arg1: u32,
    #[pallet::compact] arg1_compact: u32,
    arg2: Foo,
  ) -> DispatchResult {
    // implementation
  }
}

```

---

### Dispatchables: Weight

```rust [10]
#[derive(Decode)]
struct Foo {
  x: Vec<u32>
  y: Option<String>
}

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::call_index(12)]
  #[pallet::weight(128_000_000)]
  fn dispatch(
    origin: OriginFor<T>,
    arg1: u32,
    #[pallet::compact] arg1_compact: u32,
    arg2: Foo,
  ) -> DispatchResult {
    // implementation
  }
}
```

---v

### Dispatchables: Weight

Weight = u64\*

A measure of how much **resources** this dispatch is consuming, alongside more **static** information.

The **tx-fee** of a typical FRAME-based runtime is also *partially* a function of weight.

Notes:

Elaborate a lot on why weight is a static term: You want to know it, pre-dispatch. If you would
execute something, then you would already know its real weight.

One of the reasons is that during block building, we want to know if the next transaction will
exhaust the block or not, without actually needing to execute it.

Later on, when we peek into `apply`, you should see that the weight

---v

### Dispatchables: Weight

* The weight expression must be something that implement all 3 of these..

```rust
pub type Weight = u64;

// first trait that needs to be implemented.
pub trait WeighData<T> {
	fn weigh_data(&self, target: T) -> Weight;
}
```

---v

### Dispatchables: Weight

```rust
pub enum DispatchClass {
	Normal,
	Operational,
	Mandatory,
}

// second trait that needs to be implemented.
pub trait ClassifyDispatch<T> {
	fn classify_dispatch(&self, target: T) -> DispatchClass;
}
```

---v

### Dispatchables: Weight

```rust
#[derive(Clone, Copy, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum Pays {
	Yes,
	No,
}

// third trait that needs to be implemented.
pub trait PaysFee<T> {
	fn pays_fee(&self, _target: T) -> Pays;
}
```

---v

### Dispatchables: Weight

But we have some auto-implementations:

```rust
impl<T> WeighData<T> for Weight {
	fn weigh_data(&self, _: T) -> Weight {
		*self
	}
}

impl<T> ClassifyDispatch<T> for Weight {
	fn classify_dispatch(&self, _: T) -> DispatchClass {
		DispatchClass::Normal
	}
}

impl<T> PaysFee<T> for Weight {
	fn pays_fee(&self, _: T) -> Pays {
		Pays::Yes
	}
}
```

---v

### Dispatchables: Weight

Therefore:

```rust
#[pallet::weight(128_000_000)]
fn dispatch(..) {..}

// is essentially
#[pallet::weight((128_000_000, DispatchClass::Normal, Pays::Yes))]
fn dispatch(..) {..}
```

---v

### Dispatchables: Weight

And we have partial implementations for things like `(Weight, Pays)` etc.

- `(u64, DispatchClass, Pays)`
- `(u64, DispatchClass)`
- `(u64, Pays)`

---v

```rust [1,2 | 4,5 | 7,8 | 10,11 | 13,14 | 16,17 | 19-26 | 28-38]
#[pallet::weight(128_000_000)]
fn dispatch(..) {..}

#[pallet::weight((128_000_000, DispatchClass::Mandatory))]
fn dispatch(..) {..}

#[pallet::weight((128_000_000, DispatchClass::Mandatory, Pays::No))]
fn dispatch(..) {..}

#[pallet::weight((128_000_000, Pays::Yes))]
fn dispatch(..) {..}

#[pallet::weight(T::some_weight())]
fn dispatch(..) {..}

#[pallet::weight(T::some_weight(a, b))]
fn dispatch(_: OriginFor<T>, a: u32, b: u32) {..}

#[pallet::weight(
  if *a % 2 == {
    *a * 100
  } else {
    0
  }
)]
fn dispatch(_: OriginFor<T>, a: u32, b: u32) {..}

#[pallet::weight(
  (
    if *a % 2 == {
      *a * 100
    } else {
      0
    },
    DispatchClass::Operational
  )
)]
fn dispatch(_: OriginFor<T>, a: u32, b: u32) {..}
```

---v

### Block Limits

Now that we know about weight, let's expand it a bit further.

A block is limited by at least two axis:

- Length
- Weight

<br>

> Weight, in itself, can be multi-dimensional, but for now assume it is one, and it represents *time*.

---v

### Block Limits: Length

TWO ✌️ important points to remember:

1. Encoded length of the transactions needs to be lower than some other limited defined in system pallet.


2. Static/Stack size ([size_of in std::mem - Rust](https://doc.rust-lang.org/std/mem/fn.size_of.html)) of the transactions need to be as small as possible. Let's see why:

<hr>

Once `Decode`, we allocates memory based on this "static size hint".

Our transaction is composed of `enum Call`. What is the static size of an `enum`?

---v

### Block Limits: Length

Let's explore in the playground.

NOTE:

```
struct ComplicatedStuff {
    who: [u8; 32],
    data: [u8; 1024],
}

enum Calls {
    Transfer(u32, [u8; 32], [u8; 32]),
    SetCode(Vec<u8>),
    Complicated(u32, ComplicatedStuff),
}

fn main() {
    dbg!(std::mem::size_of::<Calls>());
    dbg!(std::mem::size_of::<Vec<u8>>());
    dbg!(std::mem::size_of::<Calls>());
}
```

---v

### Block Limits: Length

> `Box` 🎁! Using it reduces the size of the Call enum.

<hr>

> Not to be mistaken, `Box` has nothing to do with how much data you actually decode, it is all
> about how much data is *allocated*.

---v

### Block Limits: Wrapping Up

TLDR:

- `Weight` measure of how much time (and other resources) is consumed, tracked in the system pallet.
- `Length` Similarly.
- `DispatchClass`: 3 opinionated categories of weight/length used in FRAME.
- `Pays` is used by another (optional) pallet (transaction-payment) to charge for fees. The fee is a
  function of both the weight, and other stuff.
- `Box`: useful utility to lessen the size of a `Call` enum.

---

### Dispatchables: Origin

```rust [12]
#[derive(Decode)]
struct Foo {
  x: Vec<u32>
  y: Option<String>
}

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::call_index(12)]
  #[pallet::weight(128_000_000)]
  fn dispatch(
    origin: OriginFor<T>,
    arg1: u32,
    #[pallet::compact] arg1_compact: u32,
    arg2: Foo,
  ) -> DispatchResult {
    // implementation
  }
}
```

---v

### Dispatchables: Origin

<br>

> **Where the message was coming from.**

- [`ensure_signed()`](https://paritytech.github.io/substrate/master/frame_system/fn.ensure_signed.html).
- `ensure_none()`.
- `ensure_root()`.

Notes:

look into the documentation of these two from crates.io.

---v

### Dispatchables: Origin

```rust [10-13]
#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::call_index(12)]
  #[pallet::weight(128_000_000)]
  fn dispatch(
    origin: OriginFor<T>,
    arg1: u32,
    #[pallet::compact] arg1_compact: u32,
    arg2: Foo,
  ) -> DispatchResult {
    let who: T::AccountId = ensure_signed(origin)?;
    // do something with `who`
  }
}
```

---v

### Dispatchables: Origin

```rust [10-13]
#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::call_index(12)]
  #[pallet::weight(128_000_000)]
  fn dispatch(
    origin: OriginFor<T>,
    arg1: u32,
    #[pallet::compact] arg1_compact: u32,
    arg2: Foo,
  ) -> DispatchResult {
    let _ = ensure_none(origin)?;
  }
}
```

---

### Dispatchables: Return Type

```rust [16]
#[derive(Decode, Default, Eq, PartialEq, Debug, Clone)]
struct Foo {
  x: Vec<u32>
  y: Option<String>
}

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::call_index(12)]
  #[pallet::weight(128_000_000)]
  fn dispatch(
    origin: OriginFor<T>,
    arg1: u32,
    #[pallet::compact] arg1_compact: u32,
    arg2: Foo,
  ) -> DispatchResult {
    // implementation
  }
}
```

---v

### Dispatchables: Return Type


```rust
type DispatchResult = Result<(), DispatchError>;
```

https://paritytech.github.io/substrate/master/frame_support/dispatch/enum.DispatchError.html

---v

### Dispatchables: Return Type

```rust [16]
#[derive(Decode, Default, Eq, PartialEq, Debug, Clone)]
struct Foo {
  x: Vec<u32>
  y: Option<String>
}

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::call_index(12)]
  #[pallet::weight(128_000_000)]
  fn dispatch(
    origin: OriginFor<T>,
    arg1: u32,
    #[pallet::compact] arg1_compact: u32,
    arg2: Foo,
  ) -> DispatchResultWithPostInfo {
    // implementation
  }
}
```

---v

### Dispatchables: (The Advanced) Return Type

```rust
pub struct PostDispatchInfo {
  // if set, this is the real consumed weight, else, whatever we set in pre-dispatch.
  pub actual_weight: Option<Weight>,
  // if set, overwrite the previous weight.
  pub pays_fee: Pays,
}

pub type DispatchResultWithPostInfo = Result<
  PostDispatchInfo,
  DispatchErrorWithPostInfo<PostDispatchInfo>
>;
```

Code time: Look at the `From<_>` implementations of `PostDispatchInfo`.

---v

### Dispatchables: Return Type

```rust [1-4 | 6-9 | 11-14 | 16-29 | 31-35 |  37-42 | 44-50]
fn dispatch(origin: OriginFor<T>) -> DispatchResult {
  // stuff
  Ok(())
}

fn dispatch(origin: OriginFor<T>) -> DispatchResult {
  // stuff
  Err("SomeError".into())
}

fn dispatch(origin: OriginFor<T>) -> DispatchResult {
  // stuff
  Err(DispatchError::BadOrigin)
}

#[pallet::weight(typical_weight)]
fn dispatch(origin: OriginFor<T>) -> DispatchResultWithInfo {
  // stuff

  if condition {
    return Err(PostDispatchInfo {
      actual_weight: less_weight,
      ..Default::default()
    })
  }

  // recall `impl From<()> for PostDispatchInfo`
  Ok(().into())
}

#[pallet::weight(more_weight)]
fn dispatch(origin: OriginFor<T>) -> DispatchResultWithInfo {
  // stuff
  Ok(Some(success_full_execution_weight).into())
}

#[pallet::weight((accurate_weight, Pays::Yes))]
fn dispatch(origin: OriginFor<T>) -> DispatchResultWithInfo {

  // useful dispatch, one time only, let's make it free.
  Ok(Pays::No.into())
}

#[pallet::weight((worse_weight, Pays::Yes))]
fn dispatch(origin: OriginFor<T>) -> DispatchResultWithInfo {

  // useful dispatch, one time only, let's make it free.
  Ok((Some(accurate_weight), Pays::No))
  // Question 🤔: why would we want to refund the
}

// You probably NEVER want to do this ❌.
#[pallet::weight(lenient_weight)]
fn dispatch(origin: OriginFor<T>) -> DispatchResultWithInfo {

  // Any error beforehand might have consumed less weight.
  Ok(Some(accurate_weight))
}
```

---v

### Dispatchables: Return Type / Weight

> An inaccurate weight will cause an overweight block. This could potentially cause blocks that
> exceed the desired block-time (forgiving in a solo-chain, not so much in a parachain).

NOTE:

which

---

### Dispatchables: Arguments

```rust [1-5,13-15]
#[derive(Decode, Eq, PartialEq, Debug, Clone)]
struct Foo {
  x: Vec<u32>
  y: Option<String>
}

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::call_index(12)]
  #[pallet::weight(128_000_000)]
  fn dispatch(
    origin: OriginFor<T>,
    arg1: u32,
    #[pallet::compact] arg1_compact: u32,
    arg2: Foo,
  ) -> DispatchResult {
    // implementation
  }
}
```

---v

Notes:

- `#[pallet::compact]`
- Why derive each type.
  - Debug
  - Eq, PartialEq for making the outer Call dispatchable.
- Question: What are the

---

### Dispatchable: Revisiting An Expanded `Call`

Notes:

- Talk about the encoding of `Call`.

---

### Dispatchables: Call Index

```rust [9]
#[derive(Decode, Eq, PartialEq, Debug, Clone)]
struct Foo {
  x: Vec<u32>
  y: Option<String>
}

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::call_index(12)]
  #[pallet::weight(128_000_000)]
  fn dispatch(
    origin: OriginFor<T>,
    arg1: u32,
    #[pallet::compact] arg1_compact: u32,
    arg2: Foo,
  ) -> DispatchResult {
    // implementation
  }
}
```

}

Call indexing matters!

https://github.com/paritytech/substrate/issues/11896

### A Note on Dispatch Filtering

- Much more in the origin lecture.
