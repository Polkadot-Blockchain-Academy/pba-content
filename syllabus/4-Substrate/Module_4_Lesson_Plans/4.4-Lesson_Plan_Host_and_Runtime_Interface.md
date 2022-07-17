# Lesson 4: Host/Runtime Interface

20 min - Lecture
40 min - Write toy runtimes activity
20 min - Lecture
10 min - Break
90 min - Write frameless runtime and subsequent exercises.

## Why is this topic Important?

- Client / Runtime division in substrate is one of the most important decisions that substrate has taken. Thus, the communication between the aforementioned is also important.

## Core Ideas to Convey

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

## Prerequisite Knowledge or Skills

- Substrate node architecture

## Learning Outcome

- Understand the runtime client communication, both ways, with some degree of detail about the most important ones (e.g. version).

## Learning Objectives

- Describe the role of host functions and runtime APIs of a Substrate node
- Understand why the runtime cannot do certain things and needs a host to assist it.
  - Offchain workers are a middle ground and have access to more host functions
- Distinguish the implications of adding/removing host functions (and the lack of implications when adding runtime api).

<aside>
💡 Bonus: Versioning of host functions and runtime apis

</aside>

## Activities and Exercises

- Write (1) a new host function and (2) a runtime api for your node.
- Case study: Look into how the benchmarking system utilizes both runtime apis and host function to achieve its goal.
- Write a handfull of "toy" runtimes that use a simplified aruntime and host api.
- **Complete a FRAME-less runtime**
  - Initially just a "flipper"
  - block builder
  - execute_block
  - Signed transactions
  - exercises for students to continue afterwards
