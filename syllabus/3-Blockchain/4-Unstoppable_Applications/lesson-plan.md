# Lesson 4: Unstoppable Applications

## Why is this important

A blockchain is just one layer of an application stack. Like cryptography where attackers will go after the weakest link, blockchain attackers will also find the weakest point in the stack. Web3 developers need to understand the application stack they are working within and how to design a foundation for unstoppable applications.

## Core Ideas to Convey

- Intro to application stack
  - Normal application stack
    - [s] --trust me!--> [c]
    - We want to do better than that
  - Web3 Tech Stack
    - Users realistically won't run archive/full nodes
    - There's a lot more than blockchain
    - How some pieces fit together
- Network level
  - How many validators do we need?
    - Validators are not free. Most chains pay with inflation
    - Many chains try to find "minimum that's enough to call it decentralized"
    - Polkadot opens up a lot of economic possibilities here because the security is outsourced
    - Polkadot needs `n` per parachain. Updating is `O(n_paras)`, so more validators really increases the network throughput.
    - We might need to do some architecture here about parachain headers and whatnot
  - Consensus Level
    - Consensus system != blockchain
    - Many blockchains can be part of the same consensus system
    - Slashing and removing bad validators
    - Mining pools?
- Network Access
  - Ideal: User runs a node
    - Reality: [Node] --> [Infura] --trust me!--> [user]
  - Multi-chain applications
    - Marketplace using NFTs, Contracts, Identity, etc.
    - That's a lot of blockchains. Users will never run those nodes!
  - Light Clients
    - Only store headers and consensus-critical information
    - Storage and bandwidth requirements low enough to use in a browser extension or mobile device
    - Use hash-based data structures to verify the integrity of hosting services
    - [Node] --> [RPC Service] --> {[Light Client] --> [User]}
- Validator Power
  - Upgradeability and autonomous enactment
  - Takes power away from validators, service providers, aggregators, etc.
  - Metaprotocol
    - Nodes just run a WebAssembly host.
    - Polkadot’s business logic is stored in the blockchain itself. It’s self describing.
    - This makes the STF upgradeable, because the core protocol is extremely abstract and well adopted.
    - Parachains too.
  - Free vs. Transactional Execution
    - Authors can select transactions, but they can't select autonomous logic
    - Developers have free reign over their STF
    - Possibility of a transactionless chain (in fact, our goal for the Relay Chain)
    - Smart contracts in this context: ability for untrusted users to deploy untrusted code.
- Dependencies
  - Messaging within a synchronous system (e.g. smart contract calls)
  - Messaging between asynchronous systems (e.g. bridges, XCMP)
  - Trusting other consensus systems
  - Bridge failures
- Wrap Up
  - Identifying weak layers in the stack
  - Involves networking, authority privileges, punishment of bad authorities, user-run light clients, performing system upgrades, selecting transactions and order, dependencies on other systems

## Learning Outcomes

At the end of this lesson, students should be able to architect a basic but secure app on the Web3 stack.

## Learning Objectives

- Identify weaknesses in networking, governance, validation, and access
- Explain the distinction between off-chain, on-chain, and on-chain-autonomous governance
- Explain the services that light clients provide

## Prerequisites

- Intro to blockchain
- Intro to consensus
- Economics

## Activity

- Emre TODO

## References

- [Unstoppable Applications Presentation](https://docs.google.com/presentation/d/12CAlrxqnY6ASWI1Tp38FlBn9N9t4sn16fTwhjkxYcVg/edit#slide=id.g84497e2749_0_72)
