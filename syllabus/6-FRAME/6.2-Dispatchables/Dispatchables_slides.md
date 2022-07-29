---
title: FRAME Dispatchables
description: FRAME Dispatchables for Web3 engineers.
duration: 1 hour
instructors: ["Shawn Tabrizi, Kian Paimani"]
---

# FRAME: Dispatchables

---v

### Dispatchables

<br>

> Core of a blockchain is **state transition**, and dispatchables are one of the main common ways to
> do that.

---v

### Dispatchables: Recap on Taxonomy

<br>

> Extrinsic: Signed / Unsigned / Inherent

`Call` is the part of the extrinsic that can be *executed*, i.e. *dispatched*.

```rust
struct Extrinsic {
  call: Call,
  signature_stuff: Option<_>
}
```

---

### Dispatchables: Recap On `Call`

```rust
// somewhere in your pallet, called `my_pallet`
#[pallet::call]
impl<T: Config> Pallet<T> {
  fn transfer(origin: OriginFor<T>, from: T::AccountId, to: T::AccountId, amount: u128);
  fn update_runtime(origin: OriginFor<T>, new_code: Vec<u8>);
}

// expanded in your pallet
enum Call {
  transfer { from: T::AccountId, to: T::AccountId, amount: u128 },
  update_runtime { new_code: Vec<u8> },
}

// in your outer runtime
enum Call {
  System(frame_system::Call),
  MyPallet(my_pallet::Call),
}

// Calls can be dispatched.
Call::MyPallet(my_pallet::Call::transfer { .. }).dispatch();
// Which merely forwards the call to:
my_pallet::Pallet::transfer( .. )
```

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

Notes:

- `#[pallet::compact]`
- Why derive each type.
  - Debug
  - Eq, PartialEq for making the outer Call dispatchable.

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

---v

### Dispatchables: Call Index

```rust
// somewhere in your pallet, called `my_pallet`
#[pallet::call]
impl<T: Config> Pallet<T> {
  fn transfer(from: T::AccountId, to: T::AccountId, amount: u128);
  fn update_runtime(new_code: Vec<u8>);
}

// expanded in your pallet
enum Call {
  transfer { from: T::AccountId, to: T::AccountId, amount: u128 },
  update_runtime { new_code: Vec<u8> },
}

// in your outer runtime
enum Call {
  System(frame_system::Call),
  MyPallet(my_pallet::Call),
}

// Think about what this is:
Call::MyPallet(my_pallet::Call::transfer { .. }).encode()
```

NOTE:

example of how it can be a PITA: https://github.com/paritytech/substrate/issues/11896

---v

### Dispatchables: Call Index

By default, **order of functions**, and pallets in `construct_runtime` MATTER.

Nowadays, you can overwrite both if needed.

```rust
#[pallet::call_index(5)]
fn dispatchable() {}

frame_support::construct_runtime!(
	pub enum Runtime where
	{
		System: frame_system = 1,
		Example: pallet_template = 0,
	}
);
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

<br>

> Weight, in itself, can be multi-dimensional, but for now assume it is one, and it represents *time*.

Notes:

Elaborate a lot on why weight is a static term: You want to know it, pre-dispatch. If you would
execute something, then you would already know its real weight.

One of the reasons is that during block building, we want to know if the next transaction will
exhaust the block or not, without actually needing to execute it.

Later on, when we peek into `apply`, you should see that the weight



---v

### Dispatchables: Weight Examples

`#[weight]` attribute  is technically a shorthand for:

```rust
type Weight = u64;

pub enum Pays {
  // Default, if you only provide a weight.
  Yes,
  No,
}

pub enum DispatchClass {
  // User operation, normal stuff.
  Normal,
  // User operations that are useful for the chain: runtime upgrade etc.
  Operational,
  // Operations that MUST happen e.g. some hooks controller by pallets.
  Mandatory,
}
```

- Default `DispatchClass::Normal`, `Pays::Yes`.

---v

### Dispatchables: Weight Examples

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

---

### Dispatchables: Weight: Under The Hood

* The weight expression must be something that implement all 3 of these..

```rust
pub type Weight = u64;

// first trait that needs to be implemented.
pub trait WeighData<T> {
	fn weigh_data(&self, target: T) -> Weight;
}
```

---v

### Dispatchables: Weight: Under The Hood

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

### Dispatchables: Weight: Under The Hood

```rust
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

### Dispatchables: Weight: Under The Hood

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

### Dispatchables: Weight: Under The Hood

And we have partial implementations for things like `(Weight, Pays)` etc.

- `(u64, DispatchClass, Pays)`
- `(u64, DispatchClass)`
- `(u64, Pays)`

---v

### Dispatchables: Weight: Under The Hood

Therefore:

```rust
#[pallet::weight(128_000_000)]
fn dispatch(..) {..}

// rust blanket implementation expands this to:
#[pallet::weight((128_000_000, DispatchClass::Normal, Pays::Yes))]
fn dispatch(..) {..}
```

---

## Block Limits: Length

Now that we know about weight, let's expand it a bit further.

A block is limited by at least two axis:

- Length
- Weight

---v

### Block Limits: Length

TWO ✌️ important points to remember:

1. Encoded length of the transactions needs to be lower than some other limited defined in system
   pallet (default: `5MB`).

---v

### Block Limits: Length

2. Static/Stack size ([size_of in std::mem -
   Rust](https://doc.rust-lang.org/std/mem/fn.size_of.html)) of the transactions need to be as small
   as possible.

<br>

Our transaction is composed of `enum Call`. What is the stack size of an `enum`?

---v

### Block Limits: Length


```rust
struct ComplicatedStuff {
    who: [u8; 32],
    data: [u8; 1024],
}

enum Calls {
    Transfer(u32, [u8; 32], [u8; 32]),
    SetCode(Vec<u8>),
    Complicated(u32, ComplicatedStuff),
}

std::mem::size_of::<ComplicatedStuff>() // 1056
std::mem::size_of::<Vec<u8>>(); // 24
std::mem::size_of::<Calls>() // 1056;
```

---v

### Block Limits: Length


```rust
struct ComplicatedStuff {
    who: [u8; 32],
    data: [u8; 1024],
}

enum Calls {
    Transfer(u32, [u8; 32], [u8; 32]),
    SetCode(Vec<u8>),
    Complicated(u32, Box<ComplicatedStuff>),
}

std::mem::size_of::<ComplicatedStuff>() // 1056
std::mem::size_of::<Vec<u8>>(); // 24
std::mem::size_of::<Calls>() // 72;
```

---v

### Block Limits: Length

<br>

> `Box` 🎁! Using it reduces the size of the Call enum.

<hr>

> Not to be mistaken, `Box` has nothing to do with how much data you actually **decode/encode**, it
> is all about how much data is ***allocated*** in the stack.

- Further reading: [Using Box&lt;T&gt; to Point to Data on the Heap - The Rust Programming Language](https://doc.rust-lang.org/book/ch15-01-box.html)

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

([src](https://paritytech.github.io/substrate/master/frame_support/dispatch/enum.DispatchError.html))

```rust
type DispatchResult = Result<(), DispatchError>;
```


---v

### Dispatchables: Return Type

```rust [1-4 | 6-9 | 11-14]
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
```

---v

### Dispatchables: (The Advanced) Return Type 💪🏻

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


([src](https://paritytech.github.io/substrate/master/frame_support/weights/struct.PostDispatchInfo.html))

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
### Dispatchables: (The Advanced) Return Type


```rust [1-14 | 16-20 | 22-27 | 29-35 | 37-43]
#[pallet::weight(worse_weight)]
fn dispatch(origin: OriginFor<T>) -> DispatchResultWithInfo {
  // stuff

  if condition {
    // early exist
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
  // Question 🤔: why would we want to refund the weight if it is free?
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

<br>

> An inaccurate weight will cause an **overweight block** 😱. This could potentially cause blocks that
> exceed the desired block-time (forgiving in a solo-chain, not so much in a parachain).
