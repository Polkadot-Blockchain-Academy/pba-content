---
title: Deep Dive, Availability Cores
description: The Polkadot Abstraction for Flexible Blockspace Allocation
duration: 1 hour
---

# Deep Dive, Availability Cores

Notes:

Hello!

I'm Bradley Olson

Was student at first Academy in Cambridge

Currently on Parachains Core Team at Parity

Will present 3 lectures providing a window into Polkadot core, a slice of where we're at and where we're headed.

First a look at availability cores, the abstraction enabling flexible purchases of blockspace under the umbrella of Polkadot shared security.

Lets get to it

---

## Availability Core Defined

<pba-cols>
<pba-col center>

  - Availability cores are the abstraction that allocates Polkadot's validation of parachain block candidates.
  - Cores divide validation into discrete units of capacity, ~1 core per 5 validators
  - Why "availability"?
  - Why "core"?

</pba-col>
<pba-col center>

<img rounded style="width: 500px" src="../../../assets/img/5-Polkadot/Availability_Cores_Deep_Dive/Processor_Cores.jpeg" alt="Processor cores image">

</pba-col>
</pba-cols>




Notes:

- "Availability", because a core is considered occupied by a parachain block candidate during the period when that candidate is being made available via erasure coding.
- "Core", because many candidates can be made available in parallel, mimicking the parallel computation per core in a computer processor.

---

## Overview

<pba-flex center>

- What does an availability core represent?
- How are availability cores scheduled?
- How do cores gate each step of the parachains protocol?
- What do cores give us now?
- What roadmap items do cores accomodate?

</pba-flex>

---

## Resources

<pba-col center>

1. [Implementers Guide: Scheduler Pallet](https://paritytech.github.io/polkadot/book/runtime/scheduler.html)
1. 

</pba-col>

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions