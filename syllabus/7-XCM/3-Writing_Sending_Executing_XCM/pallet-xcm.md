---
title: pallet-xcm
description: Introducing pallet-xcm, its interface and features implemented here.
duration: 1 hour
---

# pallet-xcm

---

## _At the end of this lecture, you will be able to:_

<!-- TBD  -->

- Understand what the interface of the pallet is and the pallet implements.
- What is the Subscription Service and its role.
- Response Handler
- Asset Trapping

---

## The XCM pallet

We have now learnt about the XCVM and FRAME.

The XCM pallet is the bridge between the XCVM subsystem and the FRAME subsystem.

---
<!-- Following slides are a quick overview of the pallet interface, focusing on the calls it exposes -->
## pallet-xcm

`pallet-xcm` provides default implementations for many traits required by XcmConfig.

`pallet-xcm` also provides an interface containing 10 different extrinsics, which can be split into three categories:

- Primitive functions to locally `execute` or `send` an XCM.
- High-level functions for asset tranfers.
- Extrinsics aimed exclusively at version negotiation.

---

## `pallet-xcm` Primitive extrinsics

`execute`
Direct access to the XCM executor. It checks the origin to ensure that the configured `SendXcmOrigin` filter is not blocking the execution. Then it executes the message **locally** and returns the outcome as an event. It is necessarily executed on behalf of the account that signed the extrinsic (i.e. the origin).

`send`
This extrinsic is a function to send a message to a destination. It checks the origin, the destination and the message. Then it lets the `XcmRouter` handle the forwarding of the message.

---

## `pallet-xcm` Asset Transfer extrinsics

`teleport_assets` & `limited_teleport_assets`

Allow the user to perform an asset teleport. The limited version takes an extra argument (`Option<WeightLimit>`).

`reserve_transfer_assets` & `limited_reserve_transfer_assets`

Allow the user to perform a reserve-backed transfer. Its limited version takes an extra argument as well (`Option<WeightLimit>`).

---

## pallet-xcm::Version Negotiation extrinsics

Every extrinsic in this category requires root as the FRAME origin.

`force_xcm_version`

Modifies the `SupportedVersion` storage item. This storage item is a double map containing information about the XCM version supported by known destinations.

`force_default_xcm_version`

Modifies the `SafeXcmVersion` storage value. Its value is used as fallback version when the destination's supported XCM version is unknown.

`force_subscribe_version_notify`

Sends an XCM with the `SubscribeVersion` instruction to some destination.

`force_unsubscribe_version_notify`

Sends an XCM with the `UnsubscribeVersion` instruction to some destination.

---

<!-- Subscrption service and version negotiation -->

## Subscription Service

Any system can be notified of when another system changes its latest supported XCM version. This is done via the `SubscribeVersion` and `UnsubscribeVersion` instructions.

The `SubscriptionService` type defines what action to take when processing a `SubscribeVersion` instruction.
This type expects an implementation of the `VersionChangeNotifier` trait, which has a `start` and a `stop` function, along with `is_subscribed`.

`pallet-xcm` provides a default implementaiton of this trait. When receiving a `SubscribeVersion`, the chain sends back an XCM with the `QueryResponse` instruction containing its current version.

---

## Version Negotiation

The subscription service leverages any kind of exchange of XCMs between two systems to begin the process of version negotiation.

Each time a system needs to send a message to a destination with an unknown supported XCM version, its location will be stored in the `VersionDiscoveryQueue`. This queue will then be checked in the next block and `SubscribeVersion` instructions will be sent out to those locations present in the queue.

---v

## Version Negotiation Instructions

```rust
enum Instruction {
    SubscribeVersion { query_id: QueryId, max_response_weight: Weight },
    UnsubscribeVersion,
}
```

Notes:

SubscribeVersion - instructs the local system to notify the sender whenever the former has its XCM version upgraded or downgraded.
UnsubscribeVersion - if the sender was previously subscribed to XCM version change notifications for the local system, then this instruction tells the local system to stop notifying the sender on version changes.

---v

## 🗣️ XCM Version Negotiation

XCM version negotiation:
<pba-flex center>

1. Chain A sends `SubscribeVersion` to chain B.
1. Chain B responds `QueryResponse` to chain A with the same query_id and max_weight params, and puts the XCM version in the response
1. Chain A stores chain B's supported version on storage.
1. The same procedure happens from chain B to chain A.
1. Communication is established using the highest mutually supported version.

---v

## 🗣️ XCM Version Negotiation

<img style="width: 900px;" src="../../../assets/img/7-XCM/xcm-versioning.png" alt="Xcm Versioning" />

---

## Response Handler

Version negotiation is just one example among many kinds of queries one chain can make to another. Regardless of which kind of query was made, the response usually takes the form of a `QueryResponse` instruction.

The `ResponseHandler` type requires an implementation of the `OnResponse`trait, which defines actions to be performed when receiving `QueryResponse` instructions back from a query.

A default implementation is provided by the `pallet-xcm`. It checks that the response ID is expected, and processes the response accordingly when receiving it. It rejects any response that it did not query beforehand.

---v

## Information Reporting Instructions

```rust
enum Instruction {
    ReportError(QueryResponseInfo),
    ReportHolding { response_info: QueryResponseInfo, assets: MultiAssetFilter },
    ReportTransactStatus(QueryResponseInfo),
    QueryPallet { module_name: Vec<u8>, response_info: QueryResponseInfo },
}
```

```rust
pub struct QueryResponseInfo {
    pub destination: MultiLocation,
    pub query_id: QueryId,
    pub max_weight: Weight,
}
```

Notes:

All of these instructions contain a `QueryResponseInfo` struct, which contains information about the intended destination of the response, the ID of the query, and the maximum weight that the dispatchable call function can use.
The dispatchable call function is an optional operation that XCM author can specify, and is executed upon receiving the response, effectively acting as a lifecycle hook on response.

---v

## Information

```rust
enum Instruction {
    QueryResponse { query_id: QueryId, response: Response, max_weight: Weight, querier: Option<MultiLocation> },
}
```

Offering some requested information that the local system is expecting

Notes:

This instruction is generally safe to execute, the only tidbit here is that the local system may not be expecting the response from the sender.
Therefore, the `querier` parameter should be checked to ensure that the system that requested the information matches with what is expected.

---
## Asset Trapping

What happens when there are still funds in the `HoldingRegister` after the execution of every instruction is done ?

`pallet-xcm` implements the `DropAssets` trait which defines what to do with the funds. The current implementation traps these assets by adding an asset trap. Then these funds will need to be eventually claimed back by the same origin via `AssetClaims` instruction, which requires the implementation of `ClaimAssets` trait.

These traps are stored in `AssetTraps`, a map where the key is the blake2 (256bits) of the `(origin, assets)` tuple. The storage value is the number of times this tuple has been trapped. The user have to perform as many claims as traps occured.

---

