# Block concepts

In this section, we will learn how Substrate interprets a block and what it expects from it.

---

## What is a Block in Substrate?

(Include a diagram of a block, with just header and body)

A Block in Substrate is just two simple parts:

* The block header
* The block body

These parts are generally opaque in the client, which gives them minimal assumptions, and maximal flexibility.

---

## What is the Block Header?

(make diagram - expanded from previous)
* State root
* Extrinsic root
* Block number
* Parent hash
* Digests (light clients, author signature etc etc)

---

## What is the Block Body?

(make diagram - expanded from previous)

* Block body is simply a collection of opaque extrinsics. Examples:
	* Signed extrinsics
    * Unsigned extrinsics
    * Inherents

---

## What an Extrinsic really is?

> Extrinsics are data that come from outside of the runtime.

Where:

* The client sees all transactions opaque: `Vec<u8>`
* Contrary, the runtime should decode, interpret, and execute the transactions.

(diagram to show the flow of transaction from client to runtime, and where it gets executed)


NOTE: when to use vs. unsigned extrinsics

TODO add `<code>` style for `Vec<u8>` above.

## Polkadot Block Assumptions

TODO What cannot be changed about a block to be substrate compatible?
