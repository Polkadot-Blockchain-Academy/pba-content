---
title: FRAME Benchmarking
description: How to benchmark Pallets in FRAME.
duration: 2 hours
instructors: ["Shawn Tabrizi"]
---

# FRAME Benchmarking

---

## Overview

- Quick Recap of Weights
- Deep Dive Into Benchmarking
- Our Learnings Throughout Development
- Best Practices and Common Patterns

---

## Blockchains are Limited

Blockchain systems are extremely limited environments.

Limited in:

- Execution Time / Block Time
- Available Storage
- Available Memory
- Network Bandwidth
- etc...

---

## Performance vs Centralization

Nodes are expected to be decentralized and distributed.

Increasing the system requirements can potentially lead to centralization in who can afford to run that hardware, and where such hardware may be available.

---

## Why do we need benchmarking?

Benchmarking ensures that when users interact with our Blockchain, they are not using resources beyond what is available and expected for our network.

TODO: improve wording.

---

## What is Weight?

Weight is a general concept used to track consumption of limited blockchain resources.

---

## What is Weight in Substrate?

We currently track just two main limitations:

- Execution Time on "Reference Hardware"
- Size of Data Required to Create a Merkle Proof

```rust
pub struct Weight {
	/// The weight of computational time used based on some reference hardware.
	ref_time: u64,
	/// The weight of storage space used by proof of validity.
	proof_size: u64,
}
```

This was already expanded once, and could be expanded in the future.

---

## Weight limits are specific to each blockchain.

- 1 second of compute on different computers allows for different amounts of computation.
- Weights of your blockchain will evolve over time.
- Higher hardware requirements will result in a more performant blockchain (i.e. TXs per second), but will limit the kinds of validators that can safely participate in your network.

---

## What can affect relative Weight?

<pba-cols>

<pba-col>

- Processor
- Memory
- Hard Drive
	- HDD vs. SSD vs. NVME
- Operating System
- Drivers

</pba-col>
<pba-col>

- Rust Compiler
- Runtime Execution Engine
	- compiled vs. interpreted
- Database
	- RocksDB vs. ParityDB vs. ?
- and more!

</pba-col>
</pba-cols>

---

## Block Import Weight Breakdown

<table style="width: 1000px">
<tr>
<td>Available</td>
<td>Buffer</td>
</tr>

</table>
