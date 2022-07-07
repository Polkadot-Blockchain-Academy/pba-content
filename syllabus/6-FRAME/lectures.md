# New Frame Lectures List

## Pre-requisite:

- sp_runtime stuff

## FRAME Lectures

> Ordering constraint
- call refund >> weight
-

### Essentials

- Intro to FRAME (kian)
  - simple-pallet
  - breeze over system, covers
    - Transient Storage Items
    - Some of the types
  - simple-runtime
  - executive

- Dispatchable, Calls and Transactions
  - Call Encoding (indices)
  - Weight basics

- Frame Storage (shawn)

- Pallet Hooks

- Error
  - nested errors
  - sp_runtime DispatchError
  - PostInfo and WeightRefund

- Events
  - Phases,

- Origin
  - filtering
  - custom origins (system)
  - collective origin
  - example

---

### Essential.. But a bit less.

- Inherents

- Frame metadata

- Benchmarking (shawn)
  - Weight classes

- Frame System: Revisiting and Leftover
  - PalletInfoAccess
  - Account Details: consumers, providers, sufficiency, etc
  - ChainContext, whatever the fuck it is.

- Frame Executive: Revisiting and Leftover
  -

- Testing a Pallet (kian)
  - How to write tests for a pallet.
  - ExtBuilder patter
  - `sp_io::TestExternalities`

### Part 2: Advance Topics

- Pallets that receive/store calls + Dispatch Them
  - `type Call` and how to deal with it.

- Parity Patterns + Best-Practices
  - sanity-checks by Kian
  - ...

- Transactional layers/dispatch (shawn)
- Signed Extensions, ValidateUnsigned TransactionValidity (priority) (kian)
- Digests
- Migration + Try-Runtime (kian)
- Custom RPCs + Runtime APIs (maybe, we will probably deprecate it)
- Pallet Instancing

### Part 3: Pallet Gallery!

- Common Pallets: High level, no detail, as they might be subject to change.
  - Balances
  - Transaction Payment
  - Timestamp
  - Utility
  - MultiSig + Proxy
  - Babe + Aura + Grandpa


### Part 4: Interesting Case Studies (for Q&A Sessions and such)
- Staking, Offchain Workers etc.
-
