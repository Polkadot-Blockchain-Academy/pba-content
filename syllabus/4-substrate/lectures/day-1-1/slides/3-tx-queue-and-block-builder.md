# Lesson 3: Transaction Queue and Block Builder

In this lesson you will learn about the behaviour of the transaction queue in Substrate at higher
level (e.g. when interacting with a chain).

## Context

```
┌───────────┐                                                    ┌───────────────┐
│           │                                                    │               │
│           │                                                    │               │
│           │                                                    │               │
│           │                                                    │               │
│           │   ┌───────────────┐       ┌────────────────┐       │               │
│           │   │               │       │                │       │               │
│           │   │               │       │                │       │               │
│  network  │   │    Tx-pool    │       │  Block-Builder │       │     Runtime   │
│           ├───►               ├───────►                ├───────►               │
│           │   │               │       │                │       │               │
│           │   └───────────────┘       └────────────────┘       │               │
│           │                                                    │               │
│           │                                                    │               │
│           │                                                    │               │
│           │                                                    │               │
│           │                                                    │               │
│           │                                                    │               │
│           │                                                    │               │
│           │                                                    │               │
└───────────┘                                                    └───────────────┘
```

(diagram of "inputs" being network/consensus and outer world and "outputs" being block builder).


* The networking layer in Substrate gives transactions to the transaction pool.
    * Gossip protocol propagates transactions
* The pool does a few things:
    * It (re)validates transactions when needed
	    * Transaction mortality
    * Prioritization: Priority, provides, requires.
    * Banning
	* Transaction submission and status updates via RPC.

NOTE: https://substrate.stackexchange.com/questions/2148/transactions-lifecycle/2272#2272

---
## A Generic Transaction Pool

(What's unique in Substrate's)

Every blockchain has a transaction pool, however Substrate's is unique in that
it makes very little assumptions about what kind of content is in that pool.

---

## Components of the Transaction Pool

At a high level, the Transaction Pool only cares about:

- Provides
- Requires
- Priority
- Validity Function

---
## Transaction validation

How are transactions

---

## Transaction priority

---

TODO Revisit the slide
