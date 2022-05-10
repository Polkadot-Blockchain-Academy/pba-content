# Lesson 3: Transaction Queue and Block Builder

## Why is this topic Important?

- To be able to argue about the behaviour of the transaction queue at a higher level (e.g. when interacting with a chain)

## Core Ideas to Convey

**Transaction queue** 

- Input: Network, outer world, whatever, output: Block builder (diagram)
- Transaction details from pool PoV
    - priority (later: tip)
    - era and mortality
    - Provides and requires
- Transaction-related Events in the pool:
    - (Re)validation
    - banned

## Prerequisite Knowledge or Skills

- Existence of a runtime that validates transactions

## Learning Outcome

- Understand (some of) the main behavior details of the substrate transaction queue (e.g. it bans, (re)validates, propagates etc etc).

## Learning Objectives

- Describe what happens when you submit a transaction to the chain
- Describe the lifetime of a transaction (era) and including transaction priority
- Describe the process of transaction validation and block inclusion

## Activities and Exercises

- Manually draw out a “transaction graph” based on the “requirements” and “provides” of abstract transactions.
    - Could point out some transactions will never be executed
    - Could point out transactions that will not execute until after some other transactions execute, etc…
- Look into the transaction events after you submit a transaction with polkadot js api
- .js project which visualizes the transaction pool

<aside>
💡 Extra ideas: Tweak the txpool so that selection is random, FIFO, LIFO etc.

</aside>