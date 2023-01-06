---
title: FRAME Tips and Tricks
description: Substrate and FRAME Tips and Tricks for Web3 Engineers
instructors: ["Kian Paimani"]
teaching-assistants: ["Sacha Lansky"]
---

# FRAME Tips and Tricks

Notes:

A random collection of things that you should probably know about.
These are relevant for coding in FRAME and Substrate.

---

## Recap: `trait Block`, `Header`, `Extrinsic`

Generic definitions of what each of these _should be_..

.. One implementation of which can be found in `generic` folder.

---v

### `trait Block`, `Header`, `Extrinsic`

..and you should be well versed in reading such type aliases:

```rust
/// Extract the hashing type for a block.
pub type HashFor<B> = <<B as Block>::Header as Header>::Hashing;
/// Extract the number type for a block.
pub type NumberFor<B> = <<B as Block>::Header as Header>::Number;
```

or..

```rust
type BalanceOf<T, I> = <
  <T as Config<I>>::Currency
  as
  Currency<<T as frame_system::Config>::AccountId>
>::Balance;
```

---v

## Detour: Traits, Generics, Associated Types

Notes:

```
trait Engine {
    fn start() {}
}

struct BMW;
impl Engine for BMW {}

trait Brand {
    fn name() -> &'static str;
}

trait Car<E: Engine> {
    type Brand: Brand;
}

struct MyCar;
struct MyBrand;
impl Brand for MyBrand {
    fn name() -> &'static str {
        "Kian"
    }
}

impl<E: Engine> Car<E> for MyCar {
    type Brand = MyBrand;
}

fn main() {
    // Car<E1>, Car<E2> are different traits!

    // Generics can be bounded, or constrained
    // impl<E: Engine> Car<E> {}
    // impl Car<BMW> {}

    // Associated types can:
    // only be bounded when being defined,
    // Can be constrained when being implemented, or when the trait is being used.
    fn some_fn<E: Engine, C: Car<E, Brand = MyBrand>>(car: C) {
      // and we are told associated types are more like output types, lets get the brand of car
      let name = <<C as Car<E>>::Brand as Brand>::name();
    }
    fn other_fn<C: Car<BMW, Brand = MyBrand>>(car: C) {

    }

    // now, check this out
}
```

---

## `trait Get`

A very basic, yet very substrate-idiomatic way to pass _values_ through _types_.

```rust
pub trait Get<T> {
  fn get() -> T;
}

// very basic blanket implementation, which you should be very versed in reading.
impl<T: Default> Get<T> for () {
  fn get() -> T {
    T::default()
  }
}
```

---v

### `trait Get`

```rust
parameter_types! {
  pub const Foo: u32 = 10;
}

// expands to:
pub struct Foo;
impl Get<u32> for Foo {
  fn get() -> u32 {
    10;
  }
}
```

> Helps convey **values** using **types**.

---

## `bounded`

- `BoundedVec`, `BoundedSlice`, `BoundedBTreeMap`, `BoundedSlice`

```rust
#[cfg_attr(feature = "std", derive(Serialize), serde(transparent))]
#[derive(Encode)]
pub struct BoundedVec<T, S: Get<u32>>(
  pub(super) Vec<T>,
  PhantomData<S>,
);
```

> `PhantomData`?

---v

### `bounded`

Food for your thought.

```rust
#[cfg_attr(feature = "std", derive(Serialize))]
#[derive(Encode)]
pub struct BoundedVec<T>(
  pub(super) Vec<T>,
  u32,
);
```

---

## `trait Convert`

```rust
pub trait Convert<A, B> {
	fn convert(a: A) -> B;
}

pub struct Identity;
// blanket implementation!
impl<T> Convert<T, T> for Identity {
	fn convert(a: T) -> T {
		a
	}
}
```

Notes:
this one's much simpler, but good excuse to teach them blanket implementations.

---v

### Example of `Get` and `Convert`

```rust
/// Some configuration for my module.
trait InputConfig {
  /// Something that gives you a `usize`.
  type MaximumSize: Get<usize>;
  /// Something that is capable of converting `u64` to `u32`.
  type ImpossibleConvertor: Convertor<u64, u32>;
}

struct Pallet<T: Config> {
  fn foo() {
    let outcome: u32 = T::ImpossibleConvertor::convert(u64::max_value());
  }
}

struct Runtime;
impl InputConfig for Runtime {
  type MaximumSize = (); // remember what this means?
  type ImpossibleConvertor = ...
}
```

---

## Implementing Traits For Tuples

```rust
struct Module1;
struct Module2;
struct Module3;

trait OnInitialize {
  fn on_initialize();
}

impl OnInitialize for Module1 { fn on_initialize() {} }
impl OnInitialize for Module2 { fn on_initialize() {} }
impl OnInitialize for Module3 { fn on_initialize() {} }
```

How can I easily invoke `OnInitialize` on all 3 of `Module1, Module2, Module3`? Explore in Rust
Playground!

Notes:

take this to rust playground.

add:

trait OnInitializeDyn {
fn on_initialize(&self);
}

impl OnInitializeDyn for Module1 { fn on_initialize(&self) {} }
impl OnInitializeDyn for Module2 { fn on_initialize(&self) {} }
impl OnInitializeDyn for Module3 { fn on_initialize(&self) {} }

fn main() {
// let x = vec![Module1, Module1, Module1];
// let x: Vec<Box<dyn OnInitialize>> = vec![Box::new(Module1), Box::new(Module2)];
let x: Vec<Box<dyn OnInitializeDyn>> = vec![Box::new(Module1), Box::new(Module2)];
x.for_each(|i| i.on_initialize())
x.for_each(OnInitialize::on_initialize)
}

---v

### Implementing Traits For Tuples

> Dynamic dispatch could help us achieve what we want, but it adds runtime overhead.

1. `on_initialize`, in its ideal form, does not have `&self`, it is defined on the **type**, not a **value**.
2. **Tuples** are the natural way to group **types** together (analogous to have a **vector** is the
   natural way to group **values** together..)

```rust
// fully-qualified syntax - turbo-fish.
<(Module1, Module2, Module3) as OnInitialize>::on_initialize();
```

---v

### Implementing Traits For Tuples

```rust
struct Module1;
struct Module2;
struct Module3;

trait OnInitialize {
  fn on_initialize();
}

impl OnInitialize for Module1 { fn on_initialize() {} }
impl OnInitialize for Module2 { fn on_initialize() {} }
impl OnInitialize for Module3 { fn on_initialize() {} }

impl<T1: OnInitialize, T2: OnInitialize> OnInitialize for (T1, T2) {
  fn on_initialize() {
    T1::on_initialize();
    T2::on_initialize();
  }
}

impl<T1: OnInitialize, T2: OnInitialize, T3: OnInitialize> OnInitialize for (T1, T2, T3) {
  fn on_initialize() {
    T1::on_initialize();
    T2::on_initialize();
    T3::on_initialize();
  }
}
```

---v

### Implementing Traits For Tuples

Only problem: A lot of boilerplate. Macros!

Historically, we made this work with `macro_rules!`

e.g. https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4822dfa5bc2acc528a0a1487789eb064

Notes:

```rust
macro_rules! impl_for_tuples {
    ( $( $elem:ident ),+ ) => {
        impl<$( $elem: OnInitialize, )*> OnInitialize for ($( $elem, )*) {
            fn on_initialize() {
                $( $elem::on_initialize(); )*
            }
        }
    }
}

impl_for_tuples!(A, B, C, D);
impl_for_tuples!(A, B, C, D, E);
impl_for_tuples!(A, B, C, D, E, F);
```

---v

### Implementing Traits For Tuples

But then Basti saved us:

```rust
// basic
#[impl_for_tuples(30)]
pub trait OnTimestampSet<Moment> {
	fn on_timestamp_set(moment: Moment);
}

// slightly more advance
#[impl_for_tuples(30)]
impl OnRuntimeUpgrade for Tuple {
  fn on_runtime_upgrade() -> crate::weights::Weight {
    let mut weight = 0;
    for_tuples!( #( weight = weight.saturating_add(Tuple::on_runtime_upgrade()); )* );
    weight
  }
}
```

---v

### Implementing Traits for Tuples: Further Reading

- https://stackoverflow.com/questions/64332037/how-can-i-store-a-type-in-an-array
- https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch
- https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
- https://turbo.fish/
- https://techblog.tonsser.com/posts/what-is-rusts-turbofish
- https://docs.rs/impl-trait-for-tuples/latest/impl_trait_for_tuples/

---

## The `std` Paradigm

https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html

> #![no_std] is a crate level attribute that indicates that the crate will link to the `core` crate
> instead of the `std` crate.. std crate is Rust's standard library. It contains functionality
> that **assumes that the program will run on an operating system rather than directly on the
> metal**... std provides a standard API over functionality one usually finds in such operating
> systems: Threads, files, sockets, a filesystem, processes, etc... the `core` crate is a subset of the `std` crate that makes zero assumptions about the system the program will run on.. However it lacks APIs for anything that involves heap memory allocations and I/O.

---v

### The `std` Paradigm

All crates in substrate that eventually compile to WASM are compiled in a dual mode:

1. with `std`
1. otherwise `no_std`

```rust
#![cfg_attr(not(feature = "std"), no_std)]
```

> The name "`std`" is just an idiom in the rust ecosystem.
> https://rust-lang.github.io/api-guidelines/naming.html#feature-names-are-free-of-placeholder-words-c-feature

---v

### The `std` Paradigm: Adding dependencies

```rust
[package]
name = "simple-runtime"
version = "0.1.0"
edition = "2021"

[build-dependencies]
substrate-wasm-builder = { git = "https://github.com/paritytech/substrate", branch = "master" }

[dependencies]
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false }
frame-support = { git = "https://github.com/paritytech/substrate", branch = "master", default-features = false }

simple-pallet = { path = "./simple-pallet", default-features = false }

tokio = { git = "...", optional = true }

[dev-dependencies]
sp-io = { git = "https://github.com/paritytech/substrate", branch = "master" }

[features]
default = ["std"]
std = [
  "codec/std",
  "frame-support/std",

  "simple-pallet/std",
]
async-shenanigans = ["tokio"]
```

---v

### The `std` Paradigm: Adding dependencies

```
error: duplicate lang item in crate sp_io (which frame_support depends on): panic_impl.
  |
  = Notes:
 the lang item is first defined in crate std (which serde depends on)

error: duplicate lang item in crate sp_io (which frame_support depends on): oom.
  |
  = Notes:
 the lang item is first defined in crate std (which serde depends on)
```

---v

### The `std` Paradigm: Adding dependencies

- Tips:
  - `SKIP_WASM_BUILD=1`
  - `.maintain/build-only-wasm.sh`

---v

### The `std` Paradigm

A subset of the standard types in rust that also exist in rust `core` are re-exported from `sp_std`.

- https://doc.rust-lang.org/core/index.html
- https://doc.rust-lang.org/std/index.html

```rust
sp_std::prelude::*;
```

Notes:

Hashmap not exported due to non-deterministic concerns.

---v

### The `std` Paradigm

```rust [1-13|13-100]
#[cfg(feature = "std")]
#[macro_export]
macro_rules! if_std {
	( $( $code:tt )* ) => {
    $( $code )*
	}
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! if_std {
  ( $( $code:tt )* ) => {};
}

// somewhere in your runtime code
fn foo() {
  sp_std::if_std! {
    // testing, debugging..
    println!("Debug test only printed in wasm and native");
    use std::*;

    // this bricks the chain..
    sp_io::storage::set(b"foo", b"bar");
  }
}
```

---v

### The `std` Paradigm: Further Reading:

- https://paritytech.github.io/substrate/master/sp_std/index.html

---

## Logging And Prints In The Runtime.

First, why the fuss?

Size of the wasm blob matters..

<!-- .element: class="fragment" -->

Any logging increases the size of the WASM blob. **String literals** are stored somewhere in your
program!

<!-- .element: class="fragment" -->

---v

### Logging And Prints In The Runtime.

> wasm2wat polkadot_runtime.wasm > dump | rg stripped

Should get you the `.rodata` (read-only data) line of the wasm blob, which contains all the logging
noise.

> This contains string literals form errors, logs, metadata, etc.

---v

### Logging And Prints In The Runtime.

- `Debug` vs. `RuntimeDebug`.

```rust
#[derive(RuntimeDebug)]
pub struct WithDebug {
    foo: u32,
    bar: u32,
}

impl ::core::fmt::Debug for WithDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        #[cfg(feature = "std)]
        {
          fmt.debug_struct("WithRuntimeDebug")
            .field("foo", &self.foo)
            .field("bar", &self.bar)
            .finish()
        }
        #[cfg(not(feature = "std))]
        {
          fmt.write("<wasm:stripped>")
        }
    }
}
```

---v

### Logging And Prints In The Runtime.

Once types implement `Debug` or `RuntimeDebug`, they can be printed. Various ways:

1. If you only want something in tests, native builds etc

```rust
sp_std::if_std! {
  println!("hello world!")
}
```

1. Or you can use the common frame-support logging (which is just the `log` crate re-exported):

```rust
frame_support::log::info!(target: "target", "hello world!");
frame_support::log::debug!(target: "target", "hello world! ({})", 10u32);
```

---v

### Logging And Prints In The Runtime.

1. Log statements are only evaluated if the corresponding level and target is met.

```
/// only executed if `RUST_LOG=target=trace`
frame_support::log::trace!(target: "target", "({:?})", (0..100000).into_iter().collect());
```

2. `disable-logging` compilation flag blocks all sp-io calls to do any logging. This is used in
   official polkadot releases.

Notes:

`log` in rust does not do anything -- it only tracks what needs to be logged. Then you need a logger
to actually export them. In rust this is often `env_logger` or `sp_tracing` in substrate tests.

In the runtime, the log messages are sent via the host functions to the client to be printed.

If the interface is built with `disable-logging`, it omits all log messages.

---

## Arithmetic Helpers, and the `f32`, `f64` Story.

Floating point numbers have different standards, and (**_slightly_**) different implementations on
different architectures and vendors.

> If my balance is `10.000000000000001` DOT on one validator and `10.000000000000000` DOT on another
> validator, game over for your consensus.

---v

### PerThing.

```python
> .2 + .2 + .2 == .6
> false
```

```
> a = 10
> b = 0.1
> c = 0.2
> a*(b+c) == a*b + a*c
> false
```

Google "weird float behavior" fro more entertainment around this.

---v

### PerThing.

- We store ratios and such in the runtime with "Fixed-Point" arithmetic types.

```rust
implement_per_thing!(
  Percent,
  100u8,
  u8,
  "_Percent_"
);
implement_per_thing!(
	Perbill,
	1_000_000_000u32,
	u32,
	"_Parts per Billion_",
);
implement_per_thing!(
	Perquintill,
	1_000_000_000_000_000_000u64,
	u64,
	"_Parts per Quintillion_",
);
```

---v

### PerThing.

```
let p = Perbill::from_part_parts(1_000_000_000u32 / 4);
let p = Perbill::from_percent(25);
let p = Perbill::from_rational(1, 4);

> p * 100u32;
> 25u32;
```

Some precision concerns exist, but that's a story for another day.

---v

### Fixed Point Numbers

`Per-thing` is great for representing `[0, 1]` range.

What if we need more?

```
100 ~ 1
200 ~ 2
300 ~ 3
350 ~ 3.5
```

---v

### Fixed Point Numbers

```rust
implement_fixed!(
	FixedU64,
	u64,
	1_000_000_000,
	"_Fixed Point 64 bits unsigned, range = [0.000000000, 18446744073.709551615]_",
);

implement_fixed!(
	FixedU128,
	u128,
	1_000_000_000_000_000_000,
	"_Fixed Point 128 bits unsigned, range = \
		[0.000000000000000000, 340282366920938463463.374607431768211455]_",
);
```

---v

### Larger Types

- `U256`, `U512`: battle-tested since the ethereum days.
- substrate-fixed: community project. Supercharged `PerThing` and `Fixed`.
- `big_uint.rs` (unaudited)

```rust

pub struct BigUint {
	/// digits (limbs) of this number (sorted as msb -> lsb).
	pub(crate) digits: Vec<Single>,
}
```

---v

### Further Reading:

- https://paritytech.github.io/substrate/master/sp_arithmetic/index.html
- https://paritytech.github.io/substrate/master/sp_core/uint/index.html

---v

### Fallibility

**Conversions** are very much fallible operations. So are things like **addition**,
**multiplication**, **division** (all in `std::ops`, if keen on reading some rustdocs).

- Panic

`u32::MAX * u32::MAX / 2` (in debug builds)

`100 / 0`

- Overflow

`u32::MAX * u32::MAX / 2` (in release builds)

---v

### Fallibility

1. `Checked` -- recover

`if let Some(outcome) = a.checked_mul(b) { ... } else { ... }`

2. `Saturating` -- soft recovery

`let certain_output = a.saturating_mul(b);`

<hr>

There's also `wrapping_op` and `carrying_op` etc on all rust primitives, but not quite relevant.

https://doc.rust-lang.org/std/primitive.u32.html

---v

### Fallibility

Luckily, rust is already pretty strict for the primitive types.

- `TryInto` / `TryFrom` / `From<u32>` / `Into`

```rust
/// T is u32 or larger.
struct Foo<T: From<u32>>

/// T is u32 or smaller
struct Foo<T: Into<u32>>

/// It can maybe be converted to u32
struct Foo<T: TryInto<u32>>

/// It can be converted into u32 at the cost of loss of accuracy.
/// This is a substrate trait.
struct Foo<T: UniqueSaturatedInto<u32>>

assert_eq!(u128::MAX.saturating_into::<u32>(), u32::MAX);
```

---

## Defensive Programming

> Defensive programming is a form of defensive design intended to ensure the continuing function of
> a piece of software under unforeseen circumstances. Defensive programming practices are often
> used where **high availability**, **safety**, or **security** is needed.

- As you know, you should (almost) never panic in your runtime code.
- `.unwrap()`? no no
- be careful with implicit unwraps in standard operations!
  - slice/vector indexing can panic if out of bound
  - `.insert`, `remove`
  - division by zero.

---v

### Defensive Programming

When using operations that could panic, comment exactly above it why you are sure it won't panic.

```
let pos = announcements
  .binary_search(&announcement)
  .ok()
  .ok_or(Error::<T, I>::MissingAnnouncement)?;
// index coming from `binary_search`, therefore cannot be out of bound.
announcements.remove(pos);
```

---v

### Defensive Programming

Or when using options or results that need to be unwrapped but are known to be `Ok(_)`, `Some()`:

```
let maybe_value: Option<_> = ...
if maybe_value.is_none() {
  return "..."
}

let value = maybe_value.expect("value checked to be 'Some'; qed");
```

> Q.E.D. or QED is an initialism of the Latin phrase quod erat demonstrandum, meaning "which was to be demonstrated".

---v

### Defensive Programming

When writing APIs that could panic, explicitly document them, just like the core rust documentation.

```rust
/// Exactly the same semantics as [`Vec::insert`], but returns an `Err` (and is a noop) if the
/// new length of the vector exceeds `S`.
///
/// # Panics
///
/// Panics if `index > len`.
pub fn try_insert(&mut self, index: usize, element: T) -> Result<(), ()> {
  if self.len() < Self::bound() {
    self.0.insert(index, element);
    Ok(())
  } else {
    Err(())
  }
}
```

---v

### Defensive Programming

The overall ethos of defensive programming is along the lines of:

```
// we have good reasons to believe this is `Some`.
let y: Option<_> = ...

// I am really really sure about this
let x = y.expect("hard evidence; qed");

// either return a reasonable default..
let x = y.unwrap_or(reasonable_default);

// or return an error (in particular in dispatchables)
let x = y.ok_or(Error::DefensiveError)?;
```

---v

### Defensive Programming

Slightly better:

[Defensive traits](https://paritytech.github.io/substrate/master/frame_support/traits/trait.Defensive.html):

```
// either return a reasonable default..
let x = y.defensive_unwrap_or(reasonable_default);

// or return an error (in particular in dispatchables)
let x = y.defensive_ok_or(Error::DefensiveError)?;
```

It adds some boilerplate to:

1. Panic when `debug_assertions` are enabled (tests).
2. append a `log::error!`.

---

## Bonus: More Pages in History Page of Substrate:

- Rust didn't have u128 until not too long ago! https://github.com/paritytech/substrate/pull/163/files
- `TryFrom`/`TryInto` are also not too old! https://github.com/paritytech/substrate/pull/163/files#r188938077
- Remove `As`, which tried to fill the lack of `TryFrom/TryInto` https://github.com/paritytech/substrate/pull/2602
- Runtime Logging PR: https://github.com/paritytech/substrate/pull/3821
