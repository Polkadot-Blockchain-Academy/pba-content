---
title: Substrate's Transaction Pool and its Runtime API
duration: 30 minutes
---

# Substrate's Transaction Pool

---

## Transaction Pools

<img src="./img/tx-pool/BlockspaceBooth.png" />


Notes:
The blockchain produces blockspace, and users buy that blockspace.
Why do they buy it?
So they can contribute to the shared story.
So they can interact with the shared state machine.
You can think of these users standing in line with transactions in their hand, waiting for the chance to put their transactions into the chain's blockspace.
Sometimes the demand for blockspace is low and the queue is short.
In this case the queue gets completely emptied each time a new block is created.
Other times it gets long and backed up.
Then when a block comes, only a part of the queue gets emptied.

This simple model provides some good intuition about how the transaction pool works, but it is a bit simplified.

First, It is actually a priority queue.
You can jump the line by offering to bribe the block producers.

Second, it is more accurate to think of the transactions themselves waiting in line, not the users who sent those transactions.

Let's take a closer look.

---v

### Paths of a Transaction

TODO figure of multiple nodes each with its own view of chain and pool

Notes:
There are many paths a transaction can take from the user who signed it to a finalized block.
Let's talk through some.
Directly to user's authoring node and into chain is simplest.
Could also be gossiped to other author.
Could even go in a block, get orphaned off, back to tx pool, and then in a new block

---v

### Pool Validation

* Check signature
* Check that sender can afford fees
* Make sure state is ready for application

Notes:
When a node gets a transaction, it does some light pre-validation sometimes known as pool validation.
This checking determines whether the transactions is {valid now, valid in some potential future, invalid}.
There is periodic re-validation if transactions have been in the pool for a long time.

---v

### Pool Prioritization

* Priority Queue
* Prioritized by...
  * Fee
  * Bribe
  * Fee per blockspace
* This is all off-chain

---

## Tx Pool Runtime Api

TODO paste trait definition

Notes:

This is another runtime api, similar to the block builder and the core that are used for creating and importing blocks.
This one is slightly different in that it is actually called from off-chain, and is not part of your STF.
So let's talk about that for a little bit.

---v

### slide title

<img src="./img/tx-pool/peter-parker-glasses-off.png" />

Notes:
It is commonly said that the runtime is basically your STF.
This is a good first order approximation.
It is nearly true.

---v

### slide title

<img src="./img/tx-pool/peter-parker-glasses-on.png" />

Notes:

But as we can see here, when we put our glasses on, actually only some of the apis are part of the stf.

---v

## Why is pool logic in the runtime?

* Types are upgradable
* Transactions are upgradeable
* If you want to prioritize transactions, you need to understand the transaction

Notes:

So if this is not part of the STF why is it in the runtime at all?
This logic is tightly related to the runtime application logic.
The types are opaque outside of the runtime.
So this logic must go in the runtime.

But if it is not on-chain, can individual validators customize it.
In short yes.
There is a mechanism for this.
We won't go deeply into the mechanism, but validators can specify alternate wasm blocs to use instead of the official one.