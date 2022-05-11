# Substrate Overview

In this section, we will be going over Substrate at a high level, ensuring you are able to:

* Navigate the Substrate Codebase.
* Justify why Substrate claims to be an extensible, modular framework.

---

## Substrate Architecture

We are going to look at the architecture of Substrate, which can be broken down into these pieces:

* Client and Runtime
* Runtime Components
* Client Components

---

## Client and Runtime

At a high level, you can think of Substrate in two parts:

1. A Wasm executor - the client
2. A Wasm runtime - the runtime

---

## What is the Runtime?

In Substrate, the runtime contains all of the business logic for executing the state transition
function of the blockchain.

The runtime is always a Wasm binary.

Common State Transitions:

1. Transactions
2. On-Initialize / On-Finalize (TODO)


---

## Wasm

WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine.
Wasm is designed as a portable compilation target for programming languages, enabling deployment on
the web for client and server applications.

The open standards for WebAssembly are developed in a W3C Community Group (that includes
representatives from all major browsers) as well as a W3C Working Group.

---

## Why Wasm?

---

## Wasm Assumptions

 and therefore makes the following assumptions:

- Runtimes can't use `std` libraries
- Max

TODO list all Wasm Assumptions

---

## Runtime Assumptions

On top of the Wasm assumptions, to make the runtime Wasm a compatible blockchain state transition
function, we also assume:

* Access to Client Host Functions
* Exposure of Runtime Interface

---

## Client Components

<div class="left">

* Networking - LibP2P
* Database - RocksDB or ParityDB
* Transaction Queue
* Block Production
* Consensus

</div>

<div class="right">

![Diagram of Substrate Client](http://placehold.jp/150x150.png)

</div>

TODO: Add Diagram of Substrate Client

---

## Client Assumptions

Within any blockchain network, there are always assumptions around the hardware used to run the
node. The performance of two computers are not the same, and blockchains must reach consensus in a
similar amount of time.

In Polkadot, there is a standard hardware that we expect all nodes to use.

More on this when we touch on Benchmarking.

* networking assumptions?
* other assumptions TODO

---

## EXERCISE 1

Navigate the Substrate Codebase With Us

Feel free to get lost, and ask plenty of questions.

NOTE: This can be a 10 - 15 min exercise.

TODO, remind users to already have cloned the project locally, or I guess they can use GitHub file
explorer.

See [Exercise 1](../exercises/1-navigate-substrate-codebase.md)
