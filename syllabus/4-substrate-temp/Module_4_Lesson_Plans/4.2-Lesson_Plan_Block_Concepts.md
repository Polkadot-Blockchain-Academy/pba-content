# Lesson 2: Block Concepts

## Why is this topic Important?

- To understand how Substrate understands a block and what it expects from it (what expectations and opinions does Substrate have for a block)

## Core Ideas to Convey

- Header
  - State root
  - Digests + How they are used with light clients
  - Author signatures (or no signature if parablock)
- Extrinsics
  - Signed extrinsics
  - Unsigned extrinsics
  - Opaque extrinsics
  - Inherents (when to use vs. unsigned extrinsics)

💡 Other ideas: Opaque transaction in client

## Prerequisite Knowledge or Skills

- Blockchain concepts: Merkle proofs, block validity
- Hashing
- Substrate architecture
- Light clients

## Learning Outcome

- To understand how the runtime defines a block, and what restrictions the client has on that block definition (i.e. what assumptions the client has).

## Learning Objectives

- Name the core components of a block: Header and extrinsics
- Describe a block in how the runtime views it
- Describe a block in how the client views it

_{below ported from prior work}_

- Name the core components of a block
- Describe a block in how the runtime views it
- Describe a block in how the client views it
- Describe the header
  - What is a header in Substrate, what else is in a block, what is the framework of a block, what's a log, state root in the header, extrinsics and transactions. State root, Events, Logs, Transaction root, (extrinsics, transactions and inherents (such as Off-chain workers)), weights.
  - Explain what digests are and how they are used in consensus
  - Explain the decisions made for light clients
  - Block authorship and signature / no signature
- Describe the extrinsics
  - Differentiate between different types of extrinsics
  - What is an inherent - a type of extrinsic. Has certain different properties.
  - Include the Opaque extrinsic
- Explain the differences between block and state
- Recognize what makes a block invalid
- Explain when to use an inherent vs. a unsigned transaction

_{below ported from prior work}_

- Name the core components of a block
- Describe a block in how the runtime views it
- Describe a block in how the client views it
- Describe the header
  - What is a header in Substrate, what else is in a block, what is the framework of a block, what's a log, state root in the header, extrinsics and transactions. State root, Events, Logs, Transaction root, (extrinsics, transactions and inherents (such as Off-chain workers)), weights.
  - Explain what digests are and how they are used in consensus
  - Explain the decisions made for light clients
  - Block authorship and signature / no signature
- Describe the extrinsics
  - Differentiate between different types of extrinsics
  - What is an inherent - a type of extrinsic. Has certain different properties.
  - Include the Opaque extrinsic
- Explain the differences between block and state
- Recognize what makes a block invalid
- Explain when to use an inherent vs. a unsigned transaction

## Activities and Exercises

- Explain the different parts of a block in some Polkadot block explorer.
- Classify the different use cases for each extrinsic type

Maybe:

- Fix this codebase so that all tests pass and it can produce blocks (assumes incomplete implementation is provided).
  - Give a code example that uses our block types and ask students to correctly create a chain of blocks
