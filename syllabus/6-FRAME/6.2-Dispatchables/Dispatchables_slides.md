## FRAME: Dispatchables

Module 6

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

A measure of how much resources this dispatch is consuming, alongside more **static** information.

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

Notes:

Do you see a weak API design here? That if we alter variants of `DispatchClass` in one place, you
need to update it somewhere else as well. You could solve this by something like `trait DispatchClass` etc etc.

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

- Code Time: Now walk over `with_sensible_defaults`, also in Polkadot.

---v

### Block Limits: Wrapping Up

Code time: look at the expanded pallet from the previous lecture, and see how all of this leads to
implementing `GetDispatchInfo`.


> `GetDispatchInfo` is used in Executive and *Signed Extensions*, but that's a story for another lecture.

<!-- .element: class="fragment" -->

---v

### Block Limits: Wrapping Up

TLDR:

- `Weight` is extracted by the `Executive`, passed down the dispatch stack, and is eventually
  tracked in the system pallet.
- `Length` has a similar path.
- `DispatchClass`: 3 opinionated categories of weight/length used in FRAME.
- `Pays` is used by another (optional) pallet (transaction-payment) to charge for fees. The fee is a
  function of both the weight, and other stuff.

...

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

Where the message was coming from.

Deserves, and will have its own lecture, but for now, get to know a few common helpers from `frame_system`.

- `ensure_signed()`: makes sure that this message was coming from a signed origin.
- `ensure_none()`: makes sure that this message was coming from an unsigned origin.

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
```

}

Call indexing matters!

https://github.com/paritytech/substrate/issues/11896

### A Note on Dispatch Filtering

- Much more in the origin lecture.
