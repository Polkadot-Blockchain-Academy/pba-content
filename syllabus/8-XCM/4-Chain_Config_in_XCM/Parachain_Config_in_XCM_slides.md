---
title: Parachain Configuration for XCM # Also update the h1 header on the first slide to the same name
description: Describe your slides here
duration: 1 hour
instructors: ["Gavin Wood", "Keith Yeung"]
teaching-assistants: ["Dan Shields"]
---

# Parachain Configuration for XCM

---

# Chain XCM Configurations

## _Core Concepts, Terms, and Logic_

Notes:

**Pre-requisites**

- FRAME (Storage Items, Dispatchables, Event, Errors, etc.)
- Polkadot & parachains conceptually
- XcVm
- Core concepts of XCM

---

## _At the end of this lecture, you will be able to:_

<widget-text center>

- Understand the different XCM configurable parts for a chain
- Construct different XCM configurations for chains with different needs
- Understand how Versioninig is handled across chains

---

## Configurables in XCM-config

```rust
pub struct XcmConfig;
impl Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = ?;
	type OriginConverter = ?;
	type IsReserve = Everything;
	type IsTeleporter = Nothing;
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = ?;
	type Weigher = ?;
	type Trader = ?;
	type ResponseHandler = ?;
	type AssetTrap = ();
	type AssetClaims = ();
	type SubscriptionService = ?;
}
```

Notes: Xcm-builder and Xcm-pallet are your friends!

`xcm-builder` is a polkadot module that contains a set of pre-defined structures to be set in some of the configurable sides of XCM.

`xcm-pallet` is a pallet that not only allows sending and executing XCM messages, but rather it also implements several of the configuration traits and thus can be used perform several XCM configuration actions.

---

### Configuring asset-transactors with xcm-builder
Asset transactors define how we are going to withdraw and deposit assets. What we set in here depends heavily on what do we want our chain to be able to transfer:

1. `CurrencyAdapter`: A simple adapter that uses a single currency as the assetTransactor. This is usually used for withdrawing/depositing the native token of the chain.
2. `FungiblesAdapter`: Used for depositing/withdrawing from a set of defined fungible tokens. An example of these would be `pallet-assets` tokens.

---
### Configuring origin-converter with xcm-builder
Allows us to configure how we convert an origin, defined by a MultiLocation, into a local dispatch origin. Used in the `Transact` instruction.

1. `SovereignSignedViaLocation`: Converts the multilocation origin (tipically, a parachain origin) into a signed origin.

2. `ParentAsSuperuser`: Converts the parent origin into the root origin.

3. `SignedAccountId32AsNative`: Converts a local 32 byte account multilocation into a signed origin using the same 32 byte account.

4. `SignedAccountKey20AsNative`: Converts a local 20 byte account multilocation into a signed origin using the same 20 byte account.

---
### Configuring Barriers with xcm-builder

Barriers specify whether or not an XCM is allowed to be executed on the local consensus system. In `xcm-builder` we can find the following pre-defined barriers already:

1. `TakeWeightCredit`: A barrier that substracts the maxixum weight the message can consume from the available weight credit. Usually configured for local xcm-execution

2. `AllowTopLevelPaidExecutionFrom<T>`: If the `origin` that sent the message is contained in `T`, this is a barrier that ensures the message contains weight payment instructions. In other words, if makes sure the first instruction puts asset into the holding register (`TeleportAsset`, `WithdrawAsset`, `ClaimAsset`, `ReserveAssetDeposit`), and then it checkes that there exists a `BuyExecution` instruction that is able to pay for the message weight. **Critical to avoid free DOS**.

3. `AllowUnpaidExecutionFrom<T>`: If the `origin` that sent the message is contained in `T`, allows free execution from such origin (i.e., does not check anything from the message). Useful for chains that "trust" each other (e.g., Statemine or any system parachain with the relay)

4. `AllowKnownQueryResponses<ResponseHandler>`: Allows the execution of the message if it is a `QueryResponse` message and the `ResponseHandler` is expecting such response

5. `AllowSubscriptionsFrom<T>`: If the `origin` that sent the message is contained in `T`, it allows the execution of the message if it is a `SubscribeVersion` message
---

### Configuring Weigher with xcm-builder
Weigher allows you to define how your xcm messages is measured in terms of weight. In other words, it lets you define how much each of the instructions will cost in terms of execution time.

1. `FixedWeightBounds<T, C, M>`: Fixed weight bounds allows us to specify a fixed amount of weight we will charge per instruction.
---
### Configuring WeightTrader with xcm-builder
WeightTrader allows to specify how to charge for weight inside the xcm execution. In `xcm-builder` we can find the following pre-defined traders already:

1. `FixedRateOfFungible`: Converts weight to fee at a fixed rate and charges in a specific fungible asset
2. `UsingComponents`: uses `TransactionPayment` pallet to set the right price for weight.

---
### Configuring assetTrap, assetClaim, ResponseHandler and SubscriptionService with pallet-xcm

The last 4 elements will be set to be handled by pallet-xcm. 

```rust
 impl Config for XcmConfig {
  /* snip */
	type ResponseHandler = PalletXcm;
	type AssetTrap = PalletXcm;
	type AssetClaims = PalletXcm;
	type SubscriptionService = PalletXcm;
 }
```
But what do all these elements mean:

1. `ResponseHandler`: The component in charge of handling response messages from other chains

2. `SubscriptionService`: The component in charge of handling version subscription notifications from other chains

Wait, version subscription notifications?

---

## Configuring pallet-xcm
Pallet-xcm plays a critical role in every chains xcm-setup:

1. It allows for users to interact with the xcm-executor by allowing them to execute xcm messages
2. It handles XCM version negotiation
 

```rust
impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
	type XcmExecutor = XcmExecutor;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type Weigher = XcmWeigher;
	type LocationInverter = LocationInverter<Ancestry>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}
```

## XCM Version Negotiation

XCM is an **extensible message format**.

Versioning allows chains to communicate as XCM evolves.

```rust
pub enum VersionedXcm {
    V0(v0::Xcm),
    V1(v1::Xcm),
    V2(v2::Xcm),
}
```

But chains need to be aware of the version supported by each other. `SubscribeVersion` and `QueryResponse` play a key role here:

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

XCM version negotiation:
<widget-text center>

1. Chain A sends `SubscribeVersion` to chain B.
2. Chain B responds `QueryResponse` to chain A with the same query_id and max_weight params, but the XCM version in the response
3. Chain A stores chain B's supported version on storage.
4. The same procedure happens from chain B to chain A.
5. Communication is stablished in the highest common supported version.

<widget-columns>
<widget-column>
<img style="width: 500px;" src="../../../assets/img/7-XCM/xcm-versioning.png" alt="Xcm Versioning"/>
</widget-column>
<widget-column>


### Additional config for version negotiation

<!--

TODO add slide somewhere about this. Basically scaffold gav's second blog

https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83

-->

---

## Lesson 3: Parachain Configuration for XCM

> {Likely} Instructor: RT Eng.? Shawn?

### Why is this topic Important?

### Prerequisite Knowledge or Skills

### Learning Outcome

- Understand and correctly configure XCM (pallet, executor, ...) for a parachain (exectutor and pallet) based on buisness needs
- Barriers & filters
- Asset transactors

### Learning Objectives

### Core Ideas to Convey

- XCM builder module (makes configuration easier)
- Barriers & filters
- Asset transactors
- Weight Traders
- wildcards? {maybe out of scope}

### Activities and Exercises

- Transfer native tokens from ChainA → ChainB **on Rococo** {no setup needed, }
  - Teleport
  - Reserve Asset
  - Explain what to use these given a scenario:
    - Within a chain itself `Here`
    - ParaA → ParaB
    - Relay Chain Native token to ParaA (not common good chain)
    - ... others?
