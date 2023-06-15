# Hybrid Module Outline

Content might be arranged this way when the Blockchain module is being offered as a full-week collaboration with the Smart Contracts module.

## Monday

We introduce the notion of contracts and how smart contracts fit in to history.
Students learn about expressing agreemnts as code and are left wondering where we could run such programs.

### Morning

- 📛 Introduce instructor, TAs - This module is coding heavy, we will spend a lot of time coding in class. - Lauren, Joshy
- 🗣️ Overview of Smart Contracts Lecture - Lauren
- 🚧 TODO Some activity. Maybe something like Emre's unstoppable applications from Cambridge, maybe implement a smart contract as an actix_web server. - Joshy / Aaron
- ☕ Break
- 🗣️ Digital Services and State Machines - I like to think of "state" as a double entendre - Lauren
- ⌨️ Begin BFS coding activity - specifically state machine part - Joshy, Lauren, Aaron, Andrew

### Afternoon

- 🗣️ P2P Networking - Andrew
- ⌨️ More BFS.
- ☕ Break
- Platform Agnostic Bytecodes - Lauren
- ⌨️ Web Assembly exercise - Joshy, Andrew, Lauren, Aaron
- 🗣️ Closing Discussion - Where would we actually run these contracts? - why the actix_web example sux. - Lauren



## Tuesday

We put together the pieces presented individually yesterday to form a P2P network where nodes reach consensus on a blockchain-tracked history of a state machine.
We begin discussing consensus, and show how economics and game theory underlie the security assumptions of our P2P network.

### Morning

- 🗣️ Blockchain Structure - Joshy
- ⌨️ More BFS 
- ☕ Break
- 🗣️ Consensus Part 1 - Authoring - Agreeing on Possibilities - Joshy
- 🎲 Manual Consensus Activity (aka BitStory)

### Afternoon

- 🗣️ Account and UTXO models - Andrew
- ⌨️ More BFS
- ☕ Break
- 🗣️ Econ & Game Theory in Blockchain - Maciej
- ⌨️ More BFS, or some other activity associated with fees and ordering. Maybe some kind of auction thing.

## Wednesday

We introduce the concept of deterministic finality, explore its history, and deep dive on a few PBFT mechanisms.
We also explore light clients and bridges, both of which benefit from deterministic finality.
### Morning

- 🗣️ Consensus Part 2 - Finality - Agreeing on Ordering - Joshy
- ☕ Break
- 🎲 [Grandpa Board Game Activity](https://github.com/Polkadot-Blockchain-Academy/pba-grandpa-board-game) - Joshy, Andrew, Lauren, Aaron<!-- markdown-link-check-disable-line -->
  - _Note that this repo is private, intended to be shared with the student cohort's github team for read access._
    _This also allows for people to pull up on mobile if logged in to view easier_
- ⌨🗣️ aBFT and Aleph 0 - Adam Gagol

### Afternoon

- 🗣️ Light Clients Bridges - Joshy
- ⌨️ Continue Coding on BFS
- ☕ Break
- 🧘 Flex time. Opportunity to clarify any missed points or otherwise touch up content. Or just a slot into which things can be pushed back.
- ⌨️ BFS

## Thursday

Big Contract Writing Extravaganza!
Students spend the day getting hands on experience writing smart contracts for PABs used in real-world blockchain contracting platforms.

### Morning

- 🗣️ EVM Concepts - Stephen Shelton
- ⌨️ Solidity Code Along - Stephen Shelton
- ⌨️ Vyper Code Along - Stephen Shelton
- ☕ Break
- 🗣️ Wasm Concepts - Filip Bielejec
- ⌨️ ink! Code Along - Filip Bielejec
- ⌨️ ask! Code Along (optional) - Filip Bielejec
- 🗣️ Introduce Contract Writing Workshop - Joshy

### Afternoon

- ⌨️ [Contract Writing Workshop](https://github.com/Polkadot-Blockchain-Academy/Contract-Writing-Workshop) - Joshy, Lauren, Aaron, Stephen Shelton, Filip Bielejec

## Friday

This day combines the two culminating activities from the two modules into a day-long hands-on learning and comradery activity.

### Morning

Students will launch a blockchain a la the blockchain module.
Also get a few last short talks
Then they will use that chain to launch their smart contracts a la the contracts module.

- ⌨️ Start a Blockchain Activity - Joshy
- 🗣️ Unstoppable Applications Lecture - Nuke man Dan
- ☕ Break
- 🗣️ Types of Forks - Maciej
- ⌨️ Forking Exercises - Joshy

### Afternoon

🎲⌨️☕ In the afternoon we will use the chain that we launched in the morning to execute the smart contracts competition. - Joshy, Lauren, Aaron, Stephen Shelton,  Filip Bielejec
