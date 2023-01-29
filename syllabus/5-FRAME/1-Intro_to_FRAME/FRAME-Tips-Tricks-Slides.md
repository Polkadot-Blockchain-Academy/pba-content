---
title: FRAME Tips and Tricks
description: Substrate and FRAME Tips and Tricks for Web3 Engineers
---

# FRAME Tips and Tricks

Notes:

- A random collection of things that you should probably know about.
- These are relevant for coding in FRAME and Substrate.

---

# Part 1 Substrate Stuff

---

## Recap: Blocks, Headers, Extrinsics

- The traits defining what each are in `sp-runtime/traits`
- One, somewhat opinionated set of types that implement these can be found in `sp-runtime/generic`.

---v

### `trait Block`, `Header`, `Extrinsic`

- you should be well versed in reading such type aliases:

```rust
use sp_runtime::traits::{Header as HeaderT};
/// Extract the number type for a block.
pub type NumberFor<B> = <<B as Block>::Header as HeaderT>::Number;
```

```rust
trait Block {
  type Header: sp_runtime::traits::Header;
}

trait Header {
  type Number
}
```

<!-- .element: class="fragment" -->

---v

### `trait Block`, `Header`, `Extrinsic`

- Or a common example from FRAME:

```rust
type BalanceOf<T> = <
  <T as Config>::Currency
  as
  Currency<<T as frame_system::Config>::AccountId>
>::Balance;
```

---v

## Speaking of Traits..

- What is the difference between generics and associated types?

<div class="fragment">

- Anything that can be expressed with associated types can also be expressed with generics.
- Associated Types << Generics
- Associated types usually lead to less boilerplate.

</div>

---v

## Speaking of Traits..

```rust
trait Engine {}
trait Brand {}

trait Car<E: Engine> {
  // brand is possibly the same among all, so make it associate!
  type Brand: Brand;
}

struct Car;
// Car<E1> and Car<E2> are not the same type!
// Car<E1> and Car<E2> could not have different brands.
// fn foo<E: Engine, C: Car<E, Brand = SomeBrand>>() { .. }
```

Notes:

In cambridge, I did this this. But since students should now know traits really well, I will drop it.

```rust
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

struct KianCarCo;
impl Brand for KianCarCo {
  fn name() -> &'static str {
    "KianCarCo!"
    }
}

struct MyCar;
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

## The `std` Paradigm

- Recap:
  - `std` is the interface to the common OS-abstractions.
  - `core` is a subset of `std` that makes no assumption about the operating system.

> #![no_std] is a crate level attribute that indicates that the crate will link to the `core` crate
> instead of the `std` crate

- [source](https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html)

---v

### The `std` Paradigm

- All crates in substrate that eventually compile to Wasm are compiled in a dual mode:

1. with `std`
1. otherwise `no_std`

```rust
#![cfg_attr(not(feature = "std"), no_std)]
```

- The name "`std`" is just an idiom in the rust ecosystem.
- `no_std` does NOT mean Wasm!
- `std` does not mean native!

Notes:

But in substrate, it kinda means like that:

std => native
no_std => wasm

---v

### The `std` Paradigm: Adding dependencies

```rust
[package]
name = "simple-runtime"
version = "0.1.0"
edition = "2021"

[dependencies]
codec = {
  package = "parity-scale-codec",
  version = "3.0.0",
  default-features = false
}
frame-support = {
  git = "https://github.com/paritytech/substrate",
  branch = "master",
  default-features = false
}

tokio = { git = "...", optional = true }

[dev-dependencies]
sp-io = { git = "https://github.com/paritytech/substrate", branch = "master" }

[features]
default = ["std"]
std = [
  "codec/std",
  "frame-support/std",
]
async-shenanigans = ["tokio"]
```

---v

### The `std` Paradigm: Adding dependencies

```sh
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

### The `std` Paradigm

A subset of the standard types in rust that also exist in rust `core` are re-exported from
[`sp_std`](https://paritytech.github.io/substrate/master/sp_std/index.html).

```rust
sp_std::prelude::*;
```

Notes:

Hashmap not exported due to non-deterministic concerns.
floats are usable, but also non-deterministic! (and I think they lack `encode`, `decode` impl)

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

---

## Logging And Prints In The Runtime.

- First, why bother? let's just add as many logs as we want into the runtime.

<!-- .element: class="fragment" -->

- Size of the wasm blob matters..

<!-- .element: class="fragment" -->

- Any logging increases the size of the Wasm blob. **String literals** are stored somewhere in your
  program!

<!-- .element: class="fragment" -->

---v

### Logging And Prints In The Runtime.

- `wasm2wat polkadot_runtime.wasm > dump | rg stripped`

- Should get you the `.rodata` (read-only data) line of the wasm blob, which contains all the logging
  noise.

- This contains string literals form errors, logs, metadata, etc.

---v

### Logging And Prints In The Runtime.

```rust
#[derive(RuntimeDebug)]
pub struct WithDebug {
    foo: u32,
}

impl ::core::fmt::Debug for WithDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        #[cfg(feature = "std)]
        {
          fmt.debug_struct("WithRuntimeDebug")
            .field("foo", &self.foo)
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

- If you only want something in tests, native builds etc

```rust
sp_std::if_std! {
  println!("hello world!");
  dbg!(foo);
}
```

- Or you can use the common frame-support logging (which is just the `log` crate re-exported):

```rust
frame_support::log::info!(target: "target", "hello world!");
frame_support::log::debug!(target: "target", "hello world! ({})", 10u32);
```

---v

### Logging And Prints In The Runtime.

- Log statements are only evaluated if the corresponding level and target is met.

```rust
/// only executed if `RUST_LOG=KIAN=trace`
frame_support::log::trace!(target: "KIAN", "({:?})", (0..100000).into_iter().collect());
```

- `disable-logging` compilation flag blocks all sp-io calls to do any logging. This is used in
  official polkadot releases.

Notes:

`log` in rust does not do anything -- it only tracks what needs to be logged. Then you need a logger
to actually export them. In rust this is often `env_logger` or `sp_tracing` in substrate tests.

In the runtime, the log messages are sent via the host functions to the client to be printed.

If the interface is built with `disable-logging`, it omits all log messages.

---

## Arithmetic Helpers, and the `f32`, `f64` Story.

- Floating point numbers have different standards, and (**_slightly_**) different implementations on
  different architectures and vendors.

- If my balance is `10.000000000000001` DOT on one validator and `10.000000000000000` DOT on another validator, game over for your consensus 😮‍💨.

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

- Google "weird float behavior" fro more entertainment around this.

---v

### PerThing.

- We store ratios and such in the runtime with "Fixed-Point" arithmetic types.

```rust
struct Percent(u8);

impl Percent {
  fn new(x: u8) {
    Self(x.min(100));
  }
}

impl Mul<u32> for Percent {
  ...
}

```

---v

### PerThing.

```rust
use sp_arithmetic::Perbill;

let p = Perbill::from_part_parts(1_000_000_000u32 / 4);
let p = Perbill::from_percent(25);
let p = Perbill::from_rational(1, 4);

> p * 100u32;
> 25u32;
```

- Some precision concerns exist, but that's a story for another day.

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
use sp_arithmetic::FixedU64;

let x = FixedU64::from_rational(5, 2);
let y = 10u32;
let z = x * y;
> 25
```

---v

### Larger Types

- [`U256`](https://paritytech.github.io/substrate/master/sp_core/struct.U256.html), `U512`: battle-tested since the ethereum days.
- [substrate-fixed](https://github.com/encointer/substrate-fixed): community project. Supercharged `PerThing` and `Fixed`.
- [`big_uint.rs`](https://paritytech.github.io/substrate/master/sp_arithmetic/biguint/index.html) (unaudited)

```rust

pub struct BigUint {
	/// digits (limbs) of this number (sorted as msb -> lsb).
	pub(crate) digits: Vec<Single>,
}
```

---v

### Arithmetic Types

- Everything said here can be found in
  [`sp-arithmetic`](https://paritytech.github.io/substrate/master/sp_arithmetic/index.html) and
  `sp-core`, and a lot of it is re-exported from `sp-runtime`
- Because they are used a LOT.

---

### Fallibility: Math Operations

Things like **addition**, **multiplication**, **division** could all easily fail.

- Panic

  - `u32::MAX * 2 / 2` (in debug builds)
  - `100 / 0`

- Overflow
  - `u32::MAX * 2 / 2` (in release builds)

---v

### Fallibility

- `Checked` -- prevention ✋🏻

  ```
  if let Some(outcome) = a.checked_mul(b) { ... } else { ... }
  ```

- `Saturating` -- silent recovery 🤫

  ```
  let certain_output = a.saturating_mul(b);
  ```

Notes:

Why would you ever want to saturate? only in cases where you know if the number is overflowing,
other aspects of the system is so fundamentally screwed that there is no point in doing any kind of
recovery.

There's also `wrapping_op` and `carrying_op` etc on all rust primitives, but not quite
relevant.

---v

### Fallibility: Conversion

- Luckily, rust is already pretty strict for the primitive types.
- `TryInto` / `TryFrom` / `From` / `Into`

- `struct Foo<T: From<u32>>`

T is u32 or larger.

<!-- .element: class="fragment" -->

- `struct Foo<T: Into<u32>>`

`T` is u32 or smaller.

<!-- .element: class="fragment" -->

- `struct Foo<T: TryInto<u32>>`

`T` can be any of numeric types.

<!-- .element: class="fragment" -->

---v

### Fallibility: Conversion

- Substrate also provides a trait for infallible saturated conversion as well.
- [source](https://paritytech.github.io/substrate/master/sp_arithmetic/traits/trait.SaturatedConversion.html).

```rust
trait SaturatedConversion {
  fn saturated_into<T>(self) -> T
}

assert_eq!(u128::MAX.saturating_into::<u32>(), u32::MAX);
```

---

# Part 2: FRAME Stuff

---

## `trait Get`

A very basic, yet very substrate-idiomatic way to pass _values_ through _types_.

```rust
pub trait Get<T> {
  fn get() -> T;
}
```

```rust
// very basic blanket implementation, which you should be very versed in reading.
impl<T: Default> Get<T> for () {
  fn get() -> T {
    T::default()
  }
}
```

<!-- .element: class="fragment" -->

```rust
struct Foo<G: Get<u32> = ()>;
```

<!-- .element: class="fragment" -->

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

- Helps convey **values** using **types**.

---

## `bounded`

- `BoundedVec`, `BoundedSlice`, `BoundedBTreeMap`, `BoundedSlice`

```rust
#[derive(Encode, Decode)]
pub struct BoundedVec<T, S: Get<u32>>(
  pub(super) Vec<T>,
  PhantomData<S>,
);
```

- `PhantomData`?

---v

### `bounded`

- Food for your thought.

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
```

```rust
pub struct Identity;
// blanket implementation!
impl<T> Convert<T, T> for Identity {
	fn convert(a: T) -> T {
		a
	}
}
```

<!-- .element: class="fragment" -->

Notes:
this one's much simpler, but good excuse to teach them blanket implementations.

---v

### Example of `Get` and `Convert`

```rust
/// Some configuration for my module.
trait Config {
  /// Something that gives you a `u32`.
  type MaximumSize: Get<u32>;
  /// Something that is capable of converting `u64` to `u32`, which is pretty damn impossible.
  type Convertor: Convertor<u32, u32>;
}
```

```rust
// in your top level runtime.
struct Runtime;
impl Config for Runtime {
  type MaximumSize = (); // remember what this means?
  type Convertor = Identity // remember this guy?
}
```

<!-- .element: class="fragment" -->

---v

### Example of `Get` and `Convert`

```rust
// in your pallet
impl<T: Config> Pallet<T> {
  fn foo() {
    let outcome: u32 = T::Convertor::convert(u32::max_value());
  }
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

How can I easily invoke `OnInitialize` on all 3 of `Module1, Module2, Module3`?

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

1. `on_initialize`, in its ideal form, does not have `&self`, it is defined on the **type**, not a **value**.

2. **Tuples** are the natural way to group **types** together (analogous to have a **vector** is the natural way to group **values** together..)

```rust
// fully-qualified syntax - turbo-fish.
<(Module1, Module2, Module3) as OnInitialize>::on_initialize();
```

---v

### Implementing Traits For Tuples

Only problem: A lot of boilerplate. Macros!

Historically, we made this work with `macro_rules!`

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

And then someone made `impl_for_tuples` crate.

```rust
// In the most basic form:
#[impl_for_tuples(30)]
pub trait OnTimestampSet<Moment> {
	fn on_timestamp_set(moment: Moment);
}
```

---

## Defensive Programming

> Defensive programming is a form of defensive design intended to ensure the continuing function of
> a piece of software under unforeseen circumstances. Defensive programming practices are often
> used where **high availability**, **safety**, or **security** is needed.

- As you know, you should (almost) never panic in your runtime code.

---v

### Defensive Programming

- First reminder: don't panic, unless if you want to punish someone!
- `.unwrap()`? no no

<br>

- be careful with implicit unwraps in standard operations!
  - slice/vector indexing can panic if out of bound
  - `.insert`, `.remove`
  - division by zero.

---v

### Defensive Programming

- When using operations that could panic, comment exactly above it why you are sure it won't panic.

```rust
let pos = announcements
  .binary_search(&announcement)
  .ok()
  .ok_or(Error::<T, I>::MissingAnnouncement)?;
// index coming from `binary_search`, therefore cannot be out of bound.
announcements.remove(pos);
```

---v

### Defensive Programming: QED

Or when using options or results that need to be unwrapped but are known to be `Ok(_)`, `Some(_)`:

```rust
let maybe_value: Option<_> = ...
if maybe_value.is_none() {
  return "..."
}

let value = maybe_value.expect("value checked to be 'Some'; qed");
```

- Q.E.D. or QED is an initialism of the Latin phrase "quod erat demonstrandum", meaning "**which was to be demonstrated**".

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

- Speaking of documentation, [here's a very good guideline](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)!

````rust
/// Multiplies the given input by two.
///
/// Some further information about what this does, and where it could be used.
///
/// ```
/// fn main() {
///   let x = multiply_by_2(10);
///   assert_eq!(10, 20);
/// }
/// ```
///
/// ## Panics
///
/// Panics under such and such condition.
fn multiply_by_2(x: u32) -> u32 { .. }
````

---v

### Defensive Programming

- Try and not be this guy:

```rust
/// This function works with module x and multiples the given input by two. If
/// we optimize the other variant of it, we would be able to achieve more
/// efficiency but I have to think about it. Probably can panic if the input
/// overflows u32.
fn multiply_by_2(x: u32) -> u32 { .. }
```

---v

### Defensive Programming

- The overall ethos of defensive programming is along the lines of:

```rust
// we have good reasons to believe this is `Some`.
let y: Option<_> = ...

// I am really really sure about this
let x = y.expect("hard evidence; qed");

// either return a reasonable default..
let x = y.unwrap_or(reasonable_default);

// or return an error (in particular in dispatchables)
let x = y.ok_or(Error::DefensiveError)?;
```

- But, for example, you are absolutely sure that `Error::DefensiveError` will never happen, can we enforce it better?

---v

### Defensive Programming

```rust
let x = y
  .map_err(|e| {
    #[cfg(test)]
    panic!("defensive error happened: {:?}", e);

    log::error!(target: "..", "defensive error happened: {:?}", e);
  })
  .ok_or(Error::DefensiveError)?;
```

---v

### Defensive Programming

- Yes: [Defensive traits](https://paritytech.github.io/substrate/master/frame_support/traits/trait.Defensive.html):

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

# Additional Resources! 😋

<img width="300px" rounded src="../../../assets/img/4-Substrate/thats_all_folks.png">

- Check speaker notes (click "s" 😉).
- Good luck with FRAME!

Notes:

- Rust didn't have u128 until not too long ago! https://github.com/paritytech/substrate/pull/163/files
- `TryFrom`/`TryInto` are also not too old! https://github.com/paritytech/substrate/pull/163/files#r188938077
- Remove `As`, which tried to fill the lack of `TryFrom/TryInto` https://github.com/paritytech/substrate/pull/2602
- Runtime Logging PR: https://github.com/paritytech/substrate/pull/3821

- Impl trait for tuples:

  - https://stackoverflow.com/questions/64332037/how-can-i-store-a-type-in-an-array
  - https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch
  - https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
  - https://turbo.fish/
  - https://techblog.tonsser.com/posts/what-is-rusts-turbofish
  - https://docs.rs/impl-trait-for-tuples/latest/impl_trait_for_tuples/

- std/no_std
  - https://paritytech.github.io/substrate/master/sp_std/index.html
  - https://doc.rust-lang.org/core/index.html
  - https://doc.rust-lang.org/std/index.html
  - https://rust-lang.github.io/api-guidelines/naming.html#feature-names-are-free-of-placeholder-words-c-feature

### Feedback After Lecture:

- Lecture is still kinda dense and long, try and trim
- Update on defensive ops: https://github.com/paritytech/substrate/pull/12967
- Next time, talk about making a storage struct be `<T: Config>`.
- Cargo format
