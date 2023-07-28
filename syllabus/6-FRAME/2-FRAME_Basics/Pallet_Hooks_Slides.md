---
title: Pallet Hooks
description: Pallet Hooks
duration: 1 hour
---

# Pallet Hooks

---

## Hooks: All In One

All defined in `trait Hooks`
([source](https://github.com/paritytech/substrate/blob/33c518ebbe43d38228ac47e793e4d1c76738a56d/frame/support/src/traits/hooks.rs#L214)).

1. `on_initialize`
1. `on_finalize`
1. `on_idle`
1. `on_runtime_upgrade`
1. `offchain_worker`
1. `integrity_test`

1. Plus, `GenesisConfig`.

---

## Hooks: `on_initialize`

- Useful for any kind of **automatic** operation.

- The weight you return is interpreted as `DispatchClass::Mandatory`.

- Called before any transaction.

---v

### Hooks: `On_Initialize`

- `Mandatory` Hooks should really be lightweight and predictable, with a bounded complexity.

```rust
fn on_initialize() -> Weight {
  // any user can create one entry in `MyMap` 😱🔫.
  <MyMap<T>>::iter().for_each(do_stuff);
}
```

---v

### Hooks: `On_Initialize`

- Question: If you have 3 pallets, in which order they are called?

<!-- .element: class="fragment" -->

- Question: If your runtime panics on_initialize, how can you recover from it?

<!-- .element: class="fragment" -->

- Question: If your hook consumes more than the maximum block weight?

<!-- .element: class="fragment" -->

---

## Hooks: `on_finalize`

Its weight needs to be known in advance. Therefore, less preferred compared to `on_initialize`.

Similar to `on_initialize`, `on_finalize` is also **mandatory**. This is also why its weight is
registered at the beginning of the block.

---v

### Hooks: `on_finalize`

> Generally, avoid using it unless if something REALLY needs to be happen at the end of the block.

Sometimes, rather than thinking "at the end of block N", consider writing code "at the beginning of block N+1"

---

## Hooks: `on_idle`

- **_Optional_** variant of `on_finalize`, also executed at the end of the block.

---

## Hooks: `on_runtime_upgrade`

Called once per every time that the runtime version changes, before anything else.

Your one and only chance to migrate the state if needed.

Has its own lecture!

---

## Hooks: `offchain_worker`

**Fully offchain application**:

- Read chain state via RPC.
- submit desired side effects back to the chain as transactions.

**Runtime Offchain Worker**:

- Code lives onchain, upgradable only in synchrony with the whole runtime 👎
- Ergonomic and fast state access 👍
- State writes are ignored 🤷
- Can submit transactions back to the chain as well ✅

---v

### Hooks: `offchain_worker`

Called per block **IMPORT** (!= _sync_)

has a totally separate thread pool than the normal execution.

Threads can **overlap**, each is reading the state of its corresponding block, and you can great
[`lock`s](https://github.com/paritytech/substrate/blob/49b06901eb65f2c61ff0934d66987fd955d5b8f5/frame/election-provider-multi-phase/src/lib.rs#L789)\_ if needed to make sure there is no overlap.

---v

### Hooks: `offchain_worker`

<image src="../../../assets/img/6-FRAME/ocw.svg" style="height: 500px">

---v

### Hooks: `offchain_worker`

Offchain workers have their own **special host functions**: http, dedicated storage, time, etc.

Offchain workers have the same **execution limits** as Wasm (limited memory, custom allocator).

Notes:

Word on allocator limit in Substrate Wasm execution (subject to change).

- Max single allocation limited
- Max total allocation limited.

---

## Hooks: `integrity_test`

Called upon `construct_runtime!` only.

Best to make sure all the inputs coming into your pallet as `type Foo: Get<u32> = ..` are sensible!

> Panic, panic all you want in here.

```rust
fn integrity_test() {
  assert!(
    T::MaxPointsToBalance::get() > 0,
    "Minimum points to balance ratio must be greater than 0"
  );
}
```

---

## Hooks: Others

These are all the `#[pallet::hooks]` that you can have.

..but there is one more FRAME topics that is hook-like: `GenesisConfig`

Let's have a quick look.

---

## Hooks: `genesis_build`

Each pallet can define a `struct GenesisConfig` --> `<PalletName>Config`.

```rust
construct_runtime!(
  pub enum Runtime {
    System: frame_system,
    PalletA: pallet_a,
  }
);
```

```rust
struct GenesisConfig {
  SystemConfig: pallet_system::GenesisConfig,
  PalletAConfig: pallet_a::GenesisConfig::
}
```

---v

### Hooks: `genesis_build`

```rust
// in your pallet
#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
  pub max_pools: Option<u32>,
  pub max_members: Option<u32>,
}

#[cfg(feature = "std")]
impl<T: Config> Default for GenesisConfig<T> {
  fn default() -> Self {
    Self {
      max_pools: Some(16),
      max_members: Some(16 * 32),
    }
  }
}

#[pallet::genesis_build]
impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
  fn build(&self) {
    // self.max_pools, self.max_members
  }
}


// Somewhere in the client, while building a chain spec:
NominationPoolsConfig {
  max_pools: 42,
  max_members: 24,
}
//This will then be put into your JSON/raw chain-spec.
```

---v

### Hooks: `genesis_build`

```rust
// Somewhere in the client, while building a chain spec:
// This will then be put into your JSON/raw chain-spec.
NominationPoolsConfig {
  max_pools: 42,
  max_members: 24,
}
```

---

### Hooks: Recap

<image src="../../../assets/img/6-FRAME/flow.svg" style="height: 550px">

Question: Where/when is the offchain worker called?

---v

### Hooks: Recap

offchain worker is not really a part of the consensus code in the runtime, client can technically
call it whenever it wants

---

## Further Reading, Recap.

> Hooks are **very powerful** tools to write state transition, but with them it comes a lot of responsibility as well.

- [OnPostInherents](https://github.com/paritytech/substrate/pull/10128).
- [Offchain workers talk](https://www.youtube.com/watch?v=rwzvRi1JkWU).
