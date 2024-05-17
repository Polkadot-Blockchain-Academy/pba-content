---
title: Introduction to Polkadot SDK
description: In Introduction to Polkadot SDK
duration: 120 mins
---

# Introduction to Polkadot SDK

Note:

This lecture is about learning two things:

1. Polkadot
2. Polkadot SDK
   1. Substrate
   2. FRAME
   3. XCM

There will be amazing modules going into further details of each. This will be very high level.

---

## Learning Outcomes

1. Understand what Polkadot is as a developer platform.
2. Understand the pieces of the Software Development Kit (SDK) of this platform

---

## Polkadot and its SDK

---

### 1. Polkadot

Polkadot stems from Ethereum. A single threaded smart contract blockchain.

Let's start with Ethereum.

Let's make it sharded, while sharing security. Execution sharding, the holy grain of what ETH 2.0
was meant to be.

Let's make it heterogenous. The state transition function of these shards need NOT be the same. WASM
is used.

Let's make the deployment of those shards flexible and permissionless. We arrive at Polkadot what it
is today.

> Polkadot 1.0 is a platform to launch blockchains that can progress in parallel, whilst sharing the
> security of Polkadot.

In this architecture, we call Polkadot the Relay Chain. we call each of these blockchains a
Parachain.

Each computation unit of the Relay Chain that is capable of progressing a parachain block in
parallel was called a Core.

Polkadot initially allowed parachains to compete for deployment on Polkadot via auctioned slots.
Winners get access to a Core for a long long time. Now, it is moving toward a more agile system where
parachains can purchase cores on-demand.

In some sense, Polkadot is moving towards a general purpose computer, exposing its Core as the
primary primitive. Whether they are used to build a Parachain or not, it does not care. The future
of Polkadot is the World Computer vision.

### 2. How Do We Build a Parachain?

Substrate and FRAME

Building a blockchain is damn painful hard. This is why Polkadot has pivoted for years to build a
modular and extensible blockchain framework called Substrate.

Substrate has been used to build:

1. the mighty Polkadot Relay Chain
2. all the Polkadot Parachains to date.
3. (less known) can be used to build any blockchain unrelated to Polkadot (solo-chain).

> There is another framework involved called Cumulus, but since it has a very small footprint, we
> will skim over it here. Cumulus augments Substrate to make it Polkadot-compatible.

FRAME is a subset of blockchain within Substrate that allow you to focus only on writing the
state transition function aka. Runtime of your blockchain and not deal with the rest of the
software. WASM.

### 3. Communication

XCM

Shared Security is GREAT, but it would be even more FAN-TAS-TIQUE is if these parachains could
communicate with each other. Why is that? **Shared security puts communication on steroids**.

https://x.com/TheDotsMagazine/status/1790755070490857778

Chains can trust the execution of each other to a much higher extent. But note that it is still not
100% guarantee. The Relay Chain cannot guarantee that a parachain won't upgrade itself to go rogue.

Moreover, the sender and recipient of these messages can be contracts, accounts, or any other
abstract entities within chains, not just the chain itself. "Consensus System".

Polkadot Relay Chain provides:

1. Two communication protocols for "Consensus Systems" to send payloads to one another (the
   telephone line).
   1. XCMP/HRMP
   2. VPM (UMP, DMP)
2. One language to compose messages and programs and send them over.
   1. XCM

Async and Sync: Disjoint consensus systems are in principle[^1] asynchronous by nature. This is an
important concept to keep in mind.


[^1]: Technically, if two parachains are co-scheduled in the same core in the same timeslot, they
    could have synchronous communication. But they cannot count on this. So at the programming
    model, one must assume everything is always asynchronous.

### 4. Other

All of the mentioned is a subset of what we call "Polkadot SDK for onchain development". Other components

1. Offchain Tooling: PAPI, PJS, SubXT, py-substrate-interface
2. Smart Contracts
   1. Execution Engines: PolkaVM, EVM, WASM
   2. Languages: Ink!, Solidity

### 5. Resources

A short list of good resources to get started.

---

## Lecture Recap

About Polkadot

1. Shared Security, Sharded Execution.
2. Heterogenous Sharded Execution.
3. Parachain
4. Core
5. Exposing Cores: Auctions -> On-Demand -> World Computer

About Polkadot SDK

1. Substrate
2. FRAME
3. XCM

---

## Additional Resources! 😋

> Check speaker notes (click "s" 😉)

<img width="300px" rounded src="../../../assets/img/5-Substrate/thats_all_folks.png" />

https://blog.kianenigma.nl/posts/tech/polkadot-s-build-horizon/
