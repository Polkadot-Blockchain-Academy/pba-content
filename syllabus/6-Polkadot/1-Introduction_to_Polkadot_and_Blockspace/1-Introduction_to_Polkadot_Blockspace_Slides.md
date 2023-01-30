---
title: Introduction to Polkadot, Parachains, and Blockspace
description: An introduction to the key concepts of Polkadot.
duration: 1 hour
---

# Introduction to Polkadot

---

## What to expect from this Module?

- Polkadot platform and its mechanisms
- Polkadot operating theory and inner workings
- Rationale for Polkadot's architectural choices
- Mechanisms for parachains to interaction

Notes:

- Day 1 will be more theoretical and will focus on Parachains, Sharding, and Governance.
- Day 2 will be more practical, with exercises and workshops.
- Day 3 will cover the staking system, light clients, and conclude with a workshop leading into the XCM module.

---

## What is Polkadot?

> Polkadot is a scalable, heterogeneous, sharded, multi-chain network.

---

## What is Polkadot Trying to Solve?

<pba-flex center>

1. Scalability
1. Interoperability
1. Shared Security

</pba-flex>

---

## The Value of Scalability

Web3 systems supporting 8 billion people.

Blockchains are a key part of the Web3 infrastructure.

---

## Blockchain Scalability Trilemma

<pba-cols>
<pba-col>
<pba-flex center>

1. Security: how much does it _cost_ to attack the network?
2. Scalability: how much work can the network do?
3. Decentralization: how decentralized is the network?

</pba-flex>
</pba-col>
<pba-col>

<img rounded width="800px" src="../assets/scalability-trilemma.svg"/>

</pba-col>
</pba-cols>

---

## Specificity Enhances Scalability

Specialized solutions for problems are more performant than generalized solutions, as they can incorporate more details about the problem domain.

_However, specialized solutions must work in concert with each other in order to provide coherent services._

Notes:

This is the rationale for Polkadot's architecture.

---

## The Value of Interoperability

Enables consensus systems to work together to complete, complex end-to-end scenarios.

_However, any application reliant on multiple blockchains is vulnerable to security issues on any of those blockchains.
The foundations of Web3 infrastructure must be strong._

---

## The Value of Shared Security

Multiple chains are secured by the same underlying resource, PoW (merge-mining) or PoS (tokens at stake).

This allows chains to exist under the same security umbrella.

---

## Shared Security and Economic Efficiency

Reused resources provide security for multiple chains<br>provides economic efficiency.

Fewer resources needed to provide the same amount<br>of security to all chains.

---

## Today's Multi-Chain Problems

<pba-flex center>

- Bootstrapping
- Tokens often provide no utility or value
- Inherent competition of resources

</pba-flex>

---

## The Scaling Problem

When scaling a consensus system,<br>there are three core options:

<pba-flex center>

1. Make execution of blocks more efficient
1. Reduce the number of total nodes
1. Reduce the number of nodes checking each block

</pba-flex>

---

## The Scaling Problem

**(1) Make execution of blocks more efficient**

Specialization becomes important,<br>as well as optimized code execution, disk accesses, etc.

_Limited by capabilities of modern hardware<br>and needs to be coupled with other approaches._

_Relying on highly specialized hardware<br>also introduces centralization risks._

---

## The Scaling Problem

**(2) Reduce the number of total nodes**

Essentially, scale by making the system more centralized altogether.

_This reduces coordination costs while trading off for security._

---

## The Scaling Problem

**(3) Reduce the number of nodes checking each block**

Same total nodes, split to do work in parallel.

_Challenge of maintaining the same level of security and accountability while reducing node's work._

---

## Polkadot's Solution:<br>Parallelized Execution

Polkadot combines approaches (1) and (3) by splitting work across multiple parallelized chains (or **parachains** for short), having those chains specialize for particular use-cases.

Only having a small proportion of nodes check each parachain block in the _optimistic case_.

---

<!-- .slide: data-background-color="#000" -->

# Polkadot Architecture

Notes:

A high level look into the architecture of Polkadot and the actors which maintain the network.

---

## Polkadot: Major Systems

<img rounded background-color="white" width="800px" src="../assets/polkadot-components.svg"/>

---

<img rounded width="700px" src="../assets/polkadot-architecture.svg">

### Polkadot Architecture (Parachains)

---

<img rounded width="900px" src="../assets/polkadot-architecture-simple.png">

Notes:

Simplified Polkadot Architecture (Parachains)

---

## Polkadot Validators

<pba-flex center>

Validators are elected via Nominated-Proof-of-Stake (NPoS) system.

All validators have three key responsibilities:

- Authoring and Consensus of Relay-Chain blocks
- Validation and Security of Parachain blocks
- Transport of messages between Parachain nodes.

</pba-flex>

Good validators are rewarded.

Bad validators will either miss out on rewards<br>or are slashed if their work is neglected or wrong.

---

## Polkadot Nominators

Nominators have one responsibility: to nominate validators which they believe to be trustworthy and reliable.
Nominators do not actively secure the network.

Nominators share in the rewards of the validators they nominate, as well as the risk that their nominees may misbehave.

_When a validator is slashed, its nominators are slashed proportionally._

---

## Polkadot Collators

<pba-cols>
<pba-col>

Collators are nodes which create parachain blocks.

Validators only need to execute parachain blocks.

</pba-col>
<pba-col>

<img width="500px" src="../assets/polkadot-architecture-simple-zoom.png">

</pba-col>
</pba-cols>

---

## The Relay Chain

The relay chain is the "hub" of Polkadot, providing:

<pba-cols>
<pba-col>

<pba-flex center>

- Governance
- Staking
- Registration, scheduling,<br>and advancement of parachains
- Communication between parachains
- Balance Transfers

</pba-flex>
</pba-col>
<pba-col>

Notably, the functionality of the relay chain is minimized,<br>with the expectation that more complex functionalities will be provided by parachains themselves.

</pba-col>
</pba-cols>

---

<img rounded width="900px" src="../assets/polkadot-architecture-simple.png">

Notes:

Simplified Polkadot Architecture (Parachains)

---

## Message Passing

3 Main Protocols:

<pba-flex center>

1. Upward Messages<br>&nbsp;&nbsp;_(parachain ➡ relay chain)_
1. Downward Messages<br>&nbsp;&nbsp;_(relay chain ➡ parachain)_
1. Horizontal Message Passing (HRMP)<br>&nbsp;&nbsp;_(parachain ➡ parachain)_

</pba-flex>

---

<!-- .slide: data-background-color="#000" -->

# Economics of Polkadot

---

## The DOT Token

<pba-cols>
<pba-col>

The DOT Token underpins the security and evolution of the network.

</pba-col>
<pba-col>

DOT Token core utilities:

<pba-flex center>

1. Governance
1. Staking
1. Registering,<br>Activating Parachains

</pba-flex>
</pba-col>
</pba-cols>

---

## Polkadot Governance

Polkadot has on-chain governance by referendum of DOT holders, which empowers DOT holders to coordinate the platform:

<pba-flex center>

- Forkless upgrades of the network
- Administration of the Treasury funds
- Configuration of the Parachains protocol
- Configuration of fees
- Rescue & recovery operations
- All other mechanisms of control over the platform

We will have a lesson on the mechanics of OpenGov & will not cover in detail here.

</pba-flex>

---

## Treasury

Polkadot ensures that a portion of all network fees is collected to the treasury.
The treasury is managed by governance, and tokens are burned if they are not spent.

The intention of the treasury is to pay people to help grow Polkadot itself.
As tokens are burned, this creates pressure to fund public projects.

---

## Transaction Fees

<pba-cols>
<pba-col>

Transactions executed in the relay chain incur fees for inclusion in a block.

Fees automatically adjust based on traffic and demand for transaction inclusion.

</pba-col>
<pba-col>
<pba-flex center>

- 80% of fees to the Treasury.
- 20% of fees to the block producer (authoring validator).
- An optional "tip" to the block producer to increase priority.

</pba-flex>
</pba-col>
</pba-cols>

---

## Parachain Bonding & Registration

Registering a parachain is done by posting the code of the parachain (The "PVF"), the parachain's initial header, and a DOT deposit to the relay chain (The "Bond").

When the parachain is deregistered, the bond is returned.

Scheduling the parachain for execution is done with separate mechanisms after registration.

---

<!-- .slide: data-background-color="#000" -->

# Parachain Mechanics and Scheduling

---

## Anatomy of a Parachain

Parachains are state-transition functions.

The core of a Parachain is the **Parachain Validation Function**, a piece of Wasm code which takes inputs and attempts to validate them to produce outputs.

<br>

<img rounded width="900px" src="../assets/PVF.svg">

---

## Head Data and PoVs

```rust
/// Head Data is stored on the relay-chain, and is an abstraction over
/// blockchain headers. Head Data are limited in size, and are often only
/// 32 bytes long.
struct HeadData(Vec<u8>);
/// The Proof-of-Validity (or PoV for short) is a larger piece of data encapsulating
/// the full block itself, as well as any data needed to execute the block.
struct PoV(Vec<u8>);
```

---

## Validation Parameters

```rust
/// Parameters provided to a PVF for validation
pub struct ValidationParams {
	/// The parent parachain block's Head Data
	pub parent_head: HeadData,
	/// The Proof-of-Validity.
	pub pov: PoV,
	/// The current relay-chain block number.
	pub relay_parent_number: RelayChainBlockNumber,
	/// The relay-chain block's storage root.
	pub relay_parent_storage_root: Hash,
}
```

---

## Validation Outputs

```rust
/// Outputs of _successful_ validation of a parachain block.
pub struct ValidationResult {
	/// New head data that should be included in the relay chain state.
	pub head_data: HeadData,
	/// An update to the validation code that should be scheduled
	/// in the relay chain.
	pub new_validation_code: Option<ValidationCode>,
	/// Upward messages sent by the Parachain.
	pub upward_messages: Vec<UpwardMessage>,
	/// Outbound horizontal messages sent by the parachain.
	pub horizontal_messages: Vec<OutboundHrmpMessage<Id>>,
	/// Number of downward messages that were processed by the Parachain.
	///
	/// It is expected that the Parachain processes them from first to last.
	pub processed_downward_messages: u32,
	/// The mark which specifies the block number up to which
	/// all inbound HRMP messages are processed.
	pub hrmp_watermark: RelayChainBlockNumber,
}
```

---

## Cumulus: Creating a PVF

PVFs are simply Wasm blobs that take in the parameters and provide the outputs if valid.

Substrate Runtimes use **host functions** like storage-reads/writes and signature verifications to perform an `execute_block`.
With a simple wrapper around `execute_block`, a runtime can be transformed into a PVF.
Our framework for doing this is known as **Cumulus**.

It is also possible to create a PVF without using Substrate.

---

## Execution Cores

Polkadot exposes "Execution Cores" which parachains are scheduled onto on a per-block basis.

The amount of execution cores is determined by Governance.
Like a decentralized CPU, Polkadot schedules and executes code in parallel on its cores.

While the amount of parachains which can be registered is bounded only by the DOT deposits bonded, the amount of parachains which can be scheduled and executed at a time is bounded by the work done in the runtime and the coordination cost of validators.

---

## Scheduling Parachains onto Cores

There are many possible mechanisms for scheduling parachains onto cores, but here are a few live & planned mechanisms:

<pba-flex center>

- System Parachains (live: Parachain scheduled directly by Governance)
- Slot Auctions (live: Parachains bid for guaranteed access to a core for long-term duration)
- Parathreads (in development: on-demand, pay-as-you-go access to cores)

</pba-flex>

---

## Parachain Slot Auctions

Auctions are scheduled by Governance.

The winner of an auction earns guaranteed access to a core for a medium-to-long duration: 6, 12, 18, or 24 months.
Bidders bid on behalf of parachains, and bid to _lock up_ DOT tokens, which are returned after the lease is completed.
Bidders can be either individual bidders, smart contracts, crowdloans, or anything else that can use a `Signed` origin on the relay chain.

---

## System Parachains

System Parachains are scheduled indefinitely and for free by the Governance System.

The expectation is that this mechanism will be used to provide parachains that extend the capabilities of Polkadot itself.
For instance, Staking and Governance could themselves be moved onto system parachains.

---

## Parathreads

With a cloud computing analogy, if Slot Auctions are "reserved instances" then Parathreads are "spot instances".
For chains which don't need to author blocks every 6 or 12 seconds, it is more efficient to buy scheduling on-demand.

Collators will pay a fixed price in DOT for core-time, which goes up and down depending on supply & demand.
Collators will then be reimbursed for their cost in transaction fees, the parachain's token, or other mechanisms at higher layers.

---

## Blockspace

> Blockspace is the capacity of a blockchain to finalize and commit operations

Polkadot's primary product is _blockspace_.

---

## Blockspace

Polkadot aims to provide the most secure blockspace,<br> and be the most efficient allocator of blockspace.

Blockspace is useful for evaluating the offerings<br> of different blockchain platforms on 3 properties:

<pba-flex center>

- Quality
- Availability
- Flexibility

</pba-flex>

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
