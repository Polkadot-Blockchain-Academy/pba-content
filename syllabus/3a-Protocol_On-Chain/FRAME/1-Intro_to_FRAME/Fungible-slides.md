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

## Fungible::Inspect

- Balance Inspection
- Reducible Balance

Notes:
https://github.com/paritytech/polkadot-sdk/blob/3dfbdf4a454f35238500779e503e1ec32ba7fc63/substrate/frame/support/src/traits/tokens/fungible/regular.rs#L46

---

## Fungible::Mutate

- Extends Inspect.
- Mint
- Transfer

---

## Fungible::InspectHold

- Hold Inspection

Notes:
https://github.com/paritytech/polkadot-sdk/blob/3dfbdf4a454f35238500779e503e1ec32ba7fc63/substrate/frame/support/src/traits/tokens/fungible/hold.rs#L44

---

## Fungible::MutateHold

- Allows placing a fungible asset on Hold.
- Requires a named reason.

---v

## Fungible::MutateHold

```rust
#[pallet::composite_enum]
	pub enum HoldReason {
		/// Funds on stake by a nominator or a validator.
		#[codec(index = 0)]
		Staking,
	}
```

Notes:
https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/attr.composite_enum.html

---

## Fungible::InspectFreeze

---

## Fungible::MutateFreeze

---

## Considerations

> Some sort of cost taken from account temporarily in order to offset the cost to the chain of holding some data footprint in state.

---v

## Footprint

- Number of blobs
- Size of blobs

Note:
https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/struct.Footprint.html

---v

## Footprint -> Balance

- LinearStoragePrice
- ConstantStoragePrice

Notes:
Implementations in substrate/frame/support/src/traits/storage.rs

---

## Consideration Trait

- New
- Update
- Drop

---

## Example Usage

- PreImage pallet

---
