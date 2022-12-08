---
title: XCM in the Polkadot Context
description: XCM configuration in the Polkadot chain, debugging failed messages
duration: 1 hour
instructors: ["Keith Yeung"]
teaching-assistants: ["Dan Shields"]
---


# Lesson 5: XCM in Polkadot Context


**Pre-requisites**

- Basic knowledge of parachain/relay XCM configurations.
- SCALE serde, for troubleshooting activity

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

In this case both parachains 1000 (Westmint) and 1001 (Collectives) are allowed to teleport the tok

### Activities

- Westend<->Westmint interactions
- Debugging failed messages
