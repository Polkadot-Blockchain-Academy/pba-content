---
title: The Polkadot Stack Template
description: Introduction to the polkadot-stack-template and the course project.
duration: 30 minutes
owner: Shawn Tabrizi
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# The Polkadot Stack Template

---

## The polkadot-stack-template

A reference project demonstrating the full Polkadot stack through **Proof of Existence**: claim a file hash on-chain.

<div class="text-small">

| Component          | Path             | Tech Stack                               |
| ------------------ | ---------------- | ---------------------------------------- |
| **FRAME Pallet**   | `blockchain/`    | Rust, FRAME, Cumulus, polkadot-omni-node |
| **EVM Contract**   | `contracts/evm/` | Solidity, Hardhat, solc                  |
| **PVM Contract**   | `contracts/pvm/` | Solidity, Hardhat, resolc                |
| **React Frontend** | `web/`           | React, Vite, PAPI, viem                  |
| **Rust CLI**       | `cli/`           | Rust, subxt, alloy                       |
| **Dev Scripts**    | `scripts/`       | Zombienet, docker-compose                |

</div>

Every component is **minimal, modular, and removable**.

Use the template as an **optional** starting point, not a constraint.

---

## The End-to-End Flow

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

---

## Running the Template

Two modes for local development:

<pba-cols>
<pba-col>

#### Standalone Mode

- Single parachain node
- `polkadot-omni-node` + your Wasm runtime
- Fastest to start, good for pallet and contract dev
- No relay chain, no cross-chain features

</pba-col>
<pba-col>

#### Zombienet Mode

- Full multi-chain network
- Relay chain + parachain + system chains
- Required for **Statement Store**, XCM, and cross-chain testing
- Configured via `scripts/zombienet.toml`

</pba-col>
</pba-cols>

> Start with standalone for fast iteration. Switch to zombienet when you need the full network.

---

## The Omni-Node

**`polkadot-omni-node`** is a generic parachain node binary.

<div class="text-left">

- You don't compile a custom node binary
  - You compile only the **Wasm runtime**
- The omni-node loads your runtime at startup via the **chain spec**
- Includes everything except your business logic
  - Networking, consensus, RPC server, database
- Works with **any** FRAME-based runtime

</div>

<div class="text-small">

```bash
# Build your runtime
cargo build --release -p your-runtime

# Generate a chain spec
chain-spec-builder create -r your_runtime.wasm default

# Run it
polkadot-omni-node --chain chain-spec.json --dev
```

</div>

---

## Deploying Your Project

<div class="text-small">

| Component        | How It Deploys                                                                | Where It Lives                         |
| ---------------- | ----------------------------------------------------------------------------- | -------------------------------------- |
| **Pallet**       | Compile Wasm runtime → register parachain via Coretime                        | Your parachain on the Polkadot network |
| **EVM Contract** | `npx hardhat deploy` through eth-rpc sidecar                                  | Asset Hub (or your parachain)          |
| **PVM Contract** | `npx hardhat deploy` with `@parity/hardhat-polkadot` plugin                   | Asset Hub (or your parachain)          |
| **Frontend**     | Build static bundle → upload to Bulletin Chain → register .dot name via DotNS | IPFS, accessible via any Triangle host |

</div>

<div class="text-left">

- **Local development**: deploy to your standalone node or zombienet
- **Final project**: deploy to **Paseo** — the production network for your project

</div>

---

<!-- .slide: data-background-color="#000000" -->

# Your Project

---

## What You Will Build

Every student ships a **solo project** with:

<div class="text-left">

**One backend** (pick one):

- Substrate Pallet
- Solidity smart contract on EVM
- Solidity smart contract on PVM
- Rust smart contract on PVM

**One frontend** (pick one):

- Web app (React, Svelte, Vue, etc.)
- CLI++ (TUI or MCP server)

**Mandatory**: deploy to **Bulletin Chain + DotNS**

</div>

> Full details: [`projects/PBP-Lisbon-2026/project-guidelines.md`](../../projects/PBP-Lisbon-2026/project-guidelines.md)

---

## Path Confidence

Not all paths are equally battle-tested. Pick based on your appetite for risk.

| Backend             | Confidence | Notes                                                       |
| ------------------- | ---------- | ----------------------------------------------------------- |
| **Pallet**          | ~100%      | Well-trodden, full curriculum support                       |
| **Solidity on EVM** | ~90%       | Mature tooling, standard ecosystem                          |
| **Solidity on PVM** | ~70%       | Expect toolchain quirks                                     |
| **Rust on PVM**     | ~50%       | Bleeding edge. Real chance you hit walls nobody has hit yet |

**Getting stuck is not failure** as long as you document it well.

A good bug report or PR against the stack is a legitimate deliverable.

---

## Think of This as Onboarding

There are **no grades**. This program is your **2-week onboarding at Parity**.

<div class="text-left">

- You have Parity engineers on staff the entire time as **mentors**
- You are given real tools, real code, and the freedom to **build anything**
- We are evaluating you as if this were your **first two weeks on the job**
- At the end, your project is reviewed directly by Parity for **hiring consideration**

</div>

> The question we are asking: _"Would I want this person on my team?"_

---

## What We're Looking For

The behaviors that stand out to us:

<div class="text-left text-small">

- **Self-starters**
  - You don't wait to be told what to do
  - You explore, you try things, you ask when stuck
- **Builders with taste**
  - You pick a problem worth solving
  - You build something that takes advantage of what Web3 actually provides
- **Stack improvers**
  - You find a bug? You open an issue
  - You can fix it? You open a PR
  - This is the strongest signal you can give us
- **Collaborators**
  - You help others, share what you learn, and make the people around you better
- **Active communicators**
  - You say what works, what doesn't, and what you'd do differently

</div>

---

## Deliverables

What you hand in at the end of the program:

<div class="text-left">

- **Public GitHub repo**
  - Working code (backend + frontend)
  - Professional README: what it does, how to set up, how to use, known limitations
- **Deployed on Paseo**
  - Live and reachable at your **.dot name** via dot.li
  - Frontend hosted on **Bulletin Chain**, backend on-chain
- **5-minute pitch presentation**
  - Describe and demo your project end-to-end
  - Discuss next steps, learnings, and feedback for the stack
- **Retrospective write-up**
  - What worked, what didn't, and what you'd tell to Parity

</div>

---

## What Should You Build?

The best projects aren't "blockchain projects." They're **real products** that happen to be trustless and decentralized.

Ask yourself: _What existing app or system would be fundamentally better if no single entity controlled it?_

---

## Idea: Decentralized Everyday Apps

Rebuild the apps people already use, but trustless.

<div class="text-left">

- **Collaborative documents**
  - Google Docs, but your data lives on Bulletin Chain, not Google's servers
- **Chat and messaging**
  - Signal-like messaging where the protocol is open
  - No company can read or kill it
- **Social networks**
  - Own your identity, your followers, your content
  - Move between apps freely
- **File sharing**
  - WeTransfer, but censorship-resistant with a .dot link

</div>

---

## Idea: New Financial Systems

Build financial primitives that don't exist yet, or fix the ones that are broken.

<div class="text-left">

- **Payments, invoicing, and remittances**
  - Pay back friends, send money home, settle invoices
  - Verifiable, borderless, no middlemen
- **Ticketing and event passes**
  - Verifiable, transferable, no scalpers or platform fees
- **Investment DAOs**
  - Pool funds with friends or strangers
  - Invest collectively with transparent rules
- **Micropayments**
  - Pay-per-use models that are impractical with traditional payment rails

</div>

---

## Idea: Decentralized Protocols

Build new infrastructure that doesn't exist yet — composable building blocks the whole ecosystem can use.

<div class="text-left">

- **Reputation systems**
  - Portable, verifiable reputation that follows you across apps
- **Coordination protocols**
  - Scheduling, voting, dispute resolution without a central authority
- **Data layers**
  - Indexing, curation, or discovery on top of Bulletin Chain and on-chain state
- **Tooling for developers**
  - Anything that makes the next person's job easier

</div>

---

## Idea: Onboarding and Adoption

Build systems that bring new users into Web3 — make the first experience seamless and rewarding.

<div class="text-left">

- **Games**
  - The most intuitive way to onboard casual users
  - They don't even realize it's Web3
- **Incentive systems**
  - Faucets, airdrops, reward programs
  - Give people a reason to show up
- **Free transactions**
  - Remove the fee barrier entirely for new users
- **Simple identity**
  - One-click wallet creation, social login bridges
  - Progressive decentralization

</div>

---

## Advice for Success

<div class="text-left">

- **Explore fearlessly.** Clone repos, read source code, run examples. Don't just read docs.
- **Ask early, not late.** Faculty are here as mentors. "I'm stuck after 2 hours" is a good conversation. "I'm stuck after 2 days" is a missed opportunity.
- **Build something only Web3 can do.** If your project works just as well on a centralized server, it's not the most interesting version of itself.
- **Contribute upstream.** An issue, a bug report, a small PR to polkadot-sdk or any tool you use — this is the strongest signal of all.
- **Ship honest work.** A smaller project that works and is well-documented beats a big project that's half-broken and unexplained.

</div>

---

## Key Resources

<div class="text-left">

- **Stack Template**
  - [github.com/shawntabrizi/polkadot-stack-template](https://github.com/shawntabrizi/polkadot-stack-template)
  - Fork this to start your project
- **Project Guidelines**
  - [`projects/PBP-Lisbon-2026/project-guidelines.md`](../../projects/PBP-Lisbon-2026/project-guidelines.md)
  - Requirements, evaluation, deliverables
- **Git Repositories**
  - [github.com/orgs/paritytech/teams/pba-lisbon-2026/repositories](https://github.com/orgs/paritytech/teams/pba-lisbon-2026/repositories)
  - Access to private repos for the program
  - Also check out the public repos at [github.com/paritytech](https://github.com/paritytech)

</div>

---

<!-- .slide: data-background-color="#000000" -->

# Questions
