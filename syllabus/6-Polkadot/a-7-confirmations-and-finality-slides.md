---
title: Confirmations and Finality
description: Finality providing methods and mechanisms for blockchains
duration: 1 hour
owner: Maciej Zyszkiewicz (Joshy originally)

---

# Confirmations and Finality

---

# What is Finality?

Notes:
In general finality will be conneced to some assurances or promises, implicit or explicit that transactions will not be reversed.

---v

# What is Finality?

How does BTC answer this question?

---v

## Finality in Nakamoto Consensus

<pba-flex center>

- Longest chain is "best"... for now
- Someone could always start mining a chain<br/>and, with low but non-zero probability,<br/> end up with it longer.
- There could _already_ be a longer chain<br/>that you just haven't heard of.

<pba-flex>


Notes:

This is to say that Nakamoto consensus is NOT safe on the real asynchronous internet. In practice, as long as
blocks can be downloaded and executed much more quickly than the target block time, it is usually good enough.

- Longest could also mean most accumulated work

---v

# Probabilistic Finality

- NOT binary
- Expressed as a probability (%)
- Represents confidence that a block will not be reversed
- The more work is put on top of the block the bigger the confidence
- Implicit

---

# Deterministic Finality

- Binary
- Explicit
- Requires consensus to establish

---v

# Deterministic Finality

<pba-flex center>

- Based on traditional methods (BFT)
- Requires an honest-majority finite authority set

</pba-flex>

<!-- two flexes needed so the next bullet points don't show too early, margin hack... TODO -->

<pba-flex center style="margin-left: 48px">

- Consensus protocol that assumes honest majority
- Economic game that keeps them honest

</pba-flex>
<!-- .element: class="fragment"-->

Notes:

If you want deterministic finality, it basically means employing BFT agreement protocols that we talked about in the history lesson. This means we need a finite authority set with an honest majority. And that means we need incentives to keep them honest.

---v

# Incentives: Game Theory!

Abstractly: You behave honestly<br/>when the utility of doing so exceeds the cost.

Incentive designers may potentially:

<pba-flex center>

- Reward honest behavior
- Punish (aka slash) dishonest behavior

</pba-flex>

Notes:

Many systems use both of these, but doing so is not strictly necessary. Even without slashes, the opportunity cost of staking and the missed rewards from authoring invalid blocks may be sufficient.

It is often the case that blockchain systems give rewards in the authorship and punishments in the finality. There is no fundamental reason for this; it is just a little more straightforward to implement.

---

# Case Study: Tendermint

<pba-flex center>

- Authorship is like Aura - simple round robin
- Naive but **simple** BFT implementation
- If the block has enough votes<br/>by the end of the slot, it is finalized.<br/>
  Otherwise, it is rejected via timeout.
- "Instant finality"
- Forkless - Forks are disallowed<br/>because blocks can only be authored<br/>on finalized parents.

</pba-flex>

Notes:

Tendermint assumes a partially synchronous network, like all the BFT based systems - That is to say that messages may not arrive immediately, but will arrive within a finite time bound. In practice this means it is slot-based just like so many of the authoring schemes.

Tendermint is often touted as "instant finality". It is instant in the sense that finality is tied to authorship. In practice this means that authorship, which is inherently O(n), is slowed down to stay in sync with finality which is O(n^2). They sacrifice liveness to guarantee absolute safety at all times.

---v

# Tendermint Deep Dive

<pba-flex center>
<ol>
<li class="fragment">Wait for a block (or author one if it is your turn)</li>
<li class="fragment">Prevote
  <ul>
    <li>If the block is valid, Prevote for it.</li>
    <li>If the block is invalid, Prevote `Nil`</li>
  </ul>
</li>
<li class="fragment">Precommit
  <ul>
    <li>Wait for 2/3 prevotes then Precommit</li>
    <li>If you don't get 2/3 prevotes, Precommit `Nil`</li>
  </ul>
</li>
<li class="fragment">Complete
  <ul>
  <li>Wait for 2/3 Precommits them finalize</li>
  <li>If you don't get it, throw the block away</li>
  </ul>
</li>
</ol>

[Very useful blog post](https://medium.com/softblocks/explaining-how-tendermint-consensus-works-433066cbc465) <!-- .element: class="fragment" -->

</pba-flex>

---

# Hybrid Consensus

<pba-cols>
<pba-col>

<img rounded style="width: 900px;" src="./img/grandpa-abstract.png" />

</pba-col>
<pba-col>
<pba-flex center style="font-size: 0.7em; margin-left:-50px">

- Separates block production from finality.
- Block production stays live even if finality lags.
- Allows lower overhead in the finality layer.
- Used in Substrate (BABE + GRANDPA)

</pba-flex>
</pba-col>
</pba-cols>

---v

# What About Re-Orgs

<img style="width: 500px; margin-right: 150px;" src="./img/reorgs-1.svg" />
<br/>
<img style="width: 650px;" src="./img/reorgs-2.svg" />

Notes:

Previously we talked about how a node's view of the best block can change, and that is called a re-org.

---v

## Modified Fork Choice Rule

<img style="width: 500px; margin-right: 150px;" src="./img/reorgs-finality-1.svg" />
<br/>
<img style="width: 650px" src="./img/reorgs-finality-2.svg" />

Only extend best finalized chain
Notes:

Once you have a finality gadget installed, you have to make sure you only ever author on top of finalized blocks. Even if another chain is longer.

---

# Case Study: Grandpa

<pba-flex center>

- Deterministic finality _only_
- Requires an external block authoring scheme<br/> with its own liveness proof.
- Kind of like Tendermint but better.
- Finalizes chains, not blocks.

</pba-flex>

---v

# GRANDPA

- finality gadget
- does not manage block production
- requires 2/3 for consensus
- [...]

---v

# GRANDPA

- finalizes chains and not just blocks

<img style="width: 1000px" src="./img/grandpa-round/4.6-validators-vote-on-chain.png"/>

Notes:
BFT finality with $n$ authorities is in $O(n^2)$.
Tendermint does this at **every block**.
This bounds the size of the authority set.

With separated, we treat each vote as a vote not only for one block,but also for each ancestor block.
This significantly reduces the number of total messages sent.
Allows the chain to stay live even when many validators are offline

---v

# Grandpa Prerequisite

- Only fully approved (approval checking) blocks are considered
- Finality is at the end of the ELVES Pipeline
  - Collation, Backing, Availability, Approval Checking -> FINALITY

---v

## A GRANDPA Round

Each validator **broadcasts** a **prevote** for the **highest block** that it thinks **should** be **finalized**

- For **honest** validators, this block **must include** the chain that was previously finalized
- This **new chain** could be **several blocks** longer than the **last finalized** chain

A **validator** that is designated as the **primary** broadcasts the highest block that it thinks could be final from the previous round

---v

## A GRANDPA Round

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-1.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-2.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round

Each validator **computes** the **highest block** that can be **finalized based** on the set of **prevotes**

- i.e. we find the **common ancestor** of all votes that has **support** from **more** than $\frac{2}{3}N + 1$ validators
- We refer to this block as the **prevote GHOST**

---v

## A GRANDPA Round

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-3.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round

We define the round **estimate** as the **highest ancestor** of the **prevote GHOST** for which it is **still possible** to **precommit**

- i.e. when **no precommit** votes have been **sent** out yet, then:

`estimate == prevote GHOST`

---v

## A GRANDPA Round

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-4.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round

If the **estimate** extends the **last finalized** chain, then each validator will cast a **precommit** for that chain.

Each validator **waits** to receive **enough precommits** to be able to finalize

- We again find the **common ancestor** of the **estimate** which has **threshold precommits**
- We declare that **block** as **finalized**

---v

## A GRANDPA Round

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-5.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-6.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round DEEP DIVE

---v

## A GRANDPA Round DEEP DIVE

The **round** is deemed **completable**:

- if the **estimate** is **lower** than the **prevote GHOST**
- or if it's **impossible** to have a **supermajority** on any **block higher** than the current **estimate**

Validators **start** a **new round** after it becomes **completable**.

---v

## A GRANDPA Round

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-6.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-7.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-8.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round (Alt)

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-6.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round (Alt)

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-7-alternative.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round (Alt)

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-8-alternative.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---v

## A GRANDPA Round (Alt)

<img style="width: 400px" src="./img/grandpa-round/4.6-grandpa-round-9-alternative.png"/>

Notes:

- 7 = # Validators
- 5 = # Threshold

---

# Pre-Grandpa Guarantees

- Only fully approved (approval checking) blocks are considered
- Finality is at the end of the ELVES Pipeline
  - Collation, Backing, Availability, Approval Checking, Disputes -> FINALITY

---v

# Pre-Grandpa Guarantees

## Approvals

- Block was already audited, we know it is fully secured

---v

# Pre-Grandpa Guarantees

## Availability

- Block is not fully secured yet, but 
- If it will be invalid there will be high slashes

---v

# Pre-Grandpa Guarantees

## Backing

- In most happy path situations block will make it to further but
- No availability means withholding attacks etc are possible
- Suitable for low stake operations or mid level signalling

---v

# Pre-Grandpa Guarantees

## Collation

- No security guarantees
- Useful for early signalling and confirmations (early signalling)
- Can be very low latency
- In practice most often makes it to finality
- We can make more improvements here

---v

# Pre-Grandpa Guarantees

## TX Pool

- Ultra early signalling
- Your TXs will most likely eventually make it in but is now pending
- Usefull to have near istant feedback
- TX pools do light TX validation so they will reject wrong Txs (good for UX)

---

# Summary

<pba-flex center>

- Networks can be {Synchronous, Asynchronous}
- Consensus systems can be {Deterministic, Probabilistic}
- Consensus systems can be {Open participation, Finite participation}
- There is always an assumption that at least {1/2, 2/3} participants are honest
- In decentralized systems, we use Economics and Game Theory<br/>to incentivize honest execution of the consensus protocol

---

# Questions
