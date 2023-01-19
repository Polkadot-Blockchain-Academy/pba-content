# Polkadot

Dive into the purpose, implementation, and protocols of Polkadot, the sharded multichain system.

Draw content from:

- https://polkadot.network/blog/the-path-of-a-parachain-block/
- https://github.com/paritytech/polkadot/tree/master/roadmap/implementers-guide

## Lessons Overview

### Day 1

In the first day, we'll cover history of scalability, Data Availability problem, Polkadot's protocols, the concept of Blockspace and optionally Governance.

- Blockchain scaling history. Rollups and Sharding.

- Data sharding: Data availability + exercise (impl simple erasure-coding)

- Execution sharding and security: Approval-voting and disputes

- Exercise for Polkadot protocol @BradleyOlson64

- A lecture on Blockspace @shawntabrizi + @rphmeier

- OpenGov + workshop @BradleyOlson64 (could also be moved to day 2 or 3)

- Zombienet by Javier

### Day 2

Second day is mostly about some of parachains SDK internals and how to build a parachain using it.

- Building a simple collator: adder-collator

- Introduction to Cumulus

- workshop: Solo->Parachain

- Async Backing @rphmeier?

- workshop: Hacking on Cumulus: ** change cumulus to produce > 1 parablock per relay block

### Day 3 (Half Day)

- NPoS by Kian

- Introduction to XCMP
