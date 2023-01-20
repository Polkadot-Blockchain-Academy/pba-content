---
title: XCM in the Polkadot # Also update the h1 header on the first slide to the same name
description: XCM in the Polkadot Context for web3 Engineers
duration: 1 hour
---

# XCM in Polkadot

---

## _At the end of this lecture, you will be able to:_

<pba-flex center>

- Understand the configuration of the Rococo chain
- Send real-world messages between parachain A <-> Rococo
- Identify potential errors on XCM messages

---

## 🤔 Considerations

- There should be no trust assumption between chains unless explicitly requested.
- We cannot assume chains will not act maliciously
- Spamming XCM messages creates a DoS problem

---

## 🛠️ Rococo Configuration

- Barriers
- Teleport filtering
- Fee payment
- Proper XCM Instruction Weighting

Notes:

From now on, we will use the Rococo runtime as a reference.
Rococo is a testnet for
Polkadot and Kusama that we will use in to test our XCM messages.
Most of the Rococo configuration
is identical to that in Polkadot.

---

## 🚧 XCM `Barrier` in Rococo

```rust
/// The barriers one of which must be passed for an XCM message to be executed.
pub type Barrier = (
  // Weight that is paid for may be consumed.
  TakeWeightCredit,
  // If the message is one that immediately attempts to pay for execution, then allow it.
  AllowTopLevelPaidExecutionFrom<Everything>,
  // Messages coming from system parachains need not pay for execution.
  AllowUnpaidExecutionFrom<IsChildSystemParachain<ParaId>>,
  // Expected responses are OK.
  AllowKnownQueryResponses<XcmPallet>,
  // Subscriptions for version tracking are OK.
  AllowSubscriptionsFrom<Everything>,
);
```

---v

## 🚧 XCM `Barrier in Rococo

- `TakeWeightCredit` and `AllowTopLevelPaidExecutionFrom` are used to prevent spamming for local/remote XCM execution.
- `AllowUnpaidExecutionFrom` lets a system parachain have free execution in the relay.
- `AllowKnownQueryResponses` and `AllowSubscriptionsFrom`, as we know already, are mostly used for versioning.

Notes:

- Child system parachains are parachains that contain core polkadot features, and usually they get a paraId of less than 1000. They are allocated by Polkadot governance and get free execution.
- `AllowKnownQueryResponses` will check pallet-xcm storage to know whether the response is expected.
-`AllowSubscriptionsFrom` determines that any origin is able to subscribe for version changes.

---

## 🤝 Trusted teleporters in Rococo

```rust [0|2|6-8|18-22]
parameter_types! {
  pub const RocLocation: MultiLocation = Here.into();
  pub const Rococo: MultiAssetFilter =
    Wild(AllOf { fun: WildFungible, id: Concrete(RocLocation::get()) });

  pub const Statemine: MultiLocation = Parachain(1000).into();
  pub const Contracts: MultiLocation = Parachain(1002).into();
  pub const Encointer: MultiLocation = Parachain(1003).into();

  pub const RococoForStatemine: (MultiAssetFilter, MultiLocation) =
    (Rococo::get(), Statemine::get());
  pub const RococoForContracts: (MultiAssetFilter, MultiLocation) =
    (Rococo::get(), Contracts::get());
  pub const RococoForEncointer: (MultiAssetFilter, MultiLocation) =
    (Rococo::get(), Encointer::get());
}

pub type TrustedTeleporters = (
  xcm_builder::Case<RococoForStatemine>,
  xcm_builder::Case<RococoForContracts>,
  xcm_builder::Case<RococoForEncointer>,
);
```

---v

## 🤝 Trusted teleporters in Rococo

- Teleporting involves trust between chains.
- 1000 (Statemint) and 1001 (Contracts) and 1002 (Encointer) are allowed to teleport tokens represented by the **Here**
- **Here** represents the relay token

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsTeleporter = TrustedTeleporters;
  /* snip */
}
```

Notes:

- Statemine, Rococo and Encointer are able to teleport the relay chain token
- Any other chain sending a `ReceiveTeleportedAsset` or any other token being teleported will be rejected with `UntrustedTeleportLocation`.

---

## 💱Trusted reserves in Rococo

- Rococo does not recognize any chain as reserve
- Rococo prevents reception of any **ReserveAssetDeposited** message

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsReserve = ();
  /* snip */
}
```

Notes:

- The relay chain only handles a single asset (the relay token), and therefore, it does not have the necessity to trust any other tokens being sent from other reserves.

---

## 📁 `LocationToAccountId` in Rococo

- Conversion between a multilocation to an AccountId is a key component to withdraw/deposit assets and issue `Transact` operations.
- Parachain origins will be converted to their corresponding sovereign account.
- Local 32 byte origins will be converted to a 32 byte defined AccountId.

```rust
pub type LocationConverter = (
  // We can convert a child parachain using the standard `AccountId` conversion.
  ChildParachainConvertsVia<ParaId, AccountId>,
  // We can directly alias an `AccountId32` into a local account.
  AccountId32Aliases<RococoNetwork, AccountId>,
);
```

Notes:

- Any other origin that is not a parachain origin or a local 32 byte account origin will not be convertible to an accountId.
- Question what happens if a message coming from a parachain  starts with `DescendOrigin`?.
---

## 🪙 Asset Transactors in Rococo

<div style="font-size: smaller">

```rust
pub type LocalAssetTransactor = XcmCurrencyAdapter<
  // Use this currency:
  Balances,
  // Use this currency when it is a fungible asset
  // matching the given location or name:
  IsConcrete<RocLocation>,
  // We can convert the MultiLocations
  // with our converter above:
  LocationConverter,
  // Our chain's account ID type
  // (we can't get away without mentioning it explicitly):
  AccountId,
  // It's a native asset so we keep track of the teleports
  // to maintain total issuance.
  CheckAccount,
>;

impl xcm_executor::Config for XcmConfig {
  /* snip */
  type AssetTransactor = LocalAssetTransactor;
  /* snip */
}
```

---v

## 🪙 `asset-transactors` in Rococo

- Single asset-transactor in Rococo
- Asset-transactor is matching the **Here** multilocation id to the Currency defined in **Balances**, which refers to **_pallet-balances_**
- Essentially, this is configuring XCM such that the native token (DOT) is associated with the multilocation **Here**.

Notes:

- Rococo is tracking teleports in the **CheckAccount**, which is defined in **palletXcm**. This aims at maintaining the total issuance even if assets have been teleported to another chain.

---

## 📍`origin-converter` in Rococo

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

---v

## 📍`origin-converter` in Rococo

- Defines ways in which we can convert a multilocation to a dispatch origin, typically used by the `Transact` instruction:
- Child parachain origins are converted to signed origins through **_LocationConverter_** (`OriginKind == Sovereign`).
- Child parachains can also be converted to native parachain origins (`OriginKind == Native`).
- Local 32 byte origins are converted to signed 32 byte origins (`OriginKind == Native`).

Notes:

- There exists the concept of a "parachain dispatch origin" which is used for very specific functions (like, e.g., opening a channel with another chain). This gets checked with the _ensure_parachain!_ macro.
- System parachains are able to dispatch as root origins, as they can bee seen as an extension to the rococo runtime itself.

---

## 🔧 `WeightTrader` in Rococo

- Weight is converted to fee with the **_WeightToFee_** type.
- The asset in which we charge for fee is **_RocLocation_**.
  This means we can only pay for xcm execution in the **native currency**
- Fees will go to the block author thanks to **_ToAuthor_**

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type Trader = UsingComponents<
    WeightToFee,
	RocLocation,
	AccountId,
	Balances,
	ToAuthor<Runtime>>;
  /* snip */
}
```

Notes:

- Trying to buyExecution with any other token that is not the native token **will fail**.
- **_WeightToFee_** will determine whether the assets in the holding register are sufficient to buy the necessary amount of weight. 

---

## 🎨 `XcmPallet` in Rococo

```rust
impl pallet_xcm::Config for Runtime {
  /* snip */
  type XcmRouter = XcmRouter;
  type SendXcmOrigin =
    xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
  // Anyone can execute XCM messages locally.
  type ExecuteXcmOrigin =
    xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
  type XcmExecuteFilter = Everything;
  type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
  // Anyone is able to use teleportation
  // regardless of who they are and what they want to teleport.
  type XcmTeleportFilter = Everything;
  // Anyone is able to use reserve transfers
  // regardless of who they are and what they want to transfer.
  type XcmReserveTransferFilter = Everything;
  /* snip */
}
```

---v

## 🎨 XcmPallet in Rococo

- No filter on messages for Execution, Teleporting or Reserve transferring.
- Only origins defined by **_LocalOriginToLocation_** are allowed to send/execute arbitrary messages.
- **_LocalOriginToLocation_** defined to allow council and regular account 32 byte signed origin calls

```rust
pub type LocalOriginToLocation = (
  // We allow an origin from the Collective pallet to be used in XCM
  // as a corresponding Plurality of the `Unit` body.
  CouncilToPlurality,
  // And a usual Signed origin to be used in XCM as a corresponding AccountId32
  SignedToAccountId32<RuntimeOrigin, AccountId, RococoNetwork>,
);
```

Notes:

- **_LocalOrigin_** allows to go from a frame dispatch origin to a multilocation. This is necessary because **we enter the xcm-executor with xcm origins, not with frame dispatch origins**. Note that this is an extrinsic in a frame pallet, and thus, **we call it with frame origins**.

- Council decisions are converted to `Plurality` junction multilocations.

- Signed origins are converted to `AccountId32` junction multilocations.

---

## 🛠️ Statemine Xcm Config

Statemine is a common-good parachain that allows hosting arbitrary assets.

You can visit the whole xcm configuration [here](https://github.com/paritytech/cumulus/blob/master/parachains/runtimes/assets/statemine/src/xcm_config.rs)

---

## 🪙 Statemine Asset Transactors

```rust
parameter_types! {
  pub const KsmLocation:  MultiLocation = MultiLocation::parent();
}

/// Means for transacting the native currency on this chain.
pub type CurrencyTransactor = CurrencyAdapter<
  // Use this currency:
  Balances,
  // Use this currency when it is a fungible asset
  // matching the given location or name:
  IsConcrete<KsmLocation>,
  // Convert an XCM MultiLocation into a local account id:
  LocationToAccountId,
  // Our chain's account ID type
  // (we can't get away without mentioning it explicitly):
  AccountId,
  // We don't track any teleports of `Balances`.
  (),
>;
```

Notes:

- From Statemine's point of view, the relay token is represented as **_MultiLocation { parents:1, interior: Here }_**.
- Teleports are minted into the **_Balances_** pallet.
- Teleports are not tracked in any checking account.

---v

## 🪙 Statemine Asset Transactors

<div style="font-size: smaller">

```rust
/// Means for transacting assets besides the native currency on this chain.
pub type FungiblesTransactor = FungiblesAdapter<
  // Use this fungibles implementation:
  Assets,
  // Use this currency when it is a fungible asset
  // matching the given location or name:
  ConvertedConcreteAssetId<
    AssetId, Balance,
	AsPrefixedGeneralIndex<AssetsPalletLocation, AssetId, JustTry>,
	JustTry,
  >,
  // Convert an XCM MultiLocation into a local account id:
  LocationToAccountId,
  // Our chain's account ID type
  // (we can't get away without mentioning it explicitly):
  AccountId,
  // We only want to allow teleports of known assets.
  // We use non-zero issuance as an indication that this asset is known.
  parachains_common::impls::NonZeroIssuance<AccountId, Assets>,
  // The account to use for tracking teleports.
  CheckingAccount,
>;
```

</div>

Notes:

- An asset with id `Id` are represented by `X2(PalletInstance(pallet_assets_index), GeneralIndex(Id)` thanks to **_AsPrefixedGeneralIndex_**.
- Asset teleports **are** being tracked in **_CheckingAccount_**. The reason is that Statemine is the reserve chain for those assets.

---v

## 🪙 Statemine Asset Transactors

- **FungiblesTransactor** refers to the way in which assets created in Statemine are Withdrawn/Deposited in the xcm-executor.
- It is critical that Statemine assets are sendable to other chains!
- **CurrencyTransactor** refers to the way in which teleports from the relay chain are handled.
- Every time we receive a token with the parent multilocation, we mint in Balances.

```rust
/// Prepended junction to Id
pub AssetsPalletLocation: MultiLocation =
		PalletInstance(<Assets as PalletInfoAccess>::index() as u8).into();

/// Means for transacting assets on this chain.
pub type AssetTransactors = (CurrencyTransactor, FungiblesTransactor);
```

---

## 🤝 Statemine Trusted Teleporters

- **NativeAsset**: Only allowed if the token multilocation matches the origin.
- This is the case for the relay token, `origin_multilocation == asset_multilocation`.

```rust
pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsTeleporter = NativeAsset;
}
```

```rust
impl FilterAssetLocation for NativeAsset {
  fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
    matches!(asset.id, Concrete(ref id) if id == origin)
  }
}
```

---

## 🚧 Statemine `Barrier`

Similar to Rococo, but unpaid execution is allowed from the relay chain

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

Notes:

- The relay chain has free execution
- `DenyThenTry<A, B>` denies the execution if A is true, else evaluates B for allowance
- `DenyReserveTransferToRelayChain` prevents sending a reserve transfer asset to the relay.

---

<!-- .slide: data-background-color="#4A2439" -->

# Debugging XCM

---

## 🧐 Debugging XCM message failures

Involves knowledge of the chain XCM configuration!

Common steps to debug:

1. Identify what the error means which will help you identify the context in which the error happened.
1. Look in the xcm codebase to check where this error might have been thrown.
   Was it thrown in the barrier? Or in any specific instruction?
1. Retrieve the failed received XCM.
1. Check the chain XCM configuration to verify what could have failed.

---

## 🕵️‍♂️ Identifying the error kind

Look at the `ump.ExecutedUpward` event:

<br>

<img rounded style="width: 800px;" src="../../../assets/img/7-XCM/failed-ump.png" alt="Ump failure"/>

---v

## 🕵️‍♂️ Identifying the error kind

- `UntrustedReserveLocation`: a `ReserveAssetDeposited` was received from a location we don't trust as reserve.
- `UntrustedTeleportLocation`: a `ReceiveTeleportedAsset` was received from a location we don't trust as teleporter.
- `AssetNotFound`: the asset to be withdrawn/deposited is not handled by the asset transactor.
  Usually happens when the multilocation representing an asset does not match to those handled by the chain.

---v

## 🕵️‍♂️ Identifying the error kind

- `FailedToTransactAsset`: the withdraw/deposit of the asset cannot be processed, typically it's because the account does not hold such asset, or because we cannot convert the multilocation to an account.
- `FailedToDecode`: tied to the `Transact` instruction, in which the byte-blob representing the dispatchable cannot be decoded.
- `MaxWeightInvalid`: the weight specified in the `Transact` instruction is not sufficient to cover for the weight of the transaction.
- `TooExpensive`: Typically tied to `BuyExecution`, means that the amount of assets used to pay for fee is insufficient.

---v

## 🕵️‍♂️ Identifying the error kind

- `Barrier`: One of the barriers failed, we need to check the barriers individually.
- `UnreachableDestination`: Arises when the supported XCM version of the destination chain is unknown.
  When the local chain sends an XCM to the destination chain for the very first time, it does not know about the XCM version of the destination.
  In such a case, the safe XCM version is used instead.
  However, if it is not set, then this error will be thrown.

---

## 🔨 Decoding SCALE-encoded messages

- **RelayChain**:
  - XCM can be retrieved in the `paraInherent.enter` inherent
  - The candidate for a specific parachain contains the ump messages sent to the relay.
  - **UMP messages are usually executed one block after they are received**
- **Parachain**:
  - XCM can be retrieved in the `parachainSystem.setValidationData` inherent.
  - **DMP and HRPM messages are usually executed in the block they are received**, at least, as long as the available weight permits.

---v

## 🔨 Decoding SCALE-encoded messages

But all we see is a **SCALE-encoded message** which does not give us much information.
To solve this:

- We build a SCALE-decoder to retrieve the xcm message (the hard way).
- We rely on subscan/polkaholic to see the XCM message received.

---v

## 🔨 Subscan XCM retrieval

<img rounded style="width: 800px;" src="../../../assets/img/7-XCM/subscan_xcm.png" alt="Subscan XCM tab"/>
