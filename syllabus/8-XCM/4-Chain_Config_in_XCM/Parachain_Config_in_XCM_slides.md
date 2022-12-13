---
title: Parachain Configuration for XCM # Also update the h1 header on the first slide to the same name
description: Describe your slides here
duration: 1 hour
instructors: ["Keith Yeung", "Gorka Irazoqui"]
teaching-assistants: ["Andrew Burger", "Hector Bulgarini"]
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
Common vs configurable implementation in xcm-executor

Configurable parts are defined in the xcm-executor config!

```rust

// Means of converting a multilocation into an accountId
// Used later for:
//  - OriginConverter
//  - AssetTransactor
pub type LocationToAccountId = ?;

pub struct XcmConfig;
impl Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	// How we route the XCM outside this chain
	type XcmSender = XcmRouter;
	// How we withdraw/deposit assets
	type AssetTransactor = ?;
	// How we convert a ML to a dispatch origin
	type OriginConverter = ?;
	// What do we trust as reserve chains
	type IsReserve = Everything;
	// What do we trust as teleporters
	type IsTeleporter = Nothing;
	// How do we reanchor
	type LocationInverter = LocationInverter<Ancestry>;
	// Pre-execution filters
	type Barrier = ?;
	// How do we weight a message
	type Weigher = ?;
	// How do we charge for fees
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

### Grab your chains requirements before starting

Things you should question is:

- *Is my chain going to transfer just the native token? Is my chain going to receive several assets?*

- *Is my chain going to allow the free execution? Maybe for some parachains/relay chain?*

- *Is my chain a 20 byte account chain? a 32 byte account chain?*

- *How will my chain accept fee payment? In one asset? In several?*

### Our starting example setup requirements
1. Parachain that does not charge for relay incoming messages.
2. Parachain that trusts the relay as the reserve chain for the relay token
3. Parachain that mints in pallet-balances when it receives relay chain tokens.
4. Parachain that uses 32 byte accounts and that makes it possible for users to execute local XCM.
---

### Configuring LocationToAccountId
This will define how we convert a multilocation into a local accountId. This is useful when we want to withdraw/deposit tokens from a multilocation-defined origin or when we want to dispatch as signed origins from a multilocation-defined origin.

Xcm-builder allows us to configure LocationToAccountId conversions in an easy manner. Let's look at our options:

1. `Account32Hash`: The most generic locationToAccountIdConverter. It basically hashes the multilocation and takes the lowest 32 bytes to make a 32 byte account.
```rust
fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<AccountId, ()> {
		Ok(("multiloc", location.borrow()).using_encoded(blake2_256).into())
	}
```

2. `ParentIsPreset`: A structure that allows to convert only the parent multilocation into an account of the form `b'Parent' + trailing 0s`

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

2. `ChildParachainConvertsVia`: A structure that allows to convert the child parachain multilocation into an account of the form `b'para' + para_id_as_u32 + trailing 0s`

3. `SiblingParachainConvertsVia`: A structure that allows to convert the sibling parachain multilocation into an account of the form `b'sibl' + para_id_as_u32 + trailing 0s`

```rust
fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<AccountId, ()> {
		match location.borrow() {
			MultiLocation { parents: 1, interior: X1(Parachain(id)) } =>
				Ok(ParaId::from(*id).into_account_truncating()),
			_ => Err(()),
		}
	}
```

4. `AccountId32Aliases`: A structure that allows to convert a local AccountId32 multilocation into a accountId of 32 bytes.
```rust
fn convert(location: MultiLocation) -> Result<AccountId, MultiLocation> {
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
```

5. `AccountId20Aliases`: A structure that allows to convert a local AccountKey20 multilocation into a accountId of 20 bytes.

For our example, we only need the 4th. We have a requirement of users being able to execute local XCM, and as such we need to be able to Withdraw/Deposit from their accounts

---

### Configuring asset-transactors with xcm-builder
Asset transactors define how we are going to withdraw and deposit assets. What we set in here depends heavily on what do we want our chain to be able to transfer:

1. `CurrencyAdapter`: A simple adapter that uses a single currency as the assetTransactor. This is usually used for withdrawing/depositing the native token of the chain.

```rust
pub struct CurrencyAdapter<Currency, Matcher, AccountIdConverter, AccountId, CheckedAccount>(
	PhantomData<(Currency, Matcher, AccountIdConverter, AccountId, CheckedAccount)>,
);
impl
 TransactAsset
	for CurrencyAdapter<Currency, Matcher, AccountIdConverter, AccountId, CheckedAccount>
{
	/* snip */
	fn deposit_asset(what: &MultiAsset, who: &MultiLocation) -> Result {
		// Check we handle this asset.
		let amount: u128 =
			Matcher::matches_fungible(&what).ok_or(Error::AssetNotFound)?.saturated_into();
		let who =
			AccountIdConverter::convert_ref(who).map_err(|()| Error::AccountIdConversionFailed)?;
		let balance_amount =
			amount.try_into().map_err(|_| Error::AmountToBalanceConversionFailed)?;
		let _imbalance = Currency::deposit_creating(&who, balance_amount);
		Ok(())
	}
}
```

2. `FungiblesAdapter`: Used for depositing/withdrawing from a set of defined fungible tokens. An example of these would be `pallet-assets` tokens.

For our example, it suffices to uses `CurrencyAdapter`, as all we are going to do is mint in a single currency (Balances) whenever we receive the relay token.

---
### Configuring origin-converter with xcm-builder
Allows us to configure how we convert an origin, defined by a MultiLocation, into a dispatch origin. Used mainly in the `Transact` instruction.

1. `SovereignSignedViaLocation`: Converts the multilocation origin (tipically, a parachain origin) into a signed origin.

```rust
/// Sovereign accounts use the system's `Signed` origin with an account ID derived from the `LocationConverter`.
pub struct SovereignSignedViaLocation<LocationConverter, RuntimeOrigin>(
	PhantomData<(LocationConverter, RuntimeOrigin)>,
);
impl<
		// Converts a multilocation into account
		LocationConverter: Convert<MultiLocation, RuntimeOrigin::AccountId>,
		RuntimeOrigin: OriginTrait,
	> ConvertOrigin<RuntimeOrigin> for SovereignSignedViaLocation<LocationConverter, RuntimeOrigin>
where
	RuntimeOrigin::AccountId: Clone,
{
	fn convert_origin(
		origin: impl Into<MultiLocation>,
		kind: OriginKind,
	) -> Result<RuntimeOrigin, MultiLocation> {
		let origin = origin.into();
		if let OriginKind::SovereignAccount = kind {
			let location = LocationConverter::convert(origin)?;
			Ok(RuntimeOrigin::signed(location).into())
		} else {
			Err(origin)
		}
	}
}
```

2. `ParentAsSuperuser`: Converts the parent origin into the root origin.

3. `SignedAccountId32AsNative`: Converts a local 32 byte account multilocation into a signed origin using the same 32 byte account.

```rust
pub struct SignedAccountId32AsNative<Network, RuntimeOrigin>(PhantomData<(Network, RuntimeOrigin)>);
impl<Network: Get<NetworkId>, RuntimeOrigin: OriginTrait> ConvertOrigin<RuntimeOrigin>
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
				MultiLocation { parents: 0, interior: X1(Junction::AccountId32 { id, network }) },
			) if matches!(network, NetworkId::Any) || network == Network::get() =>
				Ok(RuntimeOrigin::signed(id.into())),
			(_, origin) => Err(origin),
		}
	}
}
```

4. `SignedAccountKey20AsNative`: Converts a local 20 byte account multilocation into a signed origin using the same 20 byte account.

To meet our requirements, we only require number 3. This will allow us to execute (locally) Transact dispatchables.

---
### Configuring Barriers with xcm-builder

Barriers specify whether or not an XCM is allowed to be executed on the local consensus system. In `xcm-builder` we can find the following pre-defined barriers already:

1. `TakeWeightCredit`: A barrier that substracts the maxixum weight the message can consume from the available weight credit. Usually configured for local xcm-execution

2. `AllowTopLevelPaidExecutionFrom<T>`: If the `origin` that sent the message is contained in `T`, this is a barrier that ensures the message contains weight payment instructions. In other words, if makes sure the first instruction puts asset into the holding register (`TeleportAsset`, `WithdrawAsset`, `ClaimAsset`, `ReserveAssetDeposit`), and then it checkes that there exists a `BuyExecution` instruction that is able to pay for the message weight. **Critical to avoid free DOS**.

3. `AllowUnpaidExecutionFrom<T>`: If the `origin` that sent the message is contained in `T`, allows free execution from such origin (i.e., does not check anything from the message). Useful for chains that "trust" each other (e.g., Statemine or any system parachain with the relay)

```rust
/// Allows execution from any origin that is contained in `T` (i.e. `T::Contains(origin)`) without any payments.
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
4. `AllowKnownQueryResponses<ResponseHandler>`: Allows the execution of the message if it is a `QueryResponse` message and the `ResponseHandler` is expecting such response

5. `AllowSubscriptionsFrom<T>`: If the `origin` that sent the message is contained in `T`, it allows the execution of the message if it is a `SubscribeVersion` message
---

To meet our example usecase, we only need the relay to have free execution. Hence using `AllowUnpaidExecutionFrom` should be enough.

### Configuring WeightTrader with xcm-builder
WeightTrader allows to specify how to charge for weight inside the xcm execution. In `xcm-builder` we can find the following pre-defined traders already:

1. `FixedRateOfFungible`: Converts weight to fee at a fixed rate and charges in a specific fungible asset
```rust
/// Simple fee calculator that requires payment in a single fungible at a fixed rate.
///
/// The constant `Get` type parameter should be the fungible ID and the amount of it required for
/// one second of weight.
pub struct FixedRateOfFungible<T: Get<(AssetId, u128)>, R: TakeRevenue>(
	Weight,
	u128,
	PhantomData<(T, R)>,
);
impl<T: Get<(AssetId, u128)>, R: TakeRevenue> WeightTrader for FixedRateOfFungible<T, R> {
	/* snip */
	fn buy_weight(&mut self, weight: Weight, payment: Assets) -> Result<Assets, XcmError> {
		// get the assetId and amount per second to charge
		let (id, units_per_second) = T::get();
		// Calculate the amount to charge for the weight bought
		let amount = units_per_second * (weight as u128) / (WEIGHT_REF_TIME_PER_SECOND as u128);
		if amount == 0 {
			return Ok(payment)
		}
		// Take amount from payment
		let unused =
			payment.checked_sub((id, amount).into()).map_err(|_| XcmError::TooExpensive)?;
		self.0 = self.0.saturating_add(weight);
		self.1 = self.1.saturating_add(amount);
		Ok(unused)
	}
	/* snip */
}
```
2. `UsingComponents`: uses `TransactionPayment` pallet to set the right price for weight.

---

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


## Additional config for version negotiation

### Configuring, ResponseHandler and SubscriptionService with pallet-xcm

The last 2 elements will be set to be handled by pallet-xcm. 

```rust
 impl Config for XcmConfig {
  /* snip */
	type ResponseHandler = PalletXcm;
	type SubscriptionService = PalletXcm;
 }
```
But what do all these elements mean:

1. `ResponseHandler`: The component in charge of handling response messages from other chains

2. `SubscriptionService`: The component in charge of handling version subscription notifications from other chains

---

### Configuring pallet-xcm
Pallet-xcm plays a critical role in every chains xcm-setup:

1. It allows for users to interact with the xcm-executor by allowing them to execute xcm messages. These can be filtered through the `XcmExecuteFilter`.
2. It provides an easier interface to do reserveTransferAssets and TeleportAssets. The locations to which these messages can be sent can also be filtered by `XcmTeleportFilter` and `XcmReserveTransferFilter`
3. It handles XCM version negotiation duties
4. It allows sending arbitrary messages to other chains for certain origins. The origins that are allowed to send message can be filtered through `SendXcmOrigin`.

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