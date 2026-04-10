---
title: The Polkadot Stack
description: An introduction to the tools and resources available for building on Polkadot.
duration: 1.5 hours
owner: Shawn Tabrizi
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# The Polkadot Stack

---

## The Driving Force

The world we live in has problems:

<div class="flex-container">
<div class="left">

- Financial Corruption
- Political Control
- Digital Servitude
- Erosion of Sovereignty
- Systemic Fragility
- and more…

</div>
<div class="right">

<img style="width: 400px;" src="./img/world.png" />

</div>
</div>

These problems all stem from the same place:

> **The growing power of central authorities.**

Notes:

Before we dive into tools and technology, let's talk about WHY we're building any of this. The problems in our world, financial corruption, political control, erosion of sovereignty, all share a common root: the concentration of power in central authorities. This isn't abstract philosophy, it's the motivation behind everything Polkadot is building.

---

## A Simple Set of Conjectures

- Central authorities will always (eventually) abuse their powers.
<!-- .element: class="fragment" -->
- With the increasing globalization caused by technology, the reach of central authorities is greater than ever.
<!-- .element: class="fragment" -->
- Computers play a critical role in providing public digital infrastructure.
<!-- .element: class="fragment" -->
- We could create a world more resilient from these powers, if the worlds public digital infrastructure were itself more decentralized and resilient.
<!-- .element: class="fragment" -->

> **Web3 describes that resilient public digital infrastructure.**

<!-- .element: class="fragment" -->

Notes:

These are the conjectures that drive the Web3 movement. Central authorities will eventually abuse their power, history proves this over and over. Technology has amplified their reach. But technology can also be the solution. If the public digital infrastructure that the world relies on were decentralized and resilient, we'd have a powerful counterbalance. That's what Web3 is: resilient public digital infrastructure.

---

## The Great Mistake

Blockchain is not a product in and of itself.

The world does not want or need "more blockchains".

It is simply a tool that can be used to create **resilient products**.

Notes:

This is a critical insight that many in the blockchain space miss. Nobody wakes up wanting "a blockchain." People want products that work, that they can trust, that can't be shut down or censored. Blockchain is the tool that makes those products possible. This reframing changes everything about how we think about building.

---

## Two Different Products

Both built using blockchain technology.

<pba-cols class="text-small">
<pba-cols>
<pba-col>

#### Distributed Ledger Technology

- The primary purpose is to **store** data.
- The primary part of this product is the blockchain state.
- The backbone of decentralized finance.
- Think: Bitcoin, XRP, BNB, Solana.
- A proven product with clear market demand.

</pba-col>
<pba-col>

#### Decentralized Computer

- The primary purpose is to **execute** computation.
- The primary part of this product is the decentralized computation bandwidth.
- The backbone of the decentralized internet.
- Think: a decentralized AWS.
- A speculative but necessary future.

</pba-col>
</pba-cols>


Notes:

There are really two different things people build with blockchain technology. Distributed ledger technology is about storing data, it's the backbone of DeFi and has proven market demand. A decentralized computer is about executing computation, it's the backbone of the decentralized internet. Most blockchains are actually DLTs. Polkadot is building toward something bigger: a decentralized computer, like a Web3 version of AWS, but with resilience, verification, and no single point of control.

---

## Who Does the World Computer Appeal To?

<div class="text-small">

- **A Distributed Computer**
  - a distributed computer is one which has natural fallback, and can provide incredible uptime and replication guarantees.
- **A Verifiable Computer**
  - a verifiable computer is one where people need not require trust in the platform or the other users they interact with.
- **A Resilient Computer**
  - a resilient computer is one where applications and services need not worry about the restrictions / laws of any particular nation, and can launch unstoppable applications.
- **A Financial Computer**
  - a platform which natively integrates global finances into your platform, and enables direct integration between you and your customers.

</div>

Notes:

The decentralized computer appeals to different audiences for different reasons. Some want distribution for uptime and resilience, think critical infrastructure that can never go down. Others want verifiability, so users don't need to trust anyone. Others want resilience against censorship and regulatory overreach. And some want the native financial primitives. Polkadot is building infrastructure that serves all of these needs.

---

## Polkadot: Infrastructure for the Decentralized Web

Polkadot is building a **decentralized Web3 Cloud**:

- **Compute** - Execution sharding across multiple cores
- **Storage** - Persistent on-chain state and data availability
- **Networking** - Cross-chain messaging and trustless bridges
- **Identity** - On-chain identity, personhood, and name resolution
- **Finance** - Native token economics and DeFi primitives

> Polkadot will do to Web3 what AWS did to Web2.


Notes:

So what is Polkadot actually building? Think of it as a decentralized cloud provider. AWS gives you compute, storage, networking, identity services, and payment processing. Polkadot provides all of these, but decentralized, verifiable, and resilient. Execution sharding gives you multi-core compute. The blockchain state and bulletin chain give you storage. XCM and bridges give you networking. The People chain and DotNS give you identity. And DOT plus the DeFi ecosystem give you finance. The difference? No single entity controls the infrastructure, and every operation is verifiable.

---

## How Polkadot Delivers This

Polkadot fulfills this vision across three layers:

<diagram class="mermaid">
graph TB
    subgraph Triangle["Polkadot Triangle"]
        T1["Desktop App"]
        T2["Mobile App"]
        T3["Web App (dot.li)"]
        T4["Host SDK"]
    end

    subgraph Stack["Polkadot Stack"]
        S1["Polkadot SDK / FRAME"]
        S2["Smart Contracts"]
        S3["PAPI / subxt"]
        S4["Ethereum Tooling"]
    end

    subgraph Platform["Polkadot Platform"]
        P1["Relay Chain"]
        P2["Asset Hub"]
        P3["Bridge Hub"]
        P4["Bulletin Chain"]
        P5["Parachains"]
    end

    Triangle --> Stack
    Stack --> Platform
</diagram>

Notes:

Polkadot delivers this vision through three layers. At the bottom is the Platform: the blockchain infrastructure that provides compute, storage, and security, the decentralized cloud itself. In the middle is the Stack: the developer tools, SDKs, and frameworks that make it possible to build on this cloud. At the top is the Triangle: the user-facing applications, the decentralized browser and app hosts that bring this to real users. Together, these three layers cover the full path from infrastructure to end user.

---

## What Are We Building?

Throughout this course, you will build a project using the **polkadot-stack-template**.

This template demonstrates the full Polkadot stack through a single concept: **Proof of Existence**.

The same idea (claim a file hash on-chain) is implemented three ways:

- A **FRAME pallet** (native Substrate)
- A **Solidity contract on EVM** (Ethereum bytecode)
- A **Solidity contract on PVM** (PolkaVM / RISC-V bytecode)

Notes:

The polkadot-stack-template is your hands-on project for this course. It's a full-stack application that touches every layer of the Polkadot ecosystem. The idea is simple: prove that a file existed at a certain time by recording its hash on-chain. But we implement it three different ways to show you the different development paths available on Polkadot. You'll also build a React frontend and a Rust CLI to interact with all three implementations.

---

## Agenda

1. **Why Polkadot Exists** - The Decentralized Web
1. **Polkadot Platform** - The Web3 Cloud Infrastructure
1. **Polkadot Stack** - Developer Tools & Experience
1. **Polkadot Triangle** - User Interfaces
1. **Putting It All Together** - The Complete Picture

---

# Polkadot Platform

## Web3 Cloud Infrastructure

Notes:

Let's start with the foundation. The Polkadot Platform is the blockchain infrastructure layer. Think of it as a decentralized cloud. It provides compute, storage, and networking that your applications run on. Unlike traditional cloud providers, this infrastructure is trustless, permissionless, and governed by its stakeholders.

---v

## The Relay Chain

The **heart** of the Polkadot network.

<div class="grid grid-cols-2">
<div class="text-left">

- Provides **shared security** to all connected chains
- Validates parachain blocks (the "auditing layer")
- Runs NPoS (Nominated Proof of Stake) for validator selection
- Does **NOT** host user applications directly
- Being slimmed down post-AHM (Asset Hub Migration)

</div>
<div>

<diagram class="mermaid">
graph TB
    RC["Relay Chain<br/>(Shared Security)"]
    RC --- AH["Asset Hub"]
    RC --- BH["Bridge Hub"]
    RC --- CT["Coretime"]
    RC --- PC["People"]
    RC --- CO["Collectives"]
    RC --- BC["Bulletin"]
    RC --- P1["Your Chain"]
</diagram>

</div>
</div>

Notes:

The Relay Chain is the central chain of Polkadot. Its primary job is to provide shared security to all connected parachains. It does this by coordinating a set of validators who verify parachain blocks without re-executing every transaction. Think of it as an auditing layer. Importantly, the Relay Chain itself is NOT where users submit transactions. That's what the system parachains are for. There's actually an ongoing migration called AHM, the Asset Hub Migration, which is moving even more functionality off the Relay Chain and onto Asset Hub, making the Relay Chain purely focused on validation and security.

---v

## Asset Hub

The **primary user-facing chain** in the Polkadot ecosystem.

<div class="grid grid-cols-2">
<div class="text-left">

**Assets:**
- DOT native token management
- Fungible assets (tokens)
- NFTs (non-fungible tokens)
- DEX / asset conversion

**Governance (migrated from Relay):**
- Treasury, Referenda, Bounties

**Staking (migrating from Relay):**
- Nomination Pools, Delegated Staking

</div>
<div class="text-left">

**Smart Contracts:**
- **pallet-revive**: EVM-compatible smart contracts
- Deploy Solidity to both **EVM** and **PolkaVM**
- Full Ethereum tooling support (MetaMask, Hardhat, Foundry)

**Identity:**
- Claims, Vesting

**Utilities:**
- Proxy, Multisig, Scheduler

</div>
</div>

Notes:

Asset Hub is becoming THE chain in Polkadot. It started as a simple chain for managing assets, but it's evolving into the primary chain where users interact. It now hosts fungible tokens, NFTs, a DEX for asset conversion, and critically, smart contracts via pallet-revive. Governance and staking are migrating here from the Relay Chain. Post-AHM, if you want to do something on Polkadot, Asset Hub is probably where you'll do it. For your projects, this is the chain you'll be deploying to.

---v

## Bridge Hub

Connecting Polkadot to the outside world.

<div class="grid grid-cols-2">
<div class="text-left">

**Kusama Bridge:**
- GRANDPA light client verification
- Bidirectional message passing
- XCM routing between ecosystems

**Ethereum Bridge (Snowbridge):**
- Ethereum Beacon Chain light client
- Inbound/outbound message queues
- Token transfers between Polkadot and Ethereum

</div>
<div>

<diagram class="mermaid">
graph LR
    E["Ethereum"] <-->|"Snowbridge"| BH["Bridge Hub"]
    K["Kusama"] <-->|"GRANDPA Bridge"| BH
    BH <-->|"XCM"| AH["Asset Hub"]
</diagram>

</div>
</div>

Notes:

Bridge Hub is the dedicated chain for cross-ecosystem bridges. It runs light clients of external chains to verify their state trustlessly. The Kusama bridge uses a GRANDPA light client, and the Ethereum bridge (Snowbridge) runs an Ethereum Beacon Chain light client. This means Polkadot can verify Ethereum blocks on-chain and vice versa, enabling trustless token transfers and message passing between ecosystems.

---v

## Coretime Chain

The **marketplace for blockspace**.

<div class="grid grid-cols-2">
<div class="text-left">

The Relay Chain has **cores**, each capable of progressing one parachain at a time.

**Two ways to get coretime:**

1. **Bulk Coretime** - Purchase cores in advance through periodic sales (Broker pallet)
2. **On-Demand Coretime** - Pay per block, no commitment needed

This replaced the legacy auction/lease system with a flexible market.

</div>
<div>

<diagram class="mermaid">
graph TB
    B["Broker Pallet<br/>(Coretime Chain)"]
    B -->|"Bulk Sales"| C1["Core 1"]
    B -->|"Bulk Sales"| C2["Core 2"]
    OD["On-Demand<br/>(Relay Chain)"] -->|"Pay per Block"| C3["Core 3"]
    C1 --> P1["Parachain A"]
    C2 --> P2["Parachain B"]
    C3 --> P3["Any Parachain"]
</diagram>

</div>
</div>

Notes:

Think of Polkadot like a computer with multiple CPU cores. Each core can process one parachain's block at a time. The Coretime chain runs a marketplace where teams can purchase access to these cores. Bulk coretime is like reserving a server: you buy cores in advance for a period. On-demand coretime is like serverless: you pay per block when you need it. This is the "gas" of Polkadot, it's how you pay for your chain's execution. For development and testing, on-demand coretime is very accessible.

---v

## People Chain

On-chain identity for the Polkadot ecosystem.

<div class="text-left">

- Register your on-chain identity (display name, email, web, social handles)
- **Registrar system** for third-party identity verification
- Username system (up to 32 characters)
- Sub-accounts (up to 100 per identity)
- Used by governance, staking, and the broader ecosystem

</div>

Notes:

The People chain hosts Polkadot's identity system. If you've ever seen a verified identity on a Polkadot block explorer, that's coming from this chain. You can register your display name, email, website, and social handles, and then have registrars verify your identity. This is important for governance participation and building trust in the ecosystem.

---v

## Collectives Chain

On-chain bodies that serve the Polkadot network.

<div class="text-left">

**Technical Fellowship:**
- Ranked collective of protocol developers (Dan 1-9)
- Can whitelist runtime upgrades
- Has its own treasury and salary system

**Ambassador Program:**
- Community representatives (Candidate through Head Ambassador)
- Funded through a dedicated sub-treasury

**Secretary Collective:**
- Administrative support

</div>

Notes:

The Collectives chain hosts the governance bodies of Polkadot. The most important is the Technical Fellowship, which is a ranked collective of core protocol developers. They review and approve runtime upgrades, and members receive salaries funded by Polkadot's treasury. The Ambassador program manages community representation. These are all on-chain, transparent, and governed by the Polkadot community.

---v

## Bulletin Chain

**Persistent data storage** for the Polkadot ecosystem.

<div class="grid grid-cols-2">
<div class="text-left">

- Store arbitrary data on-chain (up to 2 MiB per transaction)
- **IPFS-compatible**: data gets Blake2b CIDs, retrievable from IPFS
- Automatic chunking for large files
- Authorization-based access (no fees)
- Default 2-week retention, renewable
- SDKs in **Rust** and **TypeScript**

</div>
<div>

```typescript
// TypeScript SDK example
import { AsyncBulletinClient } from
  "@aspect-build/bulletin-sdk";

const client = new AsyncBulletinClient(
  api, signer
);

// Store data - gets an IPFS CID back
await client.store(myData).send();
```

</div>
</div>

Notes:

The Bulletin Chain is a specialized chain for data storage. Think of it as Polkadot's built-in IPFS pinning service. You can store data on-chain and it automatically becomes available via IPFS using standard content identifiers. This is perfect for storing dApp frontends, documents, or any data you want to be persistently and decentrally available. The SDK handles chunking large files automatically. We'll use this in the stack template for uploading frontend builds.

---v

## Custom Parachains

Build your **own** application-specific blockchain.

<div class="grid grid-cols-2">
<div class="text-left">

- Use the **Polkadot SDK** (Substrate + FRAME + Cumulus)
- Fully customizable runtime logic
- Deploy via **Coretime** (bulk or on-demand)
- Get **shared security** from the Relay Chain
- Communicate with other chains via **XCM**

This is what makes Polkadot a **heterogeneous sharded network**: each parachain can have completely different logic while sharing the same security.

</div>
<div>

<diagram class="mermaid">
graph TB
    SDK["Polkadot SDK"] --> R["Custom Runtime<br/>(FRAME Pallets)"]
    R --> N["Parachain Node<br/>(Cumulus)"]
    N --> RC["Relay Chain<br/>(Shared Security)"]
</diagram>

</div>
</div>

Notes:

Beyond the system parachains that Polkadot provides, anyone can build their own parachain using the Polkadot SDK. This is the most powerful option: you get to define your own state transition function, your own storage model, your own transaction types, everything. It's like having your own blockchain that inherits Polkadot's security. You'll learn how to build runtimes with FRAME later in this course.

---v

## Platform Summary

<diagram class="mermaid">
graph TB
    subgraph System["System Parachains"]
        AH["Asset Hub<br/>Tokens, NFTs, DEX<br/>Governance, Staking<br/>Smart Contracts"]
        BH["Bridge Hub<br/>Ethereum Bridge<br/>Kusama Bridge"]
        CT["Coretime<br/>Blockspace Market"]
        PC["People<br/>Identity"]
        CO["Collectives<br/>Fellowship"]
        BC["Bulletin<br/>Data Storage"]
    end

    subgraph Custom["Custom Parachains"]
        P1["DeFi Chain"]
        P2["Gaming Chain"]
        P3["Your Chain"]
    end

    RC["Relay Chain<br/>(Shared Security + Validation)"]

    RC --- AH
    RC --- BH
    RC --- CT
    RC --- PC
    RC --- CO
    RC --- BC
    RC --- P1
    RC --- P2
    RC --- P3
</diagram>

Notes:

Here's the full picture of the Polkadot Platform. The Relay Chain sits at the center providing shared security. Around it are the system parachains, each serving a specific purpose. And then there are custom parachains that teams build for their specific applications. All of these chains can communicate with each other via XCM, the cross-consensus messaging format. This is the infrastructure your applications run on.

---

# Polkadot Stack

## Developer Tools & Experience

Notes:

Now let's move up to the developer tools layer. This is where you spend your time as a builder. The Polkadot Stack gives you multiple paths to build applications, from writing custom blockchain runtimes to deploying Solidity smart contracts to building frontends with TypeScript.

---v

## Polkadot SDK

The **monorepo** containing all core components for building on Polkadot.

<div class="grid grid-cols-3">
<div class="text-left">

**Substrate**

The blockchain framework.

- Node infrastructure
- Networking (libp2p)
- Consensus engines
- Database (RocksDB)
- RPC server
- WASM executor

</div>
<div class="text-left">

**FRAME**

The runtime framework.

- 120+ pallets
- Storage abstractions
- Dispatchable calls
- Events & errors
- Benchmarking
- Migrations

</div>
<div class="text-left">

**Cumulus**

The parachain toolkit.

- Parachain system pallet
- Collator logic
- Relay chain integration
- XCM support
- Omni-node binary

</div>
</div>

Notes:

The Polkadot SDK is a single monorepo that contains three formerly separate projects. Substrate is the blockchain framework: it provides all the node-side infrastructure like networking, consensus, and the database. FRAME is the runtime development framework: it gives you modular building blocks called pallets to compose your chain's business logic. Cumulus transforms a Substrate chain into a Polkadot parachain. Together, they give you everything you need to build a custom blockchain.

---v

## FRAME: Building Blocks

**F**ramework for **R**untime **A**ggregation of **M**odularized **E**ntities

<div class="grid grid-cols-2">
<div class="text-left">

A **pallet** is a module of encapsulated blockchain logic:

- **Config** - configurable types and values
- **Storage** - on-chain state
- **Calls** - dispatchable functions (extrinsics)
- **Events** - observable outcomes
- **Errors** - well-formed error types
- **Hooks** - lifecycle callbacks

</div>
<div>

```rust
#[frame::pallet]
pub mod pallet {
    #[pallet::storage]
    pub type Claims<T> = StorageMap<
        _, Blake2_128Concat,
        H256, Claim<T>
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn create_claim(
            origin: OriginFor<T>,
            hash: H256,
        ) -> DispatchResult {
            // ...
        }
    }
}
```

</div>
</div>

Notes:

FRAME is what you'll spend most of this course learning. Each pallet is a self-contained module with clear responsibilities. You define your storage layout, your callable functions, your events, and your errors. The construct_runtime! macro then wires everything together into a complete runtime. The code on the right is from the stack template's Proof of Existence pallet. It defines a storage map from hashes to claims, and a function to create a claim.

---v

## Smart Contracts: pallet-revive

Write **Solidity** and deploy to Polkadot.

<diagram class="mermaid">
graph LR
    SOL["Solidity<br/>Source Code"]
    SOL -->|"solc"| EVM["EVM Bytecode<br/>(REVM)"]
    SOL -->|"resolc"| PVM["PVM Bytecode<br/>(PolkaVM / RISC-V)"]
    EVM --> PR["pallet-revive<br/>(Asset Hub)"]
    PVM --> PR
</diagram>

<div class="text-left">

- **Same Solidity source** compiles to two targets
- **EVM**: traditional Ethereum bytecode, runs in REVM interpreter
- **PVM**: PolkaVM (RISC-V based), native to Polkadot, more efficient
- Both deployed through the same chain, same tooling
- `resolc` is the Solidity-to-PolkaVM compiler (from the `revive` project)

</div>

Notes:

One of the most exciting developments in Polkadot is pallet-revive. It lets you write Solidity smart contracts and deploy them to Polkadot. The same Solidity source code can be compiled to two different targets: traditional EVM bytecode using the standard solc compiler, or PolkaVM bytecode using resolc, which compiles Solidity to RISC-V instructions. PolkaVM is Polkadot's own virtual machine, designed for security and performance. In the stack template, we deploy the exact same Proof of Existence contract to both targets.

---v

## The eth-rpc Sidecar

**Bridging Ethereum tooling to Substrate.**

<diagram class="mermaid">
graph LR
    MM["MetaMask<br/>Hardhat<br/>Foundry<br/>viem / ethers.js"]
    MM -->|"Ethereum JSON-RPC<br/>(port 8545)"| ETH["eth-rpc<br/>Sidecar"]
    ETH -->|"Substrate<br/>WebSocket"| NODE["Substrate<br/>Node"]
</diagram>

<div class="text-left">

- Runs as a **standalone process** alongside your Substrate node
- Translates Ethereum JSON-RPC calls into Substrate calls
- Supports: `eth_sendRawTransaction`, `eth_call`, `eth_getLogs`, `debug_traceTransaction`, etc.
- Maintains a **SQLite index** for receipts and blocks
- Pre-configured **dev accounts** for local development

</div>

Notes:

The eth-rpc sidecar is the magic that makes Ethereum tooling work with Polkadot. It's a separate process that speaks Ethereum JSON-RPC on port 8545, just like Geth or Hardhat Network. Under the hood, it translates those calls into Substrate RPC calls using the subxt library. This means you can use MetaMask to sign transactions, Hardhat to deploy contracts, Foundry to test, and any Ethereum library to interact with your contracts, all while running on a Polkadot chain.

---v

## Two Developer Access Paths

<diagram class="mermaid">
graph TB
    subgraph Native["Native Substrate Path"]
        PAPI["PAPI<br/>(TypeScript)"]
        SUBXT["subxt<br/>(Rust)"]
    end

    subgraph Ethereum["Ethereum-Compatible Path"]
        VIEM["viem / ethers.js<br/>(TypeScript)"]
        ALLOY["alloy<br/>(Rust)"]
    end

    Native -->|"WebSocket<br/>Substrate JSON-RPC"| NODE["Substrate Node"]
    Ethereum -->|"HTTP<br/>Ethereum JSON-RPC"| ETHRPC["eth-rpc Sidecar"]
    ETHRPC --> NODE
</diagram>

<div class="text-left">

Both paths talk to the **same chain**, the **same state**, the **same contracts**.

Choose based on what you're interacting with:
- **Pallets** (native logic) -> use PAPI or subxt
- **Smart Contracts** (Solidity) -> use viem/alloy through eth-rpc

</div>

Notes:

This is a key architectural insight. There are two parallel paths to interact with the same Polkadot chain. The native Substrate path uses PAPI for TypeScript or subxt for Rust, connecting directly to the node via WebSocket. The Ethereum-compatible path uses viem or alloy, connecting through the eth-rpc sidecar. Both paths access the same chain state. In the stack template, the frontend uses PAPI for pallet interactions and viem for contract interactions. The CLI uses subxt and alloy respectively.

---v

## PAPI: Polkadot API

The modern **TypeScript** client for Polkadot.

<div class="grid grid-cols-2">
<div class="text-left">

**Key Features:**
- **Light-client first** - built on smoldot
- **Typed API** - generated from on-chain metadata
- **Native BigInt** - no heavy BigNumber libraries
- **Tree-shakeable** - only bundle what you use
- **Promise & Observable APIs**

Replaces the older polkadot.js library.

</div>
<div>

```typescript
import { createClient } from "polkadot-api";
import { getSmProvider }
  from "polkadot-api/sm-provider";

// Connect via light client
const client = createClient(
  getSmProvider(smoldot.addChain({
    chainSpec
  }))
);

// Typed storage query
const claim = await client
  .getTypedApi(descriptors)
  .query.TemplatePallet
  .Claims.getValue(hash);
```

</div>
</div>

Notes:

PAPI is THE TypeScript library for Polkadot development. Its killer feature is being light-client first. Instead of connecting to a centralized RPC endpoint, it can embed smoldot, a Substrate light client that runs directly in the browser. This means your dApp can verify chain state trustlessly. It also generates fully typed TypeScript APIs from on-chain metadata, so you get autocomplete and type-checking for every storage query, transaction, and event. This is a huge improvement over polkadot.js which provided types at runtime.

---v

## subxt: Rust Client

The **Rust** equivalent of PAPI.

<div class="grid grid-cols-2">
<div class="text-left">

**Key Features:**
- **Typed API** from metadata (proc macro)
- **Dynamic API** for untyped access
- Full chain interaction: storage, extrinsics, events, blocks
- Supports sr25519, ed25519, secp256k1 signing

Name stands for "**sub**mit e**xt**rinsics".

</div>
<div>

```rust
use subxt::{OnlineClient, PolkadotConfig};

let api = OnlineClient::<PolkadotConfig>
    ::from_url("ws://localhost:9944")
    .await?;

// Dynamic storage query
let claim = api.storage()
    .at_latest().await?
    .fetch(&subxt::dynamic::storage(
        "TemplatePallet",
        "Claims",
        vec![hash.into()],
    )).await?;
```

</div>
</div>

Notes:

subxt is the Rust counterpart to PAPI. If you're building a backend service, a CLI tool, or any Rust application that needs to talk to a Substrate chain, this is what you use. Like PAPI, it can generate typed APIs from chain metadata at compile time. The stack template's CLI tool uses subxt for all pallet interactions, and alloy for all contract interactions through the eth-rpc adapter.

---v

## Ethereum Tooling

Use your existing Ethereum skills and tools.

<div class="grid grid-cols-2">
<div class="text-left">

**Frontend (TypeScript):**
- **viem** - modern, typed Ethereum client
- **ethers.js** - the classic
- **wagmi** - React hooks for Ethereum

**Backend (Rust):**
- **alloy** - next-gen Rust Ethereum library

**Development:**
- **Hardhat** - with `@parity/hardhat-polkadot`
- **Foundry** - forge, cast, anvil
- **MetaMask** - wallet

</div>
<div>

```typescript
import { createPublicClient, http }
  from "viem";

// Connect through eth-rpc
const client = createPublicClient({
  transport: http(
    "http://localhost:8545"
  ),
});

// Read contract (same as Ethereum!)
const claim = await client
  .readContract({
    address: contractAddress,
    abi: proofOfExistenceAbi,
    functionName: "getClaim",
    args: [documentHash],
  });
```

</div>
</div>

Notes:

If you already know Ethereum development, you can bring all of that knowledge to Polkadot. viem, ethers.js, Hardhat, Foundry, MetaMask, all work through the eth-rpc sidecar. The code looks identical to what you'd write for Ethereum. In the stack template, Hardhat with the @parity/hardhat-polkadot plugin handles compiling contracts to both EVM and PVM targets, and deploying them through eth-rpc. The frontend uses viem for all contract interactions.

---v

## The polkadot-stack-template

<div class="text-small">

| Component | Path | Tech Stack |
|-----------|------|------------|
| **FRAME Pallet** | `blockchain/` | Rust, FRAME, Cumulus, polkadot-omni-node |
| **EVM Contract** | `contracts/evm/` | Solidity, Hardhat, solc, viem |
| **PVM Contract** | `contracts/pvm/` | Solidity, Hardhat, resolc, `@parity/hardhat-polkadot` |
| **React Frontend** | `web/` | React, Vite, TypeScript, PAPI, viem, Zustand |
| **Rust CLI** | `cli/` | Rust, subxt, alloy, clap |
| **Dev Scripts** | `scripts/` | Zombienet, docker-compose |

</div>

<div class="text-left">

Every component is **optional and removable**. Teams keep the slices they need.

Notes:

Here's the complete layout of the stack template. Five major components, each demonstrating a different part of the Polkadot developer experience. The blockchain directory contains a full parachain runtime with a custom pallet. The contracts directory has the same Solidity contract compiled to two targets. The web directory is a React frontend that talks to all three implementations. The CLI is a Rust tool that does the same. And the scripts directory orchestrates local development with zombienet.

---v

## Stack Template: The Pallet

```rust
#[pallet::storage]
pub type Claims<T: Config> =
    StorageMap<_, Blake2_128Concat, H256, Claim<T>>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(T::WeightInfo::create_claim())]
    pub fn create_claim(
        origin: OriginFor<T>,
        hash: H256,
    ) -> DispatchResult {
        let sender = ensure_signed(origin)?;
        ensure!(!Claims::<T>::contains_key(hash), Error::<T>::AlreadyClaimed);
        let claim = Claim { owner: sender.clone(), block: frame_system::Pallet::<T>::block_number() };
        Claims::<T>::insert(hash, claim);
        Self::deposit_event(Event::ClaimCreated { who: sender, hash });
        Ok(())
    }
}
```

Notes:

Here's the actual pallet code from the template. It's a clean, minimal FRAME pallet. A StorageMap maps 32-byte blake2 hashes to Claims, which record the owner and block number. The create_claim function checks that the hash isn't already claimed, creates a new claim, inserts it into storage, and emits an event. This is the pattern you'll learn throughout the FRAME module of this course.

---v

## Stack Template: The Contract

```solidity
contract ProofOfExistence {
    struct Claim {
        address owner;
        uint256 blockNumber;
    }

    mapping(bytes32 => Claim) public claims;

    function createClaim(bytes32 documentHash) external {
        require(claims[documentHash].owner == address(0), "Already claimed");
        claims[documentHash] = Claim(msg.sender, block.number);
        emit ClaimCreated(msg.sender, documentHash);
    }
}
```

Same logic, same ABI. Compiles to **EVM** (solc) and **PVM** (resolc).

Notes:

And here's the Solidity version of the same logic. Same concept, same structure, just in Solidity instead of Rust. The remarkable thing is that this exact same source file gets compiled to two different bytecode formats. solc produces traditional EVM bytecode, and resolc produces PolkaVM RISC-V bytecode. Both are deployed to the same chain through pallet-revive, and both are accessible through the eth-rpc sidecar. The frontend and CLI can interact with both using the same ABI.

---v

## Stack Template: The Frontend

<div class="grid grid-cols-2">
<div class="text-left">

**Six pages:**
1. Home
2. Pallet PoE (via PAPI)
3. EVM PoE (via viem)
4. PVM PoE (via viem)
5. Statements
6. Accounts

**Smart host detection:**
- Running inside Triangle host? Use host wallet
- Standalone browser? Use browser extensions

</div>
<div class="text-left">

```typescript
// Pallet interaction (PAPI)
const api = client.getTypedApi(descriptors);
await api.tx.TemplatePallet
  .create_claim({ hash })
  .signAndSubmit(signer);

// Contract interaction (viem)
const tx = await walletClient
  .writeContract({
    address: evmContract,
    abi,
    functionName: "createClaim",
    args: [hash],
  });
```

</div>
</div>

Notes:

The React frontend demonstrates both access paths side by side. For pallet interactions, it uses PAPI with typed descriptors. For contract interactions, it uses viem. Both are talking to the same chain. The frontend also has smart host detection: if it's running inside a Triangle User Agent like Polkadot Desktop, it uses the host's wallet for accounts. If it's running standalone in a browser, it falls back to browser extension wallets like Polkadot.js or Talisman.

---v

## Stack Template: The CLI

<div class="grid grid-cols-2">
<div class="text-left">

**Commands:**

```
stack-cli pallet create-claim <hash>
stack-cli pallet get-claim <hash>

stack-cli contract create-claim evm <hash>
stack-cli contract create-claim pvm <hash>

stack-cli chain info
stack-cli chain blocks

stack-cli prove <file>
```

</div>
<div class="text-left">

**Tech mapping:**

| Target | Library |
|--------|---------|
| Pallets | subxt (Substrate WS) |
| Contracts | alloy (eth-rpc HTTP) |
| Chain info | subxt |
| Prove | both + bulletin chain |

The `prove` command does it all: hash, claim, and optionally upload to the Bulletin Chain.

</div>
</div>

Notes:

The Rust CLI mirrors everything the frontend does, but from the command line. Pallet commands use subxt for native Substrate interaction. Contract commands use alloy through the eth-rpc adapter. The prove command is the all-in-one: it hashes a file, creates a claim on either the pallet or contract, and optionally uploads the file to the Bulletin Chain for persistent storage. This is a great reference for building Rust backend services.

---v

## DotNS: .dot Name Service

Human-readable names for the Polkadot ecosystem.

<div class="grid grid-cols-2">
<div class="text-left">

**Like ENS, but on Polkadot:**
- Register `myapp.dot`
- Points to an IPFS CID (your dApp frontend)
- Also resolves to Polkadot addresses
- Solidity contracts on Asset Hub (via pallet-revive)
- Commit-reveal to prevent front-running
- Proof-of-Personhood aware pricing

</div>
<div>

<diagram class="mermaid">
graph TB
    U["User types myapp.dot"] --> H["Host resolves name"]
    H -->|"namehash"| C["DotNS Contract<br/>(Asset Hub)"]
    C -->|"contenthash"| CID["IPFS CID"]
    CID -->|"fetch"| IPFS["IPFS Network"]
    IPFS --> APP["dApp loads<br/>in sandbox"]
</diagram>

</div>
</div>

Notes:

DotNS is Polkadot's name service, similar to ENS on Ethereum. You register a .dot name and point it to an IPFS content hash. When a user navigates to myapp.dot in a Triangle User Agent, the host resolves the name on-chain, gets the IPFS CID, fetches the content, and renders it in a sandboxed view. The registration uses a commit-reveal scheme to prevent front-running, and pricing is aware of Proof-of-Personhood status, names are cheaper or free for verified humans.

---v

## Development Infrastructure

<div class="grid grid-cols-2">
<div class="text-left">

**Local Development:**
- **Zombienet** - spin up local multi-chain networks
- **polkadot-omni-node** - generic parachain node binary
- **chain-spec-builder** - generate chain specifications

**Deployment:**
- Deploy to **Paseo testnet** (Polkadot's testnet)
- Deploy frontends to **IPFS** (web3.storage / Bulletin Chain)
- Register **.dot domains** via DotNS

</div>
<div class="text-left">

**Debugging & Monitoring:**
- **Blockscout** - block explorer with EVM support
- **Polkadot.js Apps** - Substrate explorer and governance UI
- **eth-rpc debug APIs** - transaction tracing
- **PAPI devtools** - chain interaction debugging

**Package Management:**
- **psvm** - Polkadot SDK Version Manager
- **Umbrella crate** - single dependency for polkadot-sdk

</div>
</div>

Notes:

A few more tools worth knowing about. Zombienet lets you spin up a local multi-chain test network with a single config file, great for development. The omni-node is a generic parachain binary that can load any runtime, so you don't need to compile a custom node. For deployment, Paseo is the canonical testnet. You can deploy frontends to IPFS and register dot domains to make them accessible through Triangle hosts. Blockscout and Polkadot.js Apps are the main block explorers, and psvm helps manage SDK versions across your Cargo.toml files.

---v

## Developer Stack Summary

<diagram class="mermaid">
graph TB
    subgraph Languages["Write In"]
        RUST["Rust<br/>(Pallets, CLI)"]
        SOL["Solidity<br/>(Contracts)"]
        TS["TypeScript<br/>(Frontend)"]
    end

    subgraph Frameworks["Build With"]
        FRAME["FRAME"]
        HH["Hardhat + resolc"]
        REACT["React + Vite"]
    end

    subgraph Libraries["Connect Via"]
        SUBXT["subxt"]
        ALLOY["alloy"]
        PAPIL["PAPI"]
        VIEML["viem"]
    end

    subgraph Access["Access Through"]
        WS["Substrate WS RPC"]
        HTTP["eth-rpc (port 8545)"]
    end

    RUST --> FRAME
    RUST --> SUBXT
    RUST --> ALLOY
    SOL --> HH
    TS --> REACT
    TS --> PAPIL
    TS --> VIEML

    FRAME --> WS
    SUBXT --> WS
    PAPIL --> WS
    ALLOY --> HTTP
    VIEML --> HTTP
    HH --> HTTP
</diagram>

Notes:

Here's the complete developer stack in one picture. Three languages feed into three build frameworks, which connect through four client libraries, accessing two RPC endpoints that both talk to the same chain. The beauty is that you can mix and match: write a pallet in Rust and interact with it from TypeScript, or write a Solidity contract and call it from your Rust CLI. Everything is interoperable.

---

# Polkadot Triangle

## User Interfaces

Notes:

Now let's look at the top layer: how users actually interact with your applications. The Polkadot Triangle is an architecture for building secure, user-friendly applications that embed Polkadot dApps.

---v

## The Triangle Architecture

<diagram class="mermaid">
graph TB
    subgraph Host["Host (Triangle User Agent)"]
        W["Wallet & Keys"]
        LC["Light Clients"]
        IPFS["IPFS / DotNS"]
        EXT["Extensions"]
    end

    subgraph Product["Product (Your dApp)"]
        UI["User Interface"]
        LOGIC["App Logic"]
    end

    subgraph Chain["Blockchain"]
        AH2["Asset Hub"]
        BC2["Bulletin Chain"]
        PC2["Custom Chains"]
    end

    Product -->|"window.host<br/>(sandboxed bridge)"| Host
    Host -->|"Light client /<br/>RPC"| Chain
</diagram>

<div class="text-left">

Three parties: **Host** owns security, **Product** runs in sandbox, **Blockchain** provides state.

</div>

Notes:

The Triangle is a three-party architecture. The Host is a trusted application shell, think of it like a web browser for Polkadot. It owns all the security-critical infrastructure: private keys, light client connections, and network access. The Product is your dApp, running inside a sandboxed container with zero direct network access. It communicates with the blockchain exclusively through the host's bridge protocol. This separation means your dApp never touches private keys and can't make unauthorized network requests.

---v

## Triangle User Agents

<div class="grid grid-cols-3">
<div class="text-left">

**Desktop**

Built with Tauri + Rust

- Full host-sdk crates
- Native keychain (Touch ID)
- Maximum performance
- macOS, Linux, Windows

</div>
<div class="text-left">

**Mobile**

iOS (Swift) + Android (Kotlin)

- UniFFI bindings to Rust
- Platform-native UX
- iOS 17+ / API 26+
- Biometric auth

</div>
<div class="text-left">

**Web (dot.li)**

Browser-based

- JS smoldot light client
- Helia for IPFS
- PAPI for chain interaction
- No install needed

</div>
</div>

Notes:

There are three main Triangle hosts being built. The desktop app uses Tauri, a lightweight alternative to Electron that wraps a Rust backend with a native webview. The mobile apps use UniFFI to bridge the Rust host-sdk to Swift and Kotlin. And dot.li is a web-based host that runs entirely in the browser using smoldot and Helia for IPFS. All three share the same core logic from the host-sdk, just with different platform shells. dot.li is the easiest to try since it requires no installation.

---v

## Host SDK (UserAgentKit)

What the host provides to your dApp:

<div class="grid grid-cols-2">
<div class="text-left">

**Core Capabilities:**
- **Wallet** - BIP-39, sr25519, Ed25519, secp256k1
- **Light Clients** - smoldot (Substrate), Helios (Ethereum), Kyoto (Bitcoin)
- **DOTNS Resolution** - `.dot` name to IPFS content
- **IPFS** - P2P Bitswap + HTTP gateway fallback
- **Statement Store** - off-chain pub/sub messaging

</div>
<div class="text-left">

**Extensions (`window.host.ext.*`):**
- **data** - peer-to-peer data channels
- **media** - audio/video calls
- **files** - file saving
- **mesh** - distributed object storage
- **crdt** - collaborative editing
- **vrf** - Bandersnatch ring VRF

</div>
</div>

Notes:

The host-sdk, also called UserAgentKit, is the Rust library for building Triangle hosts. It provides a comprehensive set of capabilities. The wallet manages keys securely with platform-native keychain integration. Light clients connect to multiple chains simultaneously. The DOTNS resolver translates dot names into IPFS content. And the extension system provides powerful P2P capabilities like data channels, media calls, and collaborative editing. These extensions are available to products through the window.host.ext namespace.

---v

## Product SDK

What your dApp uses to talk to the host:

<div class="grid grid-cols-2">
<div class="text-left">

```typescript
import {
  getAddress,
  navigateTo,
} from "@polkadot-apps/product-sdk";

// Get the current user's account
const address = await getAddress();

// Navigate to another .dot product
navigateTo("other-app.dot");
```

</div>
<div class="text-left">

```typescript
import {
  statements,
  storage,
} from "@polkadot-apps/product-sdk";

// Scoped key-value storage
await storage.set("key", "value");
const val = await storage.get("key");

// Pub/sub via statement store
statements.subscribe(topic, (msg) => {
  console.log("New message:", msg);
});
await statements.write(topic, payload);
```

</div>
</div>

Notes:

The Product SDK is what your dApp imports to communicate with the host. It's a thin TypeScript library that wraps the window.host bridge protocol. You can get the user's account address, navigate between dot products, use scoped storage, and subscribe to real-time messages through the statement store. The API is simple by design. Your dApp doesn't need to know about light clients, key management, or network protocols, the host handles all of that.

---v

## Product Isolation

Your dApp runs in a **strict sandbox**.

<div class="text-left">

**What products CANNOT do:**
- No network access (`connect-src 'none'`)
- No WebSocket, WebRTC, Workers, BroadcastChannel
- No localStorage, cookies, or caches
- No access to `window.ethereum`, `window.polkadot`, etc.
- No auto-approval of signing requests

**What products CAN do:**
- Run JavaScript and WASM
- Communicate through `window.host` bridge
- Request signatures (user must confirm each one)

</div>

Notes:

The security model is extremely strict. Products run in sandboxed iframes or webviews with essentially no permissions. A lockdown script neutralizes WebSocket, WebRTC, Workers, and other network APIs. The CSP blocks all outbound connections. Wallet extension globals are shadowed with undefined to prevent sniffing. Every signing request requires explicit user confirmation, auto-approval is prohibited. The only way a product can interact with the outside world is through the host bridge. This is a fundamental security improvement over the current browser-extension model where dApps have full network access.

---v

## The .dot Resolution Flow

What happens when a user navigates to `myapp.dot`:

<diagram class="mermaid">
sequenceDiagram
    participant U as User
    participant H as Host (dot.li)
    participant C as Asset Hub
    participant I as IPFS Network

    U->>H: Navigate to myapp.dot
    H->>H: namehash("myapp.dot")
    H->>C: ReviveApi::call(DotNS contenthash)
    C-->>H: IPFS CID
    H->>I: Fetch content (Bitswap + HTTP)
    I-->>H: dApp bundle (HTML/JS/CSS)
    H->>H: Load in sandboxed iframe
    H-->>U: dApp renders
</diagram>

Notes:

Let me walk through the complete flow. A user types myapp.dot into a Triangle host. The host computes the ENS-style namehash of the domain. It then calls the DotNS ContentResolver contract on Asset Hub via the light client. The contract returns an IPFS CID. The host fetches the content from IPFS using P2P Bitswap first, falling back to HTTP gateways. The fetched HTML/JS/CSS bundle is loaded into a sandboxed iframe, and the dApp renders. From the user's perspective, they just typed a name and got an app, no centralized servers involved.

---

# Putting It All Together

Notes:

Let's zoom out and see the complete picture, from developer to user.

---v

## The Complete Flow

<diagram class="mermaid">
graph TB
    subgraph Dev["Developer Builds"]
        D1["FRAME Pallet<br/>(Rust)"]
        D2["Solidity Contract"]
        D3["React Frontend<br/>(PAPI + viem)"]
    end

    subgraph Deploy["Deploy To"]
        E1["Parachain Runtime<br/>(via Coretime)"]
        E2["Asset Hub<br/>(via eth-rpc)"]
        E3["IPFS<br/>(via Bulletin Chain)"]
        E4["DotNS<br/>(myapp.dot)"]
    end

    subgraph User["User Accesses"]
        U1["dot.li / Desktop / Mobile"]
        U2["Resolves myapp.dot"]
        U3["Loads from IPFS"]
        U4["Interacts via Host"]
    end

    D1 --> E1
    D2 --> E2
    D3 --> E3
    E3 --> E4

    E4 --> U2
    U1 --> U2
    U2 --> U3
    U3 --> U4
    U4 -->|"PAPI / viem"| E1
    U4 -->|"PAPI / viem"| E2
</diagram>

Notes:

Here's the end-to-end flow. A developer writes their pallet in Rust, their contracts in Solidity, and their frontend in React. The pallet deploys as part of a parachain runtime via coretime. The contracts deploy to Asset Hub through eth-rpc. The frontend uploads to IPFS via the Bulletin Chain and gets a .dot domain via DotNS. A user opens a Triangle host, types the .dot name, and the host resolves it, fetches the frontend, and renders it in a sandbox. The frontend talks to the blockchain through the host's bridge, using PAPI for pallets and viem for contracts. No centralized servers anywhere in the chain.

---v

## What You Will Build

<div class="text-left">

During this course, using the **polkadot-stack-template**, you will:

1. Build a **FRAME pallet** with custom storage, calls, events, and errors
2. Write and deploy **Solidity contracts** to both EVM and PVM targets
3. Build a **React frontend** using PAPI and viem
4. Build a **Rust CLI** using subxt and alloy
5. Run a **local test network** with zombienet
6. Deploy to **IPFS** and register a **.dot domain**
7. Optionally integrate with the **Bulletin Chain** for data storage

</div>

Notes:

So what does this mean for you? Over the course of this program, you'll get hands-on experience with every layer of the stack. You'll learn FRAME deeply in the FRAME module, build smart contracts, create frontends, and deploy the full application. The stack template is designed so you can keep the pieces you need and discard the rest. Some teams will focus on the pallet, others on contracts, others on the full stack. The goal is that by the end, you can build and deploy a complete Polkadot application from scratch.

---v

## Key Resources

<div class="grid grid-cols-2">
<div class="text-left">

**Repositories:**
- `polkadot-sdk` - Core SDK
- `polkadot-stack-template` - Your project
- `host-sdk` - Triangle host SDK
- `dotns` - .dot name service
- `polkadot-bulletin-chain` - Data storage
- `polkadot-api` - PAPI

</div>
<div class="text-left">

**Documentation:**
- papi.how - PAPI docs
- paritytech.github.io/polkadot-sdk - SDK docs
- Blockscout explorer
- Polkadot Wiki

**Tools:**
- Polkadot.js Apps - Chain explorer
- Zombienet - Local networks
- psvm - SDK version manager

</div>
</div>

Notes:

Here are the key resources you'll need throughout the course. The polkadot-stack-template is your starting point. The PAPI docs at papi.how are excellent for frontend development. The Polkadot SDK docs cover everything about pallets and runtimes. Blockscout is your go-to block explorer, especially for smart contracts. And zombienet is essential for local development and testing.

---

<!-- .slide: data-background-color="#000000" -->

# Questions
