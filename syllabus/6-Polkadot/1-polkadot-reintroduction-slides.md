---
title: Polkadot Re-Introduction
description: A high level overview of Polkadot, summary of key parts and an introduction to parachains protocol
duration: 45 min
owner: Maciej Zyszkiewicz
---

# Polkadot

Notes:
Introduction

---v

# Polkadot

## Core Tenets (In My Opinion)

- Polkadot is a decentralized open-source community

- [...]

- [...]

Notes:
Before we jump into the main topic let me make a few comments on what I believe are the core tenets of Polkadot. The first one is that

Polkadot is a decentralized open-source community. Hopefully at this point you got somewhat convinced about this one. Sitting here in the PBA which is funded by the biggest DAO in the digital world. And I'm sure the Governance module also reinforced that idea.

---v

# Polkadot

## Core Tenets (In My Opinion)

- Polkadot is a decentralized open-source community

- Polkadot is a permissionless and ubiquitous computer

- [...]

Notes:
Polkadot is a permissionless and ubiquitous computer. The key pieces to this were supplied by the customizable and programmable substrate/frame chains so something you got to experience in details over the last few weeks. They are fully adjustable state transition machines that can be deployed online.

---v

# Polkadot

## Core Tenets (In My Opinion)

- Polkadot is a decentralized open-source community

- Polkadot is a permissionless and ubiquitous computer

- Polkadot is a scalable, heterogeneous, sharded, multi-chain network

Notes:
The goal of the next 2 days will be understanding how we achieve that last tenet. Understanding how Polkadot scales, manages its heterogeneity, shards and connects all those chains into a single network.
And to succeed in that mission Let's dive into the main topic for today - Parachain Consensus.

---

# What is Parachains Consensus?

Notes:
We will be looking at what is parachain consensus?

---

# Three Pillars of Polkadot

<img style="width: 500px" src="./assets/execution-sharding/polkadot-components.svg"/>

Notes:
Game analogy. NPoS decides who plays the game, parachain consensus are the rules of the game. Governance helps us evolve the game over time so it's never stale and keeps up with the times.

---

# Polkadot Relay Chain

NPoS, Parachains Consensus and Gov all live on the **Relay Chain**

Notes:
For now you can think of the Polkadot Relay Chain as normal blockchain and we will slowly build it up uncovering what it truly does

---

# NPoS

Notes:
The first pillar worth exploring is NPoS. It is tightly connected to a frame pallet called staking that lives in the relay chain runtime

---v

# NPoS

## What's the goal of this pillar?

We need to fairly elect a bounded number of players (validators) that will participate in the game (parachain consensus).

Notes:
So what's the goal of this pillar? Player election!

---v

# NPoS

## NPoS Elections

- Every 24h (era) an algorithmic election is held and a set of 300 validators is chosen - they will be the **active validator set**.
- Active set of validators is backed by ~50% of the total DOT supply
- The stake is evenly distributed between the active validators so they all have equal voting power in the upcoming game

Notes:
There's no better way than to vote with your wallet and NPoS is all about this idea.

---v

# NPoS

## NPoS Nominations

- Most validator don't own the stake they use for the elections and they simply focus on being a validator-as-a-service operators
- The stake used by those validators is provided by other tokens holders not willing to go through the trouble of owning a full-node on your own hardware
- Those tokens holders **nominate** the validator essentially entrusting their own tokens to the validator to share parts of the profits, but they also share the dangers

Notes:
Picking the best nominators and validators is actually an NP-hard problem! Same as the knapsack problem. There is a clever scheme to try and solve it on chain for those interested read more here: <link>
If you all will be more interested in the topics and there will be time we might do a short in-depth dive into NPoS and Phragmen elections.

---

# Parachain Consensus

Notes:
But for now let's move to the Parachain Consensus. This is the heart of the relay chain that distinguishes it from other blockchains. This is the secret sauce.

---v

# Parachain Consensus

## What's the goal?

Notes:
What's the goal? What does it try to accomplish? Well it's blockchain and there's one thing we really, really want to see in blockchains.

---v

# Parachain Consensus

## What's the goal?

- Scalability

Notes:

---v

# Parachain Consensus

## What's the goal?

- Scalability
- Scalability

Notes:

---v

# Parachain Consensus

## What's the goal?

- Scalability
- Scalability
- Scalability

Notes:
And security and decentralization but we'll dive into those on future lectures.
Okay, so how does it achieve scalability? You all have been learning about substrate and frame and building your own little chains. And it is true that Polkadot's Relay Chain is a also a Substrate-based chain but there's more to it. It's a substrate chain capable of aggregating the state of other substrate chains and thus sharing it's own security with them. Parachain Consensus is what validates all those other blockchains and makes them unite in the relay chain. This is what we call sharding.

---v

# Parachain Consensus

## What's the goal?

**Sharding**

Sharding crucially allows us to parallelize execution.

Notes:
Sharding crucially allows us to parallelize execution. All those sovereign substrate blockchains can connect to the relay chain and entrust it with validation duties, while the chain itself can focus on the business logic and functionality. So all those little blockchains connected to the relay chain are called parachains. But actually does anyone know where this name comes from? ...

But let's zoom out for a second.

---v

# Parachain Consensus

## Zooming out

<img style="width: 500px" src="./assets/execution-sharding/polkadot-architecture.svg"/>

Notes:
This is how Polkadot's relay chain and it's parachains can be visualized. This is just a single relay chain block slice. All those little blocks around the large circle are parachain blocks - parablocks. What I want you to take away from this picture is that Polkadot is a fundamentally just like a mothership for all those little parachains. And if they are fully capable and sovereign chains, then this makes Polkadot a layer 0 solution.

---v

# Parachain Consensus

## Zooming out

Polkadot is a layer 0 protocol

Notes:
Polkadot a layer 0 blockchain. A blockchain of blockchains, a platform for other platforms to build and flourish.

And also Polkadot being a layer 0 is why I often think that a truly widely adopted and successful Polkadot does not mean people recognize the name, it means you recognize the project building on top of Polkadot. Marketing Polkadot is nice but I strongly believe that end consumers don't need to know about it just like AWS doesn't advertise to the mom and pops, they target builders.

---v

# Parachain Consensus

## Why do we learn it?

To fully utilize what Polkadot offers you need to learn its quirks.

Notes:
And I hope that this also makes it clearer why we will be learning those more in-depth core protocol elements. Because if you all are the next generation of builders then only if you understand what Polkadot truly offers, what are its strengths and weaknesses you will be able to build the next application that truly uses it's potential to the fullest. Because there are things that you can only build on Polkadot and nowhere else.

So now that the why is covered let's look at how it all comes together. There is a lot of moving parts in the parachains consensus but let's start familiarizing ourselves with them. In the upcoming lectures we will be diving deeper into each individual step so don't worry if you miss something.

---v

# Parachain Consensus

## The Game

<div style="font-size: 0.8em">

<pba-flex left>

**Primary Goal:**

- Grow registered parachains and post only valid updates to the Relay Chain

</pba-flex>

Notes:

---v

# Parachain Consensus

## The Game

<div style="font-size: 0.8em">

<pba-flex left>

**Primary Goal:**

- Grow registered parachains and post only valid updates to the Relay Chain

</pba-flex>

<pba-flex left>

**Rules:**

</pba-flex>

- Validators are incentivized to attest to new parachain blocks
- Whichever Validator makes the next Relay Chain block includes some attested parachain blocks
- Validators are slashed if they attest to incorrect parachain blocks
  - incorrect means "not according to the parachain's Wasm code"
- Validators check each others' work to initiate the slashing procedure

</div>

<br/>

---v

# Parachain Consensus

## The Game

<div style="font-size: 0.8em">

<pba-flex left>

**Primary Goal:**

- Grow registered parachains and post only valid updates to the Relay Chain

</pba-flex>

<pba-flex left>

**Rules:**

</pba-flex>

- Validators are incentivized to attest to new parachain blocks
- Whichever Validator makes the next Relay Chain block includes some attested parachain blocks
- Validators are slashed if they attest to incorrect parachain blocks
  - incorrect means "not according to the parachain's Wasm code"
- Validators check each others' work to initiate the slashing procedure

</div>

<br/>

**The game works whenever <1/3 of validators misbehave.**

---v

# Parachain Consensus

## Overview

Notes:
Those were the simplified rules of the game but those rules will be encompassed by specific sub-protocols within Polkadot and lets take a loot at them now.

---v

# Parachain Consensus

## Overview

- Assignment: Group up validators.

Notes:
Initially we'll be grouping up and assigning the validators in the active set, those are the ones that NPoS gives us.

---v

# Parachain Consensus

## Overview

- Assignment: Group up validators.
- Collation: Collect transactions.

Notes:
Some other nodes will collect the user transactions and create collations.

---v

# Parachain Consensus

## Overview

- Assignment: Group up validators.
- Collation: Collect transactions.
- Backing: Assign responsibility.

Notes:
Some validators will receive those collations and if they think they are correct they will vouch for them. Take on the responsibility for them.

---v

# Parachain Consensus

## Overview

- Assignment: Group up validators.
- Collation: Collect transactions.
- Backing: Assign responsibility.
- Availability: Distribute data.

Notes:
All the data connected to the collation is distributed between the validators in a compressed form.

---v

# Parachain Consensus

## Overview

- Assignment: Group up validators.
- Collation: Collect transactions.
- Backing: Assign responsibility.
- Availability: Distribute data.
- Approval Checking: Verify correctness.

Notes:
Validators perform the final correctness checks

---v

# Parachain Consensus

## Overview

- Assignment: Group up validators.
- Collation: Collect transactions.
- Backing: Assign responsibility.
- Availability: Distribute data.
- Approval Checking: Verify correctness.
- Disputes: Resolve escalations.

Notes:
And if something doesn't feel right they start an official dispute. Think of it as starting the alarm bells and calling in for help.

---v

# Parachain Consensus

## Overview

- Assignment: Group up validators.
- Collation: Collect transactions.
- Backing: Assign responsibility.
- Availability: Distribute data.
- Approval Checking: Verify correctness.
- Disputes: Resolve escalations.
- Finality: Solidify state.

Notes:
But if everything went smoothly the verified collations will get finalized by GRANDPA

---

# Governance

Notes:
Up to this point maybe it wasn't clear but there are a lot of number that seem pretty arbitrary. For instance remember that the active validator set was 300? Why not 350? Who decided that? And most importantly who can change it? In Polkadot that's the token holders!

---v

# Governance

## OpenGov

Polkadot has on-chain governance by stakeholder referendum, voting on subjects such as:

<pba-flex center>

- Forkless upgrades of the network
- Administration of the Treasury funds
- Configuration of the Parachains protocol
- Configuration of fees
- Rescue & recovery operations
- All other mechanisms of control over the platform

</pba-flex>

Notes:
Token holders can vote and adjust the parameters of the protocol. Everything like the number of validators to the minimum staking, existential deposit etc can be adjusted. So The parachain consensus, this have played by the validators can adapt if it needs it. And even more importantly because the Relay Chain itself uses a runtime architecture we can even issue governance enable runtime upgrades of the relay chain to expand upon it even further. This is a bit of a double-edged sword since not all change is good change.

https://polkadot.network/features/opengov/

---

# JAM

## Polkadot 2.0

Notes:
Who here has heard of JAM?

---v

# JAM

## Polkadot 2.0

JAM is a further generalization and abstraction of Polkadot.

That means that whatever Parachain Consensus does will still be very relevant in JAM as it will adopt many of it's features.

Notes:
JAM was a recently proposed potential next step for the Polkadot Network. Generally think of it as a generalization of the protocol. JAM can do everything the current Polkadot Relay Chain can and more. In general it is still an extremely fresh idea so we will not be covering in depth in the core lectures but you can expect some more information about later this week.

And I can also guarantee you one thing, many of the steps in parachain consensus like approval checking and disputes will be recreated and ported to JAM, so understanding them now gives you an awesome headstart to understanding what JAM truly aims to achieve.
