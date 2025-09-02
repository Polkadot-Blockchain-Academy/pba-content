---
title: Migrations and Try Runtime
description: Runtime upgrades and how to survive them
instructors: ["Kian Paimani"]
teaching-assistants: [".."]
---

# Migrations and Try Runtime

---

## Runtime upgrades...

### _and how to survive them_

---

### _At the end of this lecture, you will be able to:_

- WHEN: runtime migrations are needed.
- HOW: to write a runtime migration, end-to-end.
- VERIFY: Test your runtime upgrades with `try-runtime` and `remote-externalities`.

---

## When is a Migration Required?

---v

### When is a Migration Required?

- In a typical runtime upgrade, you typically only replace `:code:`. This is _**Runtime Upgrade**_.
- If you change the _storage layout_, then this is also a _**Runtime Migration**_.

> Anything that changes **encoding** is a migration!

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub struct Foo(u32)
// new
pub struct Foo(u64)
```

- A clear migration.

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub struct Foo(u32)
// new
pub struct Foo(i32)
// or
pub struct Foo(u16, u16)
```

- The data still _fits_, but the _interpretations_ is almost certainly different!

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub struct Foo { a: u32, b: u32 }
// new
pub struct Foo { a: u32, b: u32, c: u32 }
```

- This is still a migration, because `Foo`'s decoding changed.

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub struct Foo { a: u32, b: u32 }
// new
pub struct Foo { a: u32, b: u32, c: PhantomData<_> }
```

- If for whatever reason `c` has a type that its encoding is like `()`, then this would work.

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
  // old
  pub enum Foo { A(u32), B(u32) }
  // new
  pub enum Foo { A(u32), B(u32), C(u128) }
```

- Extending an enum is even more interesting, because if you add the variant to the end, no migration is needed.

<!-- .element: class="fragment" -->

- Assuming that no value is initialized with `C`, this is _not_ a migration.

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub enum Foo { A(u32), B(u32) }
// new
pub enum Foo { A(u32), C(u128), B(u32) }
```

- You probably _never_ want to do this, but it is a migration.

<!-- .element: class="fragment" -->

---v

### 🦀 Rust Recall 🦀

Enums are encoded as the variant enum, followed by the inner data:

- The order matters! Both in `struct` and `enum`.
- Enums that implement `Encode` cannot have more than 255 variants.

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, u32>;
```

```rust
// new
#[pallet::storage]
pub type BarValue = StorageValue<_, u32>;
```

- So far everything is changing the _value_ format.<br/>

<div>

- The _key_ changing is also a migration!

</div>

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, u32>;
```

```rust
// new
#[pallet::storage_prefix = "FooValue"]
#[pallet::storage]
pub type I_can_NOW_BE_renamEd_hahAA = StorageValue<_, u32>;
```

- Handy macro if you must rename a storage type.<br/>
- This does _not_ require a migration.

<!-- .element: class="fragment" -->

---

## Writing Runtime Migrations

- Now that we know how to detect if a storage change is a **migration**, let's see how we write one.

---v

### Writing Runtime Migrations

- Once you upgrade a runtime, the code is expecting the data to be in a new format.
- Any `on_initialize` or transaction might fail decoding data, and potentially `panic!`

---v

### Writing Runtime Migrations

- We need a **_hook_** that is executed **ONCE** as a part of the new runtime...
- But before **ANY** other code (on_initialize, any transaction) with the new runtime is executed.

> This is `OnRuntimeUpgrade`.

<!-- .element: class="fragment" -->

---

## The Problem: Migration Safety

---v

### What Can Go Wrong?

```rust
impl<T: Config> OnRuntimeUpgrade for Pallet<T> {
    fn on_runtime_upgrade() -> Weight {
        // Migrate storage format
        OldStorage::<T>::translate(|old| transform(old));
        T::DbWeight::get().reads_writes(100, 100)
    }
}
```

**Problem**: What if this runs twice? 💥

- Data gets transformed again → corruption
- Already-migrated data may not decode properly
- Chain could brick!

---v

### Version Checking is Critical

```rust
// Chain at v0, upgrading to v1
if on_chain_version == 0 && current_version == 1 {
    // Safe to migrate
    do_migration();
    update_version();
}
```

But managing this manually is error-prone:

- Forgetting version checks
- Incorrect version comparisons
- Forgetting to update version after migration

---

## How We Write Migrations

---v

### Two Migration Approaches

1. **Single-Block Migrations** - Using `VersionedMigration`
   - For most migrations that fit in a single block
   - Automatic version management
2. **Multi-Block Migrations** - Using `SteppedMigration`
   - For heavy migrations that exceed block limits
   - Executes across multiple blocks

---

## Single-Block Migrations: VersionedMigration

---v

### Using VersionedMigration

Handles version checking automatically:

```rust
use frame_support::migrations::VersionedMigration;

pub type MigrateV0ToV1<T> = VersionedMigration<
    0,  // From version
    1,  // To version
    v1::MigrateToV1<T>,  // Migration logic
    Pallet<T>,           // Pallet being migrated
    <T as frame_system::Config>::DbWeight
>;
```

---v

### Writing Migration Logic

Migrations implement `UncheckedOnRuntimeUpgrade`:

```rust
pub mod v1 {
    use super::*;

    pub struct MigrateToV1<T>(PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            // Migration logic here
            let translated = v0::OldStorage::<T>::translate(|old| {
                // Transform old format to new
                transform_to_new_format(old)
            });

            T::DbWeight::get().reads_writes(1, 1)
        }
    }
}
```

---v

### Storage Version Management

Always declare your pallet's storage version:

```rust
#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
pub struct Pallet<T>(_);

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
```

This enables:

```rust
// Current version in code
let current = Pallet::<T>::current_storage_version();
// Version stored on-chain
let onchain = Pallet::<T>::on_chain_storage_version();
```

---v

### Testing Migrations with Try-Runtime

```rust
#[cfg(feature = "try-runtime")]
fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
    // Capture state before migration
    let old_state = OldStorage::<T>::get();
    Ok(old_state.encode())
}

#[cfg(feature = "try-runtime")]
fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {
    // Verify migration success
    let old_state: OldValue = Decode::decode(&mut &state[..]).unwrap();
    let new_state = NewStorage::<T>::get();
    ensure!(new_state == transform(old_state), "Migration failed");
    Ok(())
}
```

---

## Multi-Block Migrations

---v

### When to Use Multi-Block Migrations

- Migration touches millions of storage items
- Total weight exceeds block limits

Examples:

- Migrating all account balances

---v

### Multi-Block Migration Implementation

```rust
pub struct LazyMigrationV1<T>(PhantomData<T>);

impl<T: Config> SteppedMigration for LazyMigrationV1<T> {
    type Cursor = u32;  // Last processed key
    type Identifier = MigrationId<16>;

    fn id() -> Self::Identifier {
        MigrationId {
            pallet_id: *b"pallet_example__",
            version_from: 0,
            version_to: 1
        }
    }

    fn step(
        mut cursor: Option<Self::Cursor>,
        meter: &mut WeightMeter,
    ) -> Result<Option<Self::Cursor>, SteppedMigrationError> {
        let required = T::WeightInfo::migration_step();
        if meter.remaining().any_lt(required) {
            return Err(SteppedMigrationError::InsufficientWeight { required });
        }

        // Process as many items as weight allows
        loop {
            if meter.try_consume(required).is_err() {
                break; // Out of weight, return cursor
            }

            let mut iter = if let Some(last_key) = cursor {
                // Resume from where we left off
                OldStorage::<T>::iter_from(last_key)
            } else {
                // Start from beginning
                OldStorage::<T>::iter()
            };

            if let Some((key, old_value)) = iter.next() {
                // Transform and store
                let new_value = transform(old_value);
                NewStorage::<T>::insert(&key, new_value);
                cursor = Some(key);
            } else {
                // Migration complete
                return Ok(None);
            }
        }

        Ok(cursor) // Return cursor to continue next block
    }
}
```

Notes:
https://github.com/paritytech/polkadot-sdk/blob/polkadot-stable2506/substrate/frame/examples/multi-block-migrations/src/migrations/v1/mod.rs

---v

## Setting Migrations in Runtime

---v

### Single Block Migrations in Runtime

Declare single-block migrations in your runtime:

```rust
// Define all single-block migrations
pub type Migrations = (
    pallet_balances::migration::MigrateV0ToV1<Runtime>,
    pallet_staking::migration::MigrateV11ToV12<Runtime>,
    // Add more migrations here
);

// Pass to Executive
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,  // Single-block migrations execute here
>;
```

---v

### Multi-Block Migrations in Runtime

For multi-block migrations, also configure `pallet-migrations`:

```rust
// In construct_runtime!
construct_runtime! {
    System: frame_system,
    Migrations: pallet_migrations,  // Add this pallet
    // ... other pallets
}

// Configure the migrations pallet
impl pallet_migrations::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Migrations = (  // Multi-block migrations here
        big_migration::v1::MigrateMillionItems,
    );
    type ServiceWeight = MbmServiceWeight;
}
```

---

### Migration Utilities

`frame-support` provides helpful utilities:

```rust
// Remove entire pallet storage
type RemoveOldPallet = frame_support::migrations::RemovePallet<
    OldPalletName,
    Weight
>;

// Access removed storage
#[storage_alias]
type OldStorage<T> = StorageValue<OldPallet, OldType>;

// Transform storage in-place
MyStorage::<T>::translate(|old: OldType| -> NewType {
    transform(old)
});
```

---

## Testing Upgrades with Try-Runtime

---v

### Try-Runtime Overview

`try-runtime` is Substrate's testing framework for runtime upgrades:

- Test migrations against real chain state
- Verify state consistency
- And more...

```bash
# Install try-runtime CLI
cargo install --git https://github.com/paritytech/try-runtime-cli --locked
```

---v

### Try-Runtime Commands

Test your migration against a live chain:

```bash
# Test runtime upgrade
try-runtime \
    --runtime runtime.wasm \
    on-runtime-upgrade \
    --checks all \
    live --uri wss://rpc.polkadot.io
```

---v

### Remote Externalities

Load live chain state for testing:

```rust
let mut ext = Builder::<Block>::new()
    .mode(Mode::Online(OnlineConfig {
        transport: "wss://rpc.polkadot.io".to_string(),
        pallets: vec!["System", "Balances"],
        ..Default::default()
    }))
    .build()
    .await?;

// Run tests against live state
ext.execute_with(|| {
    // Your test code here
});
```

---v

### Try-Runtime Hooks

Ensure runtime invariants still holds

```rust
#[cfg(feature = "try-runtime")]
fn try_state(_: BlockNumber) -> Result<(), TryRuntimeError> {
    // Check pallet invariants
    ensure!(
        SomeCounter::<T>::get() == SomeMap::<T>::iter().count(),
        "Some values are inconsistent"
    );
    Ok(())
}
```

---v

### Migration Testing Workflow

1. Write properly benchmarked migration with try-runtime hooks
2. Test locally
3. Test against live state

---v

### Best Practices

- Always implement `pre_upgrade` and `post_upgrade`
- Use `try_state` to verify invariants
- Test against multiple networks (dev, testnet, mainnet)

---

## Additional Resources 😋

> Check speaker notes (click "s" 😉)

Notes:
A Great talk about try-runtime and further testing of your runtime: https://www.youtube.com/watch?v=a_u3KMG-n-I
