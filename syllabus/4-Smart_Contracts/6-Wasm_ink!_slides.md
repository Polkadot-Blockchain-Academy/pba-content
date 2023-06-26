---
title: Wasm smart contracts and the ink! language
description: Overview and history of the wasm contracts platform and languages that compile to it. Focus on architecting real-world smart contracts.
---

# Wasm Contracts

---

# History of Pallet Contracts

Tried to be an improvement over EVM.
Experiemented with storage rent and tombstones etc.

---v

## Other wasm contracting platforms

Compare and contrast to cosmwasm and others (if there are others?)

---v

## Turing completeness and gas metering

---v

## Idiosyncracies

* Are there any?

---

# Programming Wasm Contracts

---v

## WAT and manual assembly

show an example

---v

## High Level Languages

* ink!
* ask!
* There could be plenty of others.

---

# ink!

Based on Rust
... TODO

---v

## Semantics

IDK if this is really necessary. Up to the instructor.

---v

## Dev Environment

Make it clear that students should have these tools installed or available already or should be doing their aboslute best to install them as you go. They will need these tools immenently.

* contracts-ui
* DRink?
* Polkadot js?
* ink-playgroud?

@piotr, @filip, I'll leave it largely up to you what the standard dev environment should be. It is good to be flexible and let students use the tools they like. But many students will have no prior preference or experience, and we need to be able to recommend a fully concrete stack for them.

---v

## Flipper Example

Code along and explain as you go

---v

## Deployment and interaction with Fliper

---v

## Adder or Multiplier

write, deploy, interact

---v

## Beware Public Information

Show a few bad things that could be done to help develop blockchain thinking models.

* A call that only executes if the proper hard-coded password is passed as a param (insecure, the code and therefore the password is onchain)
* An attempted improvement where the password is not hardcoded. It is passed to the constructor and stored in a private variable. (still insecure. All storage is publicly visible from off-chain.)
* If time permits and students are digging this, try storing a hash in storage and requiring the preimage as a password. This is actually secure for only-call-once functions. But if you intend to call it multiple times, the first call leaks the password publicly.

---

# Ask!

If you are so inclined, you could show a few screenshots or somethig from ask and make the point that there will eventually be more languages, just like there is already solidity and vyper in evm world.

---

# Summary
