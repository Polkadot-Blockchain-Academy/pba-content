---
title: FRAME - What Agents Get Wrong
description: Critical concepts for reviewing and auditing AI-generated Substrate/FRAME code
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# FRAME: What Agents Get Wrong

Notes:

- AI coding agents can generate FRAME boilerplate quickly, but they routinely produce code that compiles, passes basic tests, and is still fundamentally broken for on-chain use.
- This lecture covers the concepts you need to **review and audit** agent-generated code.
- If you don't understand these topics, you won't catch the bugs that matter.

---

# Part 1: Determinism Constraints

---

## Why Determinism Matters

- Every validator re-executes every block.
- If validator A computes a different state root than validator B, **consensus breaks**.
- The runtime must be **perfectly deterministic**: same input, same output, every time, on every machine.

---

### The `f32` / `f64` Problem

- Floating point numbers have slightly different implementations across architectures and vendors.

- If my balance is `10.000000000000001` DOT on one validator and `10.000000000000000` DOT on another, game over for consensus.

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

Notes:

An agent will happily use `f64` in your pallet if you ask it to "calculate a percentage." It compiles. It passes tests on your machine. It breaks consensus in production.

---

### PerThing: Fixed-Point Ratios

- We represent ratios with "Fixed-Point" arithmetic types instead.

```rust
use sp_arithmetic::Perbill;

let p = Perbill::from_percent(25);
let p = Perbill::from_rational(1, 4);

> p * 100u32;
> 25u32;
```

- `Perbill` represents `[0, 1]` with `u32` precision (parts per billion).
- Also: `Percent`, `Permill`, `PerU16`.

---

### Fixed Point Numbers

For values outside `[0, 1]`:

```rust
use sp_arithmetic::FixedU64;

let x = FixedU64::from_rational(5, 2);
let y = 10u32;
let z = x * y;
> 25
```

- See [`sp-arithmetic`](https://paritytech.github.io/polkadot-sdk/master/sp_arithmetic/index.html) for the full set of types.

---

### Other Determinism Traps

- **`HashMap`**: iteration order is randomized. Use `BTreeMap`.
- **System time / randomness**: not available in `no_std` runtime.
- **Floating point**: banned entirely.

Notes:

When reviewing agent code, grep for: `f32`, `f64`, `HashMap`, `rand`, `SystemTime`, `Instant`. Any of these in runtime code is a red flag.

---

## Fallible Arithmetic

Addition, multiplication, and division can all silently produce wrong results:

- `u32::MAX * 2 / 2` panics in debug, **wraps silently** in release.
- `100 / 0` panics.

---

### Safe Arithmetic

- **`Checked`** -- returns `None` on overflow:

  ```rust
  if let Some(outcome) = a.checked_mul(b) { ... } else { ... }
  ```

- **`Saturating`** -- clamps to `MAX`/`MIN`:

  ```rust
  let certain_output = a.saturating_mul(b);
  ```

Notes:

Agents often write bare `+`, `*`, `-` operators. In on-chain code, every arithmetic operation should use checked or saturating variants. This is the single most common agent mistake in FRAME code.

Why would you ever want to saturate? Only when the number overflowing means the system is so fundamentally broken that clamping is the least-bad option.

---

### Reviewing Agent Code for Arithmetic

When an agent gives you pallet code, check:

1. Every `+`, `-`, `*`, `/` on balances or quantities -- should be `checked_` or `saturating_`.
2. Every `as` cast between numeric types -- should be `TryInto`/`TryFrom` or `.into()`.
3. Every division -- is zero a possible divisor?

```rust
// Agent writes this:
let share = total_reward * user_stake / total_stake;

// You should demand this:
let share = total_reward
  .checked_mul(&user_stake)
  .and_then(|x| x.checked_div(&total_stake))
  .ok_or(Error::<T>::ArithmeticOverflow)?;
```

---

# Part 2: The `no_std` / Wasm Compilation Model

---

## Why Wasm?

- The Substrate runtime compiles to **Wasm** and lives on-chain.
- Wasm runtimes cannot use the standard library (`std`) -- no filesystem, no networking, no OS.
- Every runtime crate must compile in `no_std` mode.

Notes:

- `std` is the interface to common OS abstractions.
- `core` is a subset of `std` that makes no assumption about the OS.
- A `no_std` crate relies on `core` rather than `std`.

---

### The Feature Flag Convention

Every Substrate runtime crate follows this pattern:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
```

And in `Cargo.toml`:

```toml
[dependencies]
dep1 = { version = "1.0.0", default-features = false }
dep2 = { version = "1.0.0", default-features = false }

[features]
default = ["std"]
std = [
  "dep1/std",
  "dep2/std",
]
```

Notes:

- `default-features = false` disables the dependency's `std` feature by default.
- `std = ["dep1/std"]` means "if my std is enabled, enable my dependency's std too."
- Missing a single dependency from the `std` feature list is one of the most common build errors.

---

### The Error You'll See

```sh
error: duplicate lang item in crate sp_io (which frame_support depends on): panic_impl.
  |
  = note: the lang item is first defined in crate std (which serde depends on)
```

This means a dependency pulled in `std` when compiling for Wasm. Fix: ensure that dependency has `default-features = false` and is included in the `std` feature list.

Notes:

Agents get the `Cargo.toml` boilerplate right most of the time, but when adding new dependencies they frequently forget `default-features = false` or forget to add the dep to the `std` feature list. Always check both.

---

### Wasm Blob Size Matters

- Every string literal in your runtime increases the Wasm blob size.
- `#[derive(Debug)]` adds struct/field names as string literals.
- Be mindful of how many log statements and string literals end up in your runtime code.

Notes:

`RuntimeDebug` used to strip debug strings in Wasm builds, but it is now deprecated -- just use `#[derive(Debug)]` directly. The blob size concern is still real though: excessive logging and large error message strings add up.

---

# Part 3: Defensive Programming

---

## The Core Rule

> Never panic in on-chain code.

A panic in a dispatchable means the extrinsic fails in a way that **cannot be handled gracefully**. In some contexts, a panic can halt block production.

---

### Hidden Panics

`unwrap()` is obvious, but panics also hide in:

- **Slice/vector indexing**: `my_vec[i]` panics if `i >= len`
- **`.insert()` / `.remove()`**: can panic on out-of-bounds index
- **Division**: `x / 0` panics
- **Integer overflow**: `u32::MAX + 1` panics in debug mode

Notes:

Agents produce all of these. They especially love direct indexing (`items[i]`) instead of `.get(i)`.

---

### The Error Handling Hierarchy

From least safe to most safe:

```rust
// 1. Never do this in on-chain code
let value = maybe_value.unwrap();

// 2. Acceptable for truly impossible cases, with proof
let value = maybe_value.expect("checked to be Some on line above; qed");

// 3. Return an error -- preferred in dispatchables
let value = maybe_value.ok_or(Error::<T>::ValueNotFound)?;

// 4. Best: defensive variant -- panics in tests, logs error in production
let value = maybe_value
  .defensive_ok_or(Error::<T>::ValueNotFound)?;
```

Notes:

QED = "quod erat demonstrandum" ("which was to be demonstrated"). It signals that the preceding code proves this unwrap is safe.

---

### Defensive Traits

[`Defensive`](https://paritytech.github.io/substrate/master/frame_support/traits/trait.Defensive.html) traits from `frame_support`:

```rust
use frame_support::traits::DefensiveOption;

// Panics in tests (#[cfg(debug_assertions)]), logs error in production
let x = maybe_value.defensive_unwrap_or(default);
let x = maybe_value.defensive_ok_or(Error::<T>::Impossible)?;
```

This gives you the best of both worlds:

1. **Tests catch the "impossible" case** via panic.
2. **Production gracefully degrades** via error return + log.

---

### Reviewing Agent Code for Panics

Checklist when reviewing agent-generated pallet code:

1. **`grep -n "unwrap()"` / `grep -n "expect("`** -- each one needs justification.
2. **Direct indexing** (`vec[i]`, `slice[i]`) -- should be `.get(i)`.
3. **Arithmetic operators** -- should be checked/saturating (covered in Part 1).
4. **Division** -- is the divisor guaranteed non-zero?
5. **`as` casts** -- should be `try_into()`.

---

# Part 4: Type-Level Configuration in FRAME

---

## `<Type as Trait>::AssociatedType`

The single most important Rust syntax for reading FRAME code:

```rust
trait Config {
  type Extrinsic;
  type Header: HeaderT;
}

// Type alias shorthand -- very common in Substrate
pub type ExtrinsicFor<C> = <C as Config>::Extrinsic;

// Nested associated types
trait HeaderT {
  type Number;
}
pub type NumberFor<C> = <<C as Config>::Header as HeaderT>::Number;
```

Notes:

If you can't read this syntax fluently, you can't review FRAME code. Agents produce it correctly, but you need to verify that the types are wired up right.

---

### `trait Get`: Values Through Types

```rust
pub trait Get<T> {
  fn get() -> T;
}
```

Used to pass configuration constants at the type level:

```rust
parameter_types! {
  pub const MaxItems: u32 = 100;
}

// Expands to:
pub struct MaxItems;
impl Get<u32> for MaxItems {
  fn get() -> u32 { 100 }
}
```

---

### Why Types Instead of Values?

```rust
// BoundedVec carries its bound in the TYPE, not as a runtime field
pub struct BoundedVec<T, S: Get<u32>>(
  Vec<T>,
  PhantomData<S>,
);
```

Why not just store the bound as a `u32` field?

- The bound is known at **compile time** and encoded in the **type system**.
- This enables the compiler to enforce bounds statically.
- Storage proofs and weight calculations depend on knowing max sizes at compile time.
- An agent might try to use `Vec<T>` with runtime length checks -- this is wrong for FRAME storage.

Notes:

`Get` trait is a way to convey values through types. The type system is mostly for the compiler, with minimal overhead at runtime.

---

### The `Config` Trait Pattern

Every FRAME pallet is generic over a `Config` trait:

```rust
trait Config {
  /// Maximum number of items.
  type MaxItems: Get<u32>;
  /// How to convert between balance types.
  type BalanceConverter: Convert<u64, u32>;
}
```

When reviewing agent-generated pallet configuration:

1. Are bounds (`Get<u32>`) reasonable for the chain's requirements?
2. Are the right trait bounds specified (not too loose, not too tight)?
3. Is the runtime implementation (`impl Config for Runtime`) wiring the correct concrete types?

---

## Summary: Your Agent Review Checklist

1. **Determinism**: No `f64`, `HashMap`, `rand`, or OS-dependent behavior in runtime code.
2. **Arithmetic**: Every `+`, `-`, `*`, `/` uses checked or saturating variants.
3. **no_std**: New deps have `default-features = false` and appear in the `std` feature list.
4. **No panics**: No `unwrap()` without proof, no direct indexing, no bare division.
5. **Bounded types**: Storage uses `BoundedVec`/`BoundedBTreeMap`, not bare collections.
6. **Type configuration**: `Config` trait bounds and `parameter_types!` values are sensible.

Notes:

Agents are excellent at generating FRAME boilerplate. Your job is to catch the classes of bugs they systematically produce: non-determinism, arithmetic overflow, hidden panics, and missing bounds. These are the bugs that compile, pass tests, and break chains.
