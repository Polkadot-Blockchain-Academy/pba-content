---
title: XCVM
description: Learning about the XCVM state machine
duration: 1 hour
---

# XCVM

---

## 🫀 The XCVM

At the core of XCM lies the **Cross-Consensus Virtual Machine (XCVM)**.

A “message” in XCM is an XCVM program.

The XCVM is a state machine, state is kept track in **Registers**.

Notes:

It’s an ultra-high level non-Turing-complete computer whose instructions are designed to be roughly at the same level as transactions.
Messages are one or more XCM instructions.
The program executes until it either runs to the end or hits an error, at which point it finishes up and halts.
An XCM executor following the XCVM specification is provided by Parity, and it can be extended or customized, or even ignored altogether and users can create their own construct that follows the XCVM spec.

---

### XCVM Instructions

XCM Instructions might change a register, they might change the state of the consensus system or both.

One example of such an instruction would be `TransferAsset` which is used to transfer an asset to some other address on the remote system.
It needs to be told which asset(s) to transfer and to whom/where the asset is to be transferred.

<br>

```rust
enum Instruction {
    TransferAsset {
        assets: MultiAssets,
        beneficiary: MultiLocation,
    }
    /* snip */
}
```

---

## XCVM Instructions preview

Four kinds of instructions:

<pba-flex center>

- Instruction
- Trusted Indication
- Information
- System Notification

</pba-flex>

Notes:

`Instruction` is a bad name for the kind of XCVM instructions that we have, but it means instructions that result in a state change in the local consensus system, or instruct the local consensus system to achieve some desired behavior.

TODO example of XCM message that intuitively makes sense for students that can reason about assets and fees, highlight lines in code block and talk to them.
Highlight LOCATION and ASSET instructions, that we will go into next

---

## XCVM Registers

```rust
pub struct XcmExecutor<Config: config::Config> {
    holding: Assets,
    holding_limit: usize,
    context: XcmContext,
    original_origin: MultiLocation,
    trader: Config::Trader,
    error: Option<(u32, XcmError)>,
    total_surplus: u64,
    total_refunded: u64,
    error_handler: Xcm<Config::Call>,
    error_handler_weight: u64,
    appendix: Xcm<Config::Call>,
    appendix_weight: u64,
    transact_status: MaybeErrorCode,
    fees_mode: FeesMode,
    topic: Option<[u8; 32]>,
    _config: PhantomData<Config>,
}
```

- Registers _are_ the state of XCVM
- Note that XCVM registers are temporary/transient

---

## Basic XCVM Operation

XCVM operates as a fetch-dispatch loop

_Common in state machines_

Notes:

TODO: Graphics about a state machine similar to how the XCVM operates

---

## XCM vs. Standard State Machine

<pba-flex center>

1. Error register
1. Error _handler_ register
1. Appendix register

Notes:

1. The error register is _not_ cleared when an XCM program completes successfully.
   This allows the code in the Appendix register to use its value.
1. Code that is run in the case where the XCM program fails or errors.
   Regardless of the result, when the program completes, the error handler register is cleared.
   This ensures that error handling logic from a previous program does not affect any appended code (i.e. the code in the error handler register does not loop infinitely, the code in the Appendix register cannot access the result of the code execution in the error handler).
1. Code that is run regardless of the execution result of the XCM program.

---

### 📍 The Origin Register

Contains the `Multilocation` of the cross-consensus origin where the XCM originated from.

It is always the relative view from the consensus system in which the XCM is executed.

Notes:

TODO: should there be 2 columns for this slide and the other registers? (from Nuke)

---

### 💁 The Holding Register

Expresses a number of assets in control of the xcm-execution but that have no representation on-chain.

It can be seen as the register holding "unspent assets".

Example: Let’s take a look at another XCM instruction: `WithdrawAsset`: it withdraws some assets from the account of the place specified in the Origin Register.
But what does it do with them? — if they don’t get deposited anywhere then it’s surely a pretty useless operation.
These assets are held in the holding register until they are deposited anywhere else.

---

### 💁 XCM by example: The `WithdrawAsset` instruction

`WithdrawAsset` has no location specified for assets.

They are _temporarily_ held in what in the Holding Register.

```rust
// There are a number of instructions
// which place assets on the Holding Register.
// One very simple one is the
// `WithdrawAsset` instruction.

enum Instruction {
   WithdrawAsset(MultiAssets),
}
```

---

### 💁 XCM by example: The `DepositAsset` instruction

<pba-cols>
<pba-col>

Takes assets from the holding register and deposits them in a beneficiary.

Typically an instruction that places assets into the holding register would have been executed.

</pba-col>
<pba-col>

```rust
// There are a number of instructions
// which operate on the Holding Register.
// One very simple one is the
// `DepositAsset` instruction.

enum Instruction {
    DepositAsset {
        assets: MultiAssetFilter,
        max_assets: u32,
        beneficiary: MultiLocation,
    },
    /* snip */
}
```

</pba-col>
</pba-cols>

Notes:

TODO: this slide looks right, see above todo (from Nuke)

---

### 💁 XCM by example: The `Transact` instruction

<pba-cols>
<pba-col>

Executes a scale-encoded transaction.

It dispatches from a FRAME origin derived from the origin register.

OriginKind defines the type of FRAME origin that should be derived: _root_, _signed_, _parachain_..

</pba-col>
<pba-col>

```rust
// Transact allows to execute arbitrary
// calls in a chain. It is the most generic
// instruction, as it allows the interaction
// with any runtime pallet
enum Instruction {
    Transact {
		origin_type: OriginKind,
		require_weight_at_most: u64,
		call: DoubleEncoded<RuntimeCall>,
	},
    /* snip */
}
```

</pba-col>
</pba-cols>

---

### 💁 XCM by example: The `ClearOrigin` instruction

<pba-cols>
<pba-col>

It clears the origin stored in the origin register.

Useful to execute subsequent messages without a potentially-abusable origin.

Example: we withdraw assets from a parachain controlled account, but then we don't want Transact to be executed

</pba-col>
<pba-col>

```rust
// Clear Origin is key to maintain isolation
// between instructions that are executed
// with a particular origin and instructions
// that are not.
enum Instruction {
    ClearOrigin
    /* snip */
}
```

</pba-col>
</pba-cols>
