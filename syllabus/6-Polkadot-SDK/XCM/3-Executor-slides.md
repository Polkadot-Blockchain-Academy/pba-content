---
title: XCM Executor
description: How XCMs are executed
duration: 1 hour
---

# XCM Executor

---v

## What you'll learn

<pba-flex center>

- The XCVM
- XCM Executor

---

# The XCVM

At the core of XCM lies the **Cross-Consensus Virtual Machine (XCVM)**.

The XCVM is a state machine, state is kept track in **registers**.

How the virtual machine should act is written in the XCM [specification](https://github.com/paritytech/xcm-format).

Notes:

It’s an ultra-high level non-Turing-complete computer.
Messages are one or more XCM instructions.
The program executes until it either runs to the end or hits an error, at which point it finishes up and halts.

---v

## XCVM Registers

<pba-flex center>

- Holding
- Origin
- ErrorHandler
- Appendix
- ...

Notes:

Registers are the state of the XCVM.
They are transient, temporary.

---v

## The Origin Register

Contains an `Option<Location>` of the cross-consensus origin where the message originated from.

Notes:

This `Location` can change over the course of program execution.

It might be `None` because some instructions clear the origin register.

---v

## The Holding Register

Expresses a number of assets in control of the xcm execution that have no on-chain representation.

They don't belong to any account.

It can be seen as the register holding "unspent assets".

Notes:

Some instructions, like `WithdrawAsset`, put funds in the holding register.
Other instructions, like `DepositAsset`, take funds out of the holding register.

---v

## Instructions

Instructions might change a register, the state of the consensus system, or both.

---

# XCVM Operation

---v

## Fetch/execute loop

The XCVM operates in a typical fetch/execute loop.

Each instruction is fetched and executed in order.

---v

## Error handler and appendix

Two interesting additions by the XCVM are the error handler and appendix.

They contain more instructions that are executed if there's an error or unconditionally
at the end of execution, accordingly.

---

# XCM Executor

Parity provides an implementation of the XCVM spec, the `XcmExecutor`.

It's the entrypoint for executing XCMs.

---v

<pba-flex center>

```rust
XcmExecutor::prepare_and_execute(
  origin,
  xcm,
  &mut hash,
  weight_limit,
  weight_credit,
);
```

---v

## Configuration

The `XcmExecutor` is highly configurable, so it can be implemented by many different chains
just adjusting some configuration items.

These items provide the executor access to the chain's runtime.

This is, for example, how different chains can route different asset instructions to different
pallets (pallet-balances vs pallet-assets).

Notes:

The executor's state is transient, so any state changes to the underlying system need to happen in these configuration items.

---

# Summary

<pba-flex center>

- XCVM
- XCM Executor

---

# Workshop

We'll create our own very simple implementation of the XCVM.

---

# Next steps

How does the XCM subsystem interact with FRAME.
