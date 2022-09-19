---
title: Parachains Architecture
description: Polkadot Parachains for Web3 engineers
duration: 1 hour
instructors: ["Bernhard Schuster, Robert Klotzner"]
teaching-assistants: ["Dan Shields"]
---

# Parachains Architecture

---

## Lecture Goals

<widget-text center>

- General blockchain challenges <!-- .element: class="fragment" -->
- Parachain consensus <!-- .element: class="fragment" -->

---

## Blockchain Challenges

<widget-columns>
<widget-column>

<image src="../../../assets/img/5-Polkadot/5.4-1.svg" style="width: 600px">

</widget-column>
<widget-column>

<ul>
  <li class="fragment"> Performance </li>
  <li class="fragment"> Scalability </li>
</ul>

</widget-column>
</widget-columns>

---

## Resolving Conflicts

<widget-columns>
<widget-column>

<image src="../../../assets/img/5-Polkadot/5.4-1.svg" style="width: 600px">

</widget-column>
<widget-column>

<ul>
  <li class="fragment"> Security </li>
  <li class="fragment"> Liveness </li>
  <li class="fragment"> Decentralization </li>
</ul>

</widget-column>
</widget-columns>

Notes:

Scalability: In every aspect: Throughput, but also storage.
Relay chain validators are "light clients" to parachains.
Parachains act as heterogenous shards, both in terms of execution and data availability.
In contrast to Ethereum 2, which will just offer sharded data availability.

Performance via Specialization: Parachains operate independently via "collator" nodes, can do whatever they want, as long as the provide a PoV to relay chain validators.
Relay chain validators don't have to bother about handling transaction pools and such.

Parachains help with all of these, but let's focus on scalability first.

---

## How to achieve scalability?

- Split the work <!-- .element: class="fragment" -->
- But maintain (economic) security! <!-- .element: class="fragment" -->
- Liveness? <!-- .element: class="fragment" -->

Notes:

Security: Defer solution to upcoming slides. (Also mention shared security.)
Liveness: Suggest rotation.

---

## Split the Work

<image src="../../../assets/img/5-Polkadot/5.4-2.svg" style="width: 900px">

Notes:

Introduce split up work between validators.

---

## Rotate Groups

<image src="../../../assets/img/5-Polkadot/5.4-3.svg" style="width: 775px">

Notes:

Problem: Imagine each group is its own shard/blockchain - that would mean each node, as we rotate, would need to keep state for all those shards/parachains → Does not scale too well.

---

## Introducing Collators

<image src="../../../assets/img/5-Polkadot/5.4-4.svg" style="width: 1000px">

Notes:

From the perspective of the relay chain collators are completely untrusted and even unknown, but can keep storage and take care of some of the heavy lifting, like gossiping transactions, maintaining transaction pools, ...

Idea: Visualize how performance and scalability is achieved.
For example that collators are responsible to keep state for the parachains, take care of lots of the nitty-gritty details of block production.
Validators don't have to worry about individual transactions, collecting & prioritizing them, keeping them in pools.

In ETH 2 version 1 nomenclature: Collators/Parachain provide shards for data availability, but also version 2 execution shards.

---

## Recap PoV

<widget-text center>

- Collators - external/untrusted <!-- .element: class="fragment" -->
- PoV ... Proof of Validity <!-- .element: class="fragment" -->

---

### PoV (usually)

<widget-text center>

- Parachain Block Data <!-- .element: class="fragment" -->
- Patricia Merkle Trie <!-- .element: class="fragment" -->

---

## All good?

<widget-text center>

- Bad group - security? <!-- .element: class="fragment" -->
- Large groups - scalability? <!-- .element: class="fragment" -->
- Performance <!-- .element: class="fragment" -->
- Liveness <!-- .element: class="fragment" -->

Notes:

Large group: Make it statistically unlikely enough to have a majority malicious group.
Works, but:

→ Performance: relying on statistics: groups have to be relatively large ~40 validators.
→ Liveness: If nodes don't vote we have to assume the worst → escalate.

---

# Catch Bad Guys

---

## Wait .. Bad Guys?

What can malicious nodes actually do?

<widget-text center>

- No (faithful) PVF execution <!-- .element: class="fragment" -->
- Manipulate messages <!-- .element: class="fragment" -->
- Fool Light Clients <!-- .element: class="fragment" -->

Notes:

Quick recap: What harm can a malicious validator actually do? What do we need to worry about?

Not (properly) executing the block, providing results that don't adhere to the rules of the PVF.
Such a block would likely be not accepted by other parachain nodes, but light clients might get fooled and more importantly other parachains can get fooled with manipulated messages.

---

## Catch Bad Guys

<widget-text center>

- and punish them <!-- when caught --> <!-- .element: class="fragment" -->
- gamblers ruin <!-- .element: class="fragment" -->

Notes:

- Eventually the system can be broken, but if attempts result in loss of significant funds any realistic attacker will go bankrupt way before.
- Purpose of backing checkers: Get skin in the game - collators are external.

---

## How do we do that?

<widget-text center>

- 2 (3) phase process <!-- .element: class="fragment" -->
- Validators get skin in the game (backing) <!-- .element: class="fragment" -->
- Check the checkers (approvals) <!-- .element: class="fragment" -->
- Punish (disputes) <!-- .element: class="fragment" -->

Notes:

How: Validators need to be known in advance so collator nodes can connect.
We need to be able to check the checkers.

---

## Two Phase Process

<widget-text center>

- Backing <!-- .element: class="fragment" -->
- Approval <!-- .element: class="fragment" -->

Does this work already? <!-- .element: class="fragment" -->

No <!-- .element: class="fragment" -->

What is the problem? <!-- .element: class="fragment" -->

Notes:

1. Approval checkers must not be guessable in advance - solvable via VRF.
1. Data must be guaranteed available, so backers cannot get away with not providing necessary data to approval checkers.

→ Not sufficient.

---

## Three Phase Process

<widget-text center>

- Backing <!-- .element: class="fragment" -->
- Inclusion/Availability <!-- .element: class="fragment" -->
- Approval <!-- .element: class="fragment" -->

Does this work already? <!-- .element: class="fragment" -->

Yes! <!-- .element: class="fragment" -->

---

## But how?

<widget-text center>

- Backing group is untrusted <!-- .element: class="fragment" -->
- Collators are untrusted <!-- .element: class="fragment" -->
- Availability proof on chain? <!-- .element: class="fragment" -->

Notes:

How do we actually achieve availability?

---

## Solution Attempt #1

<widget-columns>
<widget-column>

<image src="../../../assets/img/5-Polkadot/5.4-6.svg" style="width: 600px">

</widget-column>
<widget-column>

<ul>
  <li class="fragment"> Send PoV to all validators </li>
</ul>

</widget-column>
</widget-columns>

---

## Solution Attempt #1

<widget-columns>
<widget-column>

<image src="../../../assets/img/5-Polkadot/5.4-7.svg" style="width: 600px">

</widget-column>
<widget-column>

<ul>

  <li> Send PoV to all validators </li>
  <li> Sign Statements </li>

</ul>

Works, but very costly: Performance! <!-- .element: class="fragment" -->

</widget-column>
</widget-columns>

---

## Solution Attempt #2

<widget-columns>
<widget-column>

<image src="../../../assets/img/5-Polkadot/5.4-8.svg" style="width: 600px">

</widget-column>
<widget-column>

<ul>

<span class="fragment"> Problems: </span>
<span class="fragment"> Still inefficient! 40x overhead. </span>

</ul>

</widget-column>
</widget-columns>

---

## Solution Attempt #2

<widget-columns>
<widget-column>

<image src="../../../assets/img/5-Polkadot/5.4-9.svg" style="width: 600px">

</widget-column>
<widget-column>

<ul>

and <span> relatively easy to DoS </span>

</ul>

</widget-column>
</widget-columns>

---

## Solution Attempt #3

<widget-columns>
<widget-column>

<image src="../../../assets/img/5-Polkadot/5.4-10.svg" style="width: 600px">

</widget-column>
<widget-column>

<ul>
  <li class="fragment"> Erasure coding </li>
  <li class="fragment"> Distribute to all </li>
  <li class="fragment"> Overhead only 3x </li>
  <li class="fragment"> Hard to DoS </li>
</ul>

</widget-column>
</widget-columns>

---

## Recap

<widget-text center>

- Known backing group - get skin in the game <!-- .element: class="fragment" -->
- Make available <!-- .element: class="fragment" -->
- Reveal approval checkers <!-- .element: class="fragment" -->
- Catch bad guys <!-- .element: class="fragment" -->
- Punish them <!-- .element: class="fragment" -->

---

## Disputes

<widget-text center>

- Raised during approval checking <!-- .element: class="fragment" -->
- Escalation to all nodes: Performance!? <!-- .element: class="fragment" -->
- Slash Offenders - have them pay the bill! <!-- .element: class="fragment" -->
- Disputes are not expected to happen ... <!-- .element: class="fragment" -->
- ... because they exist! <!-- .element: class="fragment" -->

---

## Summary

<widget-text center>

- Secure heterogeneous sharding <!-- .element: class="fragment" -->
- Shared Security <!-- .element: class="fragment" -->
- Performance via Specialization <!-- .element: class="fragment" -->

Notes:

Relay chain validators act as light clients for all parachains and are responsible for making sure all state transactions are sound.
Therefore shards (parachains) don't have to run light clients for all other shards (parachains), but instead only for the relay chain.
