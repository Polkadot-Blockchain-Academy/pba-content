<!-- ## Core Ideas to Convey

- (Recap the big picture, then) Runtime api and host functions: two way communication (diagram)
- Most notable runtime apis worth mentioning
    - Core (VERSION!)
    - TxQueue
    - BlockBuilder
    - and maybe: metadata, offchain workers, inherents, benchmarking
- Most notable host functions worth mentioning
    - crypto and hashing
    - io
        - storage
        - print
    - allocating memory
    - smart contract function (works in an interesting and unusual way- two-way)
    - instantiating multiple wasm instances
 -->
# Lesson 4: Host/Runtime Interface

TODO: Call Host Functions + Runtime API = Runtime Interface?

In this lesson we'll learn about how the client / runtime division in Substrate is one of the most important decisions that Substrate has taken. Thus, the communication between the aforementioned is also important.

## Big picture: what does the runtime interface look like?

_(diagram of two way communication between the Runtime api and host functions)_

* The client and runtime need a way to communicate with eachother
	* The runtime uses host functions to get data from the client
	* The client uses runtime apis to get data from the runtime

---
## Example: Block Execution

To execute a block, we need both host functions and runtime apis:

* Runtime API: `execute_block` - The client provides a block to the runtime, and expects the runtime
  to execute that block.
* Host function: `storage_get` - To execute the block, the runtime will probably need to read
  storage, which only the client has access to.

---

## Runtime APIs

1. Check out the existing runtime APIs in Substrate. They include:

* Core
* TxQueue
* BlockBuilder

https://github.com/paritytech/substrate/blob/master/bin/node/runtime/src/lib.rs#L1618

---

## Host functions

Check out the [existing host
functions](https://github.com/paritytech/substrate/blob/master/primitives/io/src/lib.rs) in
Substrate.

These are used for:

* crypto and hashing
* io
    * storage
    * print
* allocating memory

---

## Speed

* Wasm interfaces are slow to access
* Analogy: when contacting host function from wasm, its like your OS contacting IO buffer

---

## Common errors (what could go wrong)

If you have a runtime which expects a host function which does not exist, or vice versa you will get a panic!

Not any Wasm can be a Substrate Runtime.

---
## Versioning and backward compatibility

(Distinguish the implications of adding/removing host functions (and the lack of implications when
adding runtime api).

* Host functions are **FOREVER**: when finalizing a block that uses a specific host function, you always
  need to keep the same host functions
* Imagine syncing your chain from scratch, every version of your blockchain's runtime will be needed
  to sync and get to the latest block.
* Thus your client must be able to provide all the host functions expected in the past and for
  present/future blocks.

---
## Feature gated runtime APIs and host functions

There are ways to implement runtime interfaces for development purposes, such as:
* Testing and verification: `try_runtime`
* Benchmarking: `frame_benchmarking`

Basically anything which should be executed locally (or non-consensus code path), and not on the public network.

---
## Polkadot Standard Proposals

* How might we introduce a new host function into the Polkadot ecosystem?

---

## How to write a Runtime API

* `impl_runtime_api!`, conveniently a rust trait, always generic over block.
* No matter the return type, client sees the outcome as `Vec<u8>`
* Client can always provide extra `at: Option<BlockNumber>`.

---
## How to write a host function

* `#[runtime_interface]`

---

## Bonus

* Why the way smart contract functions work is unusual (two-way)
* Instantiating multiple wasm instances

(TODO: see where to fit this section)

---
## Exercise

* Code walkthrough of existing host functions / runtime interfaces (e.g. benchmarking)
* Create a new host function / runtime interface behind a feature flag
* Implement an off-chain worker

---
