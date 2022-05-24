# Lesson 5: Database & Merklized Storage

## Why is this topic Important?

- To understand how to write runtime code you need to understand how the underlying storage layers work and their behavior.

## Core Ideas to Convey

- Storage layers in Substrate
  - Runtime interface (sp-io)
  - Overlays
  - Patricia-Merkle trie
    - parity-trie
  - Database backend
    - kvdb or in-mem or whatever
  - Make clear how much each read-write costs
    - Runtime -> overlay is cheap, etc.
    - Where are the bottlenecks?
    - Don't have to pay the hashing or disk access
- Dive into Databse Backend - Potential in depth explanation, if already not covered in M3 - Root - Storage Proofs
- Dive into Patricia-Merkle Trie
  - Patricia Trie
  - Merkle Tree
    - Hashing
    - Root
    - Proof
    - Complexity
- Dive into Overlay
  - Optimization
    - minimize backend writes
    - minimize calculating storage root
- Dive into Runtime Interface:
  - `root()`, `get()`, `set()`.
    - runtime sees state is KV.
  - this KV in the client is a merkle tree

**Further reading (extra):**

- **Alternative storage structur:** Verkle tries and Sparse Merkle tries
- **Child storage**

## Prerequisite Knowledge or Skills

- Hashing schemes
- SCALE / Binary encoding and representation

## Learning Outcome

- Understand the main components that Substrate storage system (and eventually service the runtime) and know the tradeoffs to be able to make informed decisions.

## Learning Objectives

- Understand why the abstraction of merkle tree is even needed (A: efficient root calculation)
- Understand which parts of storage are in memory and which parts in disk.
  - _bound to whether we will talk about overlay or not_
- List the different storage layers used in a Substrate blockchain and justify why they are used.
  - _This would cover the high level “why” (building from pre-req modules), the different components (Merkle trie, KVDB, overlay change set), why they exist, why base 16._
- Name the different complexities of the Merkle tree operations

## Activities and Exercises

1. Mental Mapping of Database Keys and Trie Keys
   - Use the Trie / `sp-io` crate to manually manipulate storage
   - Manually query a storage item in Polkadot (code, parachains messages)
2. Blockchain and Substrate Storage Behaviours

   - Circle nodes that will be pruned when a block is removed.
   - Create a trie which reuses data from old blocks.

3. Manual pruning exercise

- Retaining only the old Merkle Trie nodes
