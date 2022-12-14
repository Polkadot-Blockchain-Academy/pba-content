---
title: XCM in the Polkadot Context # Also update the h1 header on the first slide to the same name
description: XCM in the Polkadot Context for web3 Engineers
duration: 1 hour
instructors: ["Keith Yeung", "Gorka Irazoqui"]
teaching-assistants: ["Andrew Burger", "Hector Bulgarini"]
---

## _At the end of this lecture, you will be able to:_

<widget-text center>

- Understand the configuration of the Polkadot chain
- Send real-world messages between parachain A <-> Polkadot
- Identify potential errors on XCM messages

---

## 🤔 What considerations we need to take into account?

- There should be no trust assumption between chains unless explicitely requested.
- We cannot assume chains will not act maliciously
- Spamming XCM messages creates a DoS problem

---

## 🛠️ How does Polkadot configure XCM to take these considerations into account?
- Barriers
- Teleport filtering
- Fee payment
- Proper XCM Instruction Weighting

Notes: From now on, we will use the Rococo runtime as a reference. Rococo is a testnet for 
Polkadot and Kusama that we will use in to test our XCM messages. Most of the Rococo configuration
is identical to that in Polkadot.

---

## 🚧 XCM barriers in Polkadot

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

---

## 🤝 Trusted teleporters in Polkadot
Teleporting involves trust between chains, as the token is being burnt in one chain to be minted in the other.
As such, teleporting should be only enabled with very specific chains.

Polkadot configures which are the chains allowed to teleport tokens in the following manner:

```rust
parameter_types! {
  pub const RocLocation: MultiLocation = Here.into();
  pub const Rococo: MultiAssetFilter = Wild(AllOf { fun: WildFungible, id: Concrete(RocLocation::get()) });

	pub const Statemine: MultiLocation = Parachain(1000).into();
	pub const Contracts: MultiLocation = Parachain(1002).into();
	pub const Encointer: MultiLocation = Parachain(1003).into();

	pub const RococoForStatemine: (MultiAssetFilter, MultiLocation) = (Rococo::get(), Statemine::get());
	pub const RococoForContracts: (MultiAssetFilter, MultiLocation) = (Rococo::get(), Contracts::get());
	pub const RococoForEncointer: (MultiAssetFilter, MultiLocation) = (Rococo::get(), Encointer::get());
}

pub type TrustedTeleporters = (
  /* Ignore */
	xcm_builder::Case<RococoForTick>,
	xcm_builder::Case<RococoForTrick>,
	xcm_builder::Case<RococoForTrack>,

	xcm_builder::Case<RococoForStatemine>,
	xcm_builder::Case<RococoForContracts>,
	xcm_builder::Case<RococoForEncointer>,
);
```

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsTeleporter = TrustedTeleporters;
  /* snip */
}
```

In this case both parachains 1000 (Statemint) and 1001 (Contracts) and 1002 (Encointer) are allowed to teleport tokens represented by the **Here** multilocation.

## 💱Trusted reserves in Polkadot
Polkadot does not recognize any chain as reserve

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsReserve = ();
  /* snip */
}
```

---

## 📁 LocationToAccountId in Polkadot
As we know, the conversion between a multilocation to an AccountId is a key component to withdraw/deposit assets and issue Transact operations. In the case of Polkadot

```rust
pub type LocationConverter =
	(// We can convert a child parachain using the standard `AccountId` conversion.
	ChildParachainConvertsVia<ParaId, AccountId>,
	// We can directly alias an `AccountId32` into a local account.
	AccountId32Aliases<RococoNetwork, AccountId>,
  );
```
This means that:
- Parachain origins will be converted to their corresponding sovereign account
- Local 32 byte origins will be converted to a 32 byte defined AccountId.

## 👍 Asset Transactors in Polkadot
There is just a single asset-transactor in Polkadot, defined by

```rust
pub type LocalAssetTransactor = XcmCurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<RocLocation>,
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

---

## 📍Origin Converters in Polkadot
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

---

## Traders in Polkadot
Finally we are going to check how Polkadot charges for xcm execution time. In this case, we need to check the **Trader** field in the Config:

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type Trader = UsingComponents<WeightToFee, RocLocation, AccountId, Balances, ToAuthor<Runtime>>;
  /* snip */
}
```
In other words:

- Weight is converted to fee with the **WeightToFee** structure.
- The asset in which we charge for fee is **RocLocation**. This means we can only pay for xcm execution in the **native currency**
- Fees will go to the block author thanks to **ToAuthor**

---

## 🎨 XcmPallet in Polkadot
The last thing to be checked is how palletXcm is configured.

```rust
impl pallet_xcm::Config for Runtime {
	/* snip */
  type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	// Anyone can execute XCM messages locally.
	type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
	type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
	// Anyone is able to use teleportation regardless of who they are and what they want to teleport.
	type XcmTeleportFilter = Everything;
	// Anyone is able to use reserve transfers regardless of who they are and what they want to
	// transfer.
	type XcmReserveTransferFilter = Everything;
	/* snip */
}
```

As we can see, there is no filter on the Exeuction, Teleporting or Reserve transferring side. Custom XCM sending is also allowed.

---

## ⚙️ Statemine Xcm Config
Statemine is a common-good parachain that allows hosting arbitrary assets.

You can visit the whole xcm configuration [here](https://github.com/paritytech/cumulus/blob/master/parachains/runtimes/assets/statemine/src/xcm_config.rs)

---
### Statemine Asset Transactors
Statemine has **two asset transactors**

**Currency Asset Transactor**
```rust
parameter_types! {

  pub const KsmLocation:  MultiLocation = MultiLocation::parent();
}

/// Means for transacting the native currency on this chain.
pub type CurrencyTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<KsmLocation>,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports of `Balances`.
	(),
>;
```
Notice how KsmLocation is equal to **Parent**. Everytime we receive a token with the parent multilocation, we mint in Balances.

**Fungibles Asset Transactor**
```rust
/// Means for transacting assets besides the native currency on this chain.
pub type FungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation:
	Assets,
	// Use this currency when it is a fungible asset matching the given location or name:
	ConvertedConcreteAssetId<
		AssetId,
		Balance,
		AsPrefixedGeneralIndex<AssetsPalletLocation, AssetId, JustTry>,
		JustTry,
	>,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We only want to allow teleports of known assets. We use non-zero issuance as an indication
	// that this asset is known.
	parachains_common::impls::NonZeroIssuance<AccountId, Assets>,
	// The account to use for tracking teleports.
	CheckingAccount,
>;
````
FungiblesTransactor refers to the way in which assets created in Statemine are Withdrawn/Deposited in the xcm-executor. It is critical that these assets are sendable to other chains!

---
### Statemine Trusted Teleporters
Only alowed if the token multilocation matches the origin

```rust
pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsTeleporter = NativeAsset;
}
```
---
### Statemine Barriers
Similar to Polkadot, but unpaid execution is allowed from the relay chain

```rust
pub type Barrier = DenyThenTry<
	DenyReserveTransferToRelayChain,
	(
		TakeWeightCredit,
		AllowTopLevelPaidExecutionFrom<Everything>,
		// Parent and its exec plurality get free execution
		AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
		// Expected responses are OK.
		AllowKnownQueryResponses<PolkadotXcm>,
		// Subscriptions for version tracking are OK.
		AllowSubscriptionsFrom<ParentOrSiblings>,
	),
>;
```
---

# 🧐 Debugging XCM message failures
Involves knowledge of the chain XCM configuration!:

Common steps to debug:

1. Identify what the error means. This will help you identify the context in which the error happened
2. Retrieve the received XCM message.
3. Check the chain XCM configuration to verify what could have failed
---

## ⚠️ Identifying the error kind
Look at the `ump.ExecutedUpward` event:

<widget-columns>
<widget-column>
<img style="width: 500px;" src="../../../assets/img/7-XCM/failed-ump.png" alt="Ump failure"/>
</widget-column>
<widget-column>

Some common errors are:
- `UntrustedReserveLocation`: a `ReserveAssetDeposited` was received from a location we don't trust as reserve
- `UntrustedTeleportLocation`: a `ReceiveTeleportedAsset` was received from a location we don't trust as teleporter.
- `AssetNotFound`: the asset to be withdrawn/deposited is not handled by the runtime. Usually happens when the multilocation representing an asset does not match to those handled by the chain.
- `FailedToTransactAsset`: the withdraw/deposit of the asset cannot be done, typically its because the account does not hold such asset, or because we cannot convert the multilocation to an account.
- `FailedToDecode`: tied to the `Transact` instruction, in whcih the byte-blob representing the dispatchable cannot be decoded.
- `MaxWeightInvalid`: the weiht specified in the `Transact` instruction is not sufficient to cover for the weight of the transaction.
- `TooExpensive`: Typically tied to `BuyExecution`, means that the amount of assets used to pay for fee is non-sufficient.
- `Barrier`: One of the barriers failed, we need to check the barriers individually.

---
## 🔨 Decoding scale-encoded messages

The second step is to retrieve the xcm message received by the chain. We can make clear distinctions on this:
- **RelayChain**: usually the xcm message can be retrieved in the `paraInherent.enter` inherent, where the candidate for a specific parachain contains the ump messages sent to the relay. **UMP messages are usually executed one block after they are received**
- **Parachain**: usually the xcm message can be retrieved in the `parachainSystem.setValidationData` inherent, inside the field `downWardMessage` or `horizontalMessages`. **DMP and HRPM messages are usually executed in the block they are received**, at least, as long as the available weight permits.

One of the main drawbacks is that all we see is a **scale-encoded message** which does not give us much information. To cope with this:

- We build a scale-decoder to retrieve the xcm message (the hard way).
- We rely on subscan/polkaholic to see the XCM message received.

Guess which one will try out? :)