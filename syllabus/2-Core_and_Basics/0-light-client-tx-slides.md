---
title: Light Client Transaction
description: End to End Transaction Flow with a Light Client
---

# Light Client Transaction

---

In this presentation we will cover the end to end flow of a transaction.

- Starting with a light client.
- Being processed by a node.
- Updating the state of the blockchain.
- And finally state update being verified by the light client.

Along the way, we will remind you and touch on details you should already be familiar with.

---

# Part 0: What is a Light Client?

---

## Block Re-Execution

Blockchain systems are decentralized and trustless because anyone is able to fully re-execute the state transition function of the blockchain, across all blocks, and recreate the state of the chain at any point in time.

---

## Node Minimum Requirements

However, in order to "keep up" with the blockchain, you require minimum hardware, which is usually more powerful than the average phone, laptop, or other mobile device.

- Execution Speed
- SSD Requirements
- RAM Requirements
- Networking Speeds
- etc...

---

## Scalability Through Speed

As you have learned with Polkadot, we employ lots of advance engineering to achieve a secure, scalable, and resilient blockchain. However, the reality is, if you use a more powerful computer across the nodes in your network, your blockchian will inheriently perform better.

If you have a blockchain which can directly run on a phone, it is likely not performing at competitive speeds.

---

So does the trustless decentralized future exclude mobile devices and other similar lower power hardware?

---

## Introducing Light Clients

This is where Light Clients come into play.

Light Client Nodes are those which do not fully execute and sync the state of the blockchain, but use verifiable proofs to be able to communicate and receive information trustlessly from another full node.

---

## How Do Light Clients Work?

Light clients synchronize and verify **block headers** rather than the full blocks themselves.

These block headers tell

---

## Block Breakdown

A block is split into two main parts:

- Block Header
	- Parent Hash, Block Number, State Root, Extrinsic Root, Digest
- Block Body
	- Vector of Extrinsics

The block hash representing a unique block is simply the hash of the data inside the header. Since we already include the Extrinsic Root inside the block header, we need not know the block body to get the block hash.

---

## Block Header

Inside the block header is:

- **Parent Hash**: The hash of the preceding block's header. This is what links blocks together to form the blockchain.
- **Block Number**: The height of the block in the chain.
- **State Root**: This single merkle root hash represents all current data stored in the blockchain, and can be used to verify proofs that the blockchain contains some specific state.
- **Extrinsics Root**: This merkle root of the extrinsics found in the block's body, allowing a client to prove that a specific extrinsic was or was not included in the block without needing to download the entire body.
- **Digest**: A list of log items containing "auxiliary" information needed to verify the block. This is where consensus-related data lives.

For a light client, the digest is very important.

---

## Block Digest

- Consensus Logs: Data from the block production engine (e.g., BABE or AURA). This includes information like the slot number and the block author.

- Seal: A signature from the block author, proving they produced this block.

- GRANDPA Logs: This is how Polkadot's finality mechanism (GRANDPA) communicates validator set changes. A light client will parse the digest for logs like ScheduledChange or ForcedChange to know when the validator set will be updated. This is how a light client tracks the authority set without downloading the state.

---

## Light Client's Job

A light client's main goal is to verify the state of the blockchain with minimal resource usage. It achieves this by focusing on three critical tasks:

1. Following the Validator Set
2. Verifying Block Authorship
3. Confirming Finality

For the purposes of explaining how a light client works with these steps, we will go through them backwards.

---

## Confirming Finality

Light Clients will recieve new blocks from a full node, but why should it trust that this block is accurate and part of the cannonical chain?

For this, the full node will send to the light client a GRANDPA Justification, which is not part of the block, but something gossiped as part of the networking and consensus protocol.

Justification gossip happens in rounds, allowing the validators to give up to date votes on their view of the blockchain.

---

## GRANDPA Justification

The GRANDPA Justification includes signatures from current block producers that they believe some chain of blocks are part of the cannonical chain, and should be finalized. As soon as the Justification contains 2/3 of the validator signatures, the block is finalized.

Light Clients are responsible for keeping track of the current validator set, and are able to individually verify the signatures of each validator in the Justification.

By matching the block hash with the signatures from validators, they are then able to trust all the contents of the block header given to them by the full node.

---

## Block Author

Within the header itself are signatures and data from the block author who made the block.

Inside the digest is a **seal**, which is simply a signature from the block author for the block hash.

There is also a secret VRF which the block author reveals to show they are allowed to produce a block during that slot.

All of this can again be independently verified by the nodes on the network, including light clients.

---

## Following the Validator Set

Verifying the Justification assumes the light client knows the current validator set. Since it does not actually execute the blocks, it cannot simply query for that information.

Instead, this information is constantly updated in the block digest along with the other consensus critical data.

---

## Updating the Validator Set

The genesis block defines the initial state of the blockchain, including the initial validator set.

From there, based on the state transition function of the blockchain, a new validator set might be queued for some future block.

This will be pre-announced in the digest, and signed by the current validator set, showing that a new validator set will be active in the future.

With this announcement included in the digest, and backed by the GRANDPA Justification, light clients can always know who the active validators are, even as they change.

---

## Light Clients Verify Everything but the STF

As you can see, Light Clients are able to remain trustless within the blockchain ecosystem because they are able to verify all of the block headers of the canonical chain.

With this, it becomes simple to verify:

- a transaction has been included in the chain, from the extrinsics root.
- the state of the chain, from the state root.

And remember, the State Transition Function (Wasm) itself is stored on chain, but the light client is not expected to execute it.

---

# Part 1: Light Client Wants to Create a Transaction

---

So we have seen so far, in general, how a light client would trustlessly follow the canonical version of the blockchain.

But let's assume the light client wants to interact with the chain. Something simple like a balance transfer from Alice to Bob.

How would it do that?

---

## Fetching the Metadata

The first thing the light client will do is fetch the up to date metadata of the chain.

Remember that in the Polkadot ecosystem, chains can constantly upgrade and update their state transition function, changing the functionality of the chain and even what extrinsics are available.

---

## State Transition Function is Self Describing

To get the metadata itself, we must query the Runtime via the State Transition Function Code.

The Runtime exposes an API: `state_getMetadata`, which will return a SCALE encoded blob with all the data you need to know about the blockchain.

---

## Merkle Tree

TODO

---

## Merkle Proof

TODO

---

## SCALE

TODO

---

## FRAME Metadata Format

- Versioned, and occasionally changes over time.
- Defined by: https://github.com/paritytech/frame-metadata
- Automatically generated if you use FRAME macros.
- Output can be turned into JSON, which is easiest for human readability:
	- https://dev.papi.how/metadata/json

---

## Get Your Current Balance

Using the Metadata we can see the following:

- Balances is pallet index 6.
- Name of Storage Prefix is "Balances"

---

## Construct the Transfer Extrinsic
