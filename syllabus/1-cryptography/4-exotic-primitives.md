# Lesson 4: Other Exotic Primitives

## Why is this topic Important?

Blockchains use more cutting edge (and untested!) cryptography than other appications. They use these primitives for privacy, consensus and scaling.

## Learning Outcome

By the end of this lesson, learners should know what the most common exotic primitives used in blockchain are and know a little about each.

## Learning Objectives

TODO

## Core Ideas to Convey

1. All Other Signatures (some flexibiity to move some of this back to lesson 3)
    * Aggregated Signatures (will use)
    * Threshold Signatures (barely used)
    * Ring Signatures (not used yet)

1. Erasure Coding
    * *Question: should this move to Polkadot? Answer: This will be revisited during the Polkadot section, given that we actually do use this one in Polkadot)*
1. zk-SNARKS

**Transition:** — Blockchain can’t have secrets (everything is public), so 'personal' cryptography won't work the same way in the blockhain world.

Instead, we need system-based cryptography to prove that a particular database has a particular key-value pairing, based on a small amount of knowledge (rather than dumping the entire thing and checking against it).

It's an efficient way of providing a proof of existence or non-existence. It's also used for rights.

Access to memory and lots of hashing imposes a performance cost, so developers should understand the performance tradeoffs. —

## Activities and Exercises

*Perhaps an activity in which learners solve problems in a blockchain scenario and decide which type of cryptography to use...?*

Cryptography could be tested as part of an oral examination asking them how they would use cryptography in a blockchain scenarios.

There is also an option to ask them to create written responses to scenarios during this module (or at the end).

We might also ask for short formative assessments (e.g., matching use case to primitive or API).

A good option is to provide an existing solution that is broken, and have students describe why its broken.
