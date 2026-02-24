# Polkadot Blockchain Academy - Curriculum Report

> A comprehensive summary of all teachings in the PBA educational curriculum covering blockchain development, Polkadot protocol, and Web3 technologies.

**Generated:** January 2026
**Curriculum Version:** Current (Bali 2025 cohort structure)
**Total Lectures:** 76 current + 50 archived

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Program Overview](#program-overview)
3. [Teaching Methodology](#teaching-methodology)
4. [Module 1: Fundamentals](#module-1-fundamentals)
5. [Module 2: Polkadot Protocol](#module-2-polkadot-protocol)
6. [Module 3a: Protocol On-Chain (Polkadot SDK)](#module-3a-protocol-on-chain-polkadot-sdk)
7. [Module 3b: dApps Off-Chain](#module-3b-dapps-off-chain)
8. [Module 4: PVM & Smart Contracts](#module-4-pvm--smart-contracts)
9. [Module 5: Governance](#module-5-governance)
10. [Module 6: JAM (Join-Accumulate Machine)](#module-6-jam-join-accumulate-machine)
11. [Archived Content](#archived-content)
12. [Resources & Links](#resources--links)

---

## Executive Summary

The **Polkadot Blockchain Academy (PBA)** is an intensive academic program teaching blockchain technology, Substrate framework, and the Polkadot ecosystem to talented developers worldwide.

### Program Mission

The academy aims to:
- Improve the core developer hiring pipeline in both quantity and quality of candidates
- Seed the industry with Substrate knowledge and advocates
- Build a surplus of developers for product prototyping
- Improve non-developer onboarding processes

### Key Statistics

| Metric | Value |
|--------|-------|
| Current Lecture Slides | 76 |
| Archived Lecture Slides | 50 |
| Main Modules | 6 |
| Program Cohorts | 7 (2022-2025) |
| YouTube Playlists | 8 |
| Typical Class Size | 25-60 students |

---

## Program Overview

### Program History

| Cohort | Location | Date |
|--------|----------|------|
| 1 | Cambridge, UK | August 2022 |
| 2 | Buenos Aires, Argentina | January 2023 |
| 3 | Berkeley, USA | July 2023 |
| 4 | Hong Kong | January 2024 |
| 5 | Singapore | June 2024 |
| 6 | Lucerne, Switzerland | 2025 |
| 7 | Bali, Indonesia | August 2025 |

### Program Format

- **Type:** Intensive in-person course
- **Duration:** Approximately 3+ weeks
- **Class Size:** 25-60 students
- **Selection:** Competitive admission process
- **Instructors:** Professionals from Parity, Web3 Foundation, and visiting professors

### Venue Requirements

- Overhead projector visible to all students
- Aisles for instructors to assist with debugging
- Group workstations for 5-10 students
- Large whiteboard for diagrams
- Internet: >10Mbps with all students connected
- LAN supporting P2P connections for private blockchain networks

---

## Teaching Methodology

### Content Delivery Formats

#### 1. Lecture Slides (Reveal.js)
- Markdown-based presentations
- Interactive elements with progressive disclosure
- Speaker notes for instructors (accessible via 's' key)

#### 2. Workshops (30 min - 3 hours)
- Step-by-step guided activities
- Instructor-led, hands-on learning
- Collaborative group work

#### 3. Activities (Self-directed)
- Individual or small group assignments
- Varied student solutions expected
- Canonical solutions provided for review

#### 4. Exercises (5-10 minutes)
- Short in-lecture practical applications
- Integrated directly into slide decks

### Development Environment

- Local IDE (VSCode recommended)
- Rust toolchain with sccache optimization
- Online collaboration via REPL.it
- GitHub Classrooms for assignments and grading

---

## Module 1: Fundamentals

**Focus:** Core blockchain communication and infrastructure concepts

### Learning Objectives
- Understand SCALE codec encoding/decoding
- Comprehend blockchain runtime architecture
- Master metadata interpretation
- Implement JSON-RPC communication
- Build and use light clients

### Lectures (9 total)

| # | Topic | Description |
|---|-------|-------------|
| 1 | **SCALE Codec** | Simple Concatenated Aggregate Little-Endian serialization format. Covers binary data, hexadecimal representation, encoding primitives (u8-u256), and complex types (enums, options, results, structs) |
| 2 | **Runtime** | State transition functions, blockchain runtime concepts and architecture |
| 3 | **Metadata** | Chain metadata structure, interpretation, and introspection capabilities |
| 4 | **JSON-RPC** | Client-server communication protocol specification for blockchain interaction |
| 5.1 | **Light Client & Transactions (Part 1)** | Light client fundamentals and basics |
| 5.2 | **Light Client & Transactions (Part 2)** | Transaction handling mechanisms |
| 5.3 | **Light Client & Transactions (Part 3)** | Advanced light client concepts |
| 5.4 | **Light Client & Transactions (Part 4)** | Light client optimizations |
| 5.5 | **Light Client & Transactions (Part 5)** | Transaction processing and finalization |

---

## Module 2: Polkadot Protocol

**Focus:** Polkadot 2.0 architecture, consensus, and ecosystem

### Learning Objectives
- Understand Polkadot's core tenets (Accessibility, Generality, Security)
- Master sharding concepts (state, data, execution)
- Comprehend staking and consensus mechanisms
- Learn coretime allocation and ecosystem economics

### Lectures (12 total)

| # | Topic | Description |
|---|-------|-------------|
| a-1 | **Polkadot Introduction** | Core tenets, blockchain landscape overview, key decisions, actors, and technologies glossary |
| a-2 | **Staking** | Nominated Proof of Stake (NPoS), validator selection, economic incentives |
| a-3 | **Block Production** | BABE consensus, slot allocation, block authorship mechanisms |
| a-4 | **Data Availability** | Backing process, erasure coding, availability guarantees |
| a-5 | **Execution Sharding** | Approvals, disputes, parachain security model |
| a-6 | **Polkadot SDK & Tools** | Development toolkit overview and ecosystem tools |
| a-7 | **Confirmations & Finality** | GRANDPA finality gadget, confirmation times, security guarantees |
| a-8 | **State Sharding** | Collators, Cumulus framework, parachain state management |
| a-9 | **Agile Coretime** | Resource allocation, scheduling, coretime marketplace |
| b-10 | **Polkadot Cloud & Hub** | Infrastructure services, relay chain as service provider |
| b-11 | **Ecosystem & Economy** | Token economics, treasury, governance economics |
| b-12 | **Zombienet** | Testing infrastructure, local network simulation |

---

## Module 3a: Protocol On-Chain (Polkadot SDK)

**Focus:** Substrate, FRAME, and XCM development

### Sub-module: Substrate

**Duration:** ~2.5 days

#### Learning Objectives
- Understand Substrate architecture and design decisions
- Master Wasm meta-protocol concepts
- Implement merklized storage patterns
- Work with transaction pools

#### Lectures (8 total)

| # | Topic | Description |
|---|-------|-------------|
| 1 | **Introduction to Substrate** | Framework overview, architecture, node/runtime separation |
| 2 | **Wasm Meta Protocol** | Runtime APIs, host functions, Wasm as upgrade mechanism |
| 3 | **Merklized Storage** | Trie structure, state management, proof generation |
| 4 | **Transaction Pool** | Validation, ordering, priority, and transaction lifecycle |
| 9a | **SCALE (Advanced)** | Advanced encoding patterns and optimizations |
| 9b | **Substrate Interactions** | Client interactions, RPC, and external communication |
| 9c | **Substrate-FRAME Tips & Tricks** | Best practices and common patterns |
| 10 | **FRAMEless Workshop** | Building runtimes without FRAME framework |

---

### Sub-module: FRAME

**Duration:** ~6 days

#### Learning Objectives
- Build pallets from scratch
- Implement storage, events, and errors
- Configure runtime composition
- Perform benchmarking and migrations

#### Section 1: Introduction to FRAME (4 lectures)

| Topic | Description |
|-------|-------------|
| **Intro to FRAME** | Framework overview, pallet architecture, macro system |
| **Pallets & Traits** | Trait-based design, configuration traits, associated types |
| **Pallet Coupling** | Tight vs loose coupling, inter-pallet communication |
| **Fungible Assets** | Token standards, fungible trait implementations |

#### Section 2: FRAME Basics (4 lectures)

| Topic | Description |
|-------|-------------|
| **Calls** | Dispatchable functions, extrinsics, call weights |
| **Events and Errors** | Event emission, error handling, debugging patterns |
| **FRAME Storage** | StorageValue, StorageMap, StorageNMap, storage iterators |
| **Pallet Hooks** | Lifecycle hooks: on_initialize, on_finalize, on_idle |

#### Section 3: FRAME Runtime (2 lectures)

| Topic | Description |
|-------|-------------|
| **Construct Runtime** | Runtime assembly, pallet integration, runtime configuration |
| **Origins** | Permission systems, origin types, custom origins |

#### Section 4: FRAME Production (2 lectures)

| Topic | Description |
|-------|-------------|
| **Benchmarking** | Performance testing, weight calculation, worst-case analysis |
| **Migrations & Try Runtime** | Storage migrations, runtime upgrades, testing upgrades |

#### Section 5: FRAME Extras (5 lectures)

| Topic | Description |
|-------|-------------|
| **FRAME Deep Dive** | Advanced FRAME internals and patterns |
| **FRAME Extras** | Additional utilities and helper functions |
| **Outer Enum** | Aggregate types, call/event aggregation |
| **Signed Extensions** | Transaction validation, custom pre/post dispatch logic |
| **Transaction Extensions** | Extended transaction processing capabilities |

---

### Sub-module: XCM (Cross-Consensus Messaging)

**Duration:** ~3 days

#### Learning Objectives
- Understand cross-consensus messaging protocol
- Write XCM programs
- Configure chains for XCM execution
- Test with XCM emulator

#### Lectures (7 total)

| # | Topic | Description |
|---|-------|-------------|
| 1 | **Introduction** | XCM purpose, design philosophy, versioning |
| 2 | **Primitives** | MultiLocation, MultiAsset, Junction, instruction set |
| 2.5 | **Cross-Consensus Transfers** | Reserve transfers, teleports, asset bridging |
| 3 | **Executor** | Instruction execution, holding register, error handling |
| 4 | **Pallet XCM** | Runtime integration, XCM pallet configuration |
| 5 | **XCM Configuration** | Barrier, weigher, trader, asset transactor setup |
| 6 | **XCM Emulator** | Testing framework, mock networks, integration testing |

**Additional Resource:** Zombienet slides for local multi-chain testing

---

## Module 3b: dApps Off-Chain

**Focus:** Building decentralized applications

### Learning Objectives
- Connect to Polkadot chains from applications
- Use Polkadot API (PAPI)
- Handle real-time blockchain data
- Develop with ink! smart contracts

### Lectures (9 total)

| # | Topic | Description |
|---|-------|-------------|
| 1 | **Context** | dApp development landscape, Web3 architecture patterns |
| 2 | **PAPI Starter** | Getting started with Polkadot API, setup and basics |
| 3 | **Metadata** | Client-side metadata handling, dynamic type generation |
| 4 | **JSON-RPC Spec** | RPC specification details, method categories |
| 5 | **Transactions** | Transaction construction, signing, submission |
| 6 | **Real-time Data** | Subscriptions, event handling, state queries |
| 7 | **Polkadot API** | Advanced API usage, batch calls, utilities |
| 8 | **PAPI SDKs** | SDK ecosystem, language bindings, tooling |
| 9 | **ink!** | Smart contract development with ink!, Rust-based contracts |

---

## Module 4: PVM & Smart Contracts

**Focus:** Smart contract development on Polkadot

### Learning Objectives
- Understand smart contract fundamentals
- Learn EVM compatibility concepts
- Use the Revive stack for Solidity on Polkadot
- Participate in hands-on hackathon

### Lectures (4 total)

| # | Topic | Description |
|---|-------|-------------|
| 2 | **Smart Contracts 101** | Fundamentals, state transition, contract execution models |
| 6.2 | **Pallet Revive Runtime** | EVM-compatible execution environment, Solidity compilation |
| 11 | **Inkubator Presentation** | ink! incubator program, ecosystem support |
| 8 | **Hackathon** | Hands-on development challenge, team projects |

### Prerequisites
- Revive compiler installation
- Solidity compiler (for EVM content)

---

## Module 5: Governance

**Duration:** 2.5-3 days
**Focus:** Blockchain governance design and implementation

### Learning Objectives
- **Governance Literacy:** Comprehensive understanding of governance frameworks
- **Regulatory Awareness:** Navigate political, regulatory, and legal landscapes
- **Governance Design:** Design and implement effective governance systems
- **Stakeholder Engagement:** Build inclusive, collaborative communities
- **Governance Experimentation:** Innovate within blockchain governance
- **Talent Pipeline:** Develop decision-making and ethical consideration skills

### Schedule

#### Day 1: Foundation Module

**Morning**
1. Kick-Off / Sharing Circle (1 hour)
   - Introduction to module and faculty
   - Student sharing experiences with governance

2. Blockchain Governance in Theory (3 hours)
   - Governance in general
   - Governance in relation to blockchain
   - Governance principles, mechanisms, and trade-offs

**Afternoon**
3. Blockchain Governance in Practice (3 hours)
   - Governance Cookbook: Parameters of governance
   - Governance Lifecycle: Process design of governance innovation
   - Strategy: Progressive Decentralization & Exit to Community

4. Introduction to OpenGov (1 hour)
   - OpenGov as use case for governance concepts
   - Discussion and exploration

5. Take-home Assignment
   - Exercise on governance trade-offs through case study

#### Day 2: Challenge Module

- **Kick Off & Assignment Review** (1 hour): Share insights, refresh concepts
- **Governance Design Canvas Workshop** (5-6 hours):
  - Ten groups (50/50 founders & developers)
  - Work on specific governance challenges from Polkadot ecosystem
  - Address thematic questions using design canvas framework
- **Expert Feedback Session** (2 hours):
  - Faculty and mentors from challenge-providing projects
  - Each expert gives feedback to 2-3 projects

#### Day 3: Presentations

- **Group Project Pitch** (2 hours): Each group presents (10 min each)
- **Cross-Learning Session** (1 hour): Synthesize practical takeaways
- **Governance Outlook** (1 hour): Future trends and opportunities

### Hands-on Activities
- Challenge-based prototyping
- Governance challenges from Polkadot ecosystem projects
- Interactive workshops with ecosystem mentors

---

## Module 6: JAM (Join-Accumulate Machine)

**Focus:** Next-generation Polkadot architecture

### Learning Objectives
- Understand the World Computer vision
- Learn JAM architecture and services
- Develop JAM services
- Use CoreVM for generic computation

### Lectures (7 total)

| # | Topic | Description |
|---|-------|-------------|
| 12 | **The World Computer** | Web3 vision, ubiquitous computing, JAM as evolution of Polkadot |
| 13 | **Demystifying JAM** | Core concepts explained, architecture overview |
| 14 | **JAM Math to Code** | Mathematical foundations, formal specifications, implementation |
| 15 | **How to Start with JAM** | Getting started guide, development environment |
| 16 | **JAM Services** | Service development, refine/accumulate model |
| 17 | **JAM CoreVM** | Generic computation layer, PVM (Polkadot Virtual Machine) |
| 18 | **How to JAM** | Practical development guide, building services |

### Key JAM Concepts

| Concept | Description |
|---------|-------------|
| **Services** | Similar to smart contracts but with refine/accumulate execution model |
| **CoreVM** | Docker-like generic computation environment |
| **CoreChains** | Parachain compatibility layer for migration |
| **PVM** | Polkadot Virtual Machine (replacing Wasm) |
| **Safrole** | New block production mechanism |

### Building Blocks of Blockchains (Review)
- Cryptography
- Peer-to-Peer Networking
- Merklized Database
- Data Availability
- Byzantine-Fault-Tolerant Consensus
- Economic Game Theory

### Scaling Concepts Covered
- Shared Security
- Programmability Layers (Smart Contracts, Modular Runtime)
- Execution Sharding
- Data Sharding
- Blockchain Rollups
- On-Chain Governance

---

## Archived Content

The `previous_syllabus/` directory contains **50 additional lecture slides** from earlier cohorts:

### Security Module (5 lectures)
| Topic | Description |
|-------|-------------|
| Security Overview | Introduction to Web3 security landscape |
| Web3 User-Centric Security | User security considerations |
| Security Awareness | General security best practices |
| Application Security | dApp security patterns |
| Infrastructure Security | Node and deployment security |

### Formal Methods (1 lecture)
| Topic | Description |
|-------|-------------|
| Introduction to Formal Methods | Formal verification basics |

### Cryptography (2 lectures)
| Topic | Description |
|-------|-------------|
| Cryptography Introduction | Fundamentals of cryptographic primitives |
| ZK Proofs (Advanced) | Zero-knowledge proof systems |

### Economics (1 lecture)
| Topic | Description |
|-------|-------------|
| Economics & Game Theory Intro | Economic principles for blockchain |

### Blockchain Fundamentals (11 lectures)
| Topic | Description |
|-------|-------------|
| Blockchain Summary | Comprehensive blockchain overview |
| Distributed Key Generation | DKG protocols |
| Coordination & Centralization | Decentralization concepts |
| P2P Networks | Peer-to-peer networking |
| State Machines | State transition fundamentals |
| Blockchain Structure | Block and chain architecture |
| Accounts and UTXOs | Account models comparison |
| Consensus Authoring | Block production mechanisms |
| Economics & Game Theory in Blockchain | Applied game theory |
| Forks | Fork types and handling |
| Consensus Finality | Finality mechanisms |
| Light Clients and Bridges | Cross-chain communication |

### Legacy Polkadot Content (30 lectures)
- Introduction to Polkadot (original)
- Shared Security deep dives
- Data Availability and Sharding
- Execution Sharding and Security
- XCMP (Cross-Chain Message Passing)
- Blockspace concepts
- Asynchronous Backing (shallow and deep dives)
- Cumulus Deep Dive
- Build Simple Parachain
- Blockchain Scaling (Parts 1 & 2)
- Availability Cores
- Fellowship structure
- Introduction to dApps
- NPoS details
- Zombienet
- OpenGov
- Polkadot Ecosystem Economy
- Polkadot Decisions
- Light Clients and Unstoppable Apps
- Contributing to Polkadot

---

## Resources & Links

### Official Resources
- [Polkadot Blockchain Academy Website](https://polkadot.academy/)
- [Twitter/X: @AcademyPolkadot](https://x.com/AcademyPolkadot)
- [YouTube Channel](https://www.youtube.com/@PolkadotBlockchainAcademyTV)

### Video Lecture Playlists

| Module | YouTube Playlist |
|--------|------------------|
| Cryptography | [Watch](https://www.youtube.com/playlist?list=PL-w_i5kwVqblgwQfXbR-n8pC1QE1IaxBZ) |
| Economics | [Watch](https://www.youtube.com/playlist?list=PL-w_i5kwVqbn6GDAHq4uzhpDLPY1Ay8zO) |
| Blockchain | [Watch](https://www.youtube.com/playlist?list=PL-w_i5kwVqbmZ2p5b2Ml-Q410J_vy-pXR) |
| Smart Contracts | [Watch](https://www.youtube.com/playlist?list=PL-w_i5kwVqblMeWrQCAlWBpt-8wSmLRtc) |
| Substrate | [Watch](https://www.youtube.com/playlist?list=PL-w_i5kwVqbkRmfDn5nzeuU1S_FFW8dDg) |
| FRAME | [Watch](https://www.youtube.com/playlist?list=PL-w_i5kwVqbni1Ch2j_RwTIXiB-bwnYqq) |
| Polkadot | [Watch](https://www.youtube.com/playlist?list=PL-w_i5kwVqbn3HCZDHg6P5UCx8P2tF6-M) |
| XCM | [Watch](https://www.youtube.com/playlist?list=PL-w_i5kwVqbmHkxS_mJTfXdCZnXKF5lDP) |

### Cohort Snapshots

| Cohort | Repository Branch |
|--------|-------------------|
| Cambridge 2022 | [cambridge-2022](https://github.com/Polkadot-Blockchain-Academy/pba-content/tree/cambridge-2022) |
| Buenos Aires 2023 | [buenos-aires-2023](https://github.com/Polkadot-Blockchain-Academy/pba-content/tree/buenos-aires-2023) |
| Berkeley 2023 | [berkeley-2023](https://github.com/Polkadot-Blockchain-Academy/pba-content/tree/berkeley-2023) |
| Hong Kong 2024 | [hong-kong-2024](https://github.com/Polkadot-Blockchain-Academy/pba-content/tree/hong-kong-2024) |
| Singapore 2024 | [2024-singapore](https://github.com/Polkadot-Blockchain-Academy/pba-content/tree/2024-singapore) |
| Lucerne 2025 | [lucerne-2025](https://github.com/Polkadot-Blockchain-Academy/pba-content/tree/lucerne-2025) |
| Bali 2025 | [bali-2025](https://github.com/Polkadot-Blockchain-Academy/pba-content/tree/bali-2025) |

### Repository Structure

```
pba-content/
├── syllabus/                    # Current curriculum (76 lectures)
│   ├── 1-Fundamentals/          # Core concepts (9 lectures)
│   ├── 2-Polkadot/              # Polkadot protocol (12 lectures)
│   ├── 3a-Protocol_On-Chain/    # Substrate, FRAME, XCM (~32 lectures)
│   │   ├── Substrate/
│   │   ├── FRAME/
│   │   └── XCM/
│   ├── 3b-dApps_Off-Chain/      # dApp development (9 lectures)
│   ├── 4-PVM-.../               # Smart contracts (4 lectures)
│   ├── 5-Governance/            # Governance (3 days)
│   └── 6-JAM/                   # JAM protocol (7 lectures)
├── previous_syllabus/           # Archived content (50 lectures)
├── assets/                      # Shared images, PDFs, styles
├── CONTRIBUTING.md              # Contributor guidelines
└── README.md                    # Project overview
```

---

## Summary Statistics

| Category | Count |
|----------|-------|
| **Current Modules** | 6 main + 3 sub-modules |
| **Current Lectures** | 76 |
| **Archived Lectures** | 50 |
| **Total Educational Content** | 126+ lectures |
| **Program Cohorts** | 7 |
| **YouTube Playlists** | 8 |
| **Typical Program Duration** | 3+ weeks |
| **Class Size** | 25-60 students |

---

*This report provides a comprehensive overview of the Polkadot Blockchain Academy curriculum. For contribution guidelines, see [CONTRIBUTING.md](./CONTRIBUTING.md).*
