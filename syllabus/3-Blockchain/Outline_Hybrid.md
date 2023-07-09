# Hybrid Module Outline

Content might be arranged this way when the Blockchain module is being offered as a full-week collaboration with the Smart Contracts module.

## Monday

We introduce the notion of contracts and how smart contracts fit in to history.
Students learn about expressing agreements as code and are left wondering where we could run such programs.

### Morning

- 📛 Introduce instructor, TAs - This module is coding heavy, we will spend a lot of time coding in class. - Lauren, Joshy - Founders Required
- 🗣️ Overview of Smart Contracts Lecture - Lauren - Founders Required
- 🚧 TODO Some activity. Maybe something like Emre's unstoppable applications from Cambridge, maybe implement a smart contract as an actix_web server. - Joshy / Aaron - Founders NOT Required
- ☕ Break
- 🗣️ Digital Services and State Machines - I like to think of "state" as a double entendre - Lauren - Founders Required
- ⌨️ Begin BFS coding activity - specifically state machine part - Joshy, Lauren, Aaron, Andrew - Founders NOT Required

### Afternoon

- 🗣️ P2P Networking - Andrew - Founders NOT Required
- ⌨️ More BFS.
- ☕ Break
- 🗣️ Platform Agnostic Bytecodes - Lauren - Founders Required
- ⌨️ Web Assembly exercise - Joshy, Andrew, Lauren, Aaron - Founders Required
- 🗣️ Closing Discussion - Where would we actually run these contracts? - why the actix_web example sux. - Lauren - Founders Required

## Tuesday

We put together the pieces presented individually yesterday to form a P2P network where nodes reach consensus on a blockchain-tracked history of a state machine.
We begin discussing consensus, and show how economics and game theory underlie the security assumptions of our P2P network.

### Morning

- 🗣️ Blockchain Structure - Joshy - Founders Required
- ⌨️ More BFS
- ☕ Break
- 🗣️ Consensus Part 1 - Authoring - Agreeing on Possibilities - Joshy - Founders Required
- 🎲 Manual Consensus Activity (aka BitStory) - Founders Required

### Afternoon

- 🗣️ Account and UTXO models - Andrew - Founders Required
- ⌨️ More BFS
- ☕ Break
- 🗣️ Econ & Game Theory in Blockchain - Maciej - Founders Required
- ⌨️ More BFS, or some other activity associated with fees and ordering. Maybe some kind of auction thing.

## Wednesday

We introduce the concept of deterministic finality, explore its history, and deep dive on a few PBFT mechanisms.
We also explore light clients and bridges, both of which benefit from deterministic finality.

### Morning

- 🗣️ Consensus Part 2 - Finality - Agreeing on Ordering - Joshy - Founders Required
- ☕ Break
- 🎲 [Grandpa Board Game Activity](https://github.com/Polkadot-Blockchain-Academy/pba-grandpa-board-game) - Joshy, Andrew, Lauren, Aaron - Founders NOT Required<!-- markdown-link-check-disable-line -->
  - _Note that this repo is private, intended to be shared with the student cohort's github team for read access._
    _This also allows for people to pull up on mobile if logged in to view easier_
- ⌨🗣️ aBFT and Aleph 0 - Adam Gagol - Founders NOT Required

### Afternoon

- 🗣️ Light Clients Bridges - Joshy - Founders Required
- ⌨️ Continue Coding on BFS
- ☕ Break
- 🧘 Flex time. Opportunity to clarify any missed points or otherwise touch up content. Or just a slot into which things can be pushed back.
- ⌨️ BFS

## Thursday

Big Contract Writing Extravaganza!
Students spend the day getting hands on experience writing smart contracts for PABs used in real-world blockchain contracting platforms.

### Morning

- 🗣️ EVM Concepts - Stephen Shelton - Founders Required
- ⌨️ Solidity Code Along - Stephen Shelton - Founders Required
- ⌨️ Vyper Code Along - Stephen Shelton - Founders Required
- ☕ Break
- 🗣️ Wasm Concepts - Filip Bielejec and Piotr Mikolajczyk - Founders Required
- ⌨️ ink! Code Along - Filip Bielejec and Piotr Mikolajczyk - Founders Required
- 🗣️ Introduce Contract Writing Workshop - Joshy - Founders Required

### Afternoon

- ⌨️ [Contract Writing Workshop](https://github.com/Polkadot-Blockchain-Academy/Contract-Writing-Workshop) - Joshy, Lauren, Aaron, Stephen Shelton, Filip Bielejec, and Piotr Mikolajczyk - Founders NOT Required

## Friday

This day combines the two culminating activities from the two modules into a day-long hands-on learning and comradery activity.

### Morning

Students will launch a blockchain a la the blockchain module.
Also get a few last short talks
Then they will use that chain to launch their smart contracts a la the contracts module.

- ⌨️ Start a Blockchain Activity - Joshy - Founders NOT Required
- 🗣️ Unstoppable Applications Lecture - Nuke - Founders NOT Required (HIGHLY suggested?)
- ☕ Break
- 🗣️ Types of Forks - Maciej - Founders NOT Required
- ⌨️ Forking Exercises - Joshy - Founders NOT Required

### Afternoon

🎲⌨️☕ In the afternoon we will use the chain that we launched in the morning to execute the smart contracts competition. - Joshy, Lauren, Aaron, Stephen Shelton, Filip Bielejec - Founders NOT Required
