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
1. The fungible trait
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

## Scenarios

1) 100 Free; No Hold or Freeze

<img src="https://wiki.polkadot.com/assets/balance-example-1.png">

Notes:
- Free: 100 DOT
- Frozen (locked): 0 DOT
- Reserved (held): 0 DOT
- Spendable: 99 DOT
- Untouchable: 1 DOT (ED)

---v

## Scenarios

2) 60 Staked

<img src="https://wiki.polkadot.com/assets/balance-example-2.png">

Notes:
- Free: 40 DOT
- Frozen (locked) : 0 DOT
- Reserved (held): 60 DOT
- Spendable: 39 DOT (Free - ED)
- Untouchable: 1 DOT (ED)

---v

## Scenarios

3) 20 Proxy Deposit

<img src="https://wiki.polkadot.com/assets/balance-example-3.png">

Notes:
- Free: 100 DOT
- Frozen: 0 DOT
- Reserved (held): 0 DOT
- Spendable: 99 DOT
- Untouchable: 1 DOT (ED)

---v

## Scenarios

4) 20 Vote

<img src="https://wiki.polkadot.com/assets/balance-example-4.png">

Notes:
- Free: 20 DOT
- Frozen (locked): 20 DOT
- Reserved (held): 80 DOT
- Spendable: 19 DOT (Free - ED)
- Untouchable: 1 DOT (ED)

---

## Live Coding
