## FRAME: Dispatchables

Lecture X, Module 6

Instructor: Kian

---v

## Background

- `trait Get`
- `sp-arithmetic` and namely `per_thing`
- `scale-codec`
- basics of `FRAME` lecture.

---v

### Dispatchables

> Core of a blockchain is **state transition**, and dispatchables are one of the main common ways to
> do that.

---v

### Dispatchables: Recap on Taxonomy

* You might know them as transactions, but there is a difference..

1. Extrinsic: Transaction / Inherent
2. Transaction: Signed or Unsigned.

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

Weight = u64*

A measure of how much resources this dispatch is consuming, alongside more **static** information.

NOTE:

Elaborate a lot on why weight is a static term: You want to know it, pre-dispatch. If you would
execute something, then you would already know its real weight.

One of the reasons is that during block building, we want to know if the next transaction will
exhaust the block or not, without actually needing to execute it.

Later on, when we peek into `apply`, you should see that the weight

---v

* The weight expression must be something that implement all 3 of these..

```rust
pub type Weight = u64;

pub trait WeighData<T> {
	fn weigh_data(&self, target: T) -> Weight;
}
```

---v

```rust
pub enum DispatchClass {
	Normal,
	Operational,
	Mandatory,
}

pub trait ClassifyDispatch<T> {
	fn classify_dispatch(&self, target: T) -> DispatchClass;
}
```

---v

```rust
#[derive(Clone, Copy, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum Pays {
	Yes,
	No,
}

pub trait PaysFee<T> {
	fn pays_fee(&self, _target: T) -> Pays;
}
```

---v

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

And we have partial implementations for things like `(Weight, Pays)` etc.

- `(u64, DispatchClass, Pays)`
- `(u64, DispatchClass)`
- `(u64, Pays)`

---v

```rust [1,2 | 4,5 | 7,8 | 10,11]
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
  if a % 2 == {
    a * 100
  } else {
    0
  }
)]
fn dispatch(_: OriginFor<T>, a: u32, b: u32) {..}

#[pallet::weight(
  (
      if a % 2 == {
      a * 100
    } else {
      0
    },
    DispatchClass::Operational
  )
)]
fn dispatch(_: OriginFor<T>, a: u32, b: u32) {..}
```

---V

### Block Limits

Now that we know about weight, let's expand it a bit further.

A block is limited by at least two axis:

- Length
- Weight

> Weight, in itself, can be multi-dimensional, but for now assume it is one, and it represents *time*.

---v

### Block Limits: System Pallet.

```rust
// frame-system
pub trait Config {
  #[pallet::constant]
  type BlockWeights: Get<limits::BlockWeights>;

  #[pallet::constant]
  type BlockLength: Get<limits::BlockLength>;
}
```

---v

### Block Limits: Length

```rust
pub struct PerDispatchClass<T> {
	normal: T,
	operational: T,
	mandatory: T,
}

pub struct BlockLength {
	pub max: PerDispatchClass<u32>,
}
```

NOTE:

Do you see a weak API design here? That if we alter variants of `DispatchClass` in one place, you
need to update it somewhere else as well. You could solve this by something like `trait
DispatchClass` etc etc.

---v

### Block Limits: Weight

```rust
pub struct BlockWeights {
	pub base_block: Weight,
	pub max_block: Weight,
	pub per_class: PerDispatchClass<WeightsPerClass>,
}

pub struct WeightsPerClass {
	pub base_extrinsic: Weight,
	pub max_extrinsic: Option<Weight>,
	pub max_total: Option<Weight>,
	pub reserved: Option<Weight>,
}
```

---v

```rust
pub RuntimeBlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
  .base_block(10)
  .max_block(100)
  .for_class(DispatchClass::all(), |weights| {
  	weights.base_extrinsic = 1;
  })
  .for_class(DispatchClass::Normal, |weights| {
  	weights.max_total = Some(80);
    weights.max_extrinsic = 10;
  })
  .for_class(DispatchClass::Operational, |weights| {
    // reserve the other 20
  	weights.reserved = Some(20);
  })
  .for_class(DispatchClass::Mandatory, |weights| {
    // ...
  })
  .avg_block_initialization(10%)
  .build_or_panic();
```

---v

### Block Limits: Weight

* Code Time: Now walk over `with_sensible_defaults`, also in Polkadot.

---v

### Block Limits: Wrapping Up

* Code time: look at the expanded pallet from the previous lecture, and see how all of this leads to
  implementing `GetDispatchInfo`.

* And do you remember who used `get_dispatch_info`

<!-- .element: class="fragment" -->

* In executive..

<!-- .element: class="fragment" -->

Code time again: Re-visit `Executive`'s `apply`.

<!-- .element: class="fragment" -->

> To be continued in *Signed Extensions*.

<!-- .element: class="fragment" -->

---v

### Block Limits: Wrapping Up

TLDR:

- `Weight` is extracted by the `Executive`, passed down the dispatch stack, and is eventually
  tracked in the system pallet.
- `Length` has a similar path.
- `Pays` is used by another (optional) pallet (transaction-payment) to charge for fees. The fee is a
  function of both the weight, and other stuff.


...

---

### Dispatchables: Origin

```rust [11]
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

Where the message was coming from.

Deserves, and will have its own lecture, but for now, get to know a few common helpers from `frame_system`.

- `ensure_signed()`: makes sure that this message was coming from a signed origin.
- `ensure_none()`:  makes sure that this message was coming from an unsigned origin.

NOTE:

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

- as you already saw, one variant of `DispatchError` is to build it from `&'static str`

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
  DispatchErrorWithPostInfo<T>
>;
```

Code time: Look at the `From<_>` implementations of `PostDispatchInfo`.

Question: Why correct the weight if you are `pays_fee = Pays::No`?

---v

### Dispatchables: Return Type

```rust
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

#[pallet::weight((accurate_weight, Pays::Yes)]
fn dispatch(origin: OriginFor<T>) -> DispatchResultWithInfo {
  // stuff
  Ok(Pays::No.into())
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

---v

NOTE:

- `#[pallet::compact]`
- Why derive each type.
  - Debug
  - Eq, PartialEq for making the outer Call dispatchable.
- Question: What are the

---

### Dispatchable: Revisiting An Expanded `Call`

NOTE:

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
```
}

### A Note on Dispatch Filtering

* Much more in the origin lecture.
