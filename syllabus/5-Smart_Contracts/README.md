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

- Introduction (lecturer: Filip, assisting: Piotr)
  - ink! vs Solidity
  - ink! & Rust
  - ink! & Substrate
- Hands-on: my first ink! contract (lecturer: Piotr, assisting: Filip)
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
- Developing contracts (lecturer: Filip, assisting: Piotr)
  - error handling
    - revertable construtors
    - call stack
  - cross-contract calls
    - handling errors
  - shared behaviour
- Importance of events of DAPP development (lecturer: Filip, assisting: Piotr)
  - indexers
  - events as storage

### Afternoon

- Deeper dive: storage (lecturer: Filip, assisting: Piotr)
  - SCALE codec
  - storage layout types
  - Mapping
  - Lazy
  - upgradeability & storage
    - storage migrations
- Testing smart contracts (lecturer: Piotr, assisting: Filip)
  - three main testing and simulation strategies
    - unit tests
    - e2e tests
    - drink tests
  - hands-on
- Interacting with the execution environment (lecturer: Piotr, assisting: Filip)
  - environment functions
  - runtime call
  - chain extension
  - hands-on
- Exploits & vulnerabilities lecture (lecturer: Filip, assisting: Piotr)
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

ink! The Game (lecturer: Piotr, assisting: Filip)

## Assignment

Students will be given a contract with a deliberate vulnerability. They will be tasked with finding it and exploiting. Assignment will be autograded with passing grade when the storage state changes due to the exploit.
- When? End of Day 1
- Deadline for submissions: Next morning after day 2 of the lectures (e.g 7am local time as a cutoff).
