---
title: Introduction to Cross-Consensus Messaging (XCM)
description: What it is, why it's necessary, what we'll cover
duration: 45 minutes
---

# XCM module

---

## On this module, we'll see

- What is XCM?
- What primitives make up XCMs
- How XCMs are executed
- The XCM Pallet
- How to configure systems to work with XCM
- How to simulate messages on a network

---

# The problem

Substrate and FRAME encourage an ecosystem of many blockchains.

We don't want them to be general.

E.g. assets, swaps, governance, staking, identity, multisigs, etc.

---v

## Specialization

We want them to specialize.

---

# The solution

We can pass messages from one chain to another.

What type of messages?

---v

## Native messages

The messages could just be the native transactions of each chain.

---v

## Native messages downsides

- Format changes from system to system, it could also change within the same system, e.g. on a runtime upgrade.
- Common cross-consensus use-cases don't map one-to-one to a single transaction.
- Different consensus systems have different assumptions e.g. fee payment.

Notes:

- A system which intends to send messages to more than one destination would need to understand how to author a message for each.
  On that note, even a single destination may alter its native transaction/message format over time.
  Smart contracts might get upgrades, blockchains might introduce new features or alter existing ones and in doing so change their transaction format.
- Special tricks may be required to withdraw funds, exchange them and then deposit the result all inside a single transaction.
  Onward notifications of transfers, needed for a coherent reserve-asset framework, do not exist in chains unaware of others.
  Some use-cases don't require accounts.
- Some systems assume that fee payment had already been negotiated, while some do not.

---v

### Message format changes

<img style="width: 1050px;" src="../img/against-native-messaging.svg" />

---v

### Message format changes

<img style="width: 1050px;" src="../img/against-native-messaging-2.svg" />

---v

### Message format changes

<img rounded style="width: 1050px" src="../img/xcm-executor-routing-calls.png" />

Notes:

XCM abstracts away the actual on-chain operation that will be called, which lets the recipient redirect calls to always make them valid.

---v

### No one-to-one mapping

<diagram class="mermaid limit size-50">
graph TD
    subgraph Message
        WithdrawAsset(WithdrawAsset)-->DepositAlice("DepositAsset(Alice)")
        DepositAlice-->DepositBob("DepositAsset(Bob)")
    end
</diagram>

Notes:

You might want to withdraw some assets and deposit some amount to one account and another to another.
Using transactions, you'd have to send many messages to achieve this.

---v


### Different assumptions

<diagram class="mermaid">
graph LR
    A(Chain A)--"Pays for fees"-->B(Chain B)
    A--"Doesn't pay for fees"-->C(Chain C)
</diagram>

Notes:

Different systems have different assumptions.
Using native messages, you'd have to tailor your messages to all systems you want to message.

---v

## A better solution

A domain-specific language that is agnostic to the underlying chain implementing it.

Able to express multiple actions in a single message.

Flexible enough to accomodate multiple different paradigms.

---

# XCM

> XCM is a **language** for communicating **intentions** between **consensus systems**.

---v

## Language

Cross-chain DSL. Each message is a script that allows expressing multiple actions.

---v

## Intentions

Because blockchains are sovereign entities, the language only expresses what we _want_
the receiver to do, it doesn't enforce it.

Verification can be done on another layer, e.g. checking the expected result with a light client.

Notes:

It's up to the receiver to interpret the message and execute the corresponding actions.

Not every chain might implement every intention, for example swaps.

---v

## Consensus systems

We extend the actors to be consensus systems, not only chains.

This includes smart contracts and any system that achieves consensus in some way.

Notes:

Proof-of-authority web2 systems can also be considered consensus systems.

---v

## Versioned

XCM is a **versioned** language.

Each system declares what version they support.

It's currently in version 4.

What goes in each version is defined via an RFC process.

Notes:

This protects against runtime upgrades breaking everything.

---

# ✉️ A Format, not a Protocol

XCM is a **_messaging format_**.

It is akin to the post card from the post office.

It is _not_ a messaging protocol!

A post card doesn't send itself!

Notes:

It cannot be used to actually "send" any message between systems; its utility is only in expressing what should be done by the receiver.
Like many aspects core to Substrate, this separation of concerns empowers us to be far more generic and enable much more.
A post card relies on the postal service to get itself sent towards its receivers, and that is what a messaging protocol does.

The transport layer concerns itself with sending arbitrary blobs, it doesn't care about the format.
A common format has its benefits though, as we'll see next.

---

### Terminology: XCMs

**XCM**, Cross-Consensus Messaging, is the format.

**An XCM** is a Cross-Consensus Message.

It's not called an XCM message,

the same way it's not called an ATM machine.

We can also call them XCM programs, since they are executable.

Notes:

More about their executable nature later.

---

# Summary

We learned what XCM is and why it's useful.

---

# Next steps

We'll learn about the primitives that make up any XCMs.
