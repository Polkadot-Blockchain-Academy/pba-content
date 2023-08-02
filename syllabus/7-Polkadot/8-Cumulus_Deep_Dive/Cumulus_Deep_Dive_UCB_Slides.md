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

1. What is Cumulus?
1. Cumulus and Para-Relay Communication
1. How Cumulus Keeps a Parachain Node Informed
1. Collation Generation and Advertisement
1. How Cumulus Collations Enable Parablock Validation
1. How Cumulus Enables Runtime Upgrades

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
- Both Polkadot and Cumulus extend substrate
- Polkadot provides APIs to collators

---

## Review, Collators and Collations

<pba-flex center>

> What is a collator?

> What is a collation?

> What is the PoV?
<!-- .element: class="fragment" data-fragment-index="1" -->

</pba-flex>

Notes:

- Collator: 
  - Part of consensus authority set 
  - Author and submit collations
- Collation: Info necessary for validators to process and validate a parachain block.
- Collations include: upward and horizontal messages, new validation code, resulting head data, proof of validity, processed downward messages, and hrmp_watermark (relay block up to which all hrmp messages have been processed)
- PoV: The smallest bundle of information sufficient to validate a block. Will revisit in more detail.

---

## Cumulus' Key Processes

- Follow relay "new best head" to update para "new best head"
- Follow relay finalized block to update para finalized block
- Request parablocks not shared by peers from relay (data recovery)
- Collation generation and announcement

Notes:

- New best head: New block at the head of the fork most preferred by BABE

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

## Handling Incoming Relay Information

Before addressing collation generation let's first address the other three key Cumulus processes. These drive parachain consensus and ensure the availability of parachain blocks.

<br>
Together they keep parachain nodes up to date such that collating is possible.

Notes:
To recap, these key processes are:
- Follow relay "new best head" to update para "new best head"
- Follow relay finalized block to update para finalized block
- Request parablocks not shared by peers from relay (data recovery)

---

### Consensus Mechanism

Parachain consensus is modified to:

<pba-flex center>

- Achieve sequencing consensus
<!-- .element: class="fragment" data-fragment-index="1" -->
- Leave finality to the relay chain
<!-- .element: class="fragment" data-fragment-index="2" -->

</pba-flex>

Notes:

- Sequencing consensus: Decide on an accepted ordering of blocks and of transactions within a block
- Sequencing consensus requires that we update our knowledge of the new best head of the parachain. That way nodes are in agreement about which block to build on top of.
- Sequencing options: Aura consensus, tendermint style consensus
- When a parablock is included in a relay block that becomes finalized, that parablock is finalized by extension.

---

### Import Driven Block Authoring

Collators are responsible for authoring new blocks, and they do so when importing relay blocks. Honest Collators will choose to author blocks descending from the best head.

```rust[2|4-8|10]
// Greatly simplified
loop {
    let imported = import_relay_chain_blocks_stream.next().await;

    if relay_chain_awaits_parachain_candidate(imported) {
        let pov = match parachain_trigger_block_authoring(imported) {
            Some(p) => p,
            None => continue,
        };

        relay_chain_distribute_pov(pov)
    }
}
```

<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

- `parachain_trigger_block_authoring` itself can decide if it wants to build a block.
- e.g. the parachain having a block time of 30 seconds
- With asynchronous backing, parachain block authoring is untethered from relay block import.

---

### Finality

To facilitate shared security, parachains inherit their finality from the relay chain. 

</br>

```rust[2|4-8|10]
// Greatly simplified
loop {
    let finalized = finalized_relay_chain_blocks_stream.next().await;

    let finalized_parachain_block =
      match get_parachain_block_from_relay_chain_block(finalized) {
        Some(b) => b,
        None => continue,
    };

    set_finalized_parachain_block(finalized_parachain_block);
}
```

<!-- .element: class="fragment" data-fragment-index="1" -->

---

### Ensuring Block Availability

As a part of the parachains protocol, Polkadot makes parachain blocks available for several hours after they are backed.

<pba-flex center>

- Why is this needed?
  - Approvals
  - Malicious collator

</pba-flex>

Notes:

- Approvers need the PoV to validate
- Can't just trust backers to distribute the PoV faithfully
- Malicious or faulty collators may advertise collations to validators without sharing them with other parachain nodes.
- Cumulus is responsible for requesting missing blocks in the latter case

---

#### Brief Aside, Candidate Receipt

The PoV is too big to be included on-chain when a parablock is backed, so validators instead produce a constant size **Candidate Block Receipt** to represent the freshly validated block

Notes:

The Candidate Receipt contains mainly Hashes so the only valuable use is to be used to verify the correctness of known PoVs
The Candidate Receipt only represents a PoV, it does not substitute it

---

### Malicious collator example

<div class="r-stack">
<img src="../assets/malicious_collator_1.svg" style="width: 70%" />
<!-- .element: class="fragment fade-out" data-fragment-index="1" -->
<img src="../assets/malicious_collator_2.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="1" -->
<img src="../assets/malicious_collator_3.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="2" -->
</div>

Notes:

- On a Parachain, a block only needs to be accepted by the relay chain validators to be part of the canonical chain, 
- The problem is that a collator can send a block to the relay chain without distributing it in the Parachain network
- So, the relay chain could expect some parent block for the next block that no one is aware of

---

### The Availability Process

<pba-flex center>

- Erasure coding is applied to the PoV, breaking it into chunks
- 3x original PoV size, vs 300x to store copies
- 1/3 of chunks sufficient to assemble PoV
- 2/3 of validators must claim to have their chunks

</pba-flex>

---

## Availability Outcome

<img src="../assets/malicious_collator_4.svg" style="width: 70%" />

---

# Collation Generation and Advertisement

---

## Collation Generation 

The last of our key processes

<pba-flex center>

1. Relay node imports block in which this parachain's availability core was vacated
1. `CollationGeneration` requests a collation from the collator
1. Parachain consensus decides whether this collator can author
1. Collator proposes, seals, and imports a new block
1. Collator bundles the new block and information necessary to process and validate it, a **collation!**

</pba-flex>

Notes:
- Aura is current default parachain consensus, but this consensus is modular and changeable

---

## Collation Distribution

<img src="../assets/para-relay_communication_1.svg" style="width: 1100px" />

Notes:

A subset of Para-Relay communication

---

#### From Collator to Relay Node and Parachain Nodes

- Sent from Collator, which owns both `CollatorService` and `ParachainConsensus`
- Sent to tethered relay node `CollationGeneration` subsystem to be repackaged and forwarded to validators
- Sent to parachain node import queues 

```rust
let result_sender = self.service.announce_with_barrier(block_hash);

tracing::info!(target: LOG_TARGET, ?block_hash, "Produced proof-of-validity candidate.",);

Some(CollationResult { collation, result_sender: Some(result_sender) })
```

---

# How Cumulus Collations Enable Parablock Validation

---

### What is Runtime Validation?

- The relay chain ensures that every parachain block follows the rules defined by that parachain's current code.

<!-- .element: class="fragment" data-fragment-index="1" -->

- Constraint: the relay chain must be able to execute Runtime Validation of a parachain block without access to the entirety of that parachain's state

<!-- .element: class="fragment" data-fragment-index="2" -->

<div class="r-stack">
<img src="../assets/runtime_validation_1.svg" style="width: 60%" />
<img src="../assets/runtime_validation_2.svg" style="width: 60%" />
<!-- .element: class="fragment" data-fragment-index="1" -->
<img src="../assets/runtime_validation_3.svg" style="width: 60%" />
<!-- .element: class="fragment" data-fragment-index="2" -->
</div>

</br>

- Building Blocks to make this possible, the PVF and PoV, are delivered within collations

<!-- .element: class="fragment" data-fragment-index="3" -->

---

#### Parachain Validation Function - PVF

- The STF of the Parachain must be stored on the Relay Chain

```rust [6]
/// A struct that carries code of a parachain validation function and its hash.
///
/// Should be cheap to clone.
#[derive(Clone)]
pub struct Pvf {
    pub(crate) code: Arc<Vec<u8>>,
    pub(crate) code_hash: ValidationCodeHash,
}
```

</br>

- New state transitions that occur on a parachain must be validated against the registered parachain code via the PVF

Notes:

The code is hashed and saved in the storage of the relay chain. There is another map in the storage where the paraId is the key and the ValidationCodeHash (the hash of the PVF) is the value.

---

---

## Proof of Validity (PoV)

---

## How Cumulus Sets and Updates the PVF

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

## Cumulus and Parachain Runtime Upgrades

- Every Substrate blockchain supports runtime upgrades
<!-- .element: class="fragment" data-fragment-index="0" -->

##### Problem

<!-- .element: class="fragment" data-fragment-index="1" -->

- What happens if PVF compilation takes too long?
  <!-- .element: class="fragment" data-fragment-index="1" -->
  - In approval checking there may be many no-shows leading to slow finality
  - In disputes neither side may reach super-majority. Nobody will get slashed and the chain will not be reverted or finalized.

<!-- .element: class="fragment" data-fragment-index="1" -->

</br>
  
- Updating a Parachain runtime is not as easy as updating a standalone blockchain runtime
<!-- .element: class="fragment" data-fragment-index="2" -->

---v

##### Solution

The relay chain needs a fairly hard guarantee that PVFs can be compiled within a reasonable amount of time.

<!-- .element: class="fragment" data-fragment-index="0" -->

</br>

- Collators execute a runtime upgrade but it is not applied
- The relay chain executes the **PVF Pre-Chekcing Process**
- The first parachain block to be included after the end of the process needs to apply the new runtime

<!-- .element: class="fragment" data-fragment-index="1" -->

</br>

Cumulus follows the relay chain, waiting for a go ahead signal to apply the runtime change

<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

https://github.com/paritytech/cumulus/blob/master/docs/overview.md#runtime-upgrade

---v

##### PVF Pre-Checking Process

- The relay chain keeps track of all the new PVFs that need to be checked
- Each validator checks if the compilation of a PVF is valid and does not require too much time, then it votes
  - binary vote: accept or reject
- As soon as a super majority is reached the voting is concluded
- The state of the new PVF is updated on the relay chain

Notes:

reference: https://paritytech.github.io/polkadot/book/pvf-prechecking.html

---

## References

1. Gabriele Miotti, who was a huge help putting together these slides
1. https://github.com/paritytech/cumulus/blob/master/docs/overview.md

---

<!-- .slide: data-background-color="#4A2439" -->

## Questions