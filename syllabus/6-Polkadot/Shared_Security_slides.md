---
title: What is Shared Security?
description: A high level overview of Shared Security in Polkadot
duration: 1 hour
---

# What is Shared Security?

---

## On the surface...

Shared Security is an Economic Scaling Solution for Blockchains.

---

But that is just an answer that sits at the surface. The topic goes much deeper than that.

Let’s explore…

---

## What is Security?

---

Security is represented by the economic cost to change the canonical history of a blockchain.

<image src="../../../assets/img/5-Polkadot/chain-fork.svg" style="width: 1000px">


Chains with higher security are more resilient to malicious activity, like a double spend attack.


---

## Note that a double spend is not inherently an attack!

<pba-cols>
<pba-col>

It is perfectly allowed in all blockchain protocols to sign and submit two messages which conflict with one another.

It is up to the blockchain to come to consensus as to which of these two transactions are canonical.

</pba-col>
<pba-col>

<image src="../../../assets/img/5-Polkadot/double-spend.svg" style="width: 500px">

</pba-col>
</pba-cols>

---

## What does an attack look like?

In this example, someone is explicitly taking advantage of fragmentation in the network to try and create two different canonical chains.

<image src="../../../assets/img/5-Polkadot/network-attack.svg" style="width: 1000px">

---

## What is the economic cost?

Eventually, the network fragmentation will resolve, and consensus messages will allow us to prove that the malicious nodes **equivocated**.

<image src="../../../assets/img/5-Polkadot/network-attack-2.svg" style="width: 1000px">

That is, they signed messages that validated two conflicting chains.

---

So Economics and Security are tightly coupled in Blockchians.

---

### What is the Bootstrapping Problem?

Describes the struggle that new chains face to keep their chain secure, when their token economics are not yet sufficient or stable.

Arguably, the scarcest resource in blockchain is economic security - there simply is not enough to go around.

---

<pba-cols>
<pba-col>

<image src="../../../assets/img/5-Polkadot/small-market-cap.svg" style="width: 500px">


</pba-col>
<pba-col>

<image src="../../../assets/img/5-Polkadot/speculative-graph.svg" style="width: 500px">

</pba-col>
</pba-cols>
