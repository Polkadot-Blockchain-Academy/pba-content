# Module 5: Smart Contracts Module

#### Prerequisites:
```
sudo apt install binaryen
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install dylint-link
cargo install cargo-contract --force
```

## Learning Outcomes

This module introduces students to the concept of smart contracts and how they are a natural extension to the existing field of contracts.
We will learn that computer programs can express formal agreements and act as an efficient and impartial alternative to a court or judge.
We will also understand the importance of running such contracts on a trustworthy platform and see that blockchain can provide such a platform.

## Hands on activities

## Schedule

## Day 1

### Morning

- Introduction
  - ink! vs Solidity
  - ink! & Rust
  - ink! & Substrate
- Hands-on: my first ink! contract
  - Flipper, Echo or an Adder contract
  - Cargo.toml
  - code walk-through
    - storage
    - constructor
    - queries
    - mutations
    - events
  - interacting with the contract
    - cli-tool: cargo contract
    - contracts-ui
    - queries
    - mutations
- Developing contracts
  - error handling
    - revertable construtors
    - call stack
  - cross-contract calls
    - handling errors
  - shared behaviour
- Importance of events of DAPP development
  - indexers
  - events as storage

### Afternoon

- Deeper dive: storage
  - SCALE codec
  - storage layout types
  - Mapping
  - Lazy
  - upgradeability & storage
    - storage migrations
- Interacting with the execution environment
- Testing smart contracts
  - hands-on
- Exploits & vulnerabilities lecture
  - one hands-on hacking exercise during the lecture, to be decided
  - The DAO hack & reentrancy attacks
  - The Parity Wallet Hack(s)
  - BatchOverflow (and Underflow)
  - MEVs: The dark forest
    - frontrunning
    - sandwitch
    - salmonella
    - MEV: the problems
    - MEV: the solutions

## Day 2

#### Morning

ink! The Game