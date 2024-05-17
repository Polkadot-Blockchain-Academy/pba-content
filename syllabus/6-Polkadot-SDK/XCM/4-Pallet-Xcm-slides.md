---
title: XCM Pallet
description: The XCM Pallet - The link between XCM and FRAME
duration: 1 hour
---

# XCM Pallet

---v

## What you'll learn

How to link XCM and FRAME

---

# How XCM can be used

- Interacting directly with the executor by writing XCMs

or

- Packaging up all XCM-logic on extrinsics

---

# The XCM Pallet

We have now learnt about the XCVM and FRAME.

The XCM pallet is the bridge between the XCVM subsystem and the FRAME subsystem.

**It allows us both to interact with the executor directly and provides useful extrinsics**.

---v

## Some functionalities of the pallet

- Executing XCMs locally.
- Sending XCMs to a different location.
- Transferring assets to a different consensus system
- Version negotiation
- Handling responses
- Asset trapping

---

# Executing XCMs locally

<diagram class="mermaid limit size-40">
<!-- prettier-ignore-start -->
flowchart TD
subgraph paraA[Parachain A              .]
  executor --"success?"--> palletxcm
  palletxcm("pallet-xcm") --"execute"--> executor("xcm-executor")
end
execute("execute(xcm)") --> palletxcm
<!-- prettier-ignore-end -->
</diagram>

Notes:

It checks the origin to ensure that the configured `SendXcmOrigin` filter is not blocking the execution.
It executes the message **locally** and returns the outcome as an event.

---

# Sending XCMs

<diagram class="mermaid" style="display: flex; width: 150%; justify-content: center; transform: translateX(-17%);">
<!-- prettier-ignore-start -->
flowchart LR
subgraph paraA[Parachain A]
palletxcma("pallet-xcm") --"deliver"--> routera("xcm-router")
routera --> mqueuea("message queue")
end

subgraph paraB[Parachain B]
mqueueb("message queue") --> executorb("xcm-executor")
end

send("send(xcm)") --> palletxcma
mqueuea --> mqueueb

<!-- prettier-ignore-end -->
</diagram>

Notes:

This extrinsic is a function to send a message to a destination.
It checks the origin, the destination and the message.
Then it lets the `XcmRouter` handle the forwarding of the message.

---

# Cross-consensus transfers

Notes:

The two ways of transferring assets between consensus systems are teleports and reserve transfers.

---v

## 1. Asset teleportation

<img rounded style="width: 500px;" src="./img/teleport.png">

Notes:

Teleportation works by burning the assets on the source chain and minting them on the destination chain.
This method is the simplest one, but requires a lot of trust, since failure to burn or mint on either side will affect the total issuance.

---v

### 1.1. Example: System parachains?

<diagram class="mermaid">
graph LR
    BridgeHub(Bridge Hub)--"Trust"-->AssetHub(Asset Hub)
</diagram>

---v

### 1.2. Example: Polkadot and Kusama?

<diagram class="mermaid">
graph LR
    Polkadot(Polkadot)--"No trust"-->Kusama(Kusama)
</diagram>

---v

## 2. Reserve asset transfers

<img rounded style="width: 400px;" src="./img/reserve-tx.png">

Notes:

Reserve asset transfers are more complicated, since they bring in a third actor called the reserve chain.
Chain A and B needn't trust each other, they only need to trust the reserve chain.
The reserve chain holds the real assets, A and B deal only with derivatives.
The transfer is made by burning derivatives from A, moving them from A's SA to B's SA in R, then minting on B.

In some cases, the sender, A, can also be the reserve for a particular asset, in which case the process is simplified, there's no burning of derivatives.
This usually happens with parachains' native tokens.

You always trust the issuer of the token to not mint infinite tokens.

---v

### 2.1. Example: Parachain native tokens

<diagram class="mermaid">
graph LR
    subgraph A [A = R]
        Sender(Sender account)--"Move X real asset"-->BSovereignAccount(B's Sovereign Account)
    end
    A--"Mint X derivatives"-->B(B)
</diagram>

Notes:

Most parachains act as the reserve for their own token.
To transfer their token to other chains, they move the real assets to a sovereign account and then tell the chain to mint equivalent derivatives.

---v

### 2.2. Example: Polkadot to Kusama

<diagram class="mermaid">
graph LR
    Polkadot(Polkadot)-->AssetHubP
    subgraph AssetHubP [Asset Hub Polkadot]
        Sender(Sender account)--"Move X real DOT"-->KusamaSovereignAccount("Kusama's sovereign account")
    end
    AssetHubP--"Mint X DOT derivatives"-->Kusama(Kusama)
</diagram>

Notes:

AssetHub Kusama acts as the reserve for KSM.
Kusama doesn't trust Polkadot to teleport KSM to it, but it does trust its own reserve, the AssetHub.
Polkadot has a sovereign account in Kusama's AssetHub with some amount of KSM.
Whenever some user in Polkadot wants to get KSM on Kusama, they just give the DOT to Polkadot and the KSM are moved from one sovereign account to another.
No new trust relationships are added.

---v

## `transfer_assets`

The XCM pallet provides an extrinsic that figures out
whether the transfer is a teleport or reserve asset transfer all on its own.

Does this by using the XCM configuration.

---

# Version negotiation

The XCM pallet stores the latest supported version for all known chains.

The instructions `SubscribeVersion` and `UnsubscribeVersion` are used to subscribe to chains to know their versions.

If the version of a destination is not known, the lowest known XCM version will be used, in order to be compatible.

---

# Handling responses

The XCM pallet allows creating query ids and awaiting for responses to XCMs.

With an instruction like `ReportError`, you can know if your operation was successful or not.

---

# Claiming trapped assets

At the end of execution, if assets are left in the holding register, they are trapped.

These assets live in a storage item under the XCM pallet\*.

The pallet provides an extrinsic to claim them.

Notes:

- Only if the pallet is configured to be the asset trapper.

---

# Summary

<pba-flex center>

- Learnt the link between XCM and FRAME.
- Learnt what the XCM pallet does.

---

# Workshop

We're going to write our own version of some extrinsics from the XCM pallet.

---

# Next steps

What about XCM configuration? Is there a tool to test all this?
