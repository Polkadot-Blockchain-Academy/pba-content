---
title: Parachain XCM Configuration # Also update the h1 header on the first slide to the same name
description: XCM configuration overview and considerations, for parachains.
duration: 1 hour
---

# Parachain XCM Configuration

---

## _At the end of this lecture, you will be able to:_

<pba-flex center>

- Understand the different XCM configurable parts for a chain
- Construct different XCM configurations for chains with different needs
- Understand how Versioning is handled across chains

</pba-flex>

---

## 🛠️ Configurables in `XcmConfig`

- Common vs configurable implementation in `xcm-executor`

- Configurable parts are defined in the `xcm-executor` config!

Notes:

EXERCISE: ask the class to raise hands and postulate on what they think should be configurable.

---

## 🛠️ Configurables in `XcmConfig`

```rust [0|1|9|11|17|19|21|23|25|27|29|31]
pub type LocationToAccountId = ?;

pub struct XcmConfig;
impl Config for XcmConfig {
  type RuntimeCall = RuntimeCall;
  // How we route the XCM outside this chain
  type XcmSender = XcmRouter;
  // How we withdraw/deposit assets
  type AssetTransactor = ?;
  // How we convert a ML to a FRAME dispatch origin
  type OriginConverter = ?;
  // Who we trust as reserve chains
  type IsReserve = ?;
  // Who do we trust as teleporters
  type IsTeleporter = ?;
  // How we invert locations
  type UniversalLocation = ?;
  // Pre-execution filters
  type Barrier = ?;
  // How we weigh a message
  type Weigher = ?;
  // How we charge for fees
  type Trader = ?;
  // How we handle responses
  type ResponseHandler = ?;
  // How we handle asset traps
  type AssetTrap = ?;
  // How we handle asset claims
  type AssetClaims = ?;
  // How we handle version subscriptions
  type SubscriptionService = ?;
}

```

Notes:

- Means of converting a multilocation into an accountId
  Used later for: OriginConverter , `AssetTransactor`

- `xcm-builder` and `xcm-pallet` are your friends!

- `xcm-builder` is a polkadot module that contains a set of pre-defined structures to be set in some of the configurable sides of XCM.

- `xcm-pallet` is a pallet that not only allows sending and executing XCM messages, but rather it also implements several of the configuration traits and thus can be used perform several XCM configuration actions.

---

## 🛠️ XcmRouter in `XcmConfig`

- `XcmRouter` defines the means of routing a XCM to a destination.
- `ParentAsUmp` routes XCM to relay chain through UMP.
- `XcmpQueue` routes XCM to other parachains through XCMP.

```rust
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);
```

Notes:

- If the destination location matches the form of `Multilocation { parents: 1, interior: Here }`, the message will be routed through UMP.
  The UMP channel is available by default.
- If the destination matches the form of `Multilocation { parents: 1, interior: X1(Parachain(para_id)) }`, the message will be routed through XCMP.
  As of today, an HRMP channel should be established before the message can be routed.

---v

## 🛠️ XcmRouter in `XcmConfig`

- `XcmRouter` will ask for implementations of the `SendXcm` trait.
- `wrap_version`, in this case, adds versioning to the message.

<div style="font-size:smaller">

```rust [6|9|12]
pub struct ParentAsUmp<T, W>(PhantomData<(T, W)>);
impl<T: UpwardMessageSender, W: WrapVersion> SendXcm for ParentAsUmp<T, W> {
	fn send_xcm(dest: impl Into<MultiLocation>, msg: Xcm<()>) -> Result<(), SendError> {
		let dest = dest.into();

		if dest.contains_parents_only(1) {
			// An upward message for the relay chain.
			let versioned_xcm =
				W::wrap_version(&dest, msg).map_err(|()| SendError::DestinationUnsupported)?;
			let data = versioned_xcm.encode();

			T::send_upward_message(data).map_err(|e| SendError::Transport(e.into()))?;

			Ok(())
		} else {
			// Anything else is unhandled. This includes a message this is meant for us.
			Err(SendError::CannotReachDestination(dest, msg))
		}
	}
}
```

<div>

---

### 🤔 Grab your chain's requirements before starting

Questions that you should have answers for:

- _Is my chain going to transfer just the native token?_
  _Is my chain going to receive several other kinds of assets?_

- _Is my chain going to allow free execution?_
  _Maybe only limited to some parachains/relay chain?_

- _Is my chain a 20 byte account chain?_
  _a 32 byte account chain?_

- _How will my chain accept fee payment?_
  _In one asset?_
  _In several?_

Notes:

- Some of the answers to these questions might imply you need to use your own custom primitives.

---

### Our starting example setup requirements

1. Parachain that does not charge for relay incoming messages.
1. Parachain that trusts the relay as the reserve chain for the relay chain tokens.
1. Parachain that mints in `pallet-balances` when it receives relay chain tokens.
1. Parachain that uses 32 byte accounts.
1. Users can execute XCMs locally.

---

### 🤝 `IsReserve` and `IsTeleporter`

- They define filters for accepting `ReserveAssetDeposited` and `ReceiveTeleportedAsset` respectively.
- Filters are applied for specific `MultiAsset-MultiLocation` pairs.

```rust
/// Combinations of (Asset, Location) pairs which we trust as reserves.
type IsReserve: ContainsPair<MultiAsset, MultiLocation>;

/// Combinations of (Asset, Location) pairs which we trust as teleporters.
type IsTeleporter: ContainsPair<MultiAsset, MultiLocation>;
```

Notes:

- For our test excercise, it is sufficient to set this `IsReserve` to `Everything`.
- In your production network, you will need to match these values to your reserve/teleporting trust assumptions.

---

### 📁 `LocationToAccountId` via `xcm-builder`

- Defines how we convert a multilocation into a local accountId.
- Useful when we want to withdraw/deposit tokens from a multilocation defined origin
- Useful when we want to dispatch as signed origins from a multilocation defined origin.

<img src="../../../assets/img/7-XCM/location_to_account_id_withdraw.svg" alt="Withdraw location to accountId" style="width: 900px;">

Notes:

- This will define how we convert a multilocation into a local accountId.
- This is useful when we want to withdraw/deposit tokens from a multilocation defined origin or when we want to dispatch as signed origins from a multilocation defined origin.

---v

### 📁 `LocationToAccountId` via `xcm-builder`

- `Account32Hash`: Hashes the multilocation and takes the lowest 32 bytes as account.

```rust
fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<AccountId, ()> {
  // Blake2(b"multiloc"+ multilocation)
  Ok(("multiloc", location.borrow()).using_encoded(blake2_256).into())
}
```

<div>

Notes:

- This is the most generic form of converting a multilocation to an accountId.
- There are no restrictions in the multilocation input.
  If you use this with other converters, make sure this will be the last option, as otherwise the more restrictive ones will not apply.

---v

### 📁 `LocationToAccountId` via `xcm-builder`

- `ParentIsPreset`: Converts the parent multilocation into an account of the form `b'Parent' + trailing 0s`

```rust
fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<AccountId, ()> {
  if location.borrow().contains_parents_only(1) {
    Ok(b"Parent"
      .using_encoded(|b| AccountId::decode(&mut TrailingZeroInput::new(b)))
      .expect("infinite length input; no invalid inputs for type; qed"))
  } else {
    Err(())
  }
}
```

Notes:

- This converter is typically used in parachains to make sure the parent origin has an associated account.

---v

### 📁 `LocationToAccountId` via `xcm-builder`

- `ChildParachainConvertsVia`: Converts the **child** parachain multilocation into an account of the form `b'para' + para_id_as_u32 + trailing 0s`

Notes:

- Here child means a parachain from the relay's perspective

- This converter is **typically used in the relay chain** to make sure the child parachain origins have an associated account.

---v

### 📁 `LocationToAccountId` via `xcm-builder`

- `SiblingParachainConvertsVia`: Convert the **sibling** parachain multilocation into an account of the form `b'sibl' + para_id_as_u32 + trailing 0s`

```rust
fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<AccountId, ()> {
  match location.borrow() {
    MultiLocation { parents: 1, interior: X1(Parachain(id)) } =>
      Ok(ParaId::from(*id).into_account_truncating()),
    _ => Err(()),
  }
}
```

Notes:

- Here sibling means a parachain from another parachain's perspective
- This converter is **typically used in parachains** to make sure the sibling parachain origins have an associated account.

---v

### 📁 `LocationToAccountId` via `xcm-builder`

- `AccountId32Aliases`: Converts a local `AccountId32` multilocation into a `AccountID` of 32 bytes.

<div style="font-size:smaller">

```rust
pub struct AccountId32Aliases<Network, AccountId>(PhantomData<(Network, AccountId)>);
impl<Network: Get<NetworkId>, AccountId: From<[u8; 32]> + Into<[u8; 32]> + Clone>
  Convert<MultiLocation, AccountId> for AccountId32Aliases<Network, AccountId>
{
  fn convert(location: MultiLocation) -> Result<AccountId, MultiLocation> {
    // Converts if networkId matches Any or <Network>
    let id = match location {
      MultiLocation {
        parents: 0,
        interior: X1(AccountId32 { id, network: NetworkId::Any }),
      } => id,
      MultiLocation { parents: 0, interior: X1(AccountId32 { id, network }) }
        if network == Network::get() =>
        id,
      _ => return Err(location),
    };
    Ok(id.into())
  }
  /* snip */
}
```

<div>

Notes:

- Typically used for chains that want to enable local xcm execution, and which have 32 byte accounts.
- We have a requirement of users being able to execute local XCM, and as such we need to be able to Withdraw/Deposit from their accounts
  **This structure fulfills one of our requirements**

---

### 🪙 `asset-transactors` via `xcm-builder`

- Define how we are going to withdraw and deposit assets
- Heavily dependant on the assets we want our chain to transfer

<img src="../../../assets/img/7-XCM/asset_transactor_withdraw.svg" alt="Withdraw location to accountId" style="width: 900px;">

Notes:

- The relay chain is a clear example of a chain that handles a **single token**.
- Statemine on the contrary acts as an asset-reserve chain, and it needs to handle **several assets**

---v

### 🪙 `asset-transactors` via `xcm-builder`

- `CurrencyAdapter`: Single currency `asset-transactor`.
  Used for withdrawing/depositing the native token of the chain.

<div style="font-size:smaller">

```rust
impl
 TransactAsset
  for CurrencyAdapter<Currency, Matcher, AccountIdConverter, AccountId, CheckedAccount>
{
  /* snip */
  fn deposit_asset(what: &MultiAsset, who: &MultiLocation) -> Result {
    // Check we handle this asset.
    let amount: u128 =
      Matcher::matches_fungible(&what).ok_or(Error::AssetNotFound)?.saturated_into();
    // Convert multilocation to accountId
    let who =
      AccountIdConverter::convert_ref(who).map_err(|()| Error::AccountIdConversionFailed)?;
    // Convert amount to balance
    let balance_amount =
      amount.try_into().map_err(|_| Error::AmountToBalanceConversionFailed)?;
    // Deposit currency on the account
    let _imbalance = Currency::deposit_creating(&who, balance_amount);
    Ok(())
  }
}
```

</div>

Notes:

- **Matcher**: Matches the multiAsset against some filters and returns the amount to be deposited/withdrawn
- **AccountIdConverter**: Means of converting a multilocation into an account

---v

### 🪙 `asset-transactors` via `xcm-builder`

- `FungiblesAdapter`: Used for depositing/withdrawing from a set of defined fungible tokens.
  An example of these would be `pallet-assets` tokens.

Notes:

- For our example, it suffices to uses `CurrencyAdapter`, as all we are going to do is mint in a single currency (Balances) whenever we receive the relay token.

---

### 📍 `origin-converter` via `xcm-builder`

- Defines how to convert an XCM origin, defined by a MultiLocation, into a frame dispatch origin.
- Used mainly in the `Transact` instruction.

Notes:

- `Transact` needs to dispatch from a frame dispatch origin.
  However the xcm-executor works with xcm-origins which are defined by MultiLocations.
- `origin-converter` is the component that converts one into the other

---v

### 📍 `origin-converter` via `xcm-builder`

- `SovereignSignedViaLocation`: Converts the multilocation origin (typically, a parachain origin) into a signed origin.

```rust [0|6|18|20|22]
pub struct SovereignSignedViaLocation<LocationConverter, RuntimeOrigin>(
  PhantomData<(LocationConverter, RuntimeOrigin)>,
);
impl<
    // Converts a multilocation into account
    LocationConverter: Convert<MultiLocation, RuntimeOrigin::AccountId>,
    RuntimeOrigin: OriginTrait,
  > ConvertOrigin<RuntimeOrigin>
  for SovereignSignedViaLocation<LocationConverter, RuntimeOrigin>
where
  RuntimeOrigin::AccountId: Clone,
{
  fn convert_origin(
    origin: impl Into<MultiLocation>,
    kind: OriginKind,
  ) -> Result<RuntimeOrigin, MultiLocation> {
    let origin = origin.into();
    if let OriginKind::SovereignAccount = kind {
      // Convert multilocation to account
      let location = LocationConverter::convert(origin)?;
      // Return signed origin using the account
      Ok(RuntimeOrigin::signed(location).into())
    } else {
      Err(origin)
    }
  }
}
```

Notes:

- `LocationConverter ` once again defines how to convert a multilocation into an accountId.
- It basically grants access to dispatch as Signed origin after the conversion.

---v

### 📍 `origin-converter` via `xcm-builder`

- `SignedAccountId32AsNative`: Converts a local 32 byte account multilocation into a signed origin using the same 32 byte account.

```rust [0|18|19-22|24]
pub struct SignedAccountId32AsNative<
  Network,
  RuntimeOrigin
>(PhantomData<(Network, RuntimeOrigin)>);
impl<Network: Get<NetworkId>, RuntimeOrigin: OriginTrait>
  ConvertOrigin<RuntimeOrigin>
for SignedAccountId32AsNative<Network, RuntimeOrigin>
where
  RuntimeOrigin::AccountId: From<[u8; 32]>,
{
  fn convert_origin(
    origin: impl Into<MultiLocation>,
    kind: OriginKind,
  ) -> Result<RuntimeOrigin, MultiLocation> {
    let origin = origin.into();
    match (kind, origin) {
      (
        OriginKind::Native,
        MultiLocation {
		  parents: 0,
		  interior: X1(Junction::AccountId32 { id, network })
		},
      ) if matches!(network, NetworkId::Any) || network == Network::get() =>
        Ok(RuntimeOrigin::signed(id.into())),
      (_, origin) => Err(origin),
    }
  }
}
```

Notes:

- Matches a local accountId32 multilocation to a signed origin.
- Note the difference `OriginKind` filter: this is not an account controlled by another consensus system, but rather a Native dispatch.
- **This structure fulfills one of our requirements**

---v

### 📍 `origin-converter` via `xcm-builder`

- `ParentAsSuperuser`: Converts the parent origin into the root origin.

- `SignedAccountKey20AsNative`: Converts a local 20 byte account multilocation into a signed origin using the same 20 byte account.

Notes:

- `ParentAsSuperuser` can be used in common-good chains as they do not have a local root origin and instead allow the relay chain root origin to act as the root origin.

---

### 🚧 `Barrier` via `xcm-builder`

- Barriers specify whether or not an XCM is allowed to be executed on the local consensus system.
- They are checked before the actual xcm instruction execution

Notes:

- Barriers should not involve any heavy computation.
  **At the point at which barriers are checked nothing has yet been paid for its execution**.

---v

### 🚧 `Barrier` via `xcm-builder`

Physical vs Computed origin

- Physical origin: the consensus system that built this particular XCM and sent it to the recipient
- Computed origin: the entity that ultimately instructed the consensus system to build the XCM

<img src="../../../assets/img/7-XCM/physical_vs_computed_origin.svg" alt="Physical vs Computed origin">

**Must make sure which origin a barrier should apply to!**

Those that filter on the origin (e.g. `AllowTopLevelPaidExecutionFrom<T>`) would most likely be operating upon the _computed origin_.

---v

### 🚧 `Barrier` via `xcm-builder`

Barriers that operate upon **computed origins** must be put inside of `WithComputedOrigin`:

```rust
pub struct WithComputedOrigin<InnerBarrier, LocalUniversal, MaxPrefixes>(
  PhantomData<(InnerBarrier, LocalUniversal, MaxPrefixes)>,
);
```

---v

### 🚧 `Barrier` via `xcm-builder`

- `TakeWeightCredit`: Subtracts the maximum weight the message can consume from the available weight credit.
  Usually configured for local `xcm-execution`

- `AllowTopLevelPaidExecutionFrom<T>`: For origins contained in `T`, it makes sure the first instruction puts asset into the holding register (`TeleportAsset`, `WithdrawAsset`, `ClaimAsset`, `ReserveAssetDeposit`), followed by a `BuyExecution` instruction capable of buying sufficient weight.
  **Critical to avoid free DOS**.

Notes:

- A chain without `AllowTopLevelPaidExecutionFrom` could potentially receive several heavy-computation instructions without paying for it.
  Checking that the first instructions are indeed paying for execution helps to quick-discard them.

- While `BuyExecution` is crucial for messages coming from other consensus systems, local XCM execution fees are paid as any other substrate extrinsic.

---v

### 🚧 `Barrier` via `xcm-builder`

- `AllowUnpaidExecutionFrom<T>`: Allows free execution if `origin` is contained in `T`.
  Useful for chains that "trust" each other (e.g., Statemine or any system parachain with the relay)

```rust
/// Allows execution from any origin that is contained in `T`
/// (i.e. `T::Contains(origin)`) without any payments.
/// Use only for executions from trusted origin groups.
pub struct AllowUnpaidExecutionFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for AllowUnpaidExecutionFrom<T> {
  fn should_execute<RuntimeCall>(
    origin: &MultiLocation,
    _message: &mut Xcm<RuntimeCall>,
    _max_weight: Weight,
    _weight_credit: &mut Weight,
  ) -> Result<(), ()> {
    ensure!(T::contains(origin), ());
    Ok(())
  }
}
```

Notes:

- **This fulfills our requirements**
- To meet our example use case, we only need the relay to have free execution.

- XCMv3 adds a new `AllowExplicitUnpaidExecutionFrom`, which not only checks the origin but also that the **first instruction is the new `UnpaidExecution` instruction**.

---v

### 🚧 `Barrier` via `xcm-builder`

- `AllowKnownQueryResponses`: Allows the execution of the message if it is a `QueryResponse` message and the `ResponseHandler` is expecting such response

- `AllowSubscriptionsFrom<T>`: If the `origin` that sent the message is contained in `T`, it allows the execution of the message if it contains only a `SubscribeVersion` or `UnsubscribeVersion` instruction.

---

### 🏋️ `Weigher` via `xcm-builder`

- Specifies how instructions are weighed
- `FixedWeightInfoBounds`: Apply a constant weight value to all instructions except for `Transact`, `SetErrorHandler` and `SetAppendix`.
- `WeightInfoBounds`: Apply instruction-specific weight (ideally, benchmarked values) except for `Transact`, `SetErrorHandler` and `SetAppendix`.

Notes:

Benchmarking can easily be done with the `pallet-xcm-benchmarks` module.
Note that the benchmarks need to reflect what your runtime is doing, so fetching the weights done for another runtime can potentially turn into users abusing your system.

---v

### 🏋️ `Weigher` via `xcm-builder`

- `Transact` weight is defined by `require_weight_at_most` value.
- `SetErrorHandler` and `SetAppendix`, besides their own weight, need to account for the XCM instructions they will execute.

<div style="font-size:smaller">

```rust [0|6|7
  fn instr_weight_with_limit(
		instruction: &Instruction<C>,
		instrs_limit: &mut u32,
	) -> Result<Weight, ()> {
		use xcm::GetWeight;
		let instr_weight = match instruction {
			Transact { require_weight_at_most, .. } => *require_weight_at_most,
			SetErrorHandler(xcm) | SetAppendix(xcm) => Self::weight_with_limit(xcm, instrs_limit)?,
			_ => Weight::zero(),
		};
		instruction.weight().checked_add(&instr_weight).ok_or(())
	}
```

<div>

---

### 🔧 `WeightTrader` via `xcm-builder`

- Specifies how to charge for weight inside the xcm execution.
- Used in the `BuyExecution` instruction
- Used in the `RefundSurplus` instruction

---v

### 🔧 `WeightTrader` via `xcm-builder`

- `FixedRateOfFungible`: Converts weight to fee at a fixed rate and charges in a specific fungible asset

```rust [0|7|17|19-21|28-30]|34
pub struct FixedRateOfFungible<T: Get<(AssetId, u128)>, R: TakeRevenue>(
  Weight,
  u128,
  PhantomData<(T, R)>,
);
impl<
  T: Get<(AssetId, u128)>,
  R: TakeRevenue
> WeightTrader for FixedRateOfFungible<T, R> {
  /* snip */
  fn buy_weight(
	&mut self,
	weight: Weight,
	payment: Assets
  ) -> Result<Assets, XcmError> {
    // get the assetId and amount per second to charge
    let (id, units_per_second) = T::get();
    // Calculate the amount to charge for the weight bought
    let amount =
	  units_per_second * (weight as u128)
	  / (WEIGHT_REF_TIME_PER_SECOND as u128);

    if amount == 0 {
      return Ok(payment)
    }

    // Take amount from payment
    let unused =
      payment.checked_sub((id, amount).into())
	  .map_err(|_| XcmError::TooExpensive)?;

    self.0 = self.0.saturating_add(weight);
    self.1 = self.1.saturating_add(amount);
    Ok(unused)
  }
  /* snip */
}
```

Notes:

- It is crucial that we return the unused portion of the tokens, as these need to be refunded back in to the holding register.
- We keep how much it has been bought to be able to refund later on.

---v

### 🔧 `WeightTrader` via `xcm-builder`

- `UsingComponents`: uses `TransactionPayment` pallet to set the right price for weight.

Notes:

- `TransactionPayment` pallet already defines how to convert weight to fee.
  We do not need to define a rate in this case.

---

<!-- .slide: data-background-color="#4A2439" -->

# Debugging XCM

---

## 🧐 Debugging XCM message failures

Involves knowledge of the chain XCM configuration!

Common steps to debug:

1. Identify what the error means which will help you identify the context in which the error happened.
1. Look in the xcm codebase to check where this error might have been thrown.
   Was it thrown in the barrier?
   Or in any specific instruction?
1. Retrieve the failed received XCM.
1. Check the chain XCM configuration to verify what could have failed.

---

## 🕵️‍♂️ Identifying the error kind

Look at the `ump.ExecutedUpward` event:

<img rounded style="width: 800px;" src="../../../assets/img/7-XCM/failed-ump.png" alt="Ump failure" />

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

<img rounded style="width: 800px;" src="../../../assets/img/7-XCM/subscan_xcm.png" alt="Subscan XCM tab" />
