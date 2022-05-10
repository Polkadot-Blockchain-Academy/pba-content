# Lesson 1: Overview of Substrate

## Why is this topic Important?

To be able to navigate the Substrate codebase and justify why Substrate is built the way it is and what makes Substrate Extensible, Modular and Fast.

## Prerequisite Knowledge or Skills Required

- Rust
- Blockchain concepts

## Learning Outcome

By the end of this lesson, learners will understand what is meant when Substrate is described as modular, upgradable, and extensible.

## Learning Objectives

By the end of this lesson, learners will be able to:
- Describe the high-level architecture of Substrate (client and runtime)
- Navigate Substrate’s codebase
- Describe the core design decisions for each component in Substrate (Outcome should be that substrate is modular and extensible)

## Core Ideas to Convey

List the core ideas of this topic **in the order in which they should be taught**.

* Describe the high-level architecture of Substrate (client and runtime)
* Substrate high level diagram (wasm runtime / client (p2p, db. tq, native, telemetry, chain spec..)) ([example from docs](https://github.com/substrate-developer-hub/docs-sandbox/blob/sl/architecture-page/source/docs/main-docs/02-fundamentals/architecture.md))
* Navigate Substrate’s codebase 
* Describe the core design decisions for each component in Substrate.**
* Walk through at the various `sc_*` and `sp_*` crates are and what their roles are as well as why the codebase is structured the way it is
    - Know the boundry of substrate while looking at the polkadot repo, or node-template.
- (briefly) Compare extensibility in Open Ethereum and Substrate codebases

## Activities and Exercises

- Draw a diagram of the lifecycle of a transaction (which components a transaction touches at a high level in a specific scenario)
- Have students show what components of Substrate belong to Substrate and what components belong to Polkadot
- Find where the logic of some behaviour exists
- What crates contain what components

**Maybe:**

- Draw an architectural diagram of Substrate’s components and label what each does
- Write a paragraph describing the key architecture decisions
