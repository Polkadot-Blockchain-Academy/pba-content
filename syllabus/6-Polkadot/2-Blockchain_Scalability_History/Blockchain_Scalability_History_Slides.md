---
title: Blockchain Scalability History. 
description: Blockchain Scalability History. Rollups and Sharding.
duration: 1+ hour
---

# Blockchain Scalability History.

---

### Outline

<pba-flex center>

1. [Bitcoin](#bitcoin)
1. [Ethereum](#ethereum)
1. [Polkadot](#polkadot)
1. [References](#references)

</pba-flex>

Notes:

In the Bitcoin section we'll cover attempts to add scalability to Bitcoin, including payment channels used by the Lighting Network.

Then we'll move onto Ethereum scalability history and cover topics such as side chains, state channels, Plasma, rollups and sharding.

Finally we'll cover Polkadot's approach to scalability and how it compares to the rollup-centric design and some ideas for future.

At the end of the lecture, for the curious readers, there's be links for further reading.

---

## Bitcoin

<!-- TODO: Bitcoin Logo  -->

Bitcoin has a block time of 10 minutes and a block size of 1MB.

What if try to increase that limit?

<pba-flex center>

- Bitcoin Cash has a limit of 32MB.
- Bitcoin SV removed the limit altogether.
- Is there a problem with increasing the limit?

</pba-flex>

---

### Payment channels: Layer-2 solution

<!-- TODO: table with Alice and Bob balanced locked  -->

<pba-flex center>

1. Alice and Bob lock some funds in a "smart contract".
1. All transfers between them happen off-chain where both parties
sign each transaction which also invalidates previous state.
1. To close the channel, participants submit the last agreed-upon state of the channel on-chain.

</pba-flex>

Notes:

Bitcoin doesn't really have smart contract, so it uses 2-out-of-2 multisignature transactions instead.

Each new update invalidates previous updates.

To make this part of the state channel work, the locking and unlocking mechanisms have to be properly designed so that old state updates submitted to the blockchain have a chance to be corrected by the newer state updates which replaced them. The simplest way is to have any unlocking attempt start a timer, during which any newer update can replace the old update (restarting the timer as well). When the timer completes, the channel is closed and the state adjusted to reflect the last update received.

In case of Bitcoin, a Timelock could be used.


### Payment channels: Security

<!-- TODO: revamp -->

<pba-flex center>
- Fraudulent Channel Close
- Someone has to stay online to protect each individual party's interests until the channel is closed.
- In the Lighting Network the concept of "watchtower" has been developed, where trust can be outsourced to watchtower nodes to monitor for fraud.
</pba-flex>

Notes:

If a malicious party creates numerous channels and forces them to expire at the same time, which would broadcast to the blockchain, the congestion caused could overwhelm the capacity of the block. A malicious attacker might use the congestion to steal funds from parties who are unable to withdraw their funds due to the congestion.

We'll come to other limitations of channels in the state channels later.

### Payment channels: Lighting Network

<!-- TODO: Diagram of onion routing -->

Hash Time Locked Contracts can be used to route a payment from Alice to Charlie if there's a channel open for Alice and Bob as well as Bob and Charlie without trusting Bob.

1.  Alice opens a payment channel to Bob, and Bob opens a payment channel to Charlie.
1. Alice uses her payment channel to Bob to pay him, but she adds the hash Charlie gave her to the payment along with an extra condition: in order for Bob to claim the payment, he has to provide the data which was used to produce that hash.
1. Bob uses his payment channel to Charlie to pay Charlie and Bob adds a copy of the same condition that Alice put on the payment she gave Bob.
1. Charlie has the original data that was used to produce the hash (called a pre-image), so Charlie can use it to finalize his payment and fully receive the payment from Bob. By doing so, Charlie necessarily makes the pre-image available to Bob.
1. Bob uses the pre-image to finalize his payment from Alice.

---

## Ethereum

### Sidechains

Let's create another "fork" of the chain with its own validator set and security.

To move assets between different chains, a bridge is implemented as a smart contract on Ethereum.

This is partially what Polygon is to Ethereum.

<pba-flex center>
- Bridges have the "weakest link" problem.
- What if a sidechain stops producing blocks? The funds are "stuck".
- If a sidechain is cheaper than Ethereum then it’s going to be proportionally less secure than Ethereum.
</pba-flex>

---

### State channels

This is a generalization of payment channels:

<pba-flex center>
1. Part of the blockchain state is locked in a smart contract.
1. Participants update the state amongst themselves (off-chain).
1. Participants submit the state back to the blockchain.
</pba-flex>

State channels might be good for gameds, e.g. card-based ones.

Notes:

There's also a concept of virtual channels that do not require to open and close them with on-chain transactions.

### State channels: limitations

<pba-flex center>
- Cannot be used to send funds off-chain to people who are not yet participants
- Cannot be used to represent objects that do not have a clear logical owner (e.g. Uniswap)
- Require a large amount of capital to be locked up
</pba-flex>

---

### Plasma

<!-- TODO: logo -->

Are like sidechains, but the the Merkle root of each chain in published on Ethereum. The roots act sort of like “save points” in the blockchain.

Limitations:

<pba-flex center>
- Not possible to do generic computation: Polygon only supports ERC20 and ERC721 token transfers on its Plasma chain.
- Withdrawal period of 7 days on Polygon (challenge period)
</pba-flex>

Notes:

Fun fact: Astar used to be called Plasm.
Pretty much obsoleted because of the limitations by the rollup technologies.

---

### Rollups

Rollups bundle (or ’roll up’) many transactions into a single transaction on layer 1.

---

### Optimistic Rollups

<!-- TODO: logos of Arbitrum, Optimism, Boba -->

Optimistic rollups are 'optimistic' in the sense that transactions are assumed to be valid, but can be challenged if necessary. If an invalid transaction is suspected, a fraud proof is submitted and resolved onchain.

### Optimisitc Rollups: Transaction bundle

<!-- TODO a picture of what a transaction bundle includes -->

On Ethereum, these are posted to a so called "calldata" and in the future to the blob storage provided by Danksharding.

Notes:

We will cover some aspects of Danksharding soon.

### Optimisitc Rollups: How it works

<!-- TODO diagram -->

- Optimistic rollup validators must provide a **bond** before producing blocks.
- Other rollup validators validate the blocks using their copy of the rollup state.
- In case someone finds the state transition invalid, they can submit a fraud proof.

Notes:

This bond can be slashed if the validator posts an invalid block or builds on an old-but-invalid block (even if their block is valid). This way optimistic rollups utilize cryptoeconomic incentives to ensure validators act honestly.

### Optimistic Rollups: Fraud proofs

<!-- TODO merkle proofs from PoV -->

### Optimistic Rollups: Multi-round fraud proofs

<!-- TODO a picture of interactive protocol, maybe convert text into pictures?  -->

The main idea of multi-round fraud proofs (aka interactive fraud proofs) is to
reduce the number of computational steps by using interactive bisection protocol.

It requires the block producer and the challenger to merkelize the entire state of VM (including memory cells, registers, etc).

The process is divided into steps.
The computation state at each step can be described as a short commitment (merkle root) to the output of the VM. 

If the block producer fails to provide the one-step proof, or the L1 verifier deems the proof invalid, they lose the challenge.

### Optimistic Rollups: Security

The transactions posted on the Layer 1 are non "finalized" even if Layer 1 finalized the block including them until the challenge period has ended and there were no valid fraud proofs.

The length of the challenge period is a trade-off between security and finality latency.

Security model relies on at least one honest node executing rollup transactions and submitting fraud proofs to challenge invalid state transitions.

---

### zkRollups

<!-- TODO logos of zkSync and Starkware -->

zkRollups come in 2 flavors:

- zkSNARKs
- zkSTARKs

Just like Optimistic Rollups, zkRollups bundle transactions and submit them along with a succint (short) validity proof.

Checkout https://zkhack.dev/whiteboard/ to learn more about SNARKs.

Notes:

As a scalability technology, the zk part of the zkRollups is optional and not used by privacy-oriented applications.

### SNARK scalability

<!-- TODO style as quote -->

The total prover overhead relative to direct witness checking can be 1 million to 10 million or more.

Source: https://a16zcrypto.com/measuring-snark-performance-frontends-backends-and-the-future/

### zkRollups challenges

- Sequencer centralization and censorship
- Latency for producing a block
- Complexity of the technology makes it hard to audit
- Double spend attacks are still possible (51% attack)

---

### Scalability trilemma

<!-- TODO include diagram of the trilemma -->

---

### Sharding

Sharding is the process of splitting a database horizontally to spread the load - the term comes from the database world.

There are flavors of blockchain sharding:
- Data sharding
- Execution sharding

Notes:

Data sharding means that not every node store every piece of data.
Execution sharding means that not every validator checks every state transition.
Bitcoin and Ethereum don't have execution sharding, they are replicated state machines.
We will cover both data and execution sharding in depth in the next lecture.

## Polkadot

Polkadot is a scalable heterogenous sharded multi-chain network.

### Specialization Leads to Scalability

Systems which try to do everything will not do anything the best.

Generalized blockchains like Ethereum are not optimized.

Heterogeneity is the way.

Notes:

Protocols like dXdY are moving from a smart-contract/rollup to a separate L1 blockchain.

### Parachains

Each parachain is a separate L1 blockchain except for it delegates it's finality to Polkadot. 

To distinguish the fact that block producers of parachains are not responsible for finality, they are called Collators.

### Mechanics of Parachain Validation

Imagine:

- 1000 Validators
- 100 Parachains
- 30-40 Shared Randomly Assigned Validators per Parachain
- Disputes Resolution

We are able to derive strong security guarantees while splitting up our validators across different parachains.

### Sharding and security

How is splitting validators into groups is different from splitting into separate chains?

1. The random sampling prevents the attacker from concentrating their power on one shard.

2. If even one shard gets a bad block, the entire chain reorgs to avoid it.

Notes:

In a 100-chain multichain ecosystem, the attacker only needs ~0.5% of the total stake to wreak havoc: they can focus on 51% attacking a single chain. In a sharded blockchain, the attacker must have close to ~30-40% of the entire stake to do the same (in other words, the chain has shared security).

The second point ensures that processing messages is also secure.

### Parachain Validation Function (PVF)

Validators can verify the state transition of any Parachain given 2 simple pieces of data:

- The Parachain's Wasm Runtime
- The Parachain's State Proof (Proof of Validity)

### Proof of Validity (PoV)

A proof of validity constructed by Cumulus:

<!-- A diagram from https://pep.wtf/posts/parachains-consensus/ -->

### TODO: more slides on Polkadot?

What do we want to cover?

---

### Beyond

<pba-flex center>

- Polkadot Cubed
- Blitz Chains
- Non-Persistent Trie

</pba-flex>

---

## Questions?

---

## References

1. https://ethereum.org/en/developers/docs/scaling/state-channels/
1. https://ethereum.org/en/developers/docs/scaling/plasma/
1. https://vitalik.ca/general/2021/01/05/rollup.html
1. https://vitalik.ca/general/2021/04/07/sharding.html
1. https://zkhack.dev/whiteboard/
