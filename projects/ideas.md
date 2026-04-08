# PBP Project Idea Bank

A living list of project ideas for Protocol Builders Program students.

**This document is open to contributions from the Parity product team, faculty, and instructors.** If you have an idea that would make a great student project add it here via PR.

Students: you are free to pick an idea from this list, remix one, or invent your own. These are starting points, not a menu. The best projects usually come from a student taking an idea here and twisting it to fit something they personally care about.

Each idea includes:
- **What it is** - one paragraph.
- **Suggested paths** - which backend + frontend combinations fit.
- **Why it matters** - why Parity (or the ecosystem) would care about this existing.

---

## 1. Practical Apps

> Recognizable, real-world applications. The scope is well understood, the user need is obvious, and the design space is rich. Great for showing you can take a familiar product and ship a clean version of it on a new stack.

### 1.1 Versioned markdown wiki with on-chain ACL
A markdown editor / wiki where users explicitly save snapshots, stored as Bulletin Chain blobs and indexed on-chain via a pallet or contract that holds the ACL and the snapshot history. Permissioned editing, linear versioning, diff-between-snapshots view.
- **Paths**: Pallet + Web; Solidity-EVM + Web.
- **Why**: A clean, recognizable product that exercises the full stack.

### 1.2 On-chain payments / invoicing
A simple invoicing + payments app. Someone creates an invoice, another account pays it, both sides get verifiable records.
- **Paths**: Pallet + Web; Solidity-EVM + Web; Solidity-PVM + Web.
- **Why**: Payments are the bread-and-butter use case for the stack. A clean, minimal version is useful reference work.

### 1.3 Ticketing / event passes
Issue tickets as on-chain assets. Transferable, verifiable, optionally revocable. Gate access via signature check.
- **Paths**: Pallet + Web; Solidity-EVM + Web; Pallet + TUI (for gate-checker terminals).
- **Why**: Real demand, clean scope, many directions to take it (loyalty, membership, etc.).

---

## 2. Protocol Ports From Other Chains

> Take something interesting from another ecosystem and bring it to Polkadot. You learn the protocol, we get a port, and the ecosystem gets wider.


### 2.3 Ephemeral file drop ("dead drop" / time-limited share)
A WeTransfer-style ephemeral share: upload a file (up to Bulletin Chain's per-blob cap), get back a link backed by a DotNS name and a content hash, recipient downloads within the retention window, file disappears. Optional access control via an on-chain ACL pallet.
- **Paths**: Pallet + Web; Pallet + CLI++ (TUI or MCP) + DotNS landing page.
- **Why**: Honest fit for what Bulletin Chain actually is - a content-addressed blob layer with a ~2-week retention window, not persistent storage.

---

## 3. Games

> A well-built on-chain game is one of the more memorable things you can ship.

### 3.1 Fully on-chain turn-based game
Chess, Go, a card game, a simple strategy game - all state on-chain, turns enforced by the backend, with a web or TUI front.
- **Paths**: Pallet + Web; Pallet + TUI; Solidity-EVM + Web.
- **Why**: Games are the clearest possible "this stack is for real applications" demo.

### 3.2 On-chain tournament / leaderboard layer
Not a game itself - a backend that games can plug into for verifiable leaderboards, tournament brackets, prize pools.
- **Paths**: Pallet + Web; Solidity-EVM + Web.
- **Why**: Infra for games is a useful primitive and composable with anything else in this list.

---

## 4. Wildcards

Your idea doesn't have to fit a category above. Some of the best projects in programs like this are genuinely weird things a student cared about. If you have one, bring it to a faculty conversation early and we'll help you shape it.

Things we'd love to see and would rate highly: anything that *only works* because of what's unique about the Polkadot stack. If your project would work just as well on Ethereum mainnet, it is probably not the most interesting version of itself.
