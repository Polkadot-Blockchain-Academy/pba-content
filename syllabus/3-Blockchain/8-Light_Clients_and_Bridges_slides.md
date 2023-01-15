---
title: Light Clients and Bridges
description: Light clients principles and application in bridges
---

# Light Clients

<pba-cols>
    <pba-col>
        <img width = "80%" alt="Pierre Krieger aka Tomaka" src="./img/tomaka.png" />
    </pba-col>
    <pba-col>
        <blockquote>What can I say?<!-- .element: class="fragment" data-fragment-index="2" --></blockquote>
        <blockquote>It's a client but light.<!-- .element: class="fragment" data-fragment-index="3" --></blockquote>
    </pba-col>
</pba-cols>

---

# 😢 Running a Node is Hard 😭

Ideal:<br />Everyone runs their own node.

Reality:

- It takes a lot of disk, memory, etc
- It takes some know-how
- I don't need it _all the time_

Notes:
The bitcoin whitepaper clearly assumes that users will run their own nodes. This is the most trustless and decentralized way to operate, and you should do it whenever you can. If you think you can't you're probably wrong. Just ask the Monero community.

There are _some_ reasons not to run a full node and the reality is that not everyone will. So even though we should always run our own nodes, let's look at some alternatives and ways we can make node running more accessible.

---v

## RPC Nodes

AKA, trust somebody else's node.

- 🕵️ Spy on you ([like infura](https://decrypt.co/115486/infura-collect-metamask-users-ip-ethereum-addresses-after-privacy-policy-update)).
<!-- .element: class="fragment" data-fragment-index="2" -->
- 🔞 Censor you
<!-- .element: class="fragment" data-fragment-index="3" -->
- 🤥 Lie to you
<!-- .element: class="fragment" data-fragment-index="4" -->
- 💔 Steal your boyfriend
<!-- .element: class="fragment" data-fragment-index="5" -->

Notes:
The easiest thing to do is just trust some expert to run a node for you. Very web2. Lot's of things can go wrong.

So this is definitely not the best option. Let's see if we can do better.

---v

## Lighten the Load

For resource constrained systems and people in a hurry

- Phone
- Raspberry pi
- Microcontroller
- Inside Web Browser

Notes:
One of the complaints was that the node takes too much resources. This is especially true if we want people to be able to run the node in all kinds of exotic environments. And we do want that because we want people to run their own node even when they're just paying the bill at dinner from their phone or liking social posts while scrolling on the bus. Let's make the client lighter so it doesn't require as much resources.

---v

## Light Client Duties

- ❌ Sync blocks
- ❌ Execute blocks
- ✅ Sync headers
- ❔ Maintain Transaction Pool
- ✅ Checks consensus
- ❌ Maintains state

Notes:
This is what a typical light client does. There is not a single definition of light client. There are varying degrees of lightness to suit your needs.

---v

## Trustless

![Bitcoin SPV diagram checking Merkle root](./img/bitcoin-spv.png)

- Relies on full node for data
- Does not have to trust data
- State root helps a lot

Notes:
The figure is from the Bitcoin whitepaper. The concept of light clients has been around since bitcoin. At that time it was known as Simplified Payment Verification. You could confirm that a payment was sent or received. But you couldn't confirm that the tokens in question still existed or anything else about the state.

Chains with state roots can have much more powerful light clients

---v

## Syncing Strategies

- Full header sync
- Checkpoints in code
- Warp sync

Notes:
We also need to address the use case of clients that are not always on. For example if you only need your node on your phone, or when using a specific web page, that means it will have some syncing to do.

Doing a full sync is already a lot faster than on a full client because you aren't downloading or executing the blocks. But by the time you have a few million headers, it does still take some time.

The naive solution is to just have relatively recent headers hard-coded in the client. This works pretty well. You already have to trust the client developers for the entire implementation so you aren't trusting a new party at least.

Warp sync is possible when you have deterministic finality. In dead simple PoA you just check that the authorities have signed the latest block and you are good. If you have authority hand-offs, there is more work to be done. You have to check that each authority set signs the transition to the next authority set. But this is still only even N blocks instead of every block.

---v

## Self Defense

Stay in the gossip protocol or you might get got.

![You get nothing! You Lose! Good day sir!](./img/wanka.jpg)

Notes:
In the main gossip protocol, if authorities finalize two conflicting blocks, then we can prove that they have broken the rules and slash them. If we don't watch the gossip and only peer with a single full node, then our view is entirely defined by that node. They may gossip us an attack chain and we won't know. So it is important to communicate with many different full nodes.


---

# Bridges

Transport layers between independent consensus systems

![Bridge spanning space between blockchains](./img/basic-bridge.svg)

Notes:
Generally speaking bridges move arbitrary data between unrelated consensus systems. Basically between different blockchains, and those messages can evoke arbitrary side effects on the target chain. To keep it concrete, we'll mostly talk about moving tokens 
---v

# Source and Target Chain



---v

Naming - source and destination chains
Kinds
 - centralized - Send your assets to Bob, Bob will mint you new assets on another chain
       can be slightly improved with multisig or w/e but doesn't solve the problem
    WBTC Foundation
 - trust minimized - ???? Heard of this somewhere - maybe seun
 - Trustless - This is the goal of course just like everything else in this ecosystem
 So how to build it? How does anyone interact trustlessly with a blockchain? Run a node.
 A blockchain is extremely resource constrained so it is a perfect candidate for a light client.
 Trustless bridges are built from on-chain light clients
 BTC Relay contract
 one-way vs two-way bridges. A two-way bridge is really jsut two one-way bridge. Think of a two-way street. There is a dedicated lane for each direction.
 What about peers? enter the relayer - a trustless role - need at least one honest relayer - can't find one you trust? Run your own.

Game Theory
Don't accept message as finalized immediately
Validators may be equivocating

Fishermen

Slashing on source vs target chain or both

Multi-chain apps and Blockspace quality. This kind of trustless bridge with proper incentives gets us information about the source chain on the target chain with security about as high as it was on the source chain. If you are building an app that spans multiple chains consider the security guarantees on both chains. The weaker security of the two is the security your app has. More abstractly, your app consumes two different kinds of blockspace that may be of different qualities. Your app is only as quality as the lower of the blockspaces.

So we have a header, now what?
Cross chain applications build on top of this primitive.
If you need some source chain state, your app needs to require a state proof to check against the header's state root.
If you need some source chain transaction, your app needs to require an spv-style transaction proof to check against the header's extrinsics root.

Depository - Mint model - Full backing




