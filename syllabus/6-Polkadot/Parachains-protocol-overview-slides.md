---
title: Parachains Protocol Overview
description: Details the Collation, Backing, Approval-Voting, and Disputes systems and how they work together to provide secure execution sharding in Polkadot.
duration: 45 minutes
---

# Parachains Protocol

## An Overview

Notes:

[Polkadot v1.0: Sharding and Economic Security](https://polkadot.network/blog/polkadot-v1-0-sharding-and-economic-security/) is a comprehensive writeup of the content here in much more detail.
Please read it after the lesson if you would like to understand how Polkadot works from top to bottom.

---

# Meta

Questions for Shawn, Bradly, et al

How "dumb" should I start? Do I need to introduce the terms parachian, collator, etc?
Or will that be in the intro?

---

## Execution Sharding

> Execution Sharding is the process of distributing blockchain execution responsibilities across a validator set.

> In Polkadot, all validators execute every relay chain block, but only a subset execute each parachain block. <!-- .element: class="../assets/ent" -->

This enables Polkadot to scale. <!-- .element: class="../assets/ent" -->

Notes:

The big picture concept that we're trying to achieve is execution sharding. In this talk we cover Polkadot's implementation at a high level.

---

## Services Polkadot Provides to Parachains

<pba-flex center>

1. Validity
2. Data Availability
3. Finality

</pba-flex>

Notes:

Because GRANDPA finality faults require 33% or more stake to be slashed, Goal (3) implies Shared Security

---

## Parachains Protocols

<pba-flex center>

1. Collation
1. Backing
1. Availability
1. Approval Checking
1. Disputes

</pba-flex>

TODO Make this a block diagram figure

Notes:

If there is one slide to remember from this talk, this is the one.
These are the stages of the parachains protocol.
We'll look at what happens in each stage, and how they come together to make the parachains protocol work.
We won't get too deep in the inner workings of each stage or the subsystems that implement them.
Details about (some of) this will come in subsequent deep dives, others are covered in docs.

---v

## Motivation & Game Theory

Polkadot's approach is to have few validators check every parablock in the best case.

First, **backers** introduce new blocks and provide "skin in the game".

Then, randomly assigned **approval checkers** check their work, with an option to raise a dispute that involves all validators.

Notes:

Questions:
1. Is this weaker than if every validator checked every block?
2. Does this allow more throughput than every validator checking every block?

Answers:
1. Yes, it is weaker... But how much?
2. Yes, it allows more throughput, but how much?

Qualitatively: It is very very slightly weaker. (10^-30 probability of invalid parablock getting finalized.) But we get a LOT more throughput.

So this is a good engineering tradeoff. (And it is tuneable.)

---v

## Validator Groups

<img rounded width=1100, src="./assets/validator-groups.png" />

Notes:

Every Session (4 hours), validators are _partitioned_ into small **groups** which work together to validate a particular parachin.<br/>

There is a detail called execution cores, that Bradley will deep dive on later.
Groups are assigned to specific **Execution Core**s, and these assignments change every few blocks.

## Relay Chain is Forkful

Validators and collators run these protocols on every block of the relay chain.

Often they run an instance of the protocol for every parachain block in every block of the relay chain.

<img rounded width=700, src="../assets/BABE-is-forkful.png" />

Notes:

In the slides, we will look at single instances of the protocols, but it should be known that the validators are actually doing these steps in parallel with each other and often many times at a time.

---v

## Parachains Potocol Again

TODO dupe content

Notes:
This is important.
Here it is again.
And here it is with a dancing banana beside it.

# Collation

---v

## Definition: Candidate

> A **Candidate** is a parachain block<br/>which has not yet been finalized in the relay chain.

---v

## Collation

In the Collation phase, a collator for a scheduled parachain builds a parachain block and produces a candidate.

The collator sends this to validator group assigned to the parachain over the p2p network.

---v

## Proof of Validity

TODO these snippets are from memory. Find the real code, and either put it here, or acknowledge that this is pseudocode.

```rust
struct Block {
	header: Header,
	body: Vec<Extrinsic>,
}
```

```rust
struct PoV {
	header: Header,
	body: Vec<Extrinsics>,
	state_proof: MerkleProof,
}
```

---v

## Synonyms Galore

parablock, PoV block, PoV, Collation, Candidate, oh my!

Notes:

Many of these terms have subtly different meanings.
They can be synonymous in many contexts.
They can also have subtle differences.
For example, Techincally a parablock is just the block, not the state proof.
But if we are talking about "validators downloading and re-executing a parablock", context implies that we are actually talking about a PoV block.

---
# Backing

---v

## Backing: Skin in the Game

Backers are agreeing that if the parablock turns out to be bad, they will lose 100% of their stake.

Backing on its own does not provide security, only accountability.

TODO catchy image

Notes:

The current minimum validator bond as of Jan 27, 2022 is ~1.5 Million DOT.

---v

Definition: Statement

<blockquote>
TODO peer review, is this a good definition?
A statement is a signed message from a backer to the greater validator set that they are willing to back a particular candidate.
</blockquote>

---v

## Backing: Logistics

Backers distribute their candidates and statements via the P2P layer.

This is Off-Chain!

The next relay chain block author bundles candidates and statements into the relay chain block.

TODO: But why? Why have a whole separate system to aggregate statements and put them in an inherent when we already have a transaction pool and block builder. Is this just an optimization?

Notes:



---v

## Backing: Networking

TODO This was Rob's image. Bradley, wtf is this showing?

<img rounded width=1000, src="./assets/backing-networking.png" />

---

## Availability

---v

## Lots of Parties Need the PoV

* Other parachain nodes need to sync
* Other validators need to check it (more on that soon)

---v

## Making the PoV Available

Backers are responsible for making the data needed to check the parablock available to the entire network.

Validators sign statements about which data they have and post them to the relay chain.

TODO: "Data needed to check the parablock" - is this just the PoV?

---

<img rounded width=1300px src="./assets/availability-inclusion.png" />

Notes:

In practice, we allow more than a single block for availability to be timed out.

---

## Parablock Inclusion and Finality

<img rounded width=600, src="./assets/parachain-finality.png" />

---

## Parablock Inclusion and Finality

> (3) Only valid parachain blocks will become finalized

Notes:

Remember our goal from earlier?

---

## Parablock Inclusion and Finality

To fulfill this goal we need 2 things.

<pba-flex center>

1. A protocol for proving validity of included candidates
1. Consensus rules for the relay chain<br/>to avoid building on or finalizing<br/>relay chain forks containing bad candidates.

</pba-flex>

---

## What is "Checking" a Parablock?

Checking involves three operations:

<pba-flex center>

1. Recovering the data from the network
1. Executing the parablock, checking success
1. Check that outputs match the ones posted<br/>to the relay chain by backers

</pba-flex>

Notes:

Step 3 is of crucial importance.
Without it, backers could create things like messages and runtime upgrades out of thin air, by backing a valid candidate but lying about the outputs of the candidate.

---

## Security Model: Gambler's Ruin

The security argument for Polkadot is based on Gambler’s Ruin.

An attacker who can take billions of attempts to brute-force the process would eventually be successful.

But because of slashing, every failed attempt means enormous amounts of DOT slashed.

---

## Approval Checking

---v

## Approval Checking

Every validator node is running an approval checking process for every parachain block in every relay chain block.
This process has a few properties:

<pba-flex center>

1. The process on any particular node either outputs "good" or stalls.
1. The output of the process on a node is based on the statements it has seen from other validators or produced itself.
1. If the parachain block is valid (i.e. passes checks) then it will eventually output "good" on honest nodes.
1. If the parachain block is invalid then it will only output "good" on honest nodes with low probability

</pba-flex>

Notes:

Honest nodes output "good" only if there is a very large amount of malicious checkers and they mainly see votes
from those checkers as opposed to honest checkers.

Low probability here means 1 in 1 billion or so (assuming 3f < n)
Not cryptographic low probability, but good enough for crypto-economics.

---v

## Approval Checking

Approval checking involves validators generating assignments to check parablocks.

Every validator is assigned to check every parablock, but at different times.

For later-assigned validators, if it's approved by the time it's their turn, they simply do not check.

---v

## Approval Checking: Assignments and Approvals

Validator assignments are known only to the validator until revealed.

Validators reveal their assignment before downloading data and checking the parablock.

This ensures that others will notice if they disappear, leading to escalating requirements for checkers.

Notes:

If validators began downloading data before revealing their assignment, an attacker might notice this and attack them without anybody noticing.

---v

## Approval Checking: The Hydra

<img rounded width=700, src="../assets/lernaean-hydra.jpg">

Notes:

Approval Checking is like the hydra.
Every time an attacker chops off one head, two more heads appear.

---

## Disputes

---v

## Disputes

When validators disagree about the validity of a parablock, a dispute is automatically raised.

Disputes involve all validators, which must then check the block and cast a vote.

Backing and Approval statements already submitted are counted as dispute votes.

Votes are transmitted by p2p and also collected on-chain.

---v

## Dispute Resolution

<img rounded width=700, src="../assets/validator-dispute-participation.png" />

Notes:

resolution requires a supermajority in either direction.

---v

## Dispute Slashing

The validators on the losing side of the dispute are slashed.

The penalty is large when the candidate is deemed invalid by the supermajority and small when it is deemed valid.

---

## Orphaning Invalid Candidates

---v

## It's Not Too Late

If we are having a dispute, the parablock is already on-chain.
Doesn't that mean it is too late to do anything?

---v

## The Relay Chain is Forkful

Relay blocks are not finalized until disputes finish

If a dispute finds an invalid parablock, honest relay chains simply won't finalize that relay chain fork, and they will re-org to one that does not contain an invalid parachain block.

TODO: multi-step figure. This can be very clear with a diagram, but is pretty confusing in words only.

---v

## GRANDPA Voting Rules

Instead of voting for the longest chain, validators vote for the longest chain where all unfinalized candidates are a) approved and b) undisputed

<img rounded width=650, src="../assets/grandpa-voting-rule.png" />

---v

## BABE Chain Selection Rule

Validators refuse to author relay chain blocks on top of forks containing parablocks which are invalid or have lost disputes.
This causes a "reorganization" whenever a dispute resolves against a candidate.

<img rounded width=650, src="../assets/babe-chain-selection.png" />

---

## Implementers' Guide

[The Implementers' Guide](https://paritytech.github.io/polkadot/book) contains information about all subsystems, architectural motivations, and protocols used within Polkadot's runtime and node implementation.
