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

| Component | Path | Tech Stack |
|-----------|------|------------|
| **FRAME Pallet** | `blockchain/` | Rust, FRAME, Cumulus, polkadot-omni-node |
| **EVM Contract** | `contracts/evm/` | Solidity, Hardhat, solc |
| **PVM Contract** | `contracts/pvm/` | Solidity, Hardhat, resolc |
| **React Frontend** | `web/` | React, Vite, PAPI, viem |
| **Rust CLI** | `cli/` | Rust, subxt, alloy |
| **Dev Scripts** | `scripts/` | Zombienet, docker-compose |

</div>

Every component is **minimal, modular, and removable**.

Use the template as an **optional** starting point, not a constraint.

Notes:

The polkadot-stack-template is a full-stack reference application. It implements one simple idea, Proof of Existence, three different ways: as a FRAME pallet, an EVM smart contract, and a PVM smart contract. It also includes a React frontend and a Rust CLI that interact with all three. The idea is to show you every layer of the Polkadot developer experience in one place. You'll use this as the foundation for your own project.

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

Notes:

Here's the end-to-end flow the template demonstrates. Backend logic deploys to a parachain or Asset Hub. The frontend uploads to IPFS via the Bulletin Chain and gets a .dot domain. A user opens a Triangle host, types the .dot name, and the host resolves it, fetches the frontend from IPFS, and renders it in a sandbox. No centralized servers anywhere in the chain. Your project will follow this same pattern.

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

Notes:

There are two ways to run the template locally. Standalone mode uses polkadot-omni-node, a generic parachain binary that can load any Wasm runtime blob. It boots a single chain in seconds, perfect for pallet development and contract testing. Zombienet mode spins up a full network: relay chain validators, your parachain as a collator, and system chains. You need zombienet for anything that involves the Statement Store, cross-chain messaging, or testing how your chain behaves in the real Polkadot topology. The template includes config files for both.

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

Notes:

The omni-node is a key piece of the modern Polkadot developer experience. In the past, you had to compile a full custom node binary for every chain. Now, you only compile your runtime to WASM, and the omni-node provides everything else. This dramatically speeds up build times and simplifies deployment. The chain-spec-builder tool generates a chain specification from your runtime, and you pass that to the omni-node. The template is already set up to work this way.

---

## Deploying Your Project

<div class="text-small">

| Component | How It Deploys | Where It Lives |
|-----------|---------------|----------------|
| **Pallet** | Compile Wasm runtime → register parachain via Coretime | Your parachain on the Polkadot network |
| **EVM Contract** | `npx hardhat deploy` through eth-rpc sidecar | Asset Hub (or your parachain) |
| **PVM Contract** | `npx hardhat deploy` with `@parity/hardhat-polkadot` plugin | Asset Hub (or your parachain) |
| **Frontend** | Build static bundle → upload to Bulletin Chain → register .dot name via DotNS | IPFS, accessible via any Triangle host |

</div>

<div class="text-left">

- **Local development**: deploy to your standalone node or zombienet
- **Final project**: deploy to **Paseo** — the production network for your project

</div>

Notes:

Here's the deployment story for each component. Pallets are compiled to WASM and deployed as part of a parachain runtime, you acquire coretime to run your chain. Smart contracts deploy through Hardhat, using the eth-rpc sidecar, same workflow as Ethereum. The frontend builds to a static bundle that gets uploaded to the Bulletin Chain, which makes it available on IPFS. Then you register a .dot name pointing to the IPFS content hash, and your app is live on the decentralized web. During development you'll work locally. Your final deliverable deploys to Paseo with a real .dot domain — that's where reviewers will see your work running.

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

Notes:

Your project has hard requirements. You pick one backend path and one frontend path, and every project must be deployed to the Bulletin Chain with a DotNS name. The stack template gives you a working example of each path. You'll fork it, strip the parts you don't need, and build your own idea on top. Read the full project guidelines document for all the details on evaluation, deliverables, and timeline.

---

## Path Confidence

Not all paths are equally battle-tested. Pick based on your appetite for risk.

| Backend | Confidence | Notes |
|---------|-----------|-------|
| **Pallet** | ~100% | Well-trodden, full curriculum support |
| **Solidity on EVM** | ~90% | Mature tooling, standard ecosystem |
| **Solidity on PVM** | ~70% | Expect toolchain quirks |
| **Rust on PVM** | ~50% | Bleeding edge. Real chance you hit walls nobody has hit yet |

**Getting stuck is not failure** as long as you document it well.

A good bug report or PR against the stack is a legitimate deliverable.

Notes:

This is an honest assessment of the stack maturity. The pallet path is rock solid. EVM contracts are almost there. PVM paths are newer and rougher. Riskier paths have a higher ceiling though. If you ship a working Rust-on-PVM project, that's a strong signal. And if you can't ship but you file precise bug reports or PRs, that's valued too. You're helping us discover the shape of the stack.

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

Notes:

This is the most important framing for the entire program. We're not grading you on an exam. We're watching how you work. How you onboard into a new codebase, how you handle getting stuck, how you collaborate, how you communicate. The Parity engineers here aren't just lecturers, they're your mentors and potential future colleagues. Treat this like your first two weeks at a new job, because that's exactly how we'll evaluate it.

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

Notes:

These are the things that get people hired. Not just shipping a polished demo, but showing how you work. Did you file a bug report when something broke? Did you help a classmate who was stuck? Did you dig into the source code when the docs weren't enough? Did you build something that actually leverages decentralization, not just a web app with a blockchain bolted on? These are the signals that tell us you'd be a great Parity engineer.

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

Notes:

These are the four things you hand in. A public GitHub repo with working code and an honest README. A live deployment on Paseo, reachable through your .dot name on dot.li, with the frontend hosted on Bulletin Chain. A 5-minute pitch on the final day where you demo the project and talk honestly about what broke. And a retrospective where you tell us what you'd change about the stack based on what you experienced. Reviewers read the retrospective carefully. It's not busywork. It's one of the most valuable things you produce.

---

## What Should You Build?

The best projects aren't "blockchain projects." They're **real products** that happen to be trustless and decentralized.

Ask yourself: _What existing app or system would be fundamentally better if no single entity controlled it?_

Notes:

This is the key question. Don't start with "what can I build on a blockchain." Start with "what product would I want to exist that only Web3 can deliver." The blockchain is invisible to the user. What they see is a product that works, that they can trust, that nobody can shut down.

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

Notes:

People already use Google Docs, WhatsApp, Twitter, WeTransfer every day. The demand is proven. The problem is that one company controls all of it, your data, your access, your identity. A decentralized version doesn't need to be better at everything, it just needs to provide the properties that centralized versions fundamentally cannot: censorship resistance, data ownership, and no single point of failure.

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

Notes:

DeFi proved that on-chain finance works. But most of what exists is trading infrastructure for crypto-native users. The opportunity is in financial systems that serve everyone: invoicing for freelancers, transparent treasuries for organizations, micropayment models that let creators get paid directly. These are products with real demand that traditional finance serves poorly.

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

Notes:

Some of the highest-impact projects aren't end-user apps at all. They're protocols and tools that make everything else better. A reputation layer, a coordination protocol, a data indexing service — these are building blocks that other developers can compose with. If your project becomes something other people want to build on top of, that's an incredibly strong signal. And developer tooling counts too — if you hit a gap in the stack and build a tool to fill it, that's real contribution.

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

Notes:

The hardest problem in Web3 isn't building the technology. It's getting people to use it. The first experience matters enormously. If a user has to understand gas fees, seed phrases, and wallet extensions before they can do anything, you've already lost them. Games are powerful because they provide intrinsic motivation — people play because it's fun, and the Web3 part is invisible. Incentive systems give people a reason to try. Free transactions remove the biggest friction point. And simple identity flows mean people can start using your app in seconds, not minutes.

---

## Advice for Success

<div class="text-left">

- **Explore fearlessly.** Clone repos, read source code, run examples. Don't just read docs.
- **Ask early, not late.** Faculty are here as mentors. "I'm stuck after 2 hours" is a good conversation. "I'm stuck after 2 days" is a missed opportunity.
- **Build something only Web3 can do.** If your project works just as well on a centralized server, it's not the most interesting version of itself.
- **Contribute upstream.** An issue, a bug report, a small PR to polkadot-sdk or any tool you use — this is the strongest signal of all.
- **Ship honest work.** A smaller project that works and is well-documented beats a big project that's half-broken and unexplained.

</div>

Notes:

This is practical advice from people who have hired engineers this way. Explore the codebase like it's your first week at a company. Don't be afraid to dig into the Polkadot SDK source when something doesn't make sense. Ask the engineers here questions, that's what they're here for. And think about what makes your project uniquely Web3. Decentralized storage, trustless verification, censorship resistance, these are the properties that matter. If you can also improve the tools you're using along the way, that's the gold standard.

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

Notes:

Here are the key resources. Start with the stack template and the project guidelines. The idea bank has project suggestions if you need inspiration, but you're free to invent your own. Use office hours with faculty, especially if you're on a riskier path.

---

<!-- .slide: data-background-color="#000000" -->

# Questions
