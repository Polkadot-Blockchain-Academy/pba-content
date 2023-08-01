---
title: XCVM
description: Learning about the XCVM state machine
duration: 1 hour
---

# XCVM

---

# 🫀 The XCVM

At the core of XCM lies the **Cross-Consensus Virtual Machine (XCVM)**.

A “message” in XCM is an XCVM program, which is a sequence of instructions.

The XCVM is a state machine, state is kept track in **registers**.

Notes:

It’s an ultra-high level non-Turing-complete computer.
Messages are one or more XCM instructions.
The program executes until it either runs to the end or hits an error, at which point it finishes up and halts.
An XCM executor following the XCVM specification is provided by Parity, and it can be extended or customized, or even ignored altogether and users can create their own construct that follows the XCVM spec.

---

# 📜 XCVM Instructions

XCVM Instructions might change a register, they might change the state of the consensus system or both.

---v

## Kinds of instructions

<pba-flex center>

- Command
- Trusted Indication
- Information
- System Notification

---v

## Example: TransferAsset

An instruction used to transfer assets to some other address.

<pba-flex center>

```rust
TransferAsset {
    assets: Assets,
    beneficiary: Location,
}
```

Notes:

This instruction is a command.
It needs to know which assets to transfer and to which account to transfer them to.

---

# XCVM Registers

<diagram class="mermaid">
graph LR
    subgraph Registers[ ]
        Holding(Holding)
        Origin(Origin)
        More(...)
    end
</diagram>

Notes:

Registers _are_ the state of XCVM.
Note that they are temporary/transient.
We'll talk about are the `holding` and `origin` registers, but there are more.

---v

## 📍 The Origin Register

Contains the `Location` of the cross-consensus origin where the XCM originated from.

Notes:

This `Location` can change over the course of program execution.

---v

### 💸 The Holding Register

Expresses a number of assets in control of the xcm execution that have no on-chain representation.

They don't belong to any account.

It can be seen as the register holding "unspent assets".

---

# Basic XCVM Operation

<diagram class="mermaid">
graph LR
    subgraph Program
        WithdrawAsset-->BuyExecution
        BuyExecution-->DepositAsset
        DepositAsset
    end
    Program-.->Fetch
    Fetch(Fetch Instruction)-->Execute(Execute instruction)
    Execute-->Fetch
    Execute-.->Registers
    subgraph Registers
        Holding(Holding)
        Origin(Origin)
        More(...)
    end
</diagram>

Notes:

The XCVM fetches instruction from the program and executes them one by one.

---v

## XCVM vs. Standard State Machine

<pba-flex center>

1. Error register
2. Error _handler_ register
3. Appendix register

Notes:

1. The error register is _not_ cleared when an XCM program completes successfully.
   This allows the code in the Appendix register to use its value.
2. Code that is run in the case where the XCM program fails or errors.
   Regardless of the result, when the program completes, the error handler register is cleared.
   This ensures that error handling logic from a previous program does not affect any appended code (i.e. the code in the error handler register does not loop infinitely, the code in the Appendix register cannot access the result of the code execution in the error handler).
3. Code that is run regardless of the execution result of the XCM program.

---v

## More complete XCVM operation

<diagram class="mermaid">
graph LR
    subgraph Program
        WithdrawAsset-->BuyExecution
        BuyExecution-->DepositAsset
        DepositAsset
    end
    Program-.->Fetch
    Fetch(Fetch Instruction)-->Execute(Execute instruction)
    Execute-->Fetch
    Execute-.->Registers
    subgraph Registers
        Holding(Holding)
        Origin(Origin)
        ErrorRegister(Error)
        ErrorHandler(Error Handler)
        AppendixRegister(Appendix)
        More(...)
    end
    Execute-- Error -->Error(Error Handler)
    Error-.->ErrorHandler
    Error-.->ErrorRegister
    Error-->Appendix
    Appendix-.->AppendixRegister
    Execute-->Appendix
</diagram>

---

# 💁 XCM by example

---v

## The `WithdrawAsset` instruction

<pba-flex center>

```rust
enum Instruction {
    /* snip */
    WithdrawAsset(Assets),
    /* snip */
}
```

Notes:

There are a number of instructions
which place assets on the Holding Register.
One very simple one is the
`WithdrawAsset` instruction.

It withdraws some assets from the account of the location specified in the Origin Register.
But what does it do with them?
If they don’t get deposited anywhere then it's a pretty useless operation.
These assets are held in the holding register until something is done with them, for example, using the following instruction.

---v

## The `BuyExecution` instruction

<pba-flex center>

```rust
enum Instruction {
    /* snip */
    BuyExecution {
        fees: Asset,
        weight_limit: WeightLimit,
    },
    /* snip */
}
```

Notes:

This instruction uses the specified assets in the Holding register to buy weight for the execution of the following instructions.
It's used in systems that pay fees.

---v

## The `DepositAsset` instruction

<pba-flex center>

```rust
enum Instruction {
    /* snip */
    DepositAsset {
        assets: AssetFilter,
        beneficiary: Location,
    },
    /* snip */
}
```

Notes:

Takes assets from the holding register and deposits them in a beneficiary.
Typically an instruction that places assets into the holding register would have been executed previously.

---v

## Putting it all together

<pba-flex center>

```rust
Xcm(vec![
    WithdrawAsset((Here, amount).into()),
    BuyExecution { fees: (Here, amount).into(), weight_limit: Unlimited },
    DepositAsset { assets: All.into(), beneficiary: AccountId32 { ... }.into() },
])
```
