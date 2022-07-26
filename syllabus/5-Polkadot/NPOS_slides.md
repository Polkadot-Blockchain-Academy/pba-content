---
title: Nominated Proof of Stake
description: An introduction to Nominated Proof of Stake in Polkadot
duration: 1 hour
instructors: ["Kain Paimani"]
---

# Nominated Proof of Stake

## _in Polkadot_

---

## Why Proof of Stake Ser?

Why do we use PoS?

<div>

Economic Security 💸🤑

Tokens locked + prone to being slashed.

</div>

<!-- .element: class="fragment" -->

Everything else (finality, parachains, etc.) is built on top of this base layer of economic security.

<!-- .element: class="fragment" -->

---

## What is NPoS: Assumptions

Assumptions:

- **Validators**: those who wish to author blocks.
- **Nominators/Delegators**: Those who wish to support wanna-be authors.

<br>

- Validation and nomination intentions can change, therefore we need **periodic elections** to
  always choose the best validators + hold them slashable.
- Every election period is called an **_Era_**, e.g. 24hrs in Polkadot.

---v

### What is NPoS: Re-inventing the Wheel

**Solo-POS**

Authority-wanna-bees aka. validators bring their own stake. No further participation. Top validators
are elected.

Problems?

Notes:

Low amount of stake that we can capture, impossible for those who don't want to run the hardware to join.

---v

### What is NPoS: Re-inventing the Wheel

**Single-Delegation-POS**

Anyone can dilute themselves in any given validators. Top validator based on total stake are
elected.

Problems?

Notes:

Better, but funds might be delegated to non-winners, which get wasted.

---v

### What is NPoS: Re-inventing the Wheel

**Multi-Delegation-POS**

Your stake is divided $\frac{1}{N}$ among $N$ validators.

Problems?

Notes:

Same issue as before.

---v

### What is NPoS: Re-inventing the Wheel

**Nominated Proof of Stake**

You name up to `N` nominees, an _arbitrary algorithm_, computed either onchain or offchain, decides
the **winners** and **how to distribute the stake among them**.

- ✅ Can optimize other criteria other than "who had more approval votes".
- ✅ Has a much higher chance to make sure staked tokens won't get wasted.

<br>

> NPoS is **approval-based, multi-winner election**.

---

### NPoS Protocol Overview

The current NPoS protocol revolves around an **election round**, which is itself made up of 4
episodes.

---v

### NPoS Protocol Overview

1. Validator + Nominator **Snapshot**

- Allows us to index stakers, not AccountIds.
- Allows us to not need to "freeze" the staking system.

---v

### NPoS Protocol Overview

2. Signed Phase

Any signed account can come up with a **NPoS solution** based on that snapshot.

Deposits, rewards, slash, other game-theoretic tools incorporated to make to secure.

---v

### NPoS Protocol Overview

3. Unsigned Phase

As the first backup, any validator can also submit a solution as a part of their block authoring.

---v

### NPoS Protocol Overview

4. Fallbacks

If all of the above fails, the chain won't rotate validators and the governance can either:

- dictate the next validator set
- trigger an onchain election (limited in what it can do)

---

## Why NPoS

1. Polkadot validators are the source of truth for the state transition of both the relay chain and all of the parachains + bridges.
1. Polkadot validator are assigned to parachains as backing group, and swapped over time.
1. Polkadot validators all author the same number of blocks.

<br>

> What properties to we want a validator set have for the above requirements?

Notes:

Point 2 is not to imply that the polkadot validator set's security is partitioned among parachains,
security comes from approval voters.
https://polkadot.network/blog/polkadot-v1-0-sharding-and-economic-security/

---v

### Why NPoS: Election Score

```rust
pub struct ElectionScore {
  /// The minimal winner, in terms of total backing stake.
  ///
  /// This parameter should be maximized.
  pub minimal_stake: u128,
  /// The sum of the total backing of all winners.
  ///
  /// This parameter should maximized
  pub sum_stake: u128,
  /// The sum squared of the total backing of all winners, aka. the variance.
  ///
  /// Ths parameter should be minimized.
  pub sum_stake_squared: u128,
}
```

---v

### Why NPoS: Election Score

<br>

> NPoS allows us to incentivize the formation of a validator set that optimized the aforementioned `ElectionScore`.

---

## NPoS Drawbacks

<widget-text center>

- scalability. <!-- .element: class="fragment" -->
- scalability. <!-- .element: class="fragment" -->
- scalability. <!-- .element: class="fragment" -->
- scalability. <!-- .element: class="fragment" -->
- and scalability. <!-- .element: class="fragment" -->

</widget-text>

But we (strive to) get much better economic security measures in return.<br>
And solve the scalability in the mid-term too 🤫
<!-- .element: class="fragment" -->

---

## NPoS Protocol: Proportional Representation

Not strictly checked at the moment, but a core component in w3f's initial design to make NPoS what
it is now.

> Among all nominators $N$, if a minority $N^′$ has at least $t$ commonly trusted candidates, to whom it could "afford” to provide with an average support of at least $\frac{1}{n_{val}}$ $\sum_{n∈N} stake_n$, then this minority has a justified claim to be represented in $V$ by at least $t$ candidates.

---v

### NPoS Protocol: Proportional Representation

We initially chose an algorithm called `Sequential Phragmen`, which fulfills proportional justified
representation (**PJR**).

We have the code to do a PJR check, but it is not feasible to do onchain, and hasn't been a high
priority for us either.

---

## NPoS Future

- First, repay any technical debt, make everything ready for any further scaling.
- Infra for multi-block election
  - Onchain
  - Offchain
- Staking/Election parachain
- More (tax) friendly/configurable reward payout.

---

## Further Reading

- [A verifiably secure and proportional committee election rule](https://arxiv.org/abs/2004.12990)
- 4.1 in [Overview of Polkadot and its Design Considerations](https://arxiv.org/abs/2005.13456)
- [Proportional Justified Representation](https://arxiv.org/abs/1611.09928)
- [Justified representation - Wikipedia](https://en.wikipedia.org/wiki/Justified_representation)

---

## NPoS Protocol: More Details, Backup Slides

- `bags-list`: how to store an unbounded semi-sorted linked-list onchain.
- Nomination pools: best of both.
- Minimum-untrusted score.
- PJR checking: why we don't do it.
- `reduce` optimization.
