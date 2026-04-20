---
title: Constructing a Runtime
description: Deep dive into the `#[runtime]` macro
duration: 1 hour
instructors: ["Kian Paimani"]
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# `#[runtime]` 🔨

---

<img style="height: 600px" src="../../assets/img/6-FRAME/frame1.svg" />

<img style="height: 60px; position: absolute; left: 56% " src="../../assets/icons/polkadot/line/Arrow Up.svg" />
<!-- .element: class="fragment" -->

---

## Pallet <=> Runtime

A runtime is really ✌️ things:

1. A struct that implements `Config` of all pallets.
2. A type that helps `Executive` implement `RuntimeApis`.

---v

### Pallet <=> Runtime

We build a runtime - using the `#[runtime]` macro - typically twice:

1. Per pallet, there is a mock runtime.
2. A real runtime elsewhere.

Note:

Elsewhere meaning in a `runtime/lib.rs`.

Benchmarking can then use both of these runtimes.

---

## `#[runtime]`: `Runtime` type

```rust [1-100|5]
#[frame_support::runtime]
mod runtime {
  #[runtime::runtime]
  #[runtime::derive(RuntimeCall, RuntimeEvent, ...)]
  pub struct Runtime;
  #[runtime::pallet_index(0)]
  pub type System = frame_system;
  #[runtime::pallet_index(1)]
  pub type Timestamp = pallet_timestamp;
  #[runtime::pallet_index(2)]
  pub type Balances = pallet_balances;
  #[runtime::pallet_index(5)]
  pub type <NameYouChoose> = path_to_crate;
}
```

Note:
Pallet indices are mandatory in the new `#[runtime]` syntax (unlike the legacy `construct_runtime!` where they were optional).

---v

### `Runtime` type

- It implements [A LOT OF STUFF](https://paritytech.github.io/substrate/master/kitchensink_runtime/struct.Runtime.html)!
- But most importantly, the `Config` trait of all of your pallets 🫵🏻.

```rust
impl frame_system::Config for Runtime { .. }
impl pallet_timestamp::Config for Runtime { .. }
impl pallet_dpos::Config for Runtime { .. }
```

Notes:
Which means that the Runtime is configured at the type level at compile time.

---v

### `<T: Config>` ==> `Runtime`

> Anywhere in your pallet code that you have `<T: Config>` can now be replaced with `Runtime`.

```rust[1-2|3-4|5-6]
// a normal pub function defined in the pallet
frame_system::Pallet::<Runtime>::block_number();
// a storage getter of a map.
frame_system::Pallet::<Runtime>::account(42u32);
// A storage type.
frame_system::Account::<Runtime>::get(42u32);
```

---

## `#[runtime]`: Pallet List

```rust [6-11|12-13]
#[frame_support::runtime]
mod runtime {
  #[runtime::runtime]
  #[runtime::derive(RuntimeCall, RuntimeEvent, ...)]
  pub struct Runtime;
  #[runtime::pallet_index(0)]
  pub type System = frame_system;
  #[runtime::pallet_index(1)]
  pub type Timestamp = pallet_timestamp;
  #[runtime::pallet_index(2)]
  pub type Balances = pallet_balances;
  #[runtime::pallet_index(5)]
  pub type <NameYouChoose> = path_to_crate;
}
```

---v

### Pallet List

- Crucially, under the hood, this generates:

```rust
type System = frame_system::Pallet<Runtime>;
type Balances = pallet_balances::Pallet<Runtime>;
..
type DPos = pallet_dpos::Pallet<Runtime>;
```

- Recall that `Runtime` implements `<T: Config>` of all pallets.

---v

### Pallet List

```rust
frame_system::Pallet::<Runtime>::block_number(); // 🤮
System::block_number(); // 🥳

frame_system::Pallet::<Runtime>::account(42u32); // 🤮
System::account(42u32); // 🥳
```

---v

### Pallet List

- Next crucial piece of information that is generated is:

```rust
type AllPalletsWithSystem = (System, Balances, ..., Dpos);
```

</div>

- This is used in `Executive` to dispatch pallet hooks.

```rust
<AllPalletsWithSystem as OnInitialize>::on_initialize();
<AllPalletsWithSystem as OnFinalize>::on_finalize();
```

</div>

<!-- .element: class="fragment" -->

Notes:

Question: What will be the order of `fn on_initialize()`?
There's also `type AllPalletsWithoutSystem`.

---v

### Pallet List + Outer Enums

- Generates some outer types:

  - `RuntimeCall`
  - `RuntimeEvent`
  - `RuntimeError`
  - `RuntimeOrigin`
  - `RuntimeHoldReason` / `RuntimeFreezeReason`
  - `RuntimeSlashReason` / `RuntimeLockId`
  - `RuntimeTask`
  - `RuntimeViewFunction`
  - `RuntimeGenesisConfig`

Notes:

See the lecture on individual item, and the "Outer Enum" lecture.

---v

### Pallet List: `RuntimeCall` Example

```rust
// somewhere in your pallet, called `my_pallet`
#[pallet::call]
impl<T: Config> Pallet<T> {
  fn transfer(origin: OriginFor<T>, from: T::AccountId, to: T::AccountId, amount: u128);
  fn update_runtime(origin: OriginFor<T>, new_code: Vec<u8>);
}
```

```rust
// expanded in your pallet
enum Call {
  transfer { from: T::AccountId, to: T::AccountId, amount: u128 },
  update_runtime { new_code: Vec<u8> },
}
```

<!-- .element: class="fragment" -->

```rust
// in your outer runtime
enum RuntimeCall {
  System(frame_system::Call),
  MyPallet(my_pallet::Call),
}
```

<!-- .element: class="fragment" -->

---v

### Pallet List: Pallet Customization

The `#[runtime]` macro pulls pallet parts in automatically, but you can opt-out of a few things per pallet:

```rust [2-3|6-7]
#[runtime::pallet_index(1)]
// can be optionally attached to a pallet to disable unsigned calls
#[runtime::disable_unsigned]
pub type Timestamp = pallet_timestamp;
#[runtime::pallet_index(2)]
// can be optionally attached to a pallet to disable its calls
#[runtime::disable_call]
pub type Balances = pallet_balances;
```

---

## `#[runtime]`: Final Thoughts

- Order of pallets does not matter in `#[runtime]` (pallet indices drive the ordering).
- Recall `integrity_test()` is called when the runtime is built.

```sh
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
```

Notes:

### Original Lecture Script

this is your bridge from a pallet into a runtime.

a runtime amalgamator is composed of the following:

1. all pallet's `Config` implemented by a `struct Runtime`;
1. construct `Executive` and use it to implement all the runtime APIs
1. Optionally, some boilerplate to setup benchmarking.
1. invoke the `#[frame_support::runtime]` macro.
1. Alias for each pallet.

The `#[runtime]` macro itself does a few things under the hood:

1. create `struct Runtime`.
1. amalgamate `enum RuntimeCall`; // passed inwards to some pallets that want to store calls.
1. amalgamate `enum RuntimeEvent`; // passed inwards to all pallets.
1. amalgamate `enum RuntimeOrigin` (this is a fixed struct, not an amalgamation);
1. Create a very important type alias:

- `type AllPalletsWithSystem` / `type AllPalletsWithoutSystem`

1. run `integrity_test()`.

> `RuntimeError` now exists as an amalgamated type, similar to `RuntimeEvent` and `RuntimeCall`.

- Pallet indices are mandatory in `#[runtime]`.
- The legacy `construct_runtime!` macro still exists for backward compatibility but is no longer recommended for new runtimes.

```

```
