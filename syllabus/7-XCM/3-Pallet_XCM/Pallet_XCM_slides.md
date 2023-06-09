---
title: Pallet-XCM
description: Introducing pallet-xcm, its interface and features implemented here.
duration: 1 hour
---

# pallet-xcm

---

## _At the end of this lecture, you will be able to:_

- Understand what the interface of the pallet is and its implementation.
- What the Subscription Service role is.
- How Response Handler is being laveraged.
- How Asset Trapping fits into the XCM picture

---

## The XCM pallet

We have now learnt about the XCVM and FRAME.

The XCM pallet is the bridge between the XCVM subsystem and the FRAME subsystem.

---

## How XCM is expected to be used

XCM is not intended to be written by end-users.

Instead, _parachain developers_ write XCM programs, and package them up into FRAME extrinsics.

Notes:

We will see an example of this in the XCM pallet, with the teleport_assets and reserve_transfer_assets extrinsics.

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

## Extrinsic breakdown

```rust [1-4|4-11|1-100]
/// Transfer some assets from the local chain to the sovereign account of a destination
/// chain and forward a notification XCM.
/// -- snip
pub fn limited_reserve_transfer_assets(
			origin: OriginFor<T>, // Must be capable of withdrawing the `assets` and executing XCM.
			dest: Box<VersionedMultiLocation>, // Destination context for the assets.
			beneficiary: Box<VersionedMultiLocation>, // A beneficiary location for the assets in the context of `dest`.
			assets: Box<VersionedMultiAssets>, // The assets to be withdrawn. This should include the assets used to pay the fee on the `dest` side.
			fee_asset_item: u32, // The index into `assets` of the item which should be used to pay fees.
			weight_limit: WeightLimit, // The remote-side weight limit, if any, for the XCM fee purchase.
		) -> DispatchResult {
			Self::do_reserve_transfer_assets(
				origin,
				dest,
				beneficiary,
				assets,
				fee_asset_item,
				Some(weight_limit),
			)
		}
```

---v

### Extrinsic breakdown

`do_reserve_transfer_assets` beguins with:
- Checking wether runtime origin has execution permissions
- Decoding destination and assets

```rust
let origin_location = T::ExecuteXcmOrigin::ensure_origin(origin)?;
let dest = (*dest).try_into().map_err(|()| Error::<T>::BadVersion)?;
let beneficiary: MultiLocation =
	(*beneficiary).try_into().map_err(|()| Error::<T>::BadVersion)?;
let assets: MultiAssets = (*assets).try_into().map_err(|()| Error::<T>::BadVersion)?;
```

---v

### Extrinsic breakdown

Other safety checks

```rust
ensure!(assets.len() <= MAX_ASSETS_FOR_TRANSFER, Error::<T>::TooManyAssets);
let value = (origin_location, assets.into_inner());
ensure!(T::XcmReserveTransferFilter::contains(&value), Error::<T>::Filtered);
```

---v

### Extrinsic breakdown

Figure out what is the asset to use for paying fees

```rust
let fees = assets
	.get(fee_asset_item as usize)
	.ok_or(Error::<T>::Empty)?
	.clone()
	.reanchored(&dest, context)
	.map_err(|_| Error::<T>::CannotReanchor)?;
```

---v

### Extrinsic breakdown

What does reanchor mean ?

```rust [5]
let fees = assets
	.get(fee_asset_item as usize)
	.ok_or(Error::<T>::Empty)?
	.clone()
	.reanchored(&dest, context)
	.map_err(|_| Error::<T>::CannotReanchor)?;
```

---v

### Extrinsic breakdown

Set weight_limit if defined or estimate what the weight would be at destination

```rust [1-100|2|3-100]
let weight_limit = match maybe_weight_limit {
	Some(weight_limit) => weight_limit,
	None => {
		let fees = fees.clone();
		let mut remote_message = Xcm(vec![
			ReserveAssetDeposited(assets.clone()),
			ClearOrigin,
			BuyExecution { fees, weight_limit: Limited(Weight::zero()) },
			DepositAsset { assets: Wild(AllCounted(max_assets)), beneficiary },
		]);
		// use local weight for remote message and hope for the best.
		let remote_weight = T::Weigher::weight(&mut remote_message)
			.map_err(|()| Error::<T>::UnweighableMessage)?;
		Limited(remote_weight)
	},
};
```

---v

### Extrinsic breakdown

Build the actual XCM to be sent

```rust
let xcm = Xcm(vec![
	BuyExecution { fees, weight_limit },
	DepositAsset { assets: Wild(AllCounted(max_assets)), beneficiary },
]);
let mut message = Xcm(vec![
	SetFeesMode { jit_withdraw: true },
	TransferReserveAsset { assets, dest, xcm },
]);
```

---v

### Extrinsic breakdown

Weight the message locally and finally request for its execution to `XcmExecutor`

```rust
let weight =
	T::Weigher::weight(&mut message).map_err(|()| Error::<T>::UnweighableMessage)?;
let hash = message.using_encoded(sp_io::hashing::blake2_256);
let outcome =
	T::XcmExecutor::execute_xcm_in_credit(origin_location, message, hash, weight, weight);
Self::deposit_event(Event::Attempted { outcome });
```

[Source](https://github.com/paritytech/polkadot/blob/b1cc6fa14330261a305d56be36c04e9c99518993/xcm/pallet-xcm/src/lib.rs#L1190).

---