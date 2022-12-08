---
title: XCM in the Polkadot Context # Also update the h1 header on the first slide to the same name
description: XCM in the Polkadot Context for web3 Engineers
duration: 1 hour
instructors: ["Gavin Wood", "Keith Yeung"]
teaching-assistants: ["Dan Shields"]
---

## _At the end of this lecture, you will be able to:_

<widget-text center>

- Understand the configuration of the Polkadot chain
- Send real-world messages between parachain A <-> Polkadot
- Identify potential errors on XCM messages

## What considerations we need to take into account?

- There should be no trust assumption between chains unless explicitely requested.
- We cannot assume chains will not act maliciously
- Spamming XCM messages creates a DoS problem


## How does Polkadot configure XCM to take these considerations into account?
- Barriers
- Teleport filtering
- Fee payment
- Proper XCM Instruction Weighting

Notes: From now on, we will use the Westend runtime as a reference. Westend is a testnet for 
Polkadot and Kusama that we will use in to test our XCM messages. Most of the Westend configuration
is identical to that in Polkadot.
## XCM barriers in Polkadot

There are 5 barriers that are being used in Polkadot:

```rust
/// The barriers one of which must be passed for an XCM message to be executed.
pub type Barrier = (
	// Weight that is paid for may be consumed.
	TakeWeightCredit,
	// If the message is one that immediately attemps to pay for execution, then allow it.
	AllowTopLevelPaidExecutionFrom<Everything>,
	// Messages coming from system parachains need not pay for execution.
	AllowUnpaidExecutionFrom<IsChildSystemParachain<ParaId>>,
	// Expected responses are OK.
	AllowKnownQueryResponses<XcmPallet>,
	// Subscriptions for version tracking are OK.
	AllowSubscriptionsFrom<Everything>,
);
```

`TakeWeightCredit` and `AllowTopLevelPaidExecutionFrom` are used to prevent spamming for local/remote XCM execution.
`AllowUnpaidExecutionFrom` lets a system parachain have free execution in the relay.
`AllowKnownQueryResponses` and  `AllowSubscriptionsFrom`, as we know already, are mostly used for versioning.

## Trusted teleporters in Polkadot
Teleporting involves trust between chains, as the token is being burnt in one chain to be minted in the other.
As such, teleporting should be only enabled with very specific chains.

Polkadot configures which are the chains allowed to teleport tokens in the following manner:

```rust
parameter_types! {
  pub const WndLocation: MultiLocation = Here.into();
	pub const Westmint: MultiLocation = Parachain(1000).into();
	pub const Collectives: MultiLocation = Parachain(1001).into();
	pub const WestendForWestmint: (MultiAssetFilter, MultiLocation) =
		(Wild(AllOf { fun: WildFungible, id: Concrete(WndLocation::get()) }), Westmint::get());
	pub const WestendForCollectives: (MultiAssetFilter, MultiLocation) =
		(Wild(AllOf { fun: WildFungible, id: Concrete(WndLocation::get()) }), Collectives::get());
}

pub type TrustedTeleporters =
	(xcm_builder::Case<WestendForWestmint>, xcm_builder::Case<WestendForCollectives>);
```

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsTeleporter = TrustedTeleporters;
  /* snip */
}
```

In this case both parachains 1000 (Westmint) and 1001 (Collectives) are allowed to teleport tokens represented by the **Here** multilocation.

## Trusted reserves in Polkadot
Polkadot does not recognize any chain as reserve

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsReserve = ();
  /* snip */
}
```

## LocationToAccountId in Polkadot
As we know, the conversion between a multilocation to an AccountId is a key component to withdraw/deposit assets and issue Transact operations. In the case of Polkadot

```rust
pub type LocationConverter =
	(ChildParachainConvertsVia<ParaId, AccountId>, AccountId32Aliases<WestendNetwork, AccountId>);
```
This means that:
- Parachain origins will be converted to their corresponding sovereign account
- Local 32 byte origins will be converted to a 32 byte defined AccountId.

## Asset Transactors in Polkadot
There is just a single asset-transactor in Polkadot, defined by

```rust
pub type LocalAssetTransactor = XcmCurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<WndLocation>,
	// We can convert the MultiLocations with our converter above:
	LocationConverter,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// It's a native asset so we keep track of the teleports to maintain total issuance.
	CheckAccount,
>;
```

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type AssetTransactor = LocalAssetTransactor;
  /* snip */
}
```

The asset-transactor is matching the **Here** multilocation id to the Currency defined in **Balances**, which refers to **pallet-balances**. Essentially, this is configuring XCM such that the native token (DOT) is associated with the multilocation **Here**.

## Origin Converters in Polkadot
Origin converters defined ways in which we can convert a multilocation to a dispatch origin, tipically used by the **Transact** instruction:

```rust
type LocalOriginConverter = (
  // Converts to a signed origin with "LocationConverter"
	SovereignSignedViaLocation<LocationConverter, RuntimeOrigin>,
  // Converts a child parachain multilocation to a parachain origin
	ChildParachainAsNative<parachains_origin::Origin, RuntimeOrigin>,
  // Converts a local 32 byte multilocation to a signed
  // origin
	SignedAccountId32AsNative<WestendNetwork, RuntimeOrigin>,
  // Converts system parachain origins into root origin
	ChildSystemParachainAsSuperuser<ParaId, RuntimeOrigin>,
);
```

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type OriginConverter = LocalOriginConverter;
  /* snip */
}
```

Here two things should catch our eye. First, there exists the concept of a "parachain dispatch origin" which is used for very specific functions (like, e.g., opening a channel with another chain). Second, system parachins are able to dispatch as root origins, as they can bee seen as an extension to the polkadot runtime itself.

## Traders in Polkadot
Finally we are going to check how Polkadot charges for xcm execution time. In this case, we need to check the **Trader** field in the Config:

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type Trader = UsingComponents<WeightToFee, WndLocation, AccountId, Balances, ToAuthor<Runtime>>;
  /* snip */
}
```
In other words:

- Weight is converted to fee with the **WeightToFee** structure.
- The asset in which we charge for fee is **WndLocation**. This means we can only pay for xcm execution in the **native currency**
- Fees will go to the block author thanks to **ToAuthor**

### Activities and Exercises

