---
title: Balance Management in FRAME
description: Working with Fungible Tokens in FRAME pallets
duration: 2 hours
---

# Balance Management in FRAME

---

## Why?

Working with balances is fundamental to blockchain development:

- Every pallet interacts with tokens in some way
- Common use-case: staking, voting, deposits
- Critical to get right for security and correctness

---

## Agenda

<pba-flex center>

1. Core concepts: Existential Deposit
1. Core concepts: Holds vs Freezes
1. Fungible traits
1. Considerations
1. Live Coding

</pba-flex>

---

## Why Existential Deposit

- Account occupies some state in storage.
- ED forces accounts to keep a minimum balance, or dusted.

<br>

What would happen if we do not enforce ED? <!-- .element: class="fragment" -->

---

## What is Fungible?

> An asset where each unit is interchangeable and has the same value as any other unit of the same kind.

---v

## What is Fungible?

The fungible traits in FRAME provide a safe, standardized way to handle balances in the pallet.

---

## Note on Legacy Code

<pba-flex center>

You might encounter these in older codebases:

```rust
use frame_support::traits::{Currency, ReservableCurrency, LockableCurrency};
```

</pba-flex>

---v

## Note on Legacy Code

<pba-flex center>

These traits are deprecated. For new code:

- Use the modern `fungible` traits
- They provide better safety and clearer semantics
- We'll focus exclusively on the modern approach

</pba-flex>

---

## Core Concepts: Balance Arithmetic

<pba-flex center>

```text
┌─────────────────────────────────────┐
│         Total Balance               │
│  ┌─────────────┬─────────────────┐  │
│  │   Free      │     Held        │  │
│  │  Balance    │    Balance      │  │
│  └─────────────┴─────────────────┘  │
└─────────────────────────────────────┘

Spendable = free - max(frozen - reserved, ED)
```

</pba-flex>

Notes:
https://wiki.polkadot.com/learn/learn-account-balances/

---

## Mental Model

- Holds are taken out of total balance
- Freezes determine minimum total balance account needs.

---

## Holds vs Freezes

<pba-cols>
<pba-col>

### Holds 📌

- **Stack** (cumulative)
- **Slashable**
- **Named reasons**
- **Taken out of free balance**

Use for:

- Staking bonds
- Storage deposits
- Collateral

</pba-col>
<pba-col>

### Freezes 🧊

- **Overlap** (take maximum)
- **Not slashable**
- **Named reasons**
- **Total balance cannot drop below this.**

Use for:

- Voting locks
- Vesting schedules
- Minimum balance requirements

</pba-col>
</pba-cols>

---

## Examples

Notes:
https://wiki.polkadot.com/learn/learn-account-balances/
And PJS.

---

## Providers & Consumers

- Providers: Reasons this account SHOULD exist.
  - Account has money (ED).
  - Pot Account.
- Consumers: Modules currently USING this account.
  - Prevents accounts from being reaped until this is ZERO.

---

<pba-cols>
<pba-col>

## Fungible::Inspect

- Inspect balance of an account.

</pba-col>

<pba-col>

## Fungible::Mutate

- Subtrait of Inspect.
- Mint
- Transfer

</pba-col>

</pba-cols>

Notes:
https://github.com/paritytech/polkadot-sdk/blob/3dfbdf4a454f35238500779e503e1ec32ba7fc63/substrate/frame/support/src/traits/tokens/fungible/regular.rs#L46

---

<pba-cols>
<pba-col>

## InspectHold

- Inspect holds on an account.

</pba-col>
<pba-col>

## MutateHold

- Allows placing a fungible asset on Hold.
- Requires a named reason.

</pba-col>
</pba-cols>

Notes:
https://github.com/paritytech/polkadot-sdk/blob/3dfbdf4a454f35238500779e503e1ec32ba7fc63/substrate/frame/support/src/traits/tokens/fungible/hold.rs#L44

---

## Using Holds in your Pallet

```rust [3-5|9-12|16-22|29-30|37-38]
#[frame_support::pallet(dev_mode)]
pub mod pallet {
  // This trait provides apis to mutate holds on balances. It is a subtrait
  // of (regular) `Mutate` and `fungible::InspectHold`.
  use frame_support::traits::fungible::MutateHold;

#[pallet::config]
pub trait Config: frame_system::Config {
    /// Type that can mutate balance and place holds on it.
    type NativeBalance: fungible::Mutate<Self::AccountId>;
    /// Overarching hold-reason type. Must be constructible from `HoldReason`.
    type RuntimeHoldReason: From<HoldReason>;
    // -- snip --
}

/// The reasons for this pallet to place a hold on an account's balance.
#[pallet::composite_enum]
	pub enum HoldReason {
		/// Funds on stake by a nominator or a validator.
		#[codec(index = 0)]
		Staking,
	}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::call_index(0)]
    pub fn stake(origin: OriginFor<T>, value: BalanceOf<T>) -> DispatchResult {
      let who = ensure_signed(origin)?;
      // Place a hold
      <T::NativeBalance as MutateHold<T::AccountId>>::hold(&HoldReason::Stake.into(), &who, value)?;
      Ok(())
    }

    #[pallet::call_index(1)]
    pub fn unstake(origin: OriginFor<T>, value: BalanceOf<T>) -> DispatchResult {
      let who = ensure_signed(origin)?;
      // Release some of the hold
      <T::NativeBalance as MutateHold<T::AccountId>>::release(&HoldReason::Stake.into(), &who, value, Precision::BestEffort)?;
      Ok(())
    }
  }
}
```

Notes:
https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/attr.composite_enum.html

---v

## Configuring your Runtime for Holds

```rust [1-7|10-11|20-21|]
impl pallet_mypallet::Config for Test {
    /// Provide something that implements `MutateHold`.
	type NativeBalance = Balances;
    /// The overarching hold-reason type. Your pallet's `HoldReason` converts into this
    /// runtime wide type.
	type RuntimeHoldReason = RuntimeHoldReason;
}

impl pallet_balances::Config for Test {
    /// The overarching hold-reason type.
    type RuntimeHoldReason = RuntimeHoldReason;
    // -- snip --
}

#[frame_support::runtime]
mod runtime {
  /// The main runtime type.
  #[runtime::runtime]
  #[runtime::derive(
    // Derive the runtime-wide hold reason enum.
    RuntimeHoldReason,
    // -- snip --
  )]
  pub struct Test;

  // -- snip --

  /// The Balances pallet.
  #[runtime::pallet_index(1)]
  pub type Balances = pallet_balances::Pallet<Runtime>;

  /// Your pallet.
  #[runtime::pallet_index(2)]
  pub type MyPallet = pallet_mypallet::Pallet<Runtime>;
}
```

---

## Freeze

- Fungible::InspectFreeze
- Fungible::MutateFreeze

---

## Considerations

> Some sort of cost taken from account temporarily in order to offset the cost to the chain of holding some data **footprint** in state.

---v

## What is a Footprint

The resource footprint of a bunch of blobs

Note:
https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/struct.Footprint.html

---v

## Convert<Footprint, Balance>

We need a way to convert Footprint into a storage fee.
Frame support provides two implementations:

- LinearStoragePrice
- ConstantStoragePrice

Or, implement your own.

Notes:
Implementations in substrate/frame/support/src/traits/storage.rs

---

## Consideration Apis

- New: Create a ticket (create hold/freeze).
- Update: Update ticket (when footprint changes).
- Drop: Remove ticket (drop any hold/freeze).

---

## Using Consideration in your Pallet

```rust [9-10|13-18|20-23|28-33|41-43|]
#[frame_support::pallet]
pub mod pallet {
  use frame_support::traits::{Consideration, Footprint};

  pub type TicketOf<T> = <T as Config>::Consideration;

  #[pallet::config]
  pub trait Config: frame_system::Config {
    /// A means of providing some cost while data is stored on-chain.
    type Consideration: Consideration<Self::AccountId, Footprint>;
  }

  /// A reason for this pallet placing a `HoldConsideration` on funds.
  #[pallet::composite_enum]
  pub enum HoldReason {
    /// The funds are held as storage deposit.
    MyHoldReason,
  }

  /// Storage for tickets issued by this pallet.
  #[pallet::storage]
  pub type Tickets<T: Config> = StorageMap<_, Blake2_128, T::AccountId, TicketOf<T>>;

  impl<T: Config> Pallet<T> {
    #[pallet::call_index(0)]
    pub fn new(origin: OriginFor<T>, items: u64, size: u64) -> DispatchResult {
      let who = ensure_signed(origin)?;
      // Create a footprint for the data we are storing.
      let footprint = Footprint::from_parts(items as usize, size as usize);
      // Create a ticket to pay for the storage. This will create a hold.
      let ticket = T::Consideration::new(&who, footprint)?;
      // store the ticket in your pallet's storage for later use.
      Tickets::<T>::insert(who, ticket);
      Ok(())
    }

    #[pallet::call_index(1)]
    pub fn drop(origin: OriginFor<T>) -> DispatchResult {
      let who = ensure_signed(origin)?;
      // Remove the ticket if it exists. This will drop the hold as well.
      if let Some(ticket) = Tickets::<T>::take(&who) {
        ticket.drop(&who)?;
      }

      Ok(())
    }
  }
}
```

---v

## Configuring Runtime to use Consideration

```rust [10-16|]
parameter_types! {
    /// Fixed cost floor.
	pub const MyBaseDeposit: Balance = 2;
    /// Price per byte of storage.
	pub const MyByteDeposit: Balance = 1;
	pub const MyHoldReason: RuntimeHoldReason = RuntimeHoldReason::MyPallet(pallet_mypallet::HoldReason::MyHoldReason);
}

impl pallet_mypallet::Config for Test {
    /// Provide something that implements `Consideration`.
    type Consideration = HoldConsideration<
        AccountId,
        Balances,
        MyHoldReason,
        LinearStoragePrice<MyBaseDeposit, MyByteDeposit, Balance>,
    >;
}

impl pallet_balances::Config for Test {
    /// The overarching hold-reason type.
    type RuntimeHoldReason = RuntimeHoldReason;
    // -- snip --
}

#[frame_support::runtime]
mod runtime {
  /// The main runtime type.
  #[runtime::runtime]
  #[runtime::derive(
    // Derive the runtime-wide hold reason enum.
    RuntimeHoldReason,
    // -- snip --
  )]
  pub struct Test;

  // -- snip --

  /// The Balances pallet.
  #[runtime::pallet_index(1)]
  pub type Balances = pallet_balances::Pallet<Runtime>;

  /// Your pallet.
  #[runtime::pallet_index(2)]
  pub type MyPallet = pallet_mypallet::Pallet<Runtime>;
}
```
