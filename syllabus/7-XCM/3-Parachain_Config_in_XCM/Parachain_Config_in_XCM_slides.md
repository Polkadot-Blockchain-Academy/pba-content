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
