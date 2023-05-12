---
title: The Decisions of Polkadot
description: A High Level Review of the Key Decisions of Polkadot
duration: 1 hour
---

# The Decisions of Polkadot

---

## Creating an “Invention Machine”

Jeff Bezos outlined in an annual letter to Amazon shareholders how he approaches decision making, by categorizing decisions as either Type 1 or Type 2 decisions.

---

## Type 1 Decisions

> Some decisions are consequential and irreversible or nearly irreversible – <span style="color:#d92f78">**one-way doors**</span> – and these decisions must be made methodically, carefully, slowly, with great deliberation and consultation. If you walk through and don't like what you see on the other side, you can't get back to where you were before. We can call these Type 1 decisions.

---

## Type 2 Decisions

> But most decisions aren't like that – they are changeable, reversible – <span style="color:#d92f78">**they're two-way doors**</span>. If you've made a suboptimal Type 2 decision, you don't have to live with the consequences for that long. You can reopen the door and go back through. Type 2 decisions can and should be made quickly by high judgment individuals or small groups.

---

## In the context of blockchains...

<pba-cols>
<pba-col>

### Type 1 Decisions

Decisions that cannot easily be changed in the future.

- Must be a part of the original protocol design.
- Changes might as well be considered a new protocol.


</pba-col>
<pba-col>

### Type 2 Decisions

Decisions that can be easily changed in the future.

- Can be included into the protocol at a later time.
- Changes can be considered as part of the evolution of the protocol.

</pba-col>
</pba-cols>

---

## A Format for Discussing Decisions

- What is (the decision)?
- What do we need to consider when making (the decision)?
	- Is it a Type 1 or Type 2 decision?
- What decisions has (chain) decided to make and why?
	- What tradeoffs have they chosen?
- What decisions have others decided to make?
	- How might those decisions be better or worse?
- Where can the blockchain community still improve on (the decision)?

---

## The Philosophies of Polkadot

---

<image src="../../../assets/img/5-Polkadot/less-trust-more-truth.svg" style="width: 1000px; filter: invert();">

---

## Against Blockchain Maximalism

---

## “The best blockchain today will not be the best blockchain tomorrow.”

---

## The Goals of Polkadot

---

## The Blockchain Scalability Trilemma


- Security: How much does it cost to attack the network?

- Scalability: How much work can the network do?

- Decentralization: How decentralized is the network?


---

## In one sentence...

> Polkadot’s mission is to provide secure, scalable, and unstoppable infrastructure for Web3 applications and services.

---

Polkadot tries to accomplish that mission by solving three problems:

<pba-flex center>

1. Computational Scalability
2. Shared Security
3. Interoperability

</pba-flex
---

## The Decisions

---

## Wasm

WebAssembly is the backbone of Polkadot. It is a fast, safe, and open meta-protocol which powers all of the state transitions of our ecosystem.

It standardizes how chains execute, sandboxes that execution for improved security, and allows teams to build on Polkadot using any language that can be compiled into Wasm.

---

## Sharding

Polkadot scales primarily by parallelizing execution on separate data shards.

These parallel chains (shards) are called Parachains.

---

## App-Chains

Another key scaling decision is the choice of heterogeneous shards, allowing for application specific chains.

Specialized solutions for problems are more performant than generalized solutions, as they can incorporate more details about the problem space.

---

## Interoperability

Individual application chains will inherently lack the ability to provide a full suite of solutions for end users.

Interoperability allows parachains to work together to complete, complex end-to-end scenarios.

---

## XCMP & XCM

<pba-cols>
<pba-col>

### Cross-Chain Message-Passing

<image src="../../../assets/img/5-Polkadot/xcmp-2.svg" style="height: 500px;">


</pba-col>
<pba-col>

### Cross-Consensus Message Format

<image src="../../../assets/img/5-Polkadot/xcm-stack.svg" style="height: 500px;">

- Type 2 Decision

</pba-col>
</pba-cols>

---

## Shared Security

---

## Trust-Free Interactions

<pba-cols>
<pba-col>

<image src="../../../assets/img/5-Polkadot/xcmp-finalization.svg" style="height: 500px;">

</pba-col>
<pba-col>

A key result of shared security through the Relay Chain is that it keeps track of the state of all parachains and keeps them in lock step.

That means blocks which are finalized on Polkadot imply finalization of all interactions between all parachains at the same height.

So, shared security not only secures the individual chains, but the interactions between chains too.

</pba-col>
</pba-cols>

---

<pba-cols>
<pba-col>

### Wasm

WebAssembly is a key part of enabling trust-free shared security and interoperability. It allows the Relay Chain to trustlessly validate the blocks of connected parachains.

</pba-col>
<pba-col>

### Parachains Protocol

The Parachains Protocol is comprised of many subsystems such as the PVF (Parachain Validation Function), Approval Process, Inclusion Pipeline, AnV (Availability and Validity Protocol), etc… It is the most advanced multi-chain coordination software that exists today.

</pba-col>
<pba-col>

### Relay Chain

A coordination chain that manages its own block producers, parachain validators, consensus and finality across all parachains, and more. It is what many people think of when we say “Polkadot”, but is really just one small part of the overall ecosystem.

</pba-col>
</pba-cols>

---

## Babe & Grandpa

---

## Light Client First Mentality

Polkadot has a strong belief that light clients are a necessary component for a Web3 future. It has been uncompromising in enabling first class light client support as a primary part of its development process:

- In-Browser Wasm Client (Substrate Connect)
	- Wasm state transition function too!
- Consensus data integrated into block headers
- Merkle Tries and other LC compatible data structures
- Maximizing statically known metadata to offset reliance on full nodes.

---

## On-Chain Runtime & Forkless Upgrades

The Polkadot protocol specification defines a clear separation between the blockchain client and runtime (state transition function).

This is primarily useful to implement the Parachains protocol, but also allows for chains to “forklessly” upgrade their chain.

This gives the Polkadot Relay Chain and all connected parachains an evolutionary advantage over others in the blockchain space.

---

## On-Chain Governance

Polkadot and its parachains need to change over time to stay relevant, and the network was designed from the beginning to have a transparent and sophisticated process to not only approve or reject changes but also **enact them automatically**.

- Governance decisions can literally change the underlying code of the chain (since it is on-chain).
- 50% of the total stake in the system should be able to control the future of the system.
- Users have the option to lock their tokens to increase the “conviction” of their vote.

---

## On-Chain Treasury

Polkadot has designed at its core a self-funded treasury pool to incentivize the development and evolution of the protocol.

It is entirely controlled on-chain by the governance system of Polkadot, which means that it is immune to the regulations which would normally be imposed to centralized entities.

---

## Nominated Proof-of-Stake

- Type 2 Decision

One of Polkadot’s primary functions is to provide security not only to itself, but also to the connected Parachains. The staking system is a critical focus of the network, and we have one of the most advance staking systems to date.


- NPoS over DPoS to better allocate stake.
	- At the cost of complexity and scaling.
- Economic incentives to distribute funds evenly across validators.
- Super-linear slashing system to discourage validator centralization.
- Actual value being generated by staking, justifying rewards.

---

## Other Type 2 Decisions

Polkadot has made many Type 2 decisions, many of which have already, and will continue to change over time:

- The specifics of the governance process (most recently Gov2)
- Extensions to staking (like nomination pools, fast unstake, etc…)
- Integrated multi-sig, proxies, batching, account abstractions, etc…
- Treasury spending outlets (tips, bounties, fellowship, etc…)
- On-chain storage management, data structures, and deposits.
- Allocation of Parachain execution cores (pay-as-you-go vs permanent).
- XCM format, available instructions, and message queue behavior.
- and so on…

---

## The Discussion
