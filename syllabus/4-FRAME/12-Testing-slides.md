---
title: FRAME Testing
description: Testing and mocks for FRAME pallets
duration: 1 hour
instructors: ["Kian Paimani"]
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# FRAME Testing

---

## Testing and Mocks

A test requires a mock runtime, so we need to do a full `#[runtime]` 😱

.. but luckily, most types can be mocked 😮‍💨

<!-- .element: class="fragment" -->

---

### Testing and Mocks

- `u32` account id.
- `u128` balance.
- `u32` block number.
- ...

---

## Testing: `Get<_>`

- Next, we want to supply some value to those `Get<_>` associated types.

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
  type MaxVoters: Get<u32>;
}
```

---

### Testing: `Get<_>`

```rust
parameter_types! {
  pub const MyMaxVoters: u32 = 16;
}
```

```rust
impl pallet_template::Config for Runtime {
  type MaxVoters = MyMaxVoters;
}
```

<!-- .element: class="fragment" -->

Note: `parameter_types` generates `impl Get for MyMaxVoters`

---

### Testing: `Get<_>`

- Or, if your value is always constant:

```rust
impl pallet_dpos::Config for Runtime {
  type MaxVoters = frame_support::traits::ConstU32<16>;
}
```

---

### Testing: `Get<_>`

- Or, if you want to torture yourself:

```rust
pub struct MyMaxVoters;
impl Get<u32> for MyMaxVoters {
  fn get() -> u32 {
    100
  }
}

impl pallet_dpos::Config for Runtime {
  type MaxVoters = MyMaxVoters;
}
```

---

## Testing: `derive_impl`

Manually implementing every `Config` type for test mocks is verbose. The `derive_impl` macro fills in sensible defaults so you only specify what you need.

```rust
// Before: specify every type manually
impl frame_system::Config for Runtime {
  type AccountId = u64;
  type Block = Block;
  type BlockWeights = ();
  type Nonce = u32;
  type RuntimeEvent = RuntimeEvent;
  type RuntimeCall = RuntimeCall;
  // ... many more types
}
```

```rust
// After: derive defaults, override only what differs
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
  type Block = Block;
}
```

---

### `derive_impl`: How It Works

- Pallets define `#[pallet::config(with_default)]` to generate a `DefaultConfig` trait.
- Types marked `#[pallet::no_default_bounds]` are included in `DefaultConfig` but without trait bounds, allowing runtime injection via `#[inject_runtime_type]`.
- Pallets ship `config_preludes::TestDefaultConfig` with sensible test values.

```rust
// In your pallet definition:
#[pallet::config(with_default)]
pub trait Config: frame_system::Config {
  #[pallet::no_default_bounds]  // included in DefaultConfig but without bounds
  type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

  type MaxVoters: Get<u32>;  // will have a default
}
```

```rust
// In your test mock:
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
  type AccountStore = System;
}
```

---

## Testing: Genesis and Builder

- Next, if you want to feed some data into your pallet's genesis state, we must first setup the
  genesis config correctly.

```rust
#[pallet::genesis_config]
#[derive(frame_support::DefaultNoBound)]
pub struct GenesisConfig<T: Config> {
	pub voters: Vec<(T::AccountId, Option<Vote>)>,
}

#[pallet::genesis_build]
impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
  fn build(&self) {
    for (voter, maybe_vote) in &self.voters {
      // do stuff.
    }
  }
}
```

---

### Testing and Mocks: Genesis and Builder

- Then, we build a builder pattern to construct the genesis config.

```rust
#[derive(Default)]
pub struct Builder {
  pub voters: Vec<(u64, Option<Vote>)>,
}
```

```rust
impl Builder {
  pub fn add_voter(mut self, who: u64) -> Self {
    self.voters.push((who, None));
    self
  }
}
```

<!-- .element: class="fragment" -->

---

### Testing and Mocks: Genesis and Builder

- Finally:

```rust
impl Builder {
  pub fn build(self) -> TestExternalities {
    let system = frame_system::GenesisConfig::<Runtime>::default();
    let template_module = crate::GenesisConfig { voters: self.voters, ..Default::default() };
    RuntimeGenesisConfig { system, template_module }.build_storage().unwrap().into()
  }

  pub fn build_and_execute(self, f: impl FnOnce()) {
    let mut ext = self.build();
    ext.execute_with(f);
    // any post checks can come here.
  }
}
```

---

### Testing and Mocks

- Finally, this allows you to write a test like this:

```rust
#[test]
fn test_stuff() {
  let mut ext = Builder::default()
    .add_voter_with_vote(2, Vote::Aye)
    .add_voter(3)
    .build_and_execute(|| {
      // do stuff
    });
}
```

---

## Testing: Adjustable `parameter_types!`

- What if you want to change that `MyMaxVoters`?

<div>

```rust
parameter_types! {
  pub static MyMaxVoters: u32 = 100;
}
```

```rust
MyMaxVoters::set(200);
MyMaxVoters::get();
```

<!-- .element: class="fragment" -->

</div>

Note: This is _testing only_ and will create a static variable that you can adjust for your tests.

---

## Testing: Adjustable `parameter_types!`

```rust
parameter_types! {
  pub storage MyMaxVoters: u32 = 100;
}
```

```rust
MyMaxVoters::set(200);
MyMaxVoters::get();
```

Note: This uses the given name as a storage key, so be careful with collisions!

---

## Testing: Progressing Blocks

- Often in your test, you want to mimic the progression of an empty block.
- De-nada! We can fake everything in tests 🤠
<!-- .element: class="fragment" -->

---

### Progressing Blocks

```rust
pub fn next_block() {
  let now = System::block_number();
  Dpos::on_finalize(now);
  System::on_finalize(now);

  System::set_block_number(now + 1);

  System::on_initialize(now + 1)
  Dpos::on_initialize(now + 1);
}
```

---

### Progressing Blocks

```rust
pub fn next_block() {
  let now = System::block_number();
  AllPalletsWithSystem::on_finalize(now);

  System::set_block_number(now + 1);

  AllPalletsWithSystem::on_initialize(now + 1)
}
```

---

### Progressing Blocks

```rust
#[test]
fn test() {
  let mut ext = Builder::default()
    .add_validator(1)
    .set_minimum_delegation(200)
    .build();
  ext.execute_with(|| {
    // initial stuff
    next_block();

    // dispatch some call
    assert!(some_condition);

    next_block();

    // repeat..
  });
}
```

---

## Additional Resources 😋

> Check speaker notes (click "s" 😉)

Notes:

- This PR was actually an outcome Cambridge PBA: https://github.com/paritytech/substrate/pull/11932
- https://github.com/paritytech/substrate/pull/11818
- https://github.com/paritytech/substrate/pull/10043
- On usage of macros in Substrate: https://github.com/paritytech/substrate/issues/12331
- Discussion on advanced testing: https://forum.polkadot.network/t/testing-complex-frame-pallets-discussion-tools/356
- Reserve topic: Reading events.
- Reserve-topic: try-state.

```

---

<!-- .slide: data-background-color="#000000" -->

# Questions
```
