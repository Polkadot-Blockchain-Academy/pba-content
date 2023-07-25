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
  * fee per blockspace
* This is all off-chain

---

##