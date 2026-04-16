---
title: Substrate/FRAME Tips and Tricks
description: Essential patterns and pitfalls for building with Substrate and FRAME
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# Substrate / FRAME Tips and Tricks

Notes:

A collection of things you should know when writing FRAME pallets. These aren't features of FRAME itself -- they're the sharp edges that catch people.

---

1. **Determinism** -- Why floats and HashMaps break consensus
2. **Safe Math** -- Overflow, underflow, and how to avoid them
3. **Panics** -- Hidden ways your runtime can crash
4. **`no_std`** -- How the Wasm compilation model works and what breaks it
5. **Config** -- Type-level configuration, `Get<T>`, and bounded storage

---

# Determinism

---

## Why Determinism Matters

- Every validator re-executes every block.
- If validator A computes a different state root than validator B, **consensus breaks**.
- The runtime must be **perfectly deterministic**: same input, same output, every time, on every machine.

---

## The `f32` / `f64` Problem

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

---

## PerThing: Fixed-Point Ratios

We represent ratios with "Fixed-Point" arithmetic types instead.

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

## Fixed Point Numbers

For values outside `[0, 1]`:

```rust
use sp_arithmetic::FixedU64;

let x = FixedU64::from_rational(5, 2);
let y = 10u32;
let z = x * y;
> 25
```

See [`sp-arithmetic`](https://paritytech.github.io/polkadot-sdk/master/sp_arithmetic/index.html) for the full set of types.

---

## Other Determinism Traps

- **`HashMap`**: iteration order is randomized. Use `BTreeMap`.
- **System time / randomness**: not available in `no_std` runtime.
- **Floating point**: banned entirely.

---

# Safe Math

---

## Fallible Arithmetic

Addition, multiplication, and division can all silently produce wrong results:

- `u32::MAX * 2 / 2` panics in debug, **wraps silently** in release.
- `100 / 0` panics.

---

## Safe Arithmetic

- **`Checked`** -- returns `None` on overflow:

  ```rust
  if let Some(outcome) = a.checked_mul(b) { ... } else { ... }
  ```

- **`Saturating`** -- clamps to `MAX`/`MIN`:

  ```rust
  let certain_output = a.saturating_mul(b);
  ```

Notes:

Why would you ever want to saturate? Only when the number overflowing means the system is so fundamentally broken that clamping is the least-bad option.

---

## Unsafe vs Safe Arithmetic

```rust
// Unsafe: bare operators can overflow or panic
let share = total_reward * user_stake / total_stake;

// Safe: checked operations with explicit error handling
let share = total_reward
  .checked_mul(&user_stake)
  .and_then(|x| x.checked_div(&total_stake))
  .ok_or(Error::<T>::ArithmeticOverflow)?;
```

Rules of thumb:

1. Every `+`, `-`, `*`, `/` on balances or quantities -- use `checked_` or `saturating_`.
2. Every `as` cast between numeric types -- use `TryInto`/`TryFrom` or `.into()`.
3. Every division -- consider whether zero is a possible divisor.

---

# Panics

---

## The Core Rule

> Never panic in on-chain code.

A panic in a dispatchable means the extrinsic fails in a way that **cannot be handled gracefully**. In some contexts, a panic can halt block production.

---

## Hidden Panics

`unwrap()` is obvious, but panics also hide in:

- **Slice/vector indexing**: `my_vec[i]` panics if `i >= len`
- **`.insert()` / `.remove()`**: can panic on out-of-bounds index
- **Division**: `x / 0` panics
- **Integer overflow**: `u32::MAX + 1` panics in debug mode

---

## The Error Handling Hierarchy

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

## Defensive Traits

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

## Panic Checklist

When reviewing pallet code, check:

1. **`unwrap()` / `expect()`** -- each one needs justification.
2. **Direct indexing** (`vec[i]`, `slice[i]`) -- should be `.get(i)`.
3. **Arithmetic operators** -- should be checked/saturating.
4. **Division** -- is the divisor guaranteed non-zero?
5. **`as` casts** -- should be `try_into()`.

---

# `no_std`

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

## The Feature Flag Convention

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

## The Error You'll See

```sh
error: duplicate lang item in crate sp_io (which frame_support depends on): panic_impl.
  |
  = no​te: the lang item is first defined in crate std (which serde depends on)
```

This means a dependency pulled in `std` when compiling for Wasm. Fix: ensure that dependency has `default-features = false` and is included in the `std` feature list.

---

## Other Common Feature Flags

`std` is not the only feature flag. Pallets and runtimes use several others:

```toml
[features]
default = ["std"]
std = [...]
runtime-benchmarks = [
  "dep1/runtime-benchmarks",
  "dep2/runtime-benchmarks",
]
try-runtime = [
  "dep1/try-runtime",
  "dep2/try-runtime",
]
```

- **`runtime-benchmarks`** -- enables benchmarking code. Relaxes certain checks (e.g. allows minting tokens) so benchmarks can set up worst-case scenarios.
- **`try-runtime`** -- enables `try-state` hooks that validate storage invariants. Used by `try-runtime-cli` to test migrations against live chain state.

Notes:

These follow the same propagation pattern as `std` -- every dependency that supports the feature must be listed. The `runtime-benchmarks` feature is particularly important to understand: code gated behind it runs only during benchmarking, not in production. This is how benchmarks create accounts, mint balances, and set up state without going through normal extrinsic checks.

---

# Config

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

---

## `trait Get`: Values Through Types

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

## Why Types Instead of Values?

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
- Using `Vec<T>` with runtime length checks is wrong for FRAME storage.

Notes:

`Get` trait is a way to convey values through types. The type system is mostly for the compiler, with minimal overhead at runtime.

---

## All Storage Must Implement `MaxEncodedLen`

FRAME requires every storage type to implement `MaxEncodedLen` -- the **maximum** number of bytes that type could ever occupy when SCALE-encoded.

```rust
// This works: BoundedVec has a known max size
#[pallet::storage]
pub type Items<T> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<u8, MaxItems>>;

// This does NOT compile: Vec has no upper bound
#[pallet::storage]
pub type Items<T> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>>;
```

Why it matters:

- **PoV (Proof of Validity)**: parachains must prove storage reads to the relay chain. The proof size is bounded by `MaxEncodedLen`.
- **Weight calculation**: `proof_size` in weights comes from the max bytes each storage read could return.
- **Without it**, FRAME cannot guarantee that a block stays within its PoV budget.

Notes:

This is why `BoundedVec`, `BoundedBTreeMap`, and `BoundedBTreeSet` exist -- they are the bounded equivalents of `Vec`, `BTreeMap`, and `BTreeSet` that implement `MaxEncodedLen`. The `#[pallet::without_storage_info]` attribute silently disables this check. If you see that attribute, remove it and fix the types instead.

---

## Use Properly Sized Types

Every byte in storage costs weight to read, write, and prove. Don't use `u64` when `u8` will do.

```rust
// Bad: 100 items max doesn't need 4 bytes
type MaxItems: Get<u32>; // max 4,294,967,295

// Good: u8 is enough for a max of 100
type MaxItems: Get<u8>;  // max 255
```

This compounds in storage structs:

```rust
// Every field is bigger than it needs to be
pub struct GameState {
  round: u64,     // never exceeds 20 → use u8
  lives: u64,     // never exceeds 5  → use u8
  score: u64,     // never exceeds 10_000 → use u16
  seed: u64,      // actually needs u64 ✅
}
```

Smaller types mean smaller SCALE encoding, less PoV cost, and tighter `MaxEncodedLen` bounds.

Notes:

This also applies to storage keys and map values. A `StorageMap<_, _, u64, _>` uses 8 bytes per key in proofs. If your IDs fit in `u16`, use `u16`. The savings multiply across every storage read and write in a block.

---

## The `Config` Trait Pattern

Every FRAME pallet is generic over a `Config` trait:

```rust
trait Config {
  /// Maximum number of items.
  type MaxItems: Get<u32>;
  /// How to convert between balance types.
  type BalanceConverter: Convert<u64, u32>;
}
```

When reviewing pallet configuration:

1. Are bounds (`Get<u32>`) reasonable for the chain's requirements?
2. Are the right trait bounds specified (not too loose, not too tight)?
3. Is the runtime implementation (`impl Config for Runtime`) wiring the correct concrete types?
