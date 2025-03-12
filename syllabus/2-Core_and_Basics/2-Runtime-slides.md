---
title: Runtime
description: Runtime (AKA State Transition Function(s))
---

# Runtime

---

# Runtime

## What you will learn:

- What is the Runtime?
- Where is located?
- How does it work?
- Tradeoffs of having a dynamic STF.
- Who does it cather to?
- Types of Runtime APIs.

---

# Runtime: Introduction

## Runtime: What is it?

- It's the logic of the State Transition Fuctions that defines the business logic of the chain.
- It currently is WASM code that's stored on the state of the chain.
- It's part of the state, and it also determines how this state changes 🤯.

---

## Runtime: part of the state of the chain.

- Why?
  - In an ideal world it allows for forkless upgrades.<!-- .element: class="fragment" -->

---

## Runtime: part of the state of the chain.

- How does it work

  - Every time that the WASM changes, the node instantiates the new WASM.

- Host Functions:

  - When the WASM is instaniated, the node provides an environment so that the WASM can interact with the node.
  - Eg: Access storage, Expensive cryptografic functions, etc.

- Can a new WASM version require newer/different host functions?

Notes:

- Discuss implications of changing host functions.

---

## Tradeoffs of having a dynamic runtime.

- The good: enables forkless upgrades.
- The not so good:
  - State migrations are complicated.
  - Transactions can:
    - Become invalid.
    - Have a different outcome.
  - Makes DApps and tooling more complicated.

---

## Tradeoffs of having a dynamic runtime.

- The good: enables forkless upgrades.
- The not so good:
  - State migrations are complicated.
  - Transactions can:
    - Become invalid.
    - Have a different outcome.
  - Makes DApps and tooling more complicated.

---

## Runtime functions:

| How it started                              |            How it's going            |
| ------------------------------------------- | :----------------------------------: |
| Core_initialize_block                       |        DryRunApi_dry_run_call        |
| Core_execute_block                          |          Metadata_metadata           |
| Block_builder_apply_extrinsic               | NominationPoolsApi_points_to_balance |
| TaggedTransactionQueue_validate_transaction | Inflation_inflation_prediction_info  |

Notes:

- Discuss why it's a good idea to expose via runtime-apis functions that
  prevent DApps from re-implementing on-chain logic.

## Runtime: who does it cather to?

Runtime functions can be divided in 3 different groups:

- Functions that cather the node: applyng extrinsics, handling blocks, etc.
- Functions that cather DApps: by exposing logic that otherwise would have to be duplicated offchain.
- Functions that cather both: transaction validation transactions, calculating fees, etc.

## Non Host-specific runtime-apis:

- Metadata
- Fees
- Dry-Runs
- View Functions
