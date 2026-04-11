# PBP Project Idea Bank

A living list of project ideas for Protocol Builders Program students.

**This document is open to contributions from the Parity product team, faculty, and instructors.** If you have an idea that would make a great student project, add it here via PR.

Students: you are free to pick an idea from this list, remix one, or invent your own. These are suggestions, not constraints.

---

## What Makes a Good Project?

The best projects aren't "blockchain projects." They're **real products** that happen to be trustless and decentralized.

Ask yourself: _What existing app or system would be fundamentally better if no single entity controlled it?_

If your project would work just as well on a centralized server, it's probably not the most interesting version of itself.

---

## 1. Decentralized Everyday Apps

> Rebuild the apps people already use, but trustless. The scope is well understood, the user need is obvious, and the design space is rich.

Think about the tools people and professionals rely on every day — and what breaks when a single company controls them:

- **Productivity & collaboration** — documents, spreadsheets, wikis, project management
- **Communication** — messaging, email, video calls, forums
- **Social & media** — social networks, content platforms, streaming
- **Storage & sharing** — file hosting, backups, transfer services
- **Identity & access** — logins, credentials, permissions

Pick one. Make it trustless. Ship a version that works.

**Concrete ideas:**

### 1.1 Versioned markdown wiki with on-chain ACL

A markdown editor / wiki where users explicitly save snapshots, stored as Bulletin Chain blobs and indexed on-chain via a pallet or contract that holds the ACL and the snapshot history. Permissioned editing, linear versioning, diff-between-snapshots view.

- **Paths**: Pallet + Web; Solidity-EVM + Web.
- **Why**: A clean, recognizable product that exercises the full stack.

### 1.2 Ephemeral file drop ("dead drop" / time-limited share)

A WeTransfer-style ephemeral share: upload a file (up to Bulletin Chain's per-blob cap), get back a link backed by a DotNS name and a content hash, recipient downloads within the retention window, file disappears. Optional access control via an on-chain ACL pallet.

- **Paths**: Pallet + Web; Pallet + CLI++ (TUI or MCP) + DotNS landing page.
- **Why**: Honest fit for what Bulletin Chain actually is — a content-addressed blob layer with a ~2-week retention window, not persistent storage.

---

## 2. New Financial Systems

> Build financial primitives that don't exist yet, or fix the ones that are broken. Verifiable, borderless, no middlemen.

**High-level directions:**

- **Payments, invoicing, and remittances** — Pay back friends, send money home, settle invoices
- **Ticketing and event passes** — Verifiable, transferable, no scalpers or platform fees
- **Investment DAOs** — Pool funds with friends or strangers, invest collectively with transparent rules
- **Micropayments** — Pay-per-use models that are impractical with traditional payment rails

**Concrete ideas:**

### 2.1 On-chain payments / invoicing

A simple invoicing + payments app. Someone creates an invoice, another account pays it, both sides get verifiable records.

- **Paths**: Pallet + Web; Solidity-EVM + Web; Solidity-PVM + Web.
- **Why**: Payments are the bread-and-butter use case for the stack. A clean, minimal version is useful reference work.

### 2.2 Ticketing / event passes

Issue tickets as on-chain assets. Transferable, verifiable, optionally revocable. Gate access via signature check.

- **Paths**: Pallet + Web; Solidity-EVM + Web; Pallet + TUI (for gate-checker terminals).
- **Why**: Real demand, clean scope, many directions to take it (loyalty, membership, etc.).

### 2.3 Multi-Token Treasury

A pallet managing a multi-asset treasury with governance-gated spending tracks, sensible handling of insufficient balances and existential deposits across assets.

- **Paths**: Pallet + Web (treasury dashboard / governance UI); Pallet + TUI.
- **Why**: Treasury mechanics are a real Polkadot need, and this exercises governance APIs and asset handling at the same time.
- **Scope note**: Don't try to build a price oracle or DOT↔USD conversion in MVP; pick fixed token denominations and one or two spending tracks.

---

## 3. Decentralized Protocols

> Build new infrastructure that doesn't exist yet — composable building blocks the whole ecosystem can use.

**High-level directions:**

- **Reputation systems** — Portable, verifiable reputation that follows you across apps
- **Coordination protocols** — Scheduling, voting, dispute resolution without a central authority
- **Data layers** — Indexing, curation, or discovery on top of Bulletin Chain and on-chain state
- **Tooling for developers** — Anything that makes the next person's job easier

**Concrete ideas:**

### 3.1 Direct Delegation Proof of Stake

A pallet that manages validators (self-registered candidates) and delegators (any account staking tokens behind a validator). Every N blocks, top-K winners become the active set. Block rewards split between the producer and their backers.

- **Paths**: Pallet + Web (staking dashboard); Pallet + TUI.
- **Why**: Staking is the canonical Substrate exercise. A clean implementation with a real frontend is a strong portfolio piece.
- **Scope note**: Cut slashing and delegation chains from the MVP. Get the happy path + one good benchmark before reaching for either. Frontend should at minimum let a user delegate, undelegate, and see the active set.

### 3.2 Stateful Multisig

A pallet that lets users create and manage multisigs with a unique on-chain address per multisig, and run the full transaction lifecycle (propose → vote → execute), plus member add/remove and a clean teardown path. North star: the original Gnosis Safe.

- **Paths**: Pallet + Web (multisig management UI); Pallet + TUI.
- **Why**: A real product with a real audience. Stateful multisigs are something Polkadot itself wants, and the UX surface is deep.
- **Scope note**: Frontend matters here — a multisig with a bad UI is unusable. Pick this one if you actually enjoy frontend work too. Don't try to migrate from Polkadot's stateless multisig as part of the MVP; that's a stretch goal at best.

### 3.3 On-chain tournament / leaderboard layer

Not a game itself — a backend that games can plug into for verifiable leaderboards, tournament brackets, prize pools.

- **Paths**: Pallet + Web; Solidity-EVM + Web.
- **Why**: Infra for games is a useful primitive and composable with anything else in this list. It also promotes interoperability.

---

## 4. Onboarding and Adoption

> Build systems that bring new users into Web3 — make the first experience seamless and rewarding.

**High-level directions:**

- **Games** — The most intuitive way to onboard casual users; they don't even realize it's Web3
- **Incentive systems** — Faucets, airdrops, reward programs; give people a reason to show up
- **Free transactions** — Remove the fee barrier entirely for new users
- **Simple identity** — One-click wallet creation, social login bridges, progressive decentralization

**Concrete ideas:**

### 4.1 Fully on-chain turn-based game

Chess, Go, a card game, a simple strategy game — all state on-chain, turns enforced by the backend, with a web or TUI front.

- **Paths**: Pallet + Web; Pallet + TUI; Solidity-EVM + Web. Be careful about the complexity of the game; it could be hard to port to EVM if it's too much.
- **Why**: Games are the clearest possible "this stack is for real applications" demo.

### 4.2 Free Transaction Pallet

A pallet that lets users lock tokens to earn "weight credits" and spend them on fee-free transactions within a rolling time window, with a global per-period cap to prevent spam.

- **Paths**: Pallet + Web (a demo app that exercises the free-tx path — e.g. a faucet, mint flow, or free-claim feature); Pallet + CLI++.
- **Why**: A useful primitive on its own. The interesting design space is the rate-limit and fallback semantics.
- **Scope note**: This is the most "systems primitive" of the ideas — the frontend story is **not** "a UI for free transactions," it's "an actual app that uses the pallet to give users free transactions." Pick the demo app with care; reviewers should walk away believing the primitive is useful.

---

## 5. Wildcards

Your idea doesn't have to fit a category above. The best projects in programs like this are often genuinely weird things a student cared about or is passionate about. If you have one, bring it to a faculty conversation early and we'll help you shape it.

**Protocol ports** are also welcome — take something interesting from another chain (encrypted chat, decentralized file hosting, payment channels, etc.) and bring it to Polkadot. You learn the protocol, we get a port, and the ecosystem gets wider.
