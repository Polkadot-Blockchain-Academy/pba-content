---
title: FRAME Overview
description: FRAME Overview for Web3 engineers. FRAME, Pallets, System Pallet, Executive, Runtime Amalgamator.
duration: 1 hour
instructors: ["Shawn Tabrizi, Kian Paimani"]
---

# FRAME Overview

---v

## Agenda

Recall the following figure:

<image src="../../../assets/img/6-FRAME/frame0.svg" style="height: 600px">

Notes:

Without frame, there is the runtime and there is the client, and an API that sits in between.

---v

## Agenda

By the end of this lecture, you will fully understand this figure.

<image src="../../../assets/img/6-FRAME/frame1.svg" style="height: 600px">

---

## 1. A Pallet

> An isolated unit of business logic that executes within a runtime.

Contains:

- Dispatchable extrinsics
- Storage items
- Hooks for:
  - Block initialization,
  - Finalizing block (_!= block finality i.e. GRANDPA_)

---v

### 1. A Pallet

And some less important ones:

- Events
- Errors
- Custom validation/communication with tx-pool
- Offchain workers
- A lot more! but you will learn about them later.

---v

### "Shell" Pallet

```rust
#[frame_support::pallet]
pub mod pallet {
  use frame_support::{pallet_prelude::*};
  use frame_system::pallet_prelude::*;

  // config trait
  #[pallet::config]
  trait Config: frame_system::Config {}

  // storage items
  #[pallet::storage]
  pub type X = ..
  #[pallet::storage]
  pub type Y = ..

  // dispatchables
  #[pallet::pallet]
  pub struct Pallet<T>(PhantomData<T>);
  #[pallet::call]
  impl<T: Config> Pallet<T> {
    fn tx_1(origin: OriginFor<T>, arg: u32) -> DispatchResult { .. }
    fn tx_2(origin: OriginFor<T>, arg: u64) -> DispatchResult { .. }
  }

  // hooks
  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_initialize() {}
    fn on_finalize() {}
  }

  // other stuff, events, errors, genesis configs, unsigned validation etc.
}
```

---v

### Config Trait.

> Gateway to receive **_stuff_** from the outer world.

```rust
#[pallet::config]
trait Config: frame_system::Config {
  // An example of receiving an input type.
  type ValueType: Into<u32> + Codec + Default;

  // An example of receiving a const.
  const MAX_VALUE: u32;

  // A hook about what to do.
  fn on_value_update(new_value: Self::ValueType);
}
```

Everyone has access to system's config.

---v

### Storage Items

> The (holy) **_state_** of your blockchain!

```rust
// T::AccountId comes from system, T::ValueType comes form us.
#[pallet::storage]
pub type Values<T: Config> = StorageMap<_, T::AccountId, T::ValueType>;

// A simpler storage item.
#[pallet::storage]
pub type Counter<T: Config> = StorageValue<_, u32>;
```

NOTE: hash of the storage map.

---v

### Extrinsics

```rust
#[pallet::storage]
pub type Values<T: Config> = StorageMap<_, T::AccountId, T::ValueType>;
#[pallet::storage]
pub type Counter<T: Config> = StorageValue<_, u32>;

#[pallet::pallet]
pub struct Pallet<T>(PhantomData<T>);

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::weight(0)]
  pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
    // checks the origin to be signed -- more on this later.
    let who = ensure_signed(origin)?;

    // check that this user has not submitted already.
    if !<Values<T>>::contains_key(&who) {
      if value > T::MAX_VALUE.into() {
        return Err("failed".into())
      }

      // increment the counter .
      Counter::<T>::mutate(|x| *x += 1);
      let value: T::ValueType = value.into();
      <Values<T>>::insert(who, value);
      T::on_value_update(value);
    } else {
      return Err("already submitted".into())
    }

    Ok(())
  }
}
```

note:

- expand `DispatchError`, and how you can convert from string into it.
- expand `OriginFor`.
- touch on "check-first, write last" and how currently we have `#[transactional]` almost by default.

---v

### Hooks

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
  fn on_initialize(n: T::BlockNumber) -> Weight {
    if n % 10 == Zero::zero() {
      log::info!("count of users is {}", Counter::<T>::get());
    }
  }

  fn on_finalize(_n: T::BlockNumber) {
    // other stuff..
  }
}
```

---v

### Expanding The Code

- Let's recap the whole pallet in one go.
- Let's peek at the expanded code.

---v

### Expanding The Code

- `Pallet` implements the transactions as public functions.

- `Pallet` implements `Hooks`, and some equivalents like `OnInitialize`.

- `enum Call` that has in itself is just an encoding of the transaction's data

- and implements `UnfilteredDispatchable` (which just forward the call back to `Pallet`)
  ---v

### Expanding The Code

Make sure you understand why these 3 are the same!

```rust
let origin = ..;

Pallet::<T>::set_value(origin, 10)

Call::<T>::set_value(10).dispatch_bypass_filter(origin);
<Call<T> as UnfilteredDispatch>::dispatch_bypass_filter(Call::<T>::set_value(10), origin);
```

<hr>

And that's about it! Now let's look at what is this `frame_system` that everyone has to rely on.

---

## 2. The System Pallet

A container for common **types**, **functionality**, and opinionated **OS-style routines**.

- common types: `<T as frame_system::Config>::AccountId`.

- common functionality: `set_code`, `set_storage`, `remark`

- system-level functionality
  - extrinsic count/length/weight(gas)
  - what's the current block number?
  - events
  - runtime version, code update

NOTE:
it stores all of the accounts as well, but that can be changed.

---v

### Config Trait: Common Types

```rust
#[pallet::config]
pub trait Config: {
	// this is touched upon in the previous section:
   type AccountId;

   // these two will become relevant in executive part.
   type Hash;
   type Header;

   // these are not, but should still be comprehensible.
   type Index;
   type BlockNumber;
}
```

---v

### Storage Items: Accounting, Metering, Transient

```rust
#[pallet::storage]
pub(super) type ExtrinsicCount<T: Config> = StorageValue<_, u32>;

#[pallet::storage]
pub(super) type AllExtrinsicsLen<T: Config> = StorageValue<_, u32>;

#[pallet::storage]
pub(super) type AllExtrinsicsWeight<T: Config> = StorageValue<_, ...>;

#[pallet::storage]
pub(super) type ExtrinsicData<T: Config> = StorageMap<_, Twox64Concat, u32, Vec<u8>, ValueQuery>;

#[pallet::storage]
pub(super) type Number<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

#[pallet::storage]
pub(super) type ParentHash<T: Config> = StorageValue<_, T::Hash, ValueQuery>;
```

> set before block execution, deleted in the beginning of the next block.

NOTE:

You now know what to query from PJS api if you want to read the block number.

---v

### System Pallet

Let's See the Code.

---

## 3. `construct_runtime!` and Runtime Amalgamator.

Now, let's look at a minimal runtime amalgamator.
This is where you glue the runtime together, and expose the things you care about as runtime apis.

---v

## 3. `construct_runtime!` and Runtime Amalgamator.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion { .. };

parameter_types! { .. }
impl frame_system::Config for Runtime { .. }

parameter_types! { .. }
impl pallet_xyz::Config for Runtime { .. }

parameter_types! { .. }
impl pallet_pqr::Config for Runtime { .. }

pub mod opaque { .. }

construct_runtime!(
  pub enum Runtime where
  Block = Block,
  NodeBlock = opaque::Block,
  UncheckedExtrinsic = UncheckedExtrinsic
  {
    System: frame_system,
    PalletXyz: pallet_xyx,
    PalletPqr: pallet_pqr,
  }
);

pub type Executive = frame_executive::Executive<_, _, _, ...>;

// this is the juicy part! all implementations seem to come from Executive!
impl_runtime_apis! {
  impl sp_api::Core<Block> for Runtime {
    fn version() -> RuntimeVersion {
      VERSION
    }

    fn execute_block(block: Block) {
      Executive::execute_block(block);
    }

    fn initialize_block(header: &<Block as BlockT>::Header) {
      Executive::initialize_block(header)
    }
  }

  ...
}
```

---v

## 3. `construct_runtime!` and Runtime Amalgamator.

As you guessed it..

Let's look at the full code, and expand it.

<hr>

An alternative to expanding macros:

https://paritytech.github.io/polkadot/doc/polkadot_runtime/
https://paritytech.github.io/substrate/master/kitchensink_runtime/index.html
https://paritytech.github.io/substrate/master/kitchensink_runtime/type.AllPalletsWithSystem.html

---v

## 3. `construct_runtime!` and Runtime Amalgamator.

- struct `Runtime`
- implements the `Config` trait of all pallets.
- implements all of the runtime APIs as functions.
- `type System`, `type SimplePallet`.
- `AllPalletsWithSystem` etc.
  - and recall that all pallets implement things like `Hooks`, `OnInitialize`, and all of these
    traits are tuple-able.
- enum Call
- enum `Event`, `GenesisConfig`, etc. but we don't have them here.

---

## 4. Executive

> This part is somewhat optional to know in advance, but I want you to re-visit it in a week and
> then understand it all.

I present to you, Executive struct:

```rust
pub struct Executive<
  System,
  Block,
  Context,
  UnsignedValidator,
  AllPalletsWithSystem,
  OnRuntimeUpgrade = (),
>(
  PhantomData<(
    System,
    Block,
    Context,
    UnsignedValidator,
    AllPalletsWithSystem,
    OnRuntimeUpgrade,
  )>,
);
```

---v

#### Expanding The Generic Types.

```rust
impl<
    // System config, we know this now.
    System: frame_system::Config,
    // Constrained generics! understanding such syntax is key in rest of FRAME.
    // https://riptutorial.com/rust/example/8574/associated-types
    Block: sp_runtime::traits::Block<Header = System::Header, Hash = System::Hash>,
    // Ignore for now.
    Context: Default,
    // Ignore for now.
    UnsignedValidator,
    // Something that has all the hooks. We don't know anything else about pallets here.
    AllPalletsWithSystem: OnRuntimeUpgrade
      + OnInitialize<System::BlockNumber>
      + OnIdle<System::BlockNumber>
      + OnFinalize<System::BlockNumber>
      + OffchainWorker<System::BlockNumber>,
    // Ignore for now.
    COnRuntimeUpgrade: OnRuntimeUpgrade,
  > Executive<System, Block, Context, UnsignedValidator, AllPalletsWithSystem, COnRuntimeUpgrade>
where
  // This is the juicy party, and we have to learn more sp_runtime traits to follow.
  Block::Extrinsic: Checkable,
  <Block::Extrinsic as Checkable>::Checked: Applyable
  <<Block::Extrinsic as Checkable>::Checked as Applyable>::Call: Dispatchable<_>,
{...}
```

---v

#### `Block::Extrinsic: Checkable`

- Who implements `Checkable`?
- That's right, the `generic::UncheckedExtrinsic` that we indeed used as `Block::Extrinsic` in the
  top level runtime. Recall:

```rust
type UncheckedExtrinsic = generic::UncheckedExtrinsic<_, _, _, _>;
type Header = ..
type Block = generic::Block<Header, UncheckedExtrinsic>;
type Executive = frame_executive::Executive<_, Block, ...>;
```

---v

#### What Does `Checkable<_>` Do?

```rust
impl Checkable<_> for UncheckedExtrinsic<_, _, _, _> {
  // this is the output type.
  type Checked = CheckedExtrinsic<AccountId, Call, Extra>;

  fn check(self, lookup: &Lookup) -> Result<Self::Checked, TransactionValidityError> {
    Ok(match self.signature {
      Some((_)) => {
        // signature verification
        CheckedExtrinsic { signed: .., function: self.function }
      },
      None => {
        // nothing to do, this is not signed.
        CheckedExtrinsic { signed: None, function: self.function },
      }
    })
  }
}
```

---v

#### `<Block::Extrinsic as Checkable>::Checked: Applyable`

- `UncheckedExtrinsic::Checked` is `CheckedExtrinsic`.
- And it surely does implement `Applyable`.

---v

#### What Does `Applyable<_>` Do?

- TLDR: `Ok(self.call.dispatch(maybe_who.into()))`

---v

#### Lastly: `<<Block::Extrinsic as Checkable>::Checked as Applyable>::Call: Dispatchable`

- And guess who implemented `Dispatchable`, which we already looked at!
- The `enum Call` that we had in our expanded file!

---v

### Circling Back..

So, to recap:

```rust
struct Runtime;

impl frame_system::Config for Runtime {}
impl simple_pallet::Config for Runtime {}

enum Call {
  System(frame_system::Call<Runtime>),
  SimplePallet(simple_pallet::Call<Runtime>),
}

impl Dispatchable for Call {
  fn dispatch(self, origin: _) -> Result<_, _> {
    match self {
      Call::System(system_call) => system_call.dispatch(),
      Call::SimplePallet(simple_pallet_call) => system_pallet_call.dispatch(),
    }
  }
}

struct UncheckedExtrinsic {
  function: Call,
  signature: Option<_>,
}

type Executive = Executive<_, UncheckedExtrinsic, ...>;

//
let unchecked = UncheckedExtrinsic::new();
let checked = unchecked.check();
let _ = checked.apply();
```

---v

### Let's Put It All Together

- in the context of how the executive's `fn execute_block` is working under the hood, with some simplifications:

```rust
fn execute_block(block: Block) {
  // get some info out of the block.
  let (header, extrinsics) = block.deconstruct()
  let block_number = header.number();
  let parent_hash = header.parent_hash();

  // initialize the block, including all the on-initialize hooks.
  // go to the impl and see how this is clearing previous TRANSIENT stuff.
  <frame_system::Pallet<System>>::initialize(block_number, parent_hash, digest);
  <AllPalletsWithSystem as OnInitialize<System::BlockNumber>>::on_initialize(_);

  // execute extrinsics.
  extrinsics.into_iter().for_each(|uxt: UncheckedExtrinsic| {
    // recall how `Block::Extrinsic` was UncheckedExtrinsic, thus `uxt: UncheckedExtrinsic `
    // first, go through `Checkable`, where the signature is checked.
    let xt: CheckedExtrinsic = uxt.check(&Default::default())?;
    // then `Applyable`, where actual `.dispatch` is called.
    let _ = Applyable::apply::<_>(xt, _, _);
  });

  // execute some other hooks
  <AllPalletsWithSystem as OnIdle<System::BlockNumber>>::on_idle(block_number);
  <AllPalletsWithSystem as OnFinalize<System::BlockNumber>>::on_finalize(block_number);

  <frame_system::Pallet<System>>::finalize()

  // finally check roots.
  let new_storage_root = sp_io::storage::root(version)[..];
    assert!(header.state_root() == new_storage_root, "Storage root must match that calculated.");
  let new_extrinsic_root = todo!();
    assert!(
      header.extrinsics_root() == new_extrinsic_root,
      "Transaction trie root must be valid.",
    );
}
```

---

## Recap

<image src="../../../assets/img/6-FRAME/frame1.svg" style="height: 600px">
