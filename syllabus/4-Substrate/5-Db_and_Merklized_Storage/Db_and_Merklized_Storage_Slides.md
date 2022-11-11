---
title: Database and Merklized Storage
description: Substrate Database and Merklized Storage for Web3 engineers
duration: 1 hour
instructors: ["Shawn Tabrizi"]
teaching-assistants: ["Sacha Lansky"]
---

# Database and Merklized Storage

---

## Database and Merklized Storage

In this section, we will learn about how the underlying storage layers in Substrate work and their behavior.

---

### Why is this topic important?

- To understand the main components of Substrate's storage system and how it serves the runtime.
- To make correct decisions when designing new runtime modules.

---

<div class="flex-container">

<div class="left-small">
	<table class="storage-layer-table">
	<tr><td class="ends">Developer</td></tr>
	<tr><td>Runtime Storage API</td></tr>
	<tr><td>Storage Overlays</td></tr>
	<tr><td>Patricia-Merkle Trie</td></tr>
	<tr><td>Key-Value Database</td></tr>
	<tr><td class="ends">Computer</td></tr>
	</table>
</div>
<div class="right">

### Storage layers

There are four core layers to Substrate's storage system.

</div>
</div>

---

<div class="flex-container">

<div class="left-small">
	<table class="storage-layer-table">
	<tr><td class="active">Runtime Storage API</td></tr>
	<tr><td>Storage Overlays</td></tr>
	<tr><td>Patricia-Merkle Trie</td></tr>
	<tr><td>Key-Value Database</td></tr>
	</table>
</div>

<div class="right">

### Runtime Storage API

- Exposed to the runtime via `sp-io` crate.
- Can write to storage with a given key + value.
- Substrate has macros that generate APIs to create different storage items as well as read/write to them.
  - This is one of the things FRAME does for you.

</div>
</div>

---

<div class="flex-container">

<div class="left-small">
	<table class="storage-layer-table">
	<tr><td>Runtime Storage API</td></tr>
	<tr><td class="active">Storage Overlays</td></tr>
	<tr><td>Patricia-Merkle Trie</td></tr>
	<tr><td>Key-Value Database</td></tr>
	</table>
</div>

<div class="right">

### Storage Overlays

- Think of Storage Overlay as an in-memory representation of the changes that should be made to the underlying database.

  - Like a cache; used for optimizations.

- There are currently two kinds of storage overlay:
  - Overlay Change Set
  - "Transactional" Overlay

</div>
</div>

---

<div class="flex-container">

<div class="left-small">
	<table class="storage-layer-table">
	<tr><td>Runtime Storage API</td></tr>
	<tr><td class="active">Storage Overlays</td></tr>
	<tr><td>Patricia-Merkle Trie</td></tr>
	<tr><td>Key-Value Database</td></tr>
	</table>
</div>

<div class="right">

### Overlay Change Set

- Stages changes to the underlying DB.
- Overlay changes are committed once per block.
- Once a change gets here, it will be committed to the DB.

</div>
</div>

---

<div class="flex-container">

<div class="left-small">
	<table class="storage-layer-table">
	<tr><td>Runtime Storage API</td></tr>
	<tr><td class="active">Storage Overlays</td></tr>
	<tr><td>Patricia-Merkle Trie</td></tr>
	<tr><td>Key-Value Database</td></tr>
	</table>
</div>

<div class="right">

### "Transactional" Overlay

- Additional overlay layers than can be spawned by the runtime developer.
- Allows a set of prospective changes to be dropped or committed to the layer below.
- Thus, a change here may not always make it to the final DB. Depends on logic.

</div>
</div>

---

<div class="flex-container">

<div class="left-small">
	<table class="storage-layer-table">
	<tr><td>Runtime Storage API</td></tr>
	<tr><td>Storage Overlays</td></tr>
	<tr><td class="active">Patricia-Merkle Trie</td></tr>
	<tr><td>Key-Value Database</td></tr>
	</table>
</div>

<div class="right">

### Patricia-Merkle Trie

- [paritytech/trie](https://github.com/paritytech/trie)
- Data structure on top of KVDB
- Arbitrary Key and Value length
- Nodes are Branches or Leaves

<img src="../../../assets/img/4-Substrate/small-trie.svg" style="width: 4000px" />

</div>
</div>

---

<div class="flex-container">

<div class="left-small">
	<table class="storage-layer-table">
	<tr><td>Runtime Storage API</td></tr>
	<tr><td>Storage Overlays</td></tr>
	<tr><td>Patricia-Merkle Trie</td></tr>
	<tr><td class="active">Key-Value Database</td></tr>
	</table>
</div>

<div class="right">

### Key-Value Database

- a.k.a. KVDB
- Implemented with RocksDB and ParityDB
- Just KV mapping: `Hash -> Vec<u8>`
- Substrate: Blake2 256

<br>
<table>
<th>Key (Hash 256)</th>
<th>

Value (`Vec<u8>`)

</th>
<tr><td>0x0fd923ca5e7...</td><td>[00]</td></tr>
<tr><td>0x92cdf578c47...</td><td>[01]</td></tr>
<tr><td>0x31237cdb79...</td><td>[02]</td></tr>
<tr><td>0x581348337b...</td><td>[03]</td></tr>
</table>

Notes:

There could also be some in-memory KVDB layer for testing purposes.

Memory KVDB is also notably used as a triedb backend during proof verification.

---

### Substrate Uses a Base-16 Patricia Merkle Trie

(hopefully you remember these key terms)

https://github.com/paritytech/trie

---

### Merkle Tree

<img style="width: 1200px;" src="../../../assets/img/1-Cryptography/Merkle-Tree.png">

---

### Patricia Trie

<div class="flex-container">
<div class="left-small">

<img src="../../../assets/img/4-Substrate/patricia-trie.svg" />

</div>

<div class="right">

- Position in the tree defines the associated key.
- Space optimized for elements which share a prefix.

</div>
</div>

---

### Base 16

<div class="flex-container">
<div class="left">

<img src="../../../assets/img/4-Substrate/base-16-labeled.svg" />

</div>

<div class="right">

- We will mostly show binary trees for simplicity.
- But everything scales up as you add more nodes.
- 16 is a nice choice because it is 1/2 of a byte (two hex characters)
  - one hex character is a "nibble"

</div>
</div>

---

### Merkle Trie Complexity

- Reading
- Writing
- Proofs

---

### Merkle Read

<div class="image-container">

<img src="../../../assets/img/4-Substrate/merkle-read.svg" />

<div class="top-right">

- $O(\log{n})$ reads
- Not so great.

</div>
</div>

---

### Merkle Write

<div class="image-container">

<img src="../../../assets/img/4-Substrate/merkle-write.svg" />

<div class="top-right text-small">

- Very expensive for a database
- $O(\log{n})$ reads, hashes and writes

</div>

<div class="bottom-left black-box text-small">

1. Follow the trie path to the value: $O(\log{n})$ reads
2. Write the new value: 1 write
3. Calculate new hash: 1 hash
4. Repeat (2) + (3) up the trie path: $O(\log{n})$ times

</div>
</div>

---

### Merkle Proof

<div class="image-container">

<img src="../../../assets/img/4-Substrate/merkle-proof.svg" />

<div class="top-right text-small">

- $O(\log{n})$
- Great for light clients!
- Low bandwidth, low computation!

</div>

<div class="bottom-left black-box text-small">

1. Full Node: Follow the trie path to the value: $O(\log{n})$ reads.
1. Full Node: Upload data of trie nodes read.
1. Light Client: Download trie node data.
1. Light Client: Verify by hashing: $O(\log{n})$ hashes.

</div>
</div>

Notes:

- Message is that proof is just enough trie content (can be a bag of node or some ordered node that needs to be complete with hashing as in compact proof TODO should we make a slide for compact proof and generally proof serialization?) to build a subset/subpart of the full state trie.

This incomplete trie will then be accessed and used identically as the full state trie, but if access is not part of the proof, then the action is not finishing: Proof Incomplete case.

Invalid proof are proof where the hashing don't match (can be see as multiple trie).

TODO Could have some schema with the full state, then the proof and then two query on the proof: one that access data available and one that fail because incomplete : Probably already exists in storage deep dive

---

### Two Kinds of Keys

The next slides will try to have you understand the difference and existence of:

<br>
<div class="text-center">

1. Trie key path
2. KVDB key hash

</div>

<br>

It can be very confusing to mix these up, so lets make it clear the difference!

Notes:

Storage access path (corresponding to the `Runtime Api` level of the overview slide) can be seen as a third kind (module, storage structure and possibly key in storage structure) (translate to trie key that then query KVDB hash).

---

### What You Will See

<img src="../../../assets/img/4-Substrate/navigate-storage-toc.svg" />

---

### Navigating Substrate Storage

<img src="../../../assets/img/4-Substrate/navigate-storage-1.svg" />

---

### Navigating Substrate Storage

<img src="../../../assets/img/4-Substrate/navigate-storage-2.svg" />

---

### Navigating Substrate Storage

<img src="../../../assets/img/4-Substrate/navigate-storage-3.svg" />

---

### Navigating Substrate Storage

<img src="../../../assets/img/4-Substrate/navigate-storage-4.svg" />

---

### Navigating Substrate Storage

<img src="../../../assets/img/4-Substrate/navigate-storage-5.svg" />

---

### Navigating Substrate Storage

<img src="../../../assets/img/4-Substrate/navigate-storage-6.svg" />

---

### Navigating Substrate Storage

<img src="../../../assets/img/4-Substrate/navigate-storage-7.svg" />

---

### What You Just Saw

<div class="flex-container">
<div class="left">

<div>

<img src="../../../assets/img/4-Substrate/patricia-trie-path.svg" style="width: 1000px" />

</div>

<br>

Patricia provides the **trie path**.

</div>
<div class="right">

<div>

<img src="../../../assets/img/4-Substrate/merkle-path.svg" style="width: 1000px" />

</div>

<br>

Merkle provides the recursive **hashing** of children nodes into the parent.

</div>
</div>

---

<table style="background-color: white; color: black; width: 80%;" class="text-center">
	<tr><td style="background-color: lightgrey;" colspan="4">Trie Node</td></tr>
	<tr>
		<td style="background-color: red;">header</td>
		<td style="background-color: orange;">key</td>
		<td style="background-color: yellow;">children</td>
		<td style="background-color: green;">value</td>
	</tr>
</table>

<br>

- The Trie key path is set by you, for e.g. `:CODE`.
  - Arbitrary Length!
- Trie Node:
  - header
  - key
  - possible children
  - possible value
- KVDB key = Hash([Trie Node])

---v

### Anatomy of a node

Radix tree node:

```
  [partialkey ++ n children ++ maybe_value]
```

Merklized:

```
  [partialkey ++ n children hash ++ maybe_value_hash]
```

Node encoding variants:

```
- Branch: [Header ++ partial_key ++ maybe_value_hash ++ n_children_hash]
- Leaf: [Header ++ partial_key ++ value_hash]
- Value: [Value]
```

Notes:

- radix

Add constraint that terminal node always contains a value

- merklized

Note that merklizing this pretty common data structure just means replacing/enriching the pointers to child with a hash.

Then of course persisting nodes in some way and lazy loading by hash (unless keeping all in memory or rebuilding on flight: not substrate).

variant should be as compact as possible.

Additional subtleties that if content under hash is smaller than it s hash, then it is written in the parent node instead of hash
-> for either value or children

---

### Given a value you want to store in the database (an encoded trie node), it will always have the same database key, because the key is simply the hash.

Notes:

Rocks db implementation details, the key is dirty with some prefix.

---

## But wait... there's more!

---

### Child Trie

<div class="image-container">

<img src="../../../assets/img/4-Substrate/child-trie.svg" />

<div class="bottom-left black-box text-small">

- Allows us to get a merkle root for some subset of data.
- We aim to allow child tries to be a different trie format in the future.

</div>
</div>

---

### Pruning

<div class="image-container">

<img src="../../../assets/img/4-Substrate/pruning-1.svg" />

<div class="top-right" style="width: 40%">

- For holding older block states, and then cleaning up.
- Let’s update two values in this trie.

</div>
</div>

---

### Pruning

<div class="image-container">

<img src="../../../assets/img/4-Substrate/pruning-2.svg" />

<div class="top-right" style="width: 40%">

- We create new database entries, but keep the old ones too!

</div>
</div>

---

### Pruning

<img src="../../../assets/img/4-Substrate/pruning-3.svg" />

---

### Pruning

<div class="image-container">

<img src="../../../assets/img/4-Substrate/pruning-4.svg" />

<div class="top-left" style="width: 30%">

- Eventually, we prune the old data.

</div>
</div>

---

### Unbalanced Tree

<div class="flex-container">
<div class="left-small">

<img src="../../../assets/img/4-Substrate/unbalanced-tree.svg" />

</div>
<div class="right">

- Operations can be more (bad) or less (good) than the expected $O(\log{n})$ average.
- This can lead to DoS attacks.
  - Can happen if the user can influence the trie path.
  - In FRAME we will talk about how we prevent this.
- Or we can use this as a feature to access specific storage cheaply.
  - `:code`
  - Each pallet gets its own prefix.
  - etc...

</div>
</div>

Notes:

Here (or somewhere else) it must be evoked that trie path (key for values) are whatever the runtime want.
This is a very important design consideration:
in ethereum for instance everything is stored under hash(key), which makes the trie balanced amongst all value.
in substrate we allow random length key (there is a limit but very high in the trie impl), because the runtime
can be responsible of trie unbalance.
A slide showing an unbalance trie could be nice:

- a branch with module balance prefix and a lot of balance behind: making the query cost like 2 nodes for prefix and let's say 100_000 account so 16^5 -> ~ 5 nodes (accounts are hash and under the prefix things are balanced). -> 7 nodes to access
- a branch with some random
  odule and a constant in it : 2 nodes for prefix, + 2 nodes to access the constant. -> 4 nodes to access
- the wasm runtime at :code -> only 2 nodes

<!-- TODO broken content? -->

So choice of key (even if mainly made for you by the runtime storage macro) is very important.
And having an unbalance may sound like a bad thing, but it is not.

TODO could be moved as a slide before.

---

### Heavy Node Problem

<img src="../../../assets/img/4-Substrate/trie-with-heavy-node.svg" />

---

### Heavy Node Problem: Fixed

<img src="../../../assets/img/4-Substrate/trie-with-heavy-node-fixed.svg" />

---

### Compact Proof

- Simple encoding to remove redundant information
- Trie node codec already strives for compact encoding
- Still hashes info is redundant
- Nodes are ordered to reflect the trie structure

Notes:

- trie node codec strives to make things compact

So merkle hash is calculated over most compact number of bytes.

- Still hashes info is redundant.

again in a three node V1 and V2 only tree, if proof is for V1 only, then the proof contains two nodes: root (a branch) and V1 (a leaf).
Then the encoding of root will contains two hashes V1 leaf hashes and V2 leaf hashes.
Obviously V1 leaf hashes can be calculated by hashing V1 leaf, so we can just remove it from the root node and gain 32 non compressable bytes.

- Nodes are ordered to reflect the trie structure

by ordering in a given way we can deduce the child parent relationship of nodes.
This can be done in multiple way, for instance encode in the trie node iteration: root -> V1 then when decoding stack root and when unstack complete with V1 hash.
or the other way V1 -> root (here the building need to stack root), then when decoding V1 then root.
Most/all trie algo are about keeping a stack of node (when more memory is used there is something wrong (~ 1 or two nodes)).

---

### Proof Recorder

- Simple footprint of all trie nodes accessed
- Then re-encoded (compact proof)
- Beware of caching hiding behavior of accessed values

Notes:

Another message to convey is that producing proof is really only recording all access made during some actions (key access, value insert, value change, trie iteration...).
Any kind of changes work.
-> write is a bit tricky in the sense it only read access and in memory changes. eg three node trie with V1 and V2 and a parent node, inserting V3 can just be adding a sibling to V1 and V2, but V3 will not be in proof, just the parent node.

This could be extended by the idea that key value caching should be disable for the first action otherwise trie node would not be access and we would not register proof correctly.
-> can extend to Basti pr where there is two kind of cache: trie node level cache that is safe to use and key value cache that
Not sure it is worth going to far on cache strategy, but may be relevant to mention that by its structure trie node cache is shared between block.

---

### Storage and proof size.

- Base 16 trie good for disk storage.
- Binary trie proof footprint is smaller.
- Both concerns could be separated.

<div class="flex-container">
<div class="left" style="margin: 20px;">

<img src="../../../assets/img/4-Substrate/base-16.svg" />

</div>
<div class="right" style="margin: 20px;">

<img src="../../../assets/img/4-Substrate/base-2.svg" />

</div>
</div>

Notes:

The trie structure (hexary) is mostly related to the storage model and do not produce the more compact proofs. One direction would be to decorelate storage from merklization. eg hexary node in storage but merklization over binary node. But the model get more complex.

A final message to it should be that (eth see it), the storage model is still not the most efficient: we use merkle trie index to access node that are stored under a btree index (rocksdb), a true state db would have it's inner indexing directly using the merkle structure.
Paritydb in this sense in a good middle ground as it implement a hash map access directly so the merkle trie index is over a hash map rather than a btree map: that is a huge gain.

What works in memory as simple data structure, also work as a db over disk and also extend to being merklized. Usually things can be mapped or referred to rather naturally. For instance an optimization of radix trie is not storing the full merkle path in each node and get the key with the value: this work in memory (not a huge gain), this work on disk (huge gain as you can have fix len node which is big gain for disk access), can work with merkle proof (but tricky if codec still store the full partial key).

---

### Overlay Deep Dive

The overlay stages changes to the underlying database.

<table class="overlay-table">
<tr><td style="background-color: red;">Runtime Logic</td></tr>
<tr><td style="background-color: darkred;">Runtime Memory</td></tr>
</table>

<br>
<table class="overlay-table" style="background-color: green;">
<tr><td>Runtime Storage API</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Overlay Change Set</td></tr>
<tr>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
</tr>
</table>

<br>
<table class="overlay-table" style="background-color: blue;">
<tr><td>Memory / Database Interface</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Database</td></tr>
<tr>
	<td>Alice: 10</td>
	<td>Bob: 20</td>
	<td>Cindy: 30</td>
	<td>Dave: 40</td>
</tr>
</table>

---

### Overlay: Balance Transfer

<br>
<div class="flex-container">
<div class="left-small">

1. Runtime Logic Initiates.
1. Calls the Runtime Storage API.
1. First we query the Overlay Change Set.
   - Unfortunately it's not there.
1. Then we query the underlying Database.
   - Very slow as you have learned so far.

</div>

<div class="right">

<table class="overlay-table">
<tr><td style="background-color: red;">Runtime Logic</td></tr>
<tr><td style="background-color: darkred;">Runtime Memory</td></tr>
</table>

<br>
<table class="overlay-table" style="background-color: green;">
<tr><td>Runtime Storage API</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Overlay Change Set</td></tr>
<tr>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
</tr>
</table>

<br>
<table class="overlay-table" style="background-color: blue;">
<tr><td>Memory / Database Interface</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Database</td></tr>
<tr>
	<td>Alice: 10</td>
	<td>Bob: 20</td>
	<td>Cindy: 30</td>
	<td>Dave: 40</td>
</tr>
</table>

</div>

---

### Overlay: Balance Transfer

<br>
<div class="flex-container">
<div class="left-small">

- As we return the data back to the runtime, we cache the values in the overlay.
- Subsequent reads and writes happen in the overlay, since the data is there.

</div>
<div class="right">

<table class="overlay-table">
<tr><td style="background-color: red;">Runtime Logic</td></tr>
<tr><td style="background-color: darkred;">Runtime Memory</td></tr>
</table>

<br>
<table class="overlay-table" style="background-color: green;">
<tr><td>Runtime Storage API</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Overlay Change Set</td></tr>
<tr>
	<td>Alice: 10</td>
	<td>Bob: 20</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
</tr>
</table>

<br>
<table class="overlay-table" style="background-color: blue;">
<tr><td>Memory / Database Interface</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Database</td></tr>
<tr>
	<td>Alice: 10</td>
	<td>Bob: 20</td>
	<td>Cindy: 30</td>
	<td>Dave: 40</td>
</tr>
</table>

</div>

---

### Overlay: Balance Transfer

<br>
<div class="flex-container">
<div class="left-small">

- The actual transfer logic happens in the runtime memory.
- At some point, the runtime logic writes the new balances to storage, this updates the overlay cache.
- The underlying database is not updated yet.

</div>
<div class="right">

<table class="overlay-table">
<tr><td style="background-color: red;">Runtime Logic</td></tr>
<tr><td style="background-color: darkred;">Runtime Memory</td></tr>
</table>

<br>
<table class="overlay-table" style="background-color: green;">
<tr><td>Runtime Storage API</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Overlay Change Set</td></tr>
<tr>
	<td style="color: yellow;">Alice: 15</td>
	<td style="color: yellow;">Bob: 15</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
</tr>
</table>

<br>
<table class="overlay-table" style="background-color: blue;">
<tr><td>Memory / Database Interface</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Database</td></tr>
<tr>
	<td>Alice: 10</td>
	<td>Bob: 20</td>
	<td>Cindy: 30</td>
	<td>Dave: 40</td>
</tr>
</table>

</div>

---

### Overlay: Balance Transfer

<br>
<div class="flex-container">
<div class="left-small">

- At the end of the block, staged changes are committed to the database all at once.
- Then storage root is recomputed a single time for the final block state.

</div>
<div class="right">

<table class="overlay-table">
<tr><td style="background-color: red;">Runtime Logic</td></tr>
<tr><td style="background-color: darkred;">Runtime Memory</td></tr>
</table>

<br>
<table class="overlay-table" style="background-color: green;">
<tr><td>Runtime Storage API</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Overlay Change Set</td></tr>
<tr>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
</tr>
</table>

<br>
<table class="overlay-table" style="background-color: blue;">
<tr><td>Memory / Database Interface</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Database</td></tr>
<tr>
	<td style="color: lightgreen;">Alice: 15</td>
	<td style="color: lightgreen;">Bob: 15</td>
	<td>Cindy: 30</td>
	<td>Dave: 40</td>
</tr>
</table>

</div>

---

### Overlay: Implications

<br>
<div class="flex-container">
<div class="left-small text-small">

- Reading the same storage a second or more time is faster (not free) than the initial read.
- Writing the same value multiple times is fast (not free), and only results in a single final Database write.
- Everything in the overlay change set will be committed to the database at the end of the block, so once you call the Runtime API to write some value, you cannot undo it.
  - This is the source of "verify first; write last".

</div>
<div class="right">

<table class="overlay-table">
<tr><td style="background-color: red;">Runtime Logic</td></tr>
<tr><td style="background-color: darkred;">Runtime Memory</td></tr>
</table>

<br>
<table class="overlay-table" style="background-color: green;">
<tr><td>Runtime Storage API</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Overlay Change Set</td></tr>
<tr>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
	<td>&nbsp;</td>
</tr>
</table>

<br>
<table class="overlay-table" style="background-color: blue;">
<tr><td>Memory / Database Interface</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Database</td></tr>
<tr>
	<td>Alice: 15</td>
	<td>Bob: 15</td>
	<td>Cindy: 30</td>
	<td>Dave: 40</td>
</tr>
</table>

</div>

Notes:

also this means that cross implementation of substrate/polkadot can be tricky to ensure determinism (also true for next slide).

---

### Additional Storage Overlays (Transactional)

<br>
<div class="flex-container">
<div class="left-small text-small">

- The runtime has the ability to spawn additional storage layers, called "transactional layers".
- This can allow you to commit changes through the Runtime Storage API, but then drop the changes if you want before they get to the overlay change set.
- The runtime can spawn multiple transactional layers, each at different times, allowing the runtime developer to logically separate when they want to commit or rollback changes.

</div>
<div class="right text-small">

<table class="overlay-table">
<tr><td style="background-color: red;">Runtime Logic</td></tr>
<tr><td style="background-color: darkred;">Runtime Memory</td></tr>
</table>

<br>
<table class="overlay-table" style="background-color: green;">
<tr><td>Runtime Storage API</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Transactional Layer</td></tr>
<tr>
	<td style="color: yellow;">Alice: 25</td>
	<td>&nbsp;</td>
	<td style="color: yellow;">Cindy: 20</td>
	<td>&nbsp;</td>
</tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Overlay Change Set</td></tr>
<tr>
	<td>Alice: 15</td>
	<td>&nbsp;</td>
	<td>Cindy: 30</td>
	<td>&nbsp;</td>
</tr>
</table>

<br>
<table class="overlay-table" style="background-color: blue;">
<tr><td>Memory / Database Interface</td></tr>
</table>

<br>
<table class="overlay-table">
<tr><td colspan=4>Database</td></tr>
<tr>
	<td>Alice: 15</td>
	<td>Bob: 15</td>
	<td>Cindy: 30</td>
	<td>Dave: 40</td>
</tr>
</table>

</div>

---

### Transactional Implementation Details

- Non-Zero Overhead (but quite small)
  - 0.15% overhead per key written, per storage layer.
- Values are not copied between layers.
  - Values are stored in heap, and we just move pointers around.
  - So overhead has nothing to do with storage size, just the number of storage items in a layer.
- Storage layers use client memory, so practically no upper limit.

Notes:

For more details see:

https://github.com/paritytech/substrate/issues/10806

https://github.com/paritytech/substrate/pull/10809

In module 6, we can take a closer look at how this functionality is exposed in FRAME.

See: https://github.com/paritytech/substrate/pull/11431

---

### Transactional Layer Attack

Transactional layers can be used to attack your chain:

<br>

- Allow a user to spawn a lot of transactional layers.
- On the top layer, make a bunch of changes.
- All of those changes will need to propagate down each time.

**Solution:**

- Do not allow the user to create an unbounded number of layers within your runtime logic.

---

### RocksDB

A Persistent Key-Value Store for Flash and RAM Storage.

- Keys and values are arbitrary byte arrays.
- Fast for a general database.

See http://rocksdb.org/.

Big project, can be very tricky to configure properly.

Notes:

(also a big part of substrate compilation time).

---

### ParityDB

An Embedded Persistent Key-Value Store Optimized for Blockchain Applications.

- Keys and values are arbitrary byte arrays.
- Designed for efficiently storing Patricia-Merkle trie nodes.
  - Mostly Fixed Size Keys.
  - Mostly Small Values.
  - Uniform Distribution.
- Optimized for read performance.

Notes:

See: https://github.com/paritytech/parity-db/issues/82

Main point is that paritydb suit the triedb model.
Indeed triedb store encoded key by their hash.
So we don't need rocksdb indexing, no need to order data.
Parity db index its content by hash of key (by default), which makes access faster (hitting entry of two file generally instead of possibly multiple btree indexing node).
Iteration on state value is done over the trie structure: having a KVDB with iteration support isn't needed.

Both rocksdb and paritydb uses "Transactions" as "writes done in batches".
We typically run a transaction per block (all in memory before), things are fast (that's probably what you meant).
In blockchains, writes are typically performed in large batches, when the new block is imported and must be done atomically.
See: https://github.com/paritytech/parity-db

Concurrency does not matter in this, paritydb lock access to single writer (no concurrency).
Similarly code strive at being simple and avoid redundant feature: no cache in parity db (there is plenty in substrate).

'Quick commit' : all changes are stored in memory on commit , and actual writing in the WriteAheadLog is done in an asynchronous way.

TODO merge with content from https://github.com/paritytech/parity-db/issues/82

---

### ParityDB: Probing Hash Table

ParityDB is implemented as a probing hash table.

- As opposed to a log-structured merge (LSM) tree.
  - Used in Apache AsterixDB, Bigtable, HBase, LevelDB, Apache Accumulo, SQLite4, Tarantool, RocksDB, WiredTiger, Apache Cassandra, InfluxDB, ScyllaDB, etc...
- Because we do not require key ordering or iterations for trie operations.
- This means read performance is constant time, versus $O(\log{n})$.

---

### ParityDB: Fixed Size Value Tables

- Each column stores data in a set of 256 value tables, with 255 tables containing entries of certain size range up to 32 KB limit.

```rust
const SIZES: [u16; SIZE_TIERS - 1] = [
	32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 46, 47, 48, 50, 51, 52, 54, 55, 57, 58, 60,
	62, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 88, 90, 93, 95, 98, 101, 103, 106, 109,
	112, 115, 119, 122, 125, 129, 132, 136, 140, 144, 148, 152, 156, 160, 165, 169, 174, 179, 183,
	189, 194, 199, 205, 210, 216, 222, 228, 235, 241, 248, 255, 262, 269, 276, 284, 292, 300, 308,
	317, 325, 334, 344, 353, 363, 373, 383, 394, 405, 416, 428, 439, 452, 464, 477, 490, 504, 518,
	532, 547, 562, 577, 593, 610, 627, 644, 662, 680, 699, 718, 738, 758, 779, 801, 823, 846, 869,
	893, 918, 943, 969, 996, 1024, 1052, 1081, 1111, 1142, 1174, 1206, 1239, 1274, 1309, 1345,
	1382, 1421, 1460, 1500, 1542, 1584, 1628, 1673, 1720, 1767, 1816, 1866, 1918, 1971, 2025, 2082,
	2139, 2198, 2259, 2322, 2386, 2452, 2520, 2589, 2661, 2735, 2810, 2888, 2968, 3050, 3134, 3221,
	3310, 3402, 3496, 3593, 3692, 3794, 3899, 4007, 4118, 4232, 4349, 4469, 4593, 4720, 4850, 4984,
	5122, 5264, 5410, 5559, 5713, 5871, 6034, 6200, 6372, 6548, 6729, 6916, 7107, 7303, 7506, 7713,
	7927, 8146, 8371, 8603, 8841, 9085, 9337, 9595, 9860, 10133, 10413, 10702, 10998, 11302, 11614,
	11936, 12266, 12605, 12954, 13312, 13681, 14059, 14448, 14848, 15258, 15681, 16114, 16560,
	17018, 17489, 17973, 18470, 18981, 19506, 20046, 20600, 21170, 21756, 22358, 22976, 23612,
	24265, 24936, 25626, 26335, 27064, 27812, 28582, 29372, 30185, 31020, 31878, 32760,
];
```

- The last 256th value table size stores entries that are over 32 KB split into multiple parts.

---

### ParityDB: Fixed Size Value Tables

- More than 99% of trie nodes are less than 32kb in size.
- Small values only require 2 reads: One index lookup and one value table lookup.
- Values over 32kb may require multiple value table reads, but these are rare.
- Helps minimize unused disk space.
- For example, if you store a 670 byte value, it won't fit into 662 bucket, but will into 680 bucket, wasting only 10 bytes of space.

Note:

That fact that most values are small allows us to address each value by its index and have a simple mechanism for reusing the space of deleted values without fragmentation and periodic garbage collection.

---

### ParityDB: Asynchronous Writes

- Parity DB API exposes synchronous functions, but underlying IO is async.
- The `commit` function adds the database transaction to the write queue, updates the commit overlay, and returns as quickly as possible.
- The actual writing happens in the background.
- The commit overlay allows the block import pipeline to start executing the next block while the database is still writing changes for the previous block.

---

### Practical Benchmarks and Considerations

Let's now step away from concepts and talk about cold hard data.

---

### Common Runtime Data Size and Performance

<br>

<div class="flex-container">
<div class="left">

- Most runtime values are 80 bytes, which are user accounts.
- Of course, this would depend on your chain's logic.

</div>
<div class="right">

<div class="r-stack">
	<img src="../../../assets/img/4-Substrate/polkadot-size-histogram.png" style="width: 1000px"/>
	<img class="fragment" src="../../../assets/img/4-Substrate/polkadot-size-histogram-alpha.png" style="width: 1000px"/>
</div>
</div>

Notes:

Impact of keys size is slightly bigger encoded node.
Since eth scaling issue, we usually focus on state nodes.
Other content access can be interesting to audit enhance though (with paritydb).

See more details here:

https://substrate.stackexchange.com/questions/525/how-expensive-is-it-to-access-storage-items/526#526

---

### RocksDB vs ParityDB Performance

At 32 KB, performance decreases for each additional 4 KB.

<img src="../../../assets/img/4-Substrate/paritydb-vs-rocksdb-read.png" style="width: 1000px" />

---

### RocksDB Inconsistency

<img style="width: 1200px;" src="../../../assets/img/4-Substrate/rocksdb-hiccups.png" />

When doing benchmarking, we saw some really bizarre, but reproducible problems with RocksDB.

---

### Let's have a discussion!

- What kinds of things do we need to worry about?
- What kinds of decisions / tradeoffs do we need to make?

<!-- TODO: ask questions that relate to the work done here: https://github.com/paritytech/trie/pull/157 and https://github.com/paritytech/trie/pull/142 for students and lecturer to engage about optimizing state reads. Then look over the PR's motive. -->

Notes:

About the value node pr, the message is: when working on a merklized data structure, the choice of where you calculate the hash is really important: it will drives what needs to be included in proof. !!!if you include anything you need to have the hash checked, so you include the whole data in the proof!!!
One can imagine hashing node like hash(hash(partial_key), hash(rest_of_node)), then a proof of missing value wiould only include 'partial_key' ++ hash(rest_of_node)' and if rest_of_node is bigger than 32 bytes that's a size win at the expense of a hash computation.
But probably this use case (only hitting partial_key) is not that predominant (mostly missing values).

---

### Workshop and Activity

- [Database and Merklized Storage Activity](./Db_and_Merklized_Storage_Activities.md)
