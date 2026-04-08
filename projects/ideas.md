# PBP Project Idea Bank

A living list of project ideas for Protocol Builders Program students.

**This document is open to contributions from the Parity product team, faculty, and instructors.** If you have an idea that would make a great student project add it here via PR.

Students: you are free to pick an idea from this list, remix one, or invent your own. These are starting points, not a menu. The best projects usually come from a student taking an idea here and twisting it to fit something they personally care about.

Each idea includes:
- **What it is** - one paragraph.
- **Suggested paths** - which backend + frontend combinations fit.
- **Difficulty** - rough signal of how ambitious this is.
- **Why it matters** - why Parity (or the ecosystem) would care about this existing.

---

## 1. Practical Apps

> Recognizable, real-world applications. The scope is well understood, the user need is obvious, and the design space is rich. Great for showing you can take a familiar product and ship a clean version of it on a new stack.

### 1.1 Decentralized HackMD / collaborative markdown
A collaborative, real-time markdown editor where document state lives on-chain (or on Bulletin Chain), with permissioned editing and versioning.
- **Paths**: Pallet + Web; Solidity-EVM + Web.
- **Difficulty**: Medium.
- **Why**: Collaborative editing is a recognizable, hard-but-tractable problem. A clean decentralized version is a memorable demo.

### 1.2 On-chain payments / invoicing
A simple invoicing + payments app. Someone creates an invoice, another account pays it, both sides get verifiable records.
- **Paths**: Pallet + Web; Solidity-EVM + Web; Solidity-PVM + Web.
- **Difficulty**: Easy–Medium.
- **Why**: Payments are the bread-and-butter use case for the stack. A clean, minimal version is useful reference work.

### 1.3 Ticketing / event passes
Issue tickets as on-chain assets. Transferable, verifiable, optionally revocable. Gate access via signature check.
- **Paths**: Pallet + Web; Solidity-EVM + Web; Pallet + TUI (for gate-checker terminals).
- **Difficulty**: Easy–Medium.
- **Why**: Real demand, clean scope, many directions to take it (loyalty, membership, etc.).

---

## 2. Protocol Ports From Other Chains

> Take something interesting from another ecosystem and bring it to Polkadot. You learn the protocol, we get a port, and the ecosystem gets wider.

### 2.1 x402 payments
HTTP-native micropayments using the x402 spec, but backed by Polkadot accounts and an on-chain / Bulletin-Chain settlement layer.
- **Paths**: Pallet or Solidity-EVM backend + Web or MCP frontend.
- **Difficulty**: Medium.
- **Why**: x402 is getting real traction. A Polkadot-native implementation is a cool standalone thing and a useful primitive.

### 2.2 Anonymous / encrypted chat
End-to-end encrypted chat where metadata (ciphertext, recipient hints) lives on Bulletin Chain / Statement Store.
- **Paths**: Pallet + Web; Pallet + TUI.
- **Difficulty**: Medium–Hard (crypto done wrong is worse than no crypto - be careful and ask for review).
- **Why**: Shows what decentralized storage can do, and it's genuinely useful.

### 2.3 Decentralized file hosting
Upload, pin, retrieve, share files through Bulletin Chain, with access control via on-chain logic.
- **Paths**: Pallet + Web; Pallet + CLI++ (TUI or MCP) + DotNS landing page.
- **Difficulty**: Medium.
- **Why**: File hosting is a something that the web3 ecosystem craves and the stack has real building blocks for it.

---

## 3. Games

> A well-built on-chain game is one of the more memorable things you can ship.

### 3.1 Fully on-chain turn-based game
Chess, Go, a card game, a simple strategy game - all state on-chain, turns enforced by the backend, with a web or TUI front.
- **Paths**: Pallet + Web; Pallet + TUI; Solidity-EVM + Web.
- **Difficulty**: Medium.
- **Why**: Games are the clearest possible "this stack is for real applications" demo.

### 3.2 On-chain tournament / leaderboard layer
Not a game itself - a backend that games can plug into for verifiable leaderboards, tournament brackets, prize pools.
- **Paths**: Pallet + Web; Solidity-EVM + Web.
- **Difficulty**: Medium.
- **Why**: Infra for games is a useful primitive and composable with anything else in this list.

---

## 4. Infra & Tooling

> Less glamorous but genuinely useful. Good tooling work has a long tail of impact.

### 4.1 MCP server for a Polkadot chain
An MCP server that lets an AI agent (Claude, etc.) query and interact with a Polkadot chain - read state, submit extrinsics, inspect accounts. Your "frontend" is the MCP interface; the "backend" is a small supporting pallet or contract.
- **Paths**: Pallet + MCP; Solidity-EVM + MCP.
- **Difficulty**: Medium.
- **Why**: MCP is hot, and making the Polkadot stack legible to AI agents is a real unlock for the ecosystem.

### 4.2 DotNS-based deploy tool
A tool that publishes a built web app to Bulletin Chain and registers it under a DotNS name in one command. Your backend is a supporting pallet or contract that tracks deployments. To clear the CLI++ bar, build it as a TUI with live deploy progress, or expose it as an MCP server so an AI agent can deploy on your behalf.
- **Paths**: Pallet + CLI++ (TUI or MCP).
- **Difficulty**: Medium.
- **Why**: The stack needs frictionless deploy workflows. If yours is good, people will actually use it.

### 4.3 Local-first dev dashboard
A TUI that gives you a live view of a running local node: blocks, extrinsics, logs, storage, events. Your backend is a pallet or contract you then exercise from the TUI to prove it works.
- **Paths**: Pallet + TUI.
- **Difficulty**: Medium.
- **Why**: Good DX tooling is always undersupplied and always appreciated.

---

## 5. Wildcards

Your idea doesn't have to fit a category above. Some of the best projects in programs like this are genuinely weird things a student cared about. If you have one, bring it to a faculty conversation early and we'll help you shape it.

Things we'd love to see and would rate highly: anything that *only works* because of what's unique about the Polkadot stack. If your project would work just as well on Ethereum mainnet, it is probably not the most interesting version of itself.
