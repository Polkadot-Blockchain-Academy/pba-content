---
title: Pallet-XCM
description: Introducing pallet-xcm, its interface and features implemented here.
duration: 1 hour
---

# pallet-xcm

---

## _At the end of this lecture, you will be able to:_

- Understand what the interface of the pallet is and its implementation.
- How versioning discovery works.
- How receiving responses work.
- Understand how to craft XCM in FRAME pallets.

---

## The XCM pallet

We have now learnt about the XCVM and FRAME.

The XCM pallet is the bridge between the XCVM subsystem and the FRAME subsystem.

**It also allows us to send/execute XCM and interact with the XCM executor**.

---

## How XCM is expected to be used

XCM is not intended to be written by end-users.

Instead, _parachain developers_ write XCVM programs, and package them up into FRAME extrinsics.

Notes:

We will see an example of this in the XCM pallet, with the teleport_assets and reserve_transfer_assets extrinsics.

---

### Key roles of `pallet-xcm`

<pba-flex center>

1. Allows to interact with the `xcm-executor` by executing xcm messages.
   These can be filtered through the `XcmExecuteFilter`.
1. Provides an easier interface to do `reserveTransferAssets` and `TeleportAssets`.
   The origins capable of doing these actions can be filtered by `XcmTeleportFilter` and `XcmReserveTransferFilter`.
1. Handles XCM version negotiation duties.
1. Handles asset-trap/claim duties.
1. Allows sending arbitrary messages to other chains for certain origins.
   The origins that are allowed to send message can be filtered through `SendXcmOrigin`.

</pba-flex>

Notes:

- Even when `palletXcm` allows any FRAME origin to send XCMs, it distinguishes root calls vs any other origin calls.
  In the case of the latter, it appends the `DescendOrigin` instruction to make sure non-root origins cannot act on behalf of the parachain.

---

<!-- Following slides are a quick overview of the pallet interface, focusing on the calls it exposes -->

## pallet-xcm

`pallet-xcm` provides default implementations for many traits required by `XcmConfig`.

`pallet-xcm` also provides an interface containing 10 different extrinsics, which can be split into three categories:

- Primitive functions to locally `execute` or `send` an XCM.
- High-level functions for asset transfers between systems, e.g. teleportation and reserve asset transfers.
- Extrinsics aimed exclusively at version negotiation.

---

## `pallet-xcm` Primitive extrinsics

TODO: Split and diagrams

- `execute`
  Direct access to the XCM executor. It checks the origin to ensure that the configured `SendXcmOrigin` filter is not blocking the execution. Then it executes the message **locally** and returns the outcome as an event. It is necessarily executed on behalf of the account that signed the extrinsic (i.e. the origin).

- `send`
  This extrinsic is a function to send a message to a destination. It checks the origin, the destination and the message. Then it lets the `XcmRouter` handle the forwarding of the message.

---

## `pallet-xcm` Asset Transfer extrinsics

TODO: Split and diagrams.

`teleport_assets` & `limited_teleport_assets`

Allow the user to perform an asset teleport. The limited version takes an extra argument (`Option<WeightLimit>`).

`reserve_transfer_assets` & `limited_reserve_transfer_assets`

Allow the user to perform a reserve-backed transfer. Its limited version takes an extra argument as well (`Option<WeightLimit>`).

---

## 🗣️ version negotiation with `pallet-xcm`

TODO: Tell the versioning story.

XCM is an **extensible message format**.

Versioning allows chains to communicate as XCM evolves.

```rust
pub enum VersionedXcm {
    V2(v2::Xcm),
	V3(v3::Xcm),
}
```

Notes:

- V0 and V1 were removed with the addition of XCM v3.

---v

## 🗣️ version negotiation with `pallet-xcm`

But chains need to be aware of the version supported by each other.
`SubscribeVersion` and `QueryResponse` play a key role here:

```rust
enum Instruction {
  /* snip */
  SubscribeVersion {
        query_id: QueryId,
        max_response_weight: u64,
  },
  QueryResponse {
        query_id: QueryId,
        response: Response,
        max_weight: u64,
  },
  /* snip */
}
```

Notes:

- `query_id` would be identical in the `SubscribeVersion` and `QueryResponse` instructions.
- Likewise, `max_response_weight` should also match `max_weight` in the response

---v

## 🗣️ version negotiation with `pallet-xcm`

- `ResponseHandler`: The component in charge of handling response messages from other chains.
- `SubscriptionService`: The component in charge of handling version subscription notifications from other chains

```rust
 impl Config for XcmConfig {
  /* snip */
  type ResponseHandler = PalletXcm;
  type SubscriptionService = PalletXcm;
 }
```

Notes:

- `PalletXcm` keeps track of the versions of each chain when it receives a response.
- It also keeps track of which chains it needs to notify whenever we change our version

---

<!-- Subscrption service and version negotiation -->

## Subscription Service

TODO: Remove implementation details.

Any system can be notified of when another system changes its latest supported XCM version. This is done via the `SubscribeVersion` and `UnsubscribeVersion` instructions.

The `SubscriptionService` type defines what action to take when processing a `SubscribeVersion` instruction.
This type expects an implementation of the `VersionChangeNotifier` trait, which has a `start` and a `stop` function, along with `is_subscribed`.

`pallet-xcm` provides a default implementaiton of this trait. When receiving a `SubscribeVersion`, the chain sends back an XCM with the `QueryResponse` instruction containing its current version.

---

## Version Negotiation

The subscription service leverages any kind of exchange of XCMs between two systems to begin the process of version negotiation.

Each time a system needs to send a message to a destination with an unknown supported XCM version, its location will be stored in the `VersionDiscoveryQueue`. This queue will then be checked in the next block and `SubscribeVersion` instructions will be sent out to those locations present in the queue.

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

TODO: XCM doesn't care about responses, FRAME does.
We could trap them with a question.

Version negotiation is just one example among many kinds of queries one chain can make to another. Regardless of which kind of query was made, the response usually takes the form of a `QueryResponse` instruction.

The `ResponseHandler` type requires an implementation of the `OnResponse`trait, which defines actions to be performed when receiving `QueryResponse` instructions back from a query.

A default implementation is provided by the `pallet-xcm`. It checks that the response ID is expected, and processes the response accordingly when receiving it. It rejects any response that it did not query beforehand.

---v

## Information Reporting

Every instruction used for information reporting contains `QueryResponseInfo`.

```rust
pub struct QueryResponseInfo {
    pub destination: MultiLocation,
    pub query_id: QueryId,
    pub max_weight: Weight,
}
```

Notes:

All Information Reporting instructions contain a `QueryResponseInfo` struct, which contains information about the intended destination of the response, the ID of the query, and the maximum weight that the dispatchable call function can use.
The dispatchable call function is an optional operation that XCM author can specify, and is executed upon receiving the response, effectively acting as a lifecycle hook on response.

---v

## Information

```rust
enum Instruction {
    QueryResponse { query_id: QueryId, response: Response, max_weight: Weight, querier: Option<MultiLocation> },
}
```

The above instruction is the one used for offering some requested information that the local system is expecting.
`querier` parameter should be checked to ensure that the system that requested the information matches with what is expected.

---

TODO: Less implementation details. Describe the scenario and why it's important.
Why are these traits implemented here? Because they need storage, FRAME gives us that.

### Asset Trap/Claims with PalletXcm

What happens when there are still funds in the holding register after the execution of every instruction is done?

- `AssetTrap` determines what to do with assets that remain; any type that implements `DropAssets` can be an `AssetTrap`.
- `AssetClaim` determines how to reclaim assets that were trapped; any type that implements `AssetClaims` can be an `AssetClaim`.

Notes:

- Any situation in which the holding register contains assets after the execution of the XCM message would lead to asset trapping.
- This is handled in the `post_execute` function of the xcm-executor.

---v

### Asset Trap/Claims with `pallet-xcm`

- **`pallet-xcm` asset trapper**: Trapped assets are stored in the `AssetTraps` storage item and indexed by `BlakeTwo256((origin, assets))`

- **`pallet-xcm` asset claimer**: `pallet-xcm` also allows for claiming trapped assets, providing that:
  - the origin claiming the assets is identical to the one that trapped them.
  - the `multiAsset` being claimed is identical to the one that was trapped

Notes:

- Each map element on `AssetTraps` holds a counter of how many times such origin has trapped such `multiAsset`.
- Every time such `multiAsset` gets reclaimed, the counter decrements by one.

---

## Extrinsic breakdown

TODO: Open VS Code. That's better.

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

## Proposed Exercise

1. Code an extrinsic that creates an XCM which traps some funds.
1. Code an extrinsic that creates an XCM to claim back trapped funds.

---

Usually trapping funds is not a desired outcome. In the first XCM, what modification is needed to avoid such scenario ?
