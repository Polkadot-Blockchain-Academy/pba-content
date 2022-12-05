---
title: Parachain Configuration for XCM # Also update the h1 header on the first slide to the same name
description: Describe your slides here
duration: 1 hour
instructors: ["Gavin Wood", "Keith Yeung"]
teaching-assistants: ["Dan Shields"]
---

# Parachain Configuration for XCM

---

## XCM Configurations

Now go through and explain the various XCM configurations and hooks that can be manipulated by the user, and how that may affect XCM behavior

This then leads naturally into a concept like fee payments, which is entirely a configuration, for things like:

- What assets are supported for fees (configurable)
- How much each asset is worth in terms of weight (configurable)
- What happens to unused weight (configurable)
- Other mechanisms for being given weight (for example a stipend for certain chains, or entirely free)
- Barrier conditions

---

### XCM Version Negotiation

XCM is an **extensible message format**.

Versioning allows chains to communicate as XCM evolves.

```rust
pub enum VersionedXcm {
    V0(v0::Xcm),
    V1(v1::Xcm),
    V2(v2::Xcm),
}
```

But chains need to be aware of the version supported by each other. `SubscribeVersion` and `QueryResponse` play a key role here:

```rust
enum Instruction {
  /* snip */
  SubscribeVersion {
        query_id: QueryId,
        max_response_weight: u64,
  },
  QueryResponse {
        query_id: QueryId,
        response: Response,
        max_weight: u64,
  },
  /* snip */
}
```

XCM version negotiation:
<widget-text center>

1. Chain A sends `SubscribeVersion` to chain B.
2. Chain B responds `QueryResponse` to chain A with the same query_id and max_weight params, but the XCM version in the response
3. Chain A stores chain B's supported version on storage.
4. The same procedure happens from chain B to chain A.
5. Communication is stablished in the highest common supported version.

<widget-columns>
<widget-column>
<img style="width: 500px;" src="../../../assets/img/7-XCM/xcm-versioning.png" alt="Xcm Versioning"/>
</widget-column>
<widget-column>

<!--

TODO add slide somewhere about this. Basically scaffold gav's second blog

https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83

-->

---

## Lesson 3: Parachain Configuration for XCM

> {Likely} Instructor: RT Eng.? Shawn?

### Why is this topic Important?

### Prerequisite Knowledge or Skills

### Learning Outcome

- Understand and correctly configure XCM (pallet, executor, ...) for a parachain (exectutor and pallet) based on buisness needs
- Barriers & filters
- Asset transactors

### Learning Objectives

### Core Ideas to Convey

- XCM builder module (makes configuration easier)
- Barriers & filters
- Asset transactors
- Weight Traders
- wildcards? {maybe out of scope}

### Activities and Exercises

- Transfer native tokens from ChainA → ChainB **on Rococo** {no setup needed, }
  - Teleport
  - Reserve Asset
  - Explain what to use these given a scenario:
    - Within a chain itself `Here`
    - ParaA → ParaB
    - Relay Chain Native token to ParaA (not common good chain)
    - ... others?
