---
title: Cumulus Deep Dive
description: Cumulus, architecture and function
duration: 1 hours
---

# Cumulus Deep Dive

Notes:

Hi again everyone. 

Now it's my privelege to walk you through the basics of Cumulus. It is the glue which attaches substrate based chains to Polkadot, converting them into parachains.

---

### Outline

<pba-flex center>

1. [What is Cumulus?]()
1. [Cumulus and Para-Relay Communication]()
1. [How Cumulus Enables Parablock Validation]()
1. [How Cumulus Enables Runtime Upgrades]()

1. [Runtime Upgrades]()

</pba-flex> 

---

## What is Cumulus

A collection of code libraries extending a Substrate FRAME chain so that it can interface with the Polkadot API, run relay chain based consensus, and submit parachain blocks for validation.

---

<div class="r-stack">
<img src="../assets/spc_1.svg" style="width: 70%" />
<img src="../assets/spc_2.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="1" -->
<img src="../assets/spc_3.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="2" -->
</div>

Notes:

- Substrate is a framework for building blockchains
- But only "solo" chains
- Split into runtime/node side
- Both Polkadot and Cumulus are extend substrate to create the relay chain and parachains respectively
- Polkadot provides APIs by which a Cumulus enabled collator communicates with the relay chain

---

## Review, Collators and Collations

A collator is a parachain node which is part of the authority set. In addition to taking part in transaction and block gossip with its peers, a collator also authors and submits collations to the relay validator set.

<br>
A collation is a bundle of all the data validators need from a parachain to process and validate a particular parachain block.

Notes:

Collations include: upward and horizontal messages, new validation code, resulting head data, proof of validity, processed downward messages, and hrmp_watermark (relay block up to which all hrmp messages have been processed)

---

## Cumulus' Key Processes

- Follow relay "new best head" to update para "new best head"
- Follow relay finalized block to update para finalized block
- Request parablocks not shared by peers from relay (data recovery)
- Collation generation and announcement

Notes:

- New best head means a new block that is at the head of the fork most preferred by BABE

---

## Cumulus and Para-Relay Communication

<div class="r-stack">
<img src="../assets/para-relay_communication_1.svg" style="width: 1100px" />
<img src="../assets/para-relay_communication_2.svg" style="width: 1100px" />
<!-- .element: class="fragment" data-fragment-index="1" -->
</div>

Notes:

- How do these communication channels service our key processes?

---

## Following the Relay Best Head



## Cumulus and Parachain Validation Code

<div class="r-stack">
<img src="../assets/cumulus_sketch_1.svg" style="width: 70%" />
<img src="../assets/cumulus_sketch_2.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="1" -->
<img src="../assets/cumulus_sketch_3.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="2" -->
<img src="../assets/cumulus_sketch_4.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="3" -->
</div>

Notes:

- The parachain validation function is composed of our parachain runtime code and the function validate_block
- Updates to the parachain validation function

---

## Runtime Upgrades

- Every Substrate blockchain supports runtime upgrades
<!-- .element: class="fragment" data-fragment-index="0" -->

##### Problem

<!-- .element: class="fragment" data-fragment-index="1" -->

- What happen if the PVF compilation takes too long?
  <!-- .element: class="fragment" data-fragment-index="1" -->
  - In approval checking, there may be many no-shows, leading to slow finality
  - In disputes, neither side may reach super-majority. Nobody will get slashed and the chain will not be reverted or finalized.

<!-- .element: class="fragment" data-fragment-index="1" -->

</br>
  
- Updating a Parachain runtime is not as easy as updating a standalone blockchain runtime
<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

Now almost every change to the client and the runtime of the substrate based-chain is explained, is missing only the runtime upgrade management that is not so easy as in a normal substrate-based solo chain.

---v

##### Solution

Relay chain needs a fairly hard guarantee that the PVFs can be compiled within a reasonable amount of time

<!-- .element: class="fragment" data-fragment-index="0" -->

</br>

- Collators execute a runtime upgrade but it is not applied
- The relay chain executes the **PVF Pre-Chekcing Process**
- The first parachain block that will be included after the end of the process needs to apply the new runtime

<!-- .element: class="fragment" data-fragment-index="1" -->

</br>

Cumulus autonomously follows the relay chain to apply the runtime when it's time

<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

A Parachain will follow the same paradigm as a standard solo chain, but the relay chain needs to be informed before the update.
Cumulus will provide functionality to notify the relay chain about the runtime update.
The update will not be enacted directly; instead it takes X relay blocks (a value that is configured by the relay chain)
before the relay chain allows the update to be applied.
The first Parachain block that will be included after X relay chain blocks needs to apply the upgrade.
If the update is applied before the waiting period is finished, the relay chain will reject the Parachain block for inclusion.
The Cumulus runtime pallet will provide the functionality to register the runtime upgrade and will also make sure that the update is applied at the correct block.
https://github.com/paritytech/cumulus/blob/master/docs/overview.md#runtime-upgrade

---v

##### PVF Pre-Checking Process

- The relay chain keeps track of all the new PVFs that need to be checked
- Each validator checks if the compilation of a PVF is valid and does not require too much time, then it votes
  - binary vote: accept or reject
- As soon as the super majority is reached the voting is concluded
- The state of the new PVF is updated on the relay chain

Notes:

Only validators from the active set can participate in the vote. The set of active validators can change each session. That's why we reset the votes each session. A voting that observed a certain number of sessions will be rejected.

reference: https://paritytech.github.io/polkadot/book/pvf-prechecking.html

---

## References

1. Gabriele Miotti, who was a huge help putting together these slides
1. https://github.com/paritytech/cumulus/blob/master/docs/overview.md

---

<!-- .slide: data-background-color="#4A2439" -->

## Questions