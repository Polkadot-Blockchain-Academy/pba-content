---
title: EVM, Solidity, and Vyper
description: Overview and history of the EVM and languages that compile to it.
  Focus on architecting real-world smart contracts.
---

# EVM

---

# History of EVM

---v

## Ethereum as first smart contracting platform

---v

## Turing completeness and gas metering

---v

## Idiosyncrasies

- Everything is 256bits
- I guess there are more?

---

# Programming the EVM

---v

## EVM Assembly

show an example

---v

## High Level Languages

- Solidity
- Vyper

---

# Solidity

Inspired by java
Been around a long time
... FIXME TODO

---v

## Semantics

FIXME TODO: IDK if this is really necessary.
Up to the instructor.

---v

## Dev Environment

Make it clear that students should have these tools installed already or should be doing their absolute best to install them as you go. They will need these tools immanently.

- Remix
- Metamask?
- Polkadot js?

Notes:

FIXME TODO: @notlesh, I'll leave it largely up to you what the standard dev environment should be. It is good to be flexible and let students use the tools they like. But many students will have no prior preference or experience, and we need to be able to recommend a fully concrete stack for them.

---v

## Flipper Example

Code along and explain as you go

---v

## Deployment and interaction with Flipper

---v

## Adder or Multiplier

write, deploy, interact

---v

## Beware Public Information

Show a few bad things that could be done to help develop blockchain thinking models.

- A call that only executes if the proper hard-coded password is passed as a param (insecure, the code and therefore the password is onchain)
- An attempted improvement where the password is not hardcoded. It is passed to the constructor and stored in a private variable. (still insecure. All storage is publicly visible from off-chain.)
- If time permits and students are digging this, try storing a hash in storage and requiring the presage as a password. This is actually secure for only-call-once functions. But if you intend to call it multiple times, the first call leaks the password publicly.

---

# Vyper

TODO Continue analogously to Solidity lesson.
Explain pythonic nature
Explain focus on safety (at least compared to solidity)
Show similar coding examples

---

# Summary
