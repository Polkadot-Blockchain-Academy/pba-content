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

**Pre-requisites**

- FRAME (Storage Items, Dispatchables, Event, Errors, etc.)
- Polkadot & parachains conceptually
- XCVM
- Core concepts of XCM

---

## _At the end of this lecture, you will be able to:_

<widget-text center>

- Understand the different XCM configurable parts for a chain
- Construct different XCM configurations for chains with different needs
- Understand how Versioninig is handled across chains

---

## ⚙️ Configurables in XCM-config
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
	// How we convert a ML to a FRAME dispatch origin
	type OriginConverter = ?;
	// Who we trust as reserve chains
	type IsReserve = Everything;
	// Who do we trust as teleporters
	type IsTeleporter = Nothing;
	// How we invert locations
	type LocationInverter = ?;
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

Notes: Xcm-builder and Xcm-pallet are your friends!

`xcm-builder` is a polkadot module that contains a set of pre-defined structures to be set in some of the configurable sides of XCM.

`xcm-pallet` is a pallet that not only allows sending and executing XCM messages, but rather it also implements several of the configuration traits and thus can be used perform several XCM configuration actions.

---

### 🤔 Grab your chain's requirements before starting

Questions that you should have answers for:

- *Is my chain going to transfer just the native token? Is my chain going to receive several other kinds of assets?*

- *Is my chain going to allow free execution? Maybe only limited to some parachains/relay chain?*

- *Is my chain a 20 byte account chain? a 32 byte account chain?*

- *How will my chain accept fee payment? In one asset? In several?*

---

### Our starting example setup requirements
1. Parachain that does not charge for relay incoming messages.
2. Parachain that trusts the relay as the reserve chain for the relay chain tokens
3. Parachain that mints in pallet-balances when it receives relay chain tokens.
4. Parachain that uses 32 byte accounts and that makes it possible for users to execute XCMs locally.
---

### 📁 Configuring LocationToAccountId with xcm-builder
- Defines how we convert a multilocation into a local accountId. 
- Useful when we want to withdraw/deposit tokens from a multilocation-defined origin
- Useful when we want to dispatch as signed origins from a multilocation-defined origin.


Notes: This will define how we convert a multilocation into a local accountId. This is useful when we want to withdraw/deposit tokens from a multilocation-defined origin or when we want to dispatch as signed origins from a multilocation-defined origin.

---v
### 📁 Configuring LocationToAccountId with xcm-builder

1. `Account32Hash`: The most generic locationToAccountIdConverter. It basically hashes the multilocation and takes the lowest 32 bytes to make a 32 byte account.

```rust
fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<AccountId, ()> {
	// Blake2(b"multiloc"+ multilocation)
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
---v
### 📁 Configuring LocationToAccountId with xcm-builder

3. `ChildParachainConvertsVia`: A structure that allows to convert the child parachain multilocation into an account of the form `b'para' + para_id_as_u32 + trailing 0s`

4. `SiblingParachainConvertsVia`: A structure that allows to convert the sibling parachain multilocation into an account of the form `b'sibl' + para_id_as_u32 + trailing 0s`

```rust
fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<AccountId, ()> {
	match location.borrow() {
		MultiLocation { parents: 1, interior: X1(Parachain(id)) } =>
			Ok(ParaId::from(*id).into_account_truncating()),
		_ => Err(()),
	}
}
```

---v
### 📁 Configuring LocationToAccountId with xcm-builder

5. `AccountId32Aliases`: A structure that allows to convert a local AccountId32 multilocation into a accountId of 32 bytes.

```rust
/// Extracts the `AccountId32` from the passed `location` if the network matches.
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

Notes: For our example, we only need the 4th. We have a requirement of users being able to execute local XCM, and as such we need to be able to Withdraw/Deposit from their accounts

---

### 👍 Configuring asset-transactors with xcm-builder
- Define how we are going to withdraw and deposit assets
- Heavily dependant on the assets we want our chain to transfer

---v
### 👍 Configuring asset-transactors with xcm-builder

- `CurrencyAdapter`: A simple adapter that uses a single currency as the assetTransactor. This is usually used for withdrawing/depositing the native token of the chain.

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
---v
### 👍 Configuring asset-transactors with xcm-builder

- `FungiblesAdapter`: Used for depositing/withdrawing from a set of defined fungible tokens. An example of these would be `pallet-assets` tokens.

Notes: For our example, it suffices to uses `CurrencyAdapter`, as all we are going to do is mint in a single currency (Balances) whenever we receive the relay token.

---
### 📍 Configuring origin-converter with xcm-builder
- Defines how to convert an XCM origin, defined by a MultiLocation, into a frame dispatch origin.
- Used mainly in the `Transact` instruction.

---v
### 📍 Configuring origin-converter with xcm-builder

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
---v
### 📍 Configuring origin-converter with xcm-builder

- `SignedAccountId32AsNative`: Converts a local 32 byte account multilocation into a signed origin using the same 32 byte account.

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

---v
### 📍 Configuring origin-converter with xcm-builder

- `ParentAsSuperuser`: Converts the parent origin into the root origin.

- `SignedAccountKey20AsNative`: Converts a local 20 byte account multilocation into a signed origin using the same 20 byte account.

Notes: To meet our requirements, we only require number 3. This will allow us to execute (locally) Transact dispatchables.

---
### 🚧 Configuring Barriers with xcm-builder
- Barriers specify whether or not an XCM is allowed to be executed on the local consensus system.
- They are checked before the actual xcm instruction execution

---v
### 🚧 Configuring Barriers with xcm-builder

- `TakeWeightCredit`: A barrier that substracts the maxixum weight the message can consume from the available weight credit. Usually configured for local xcm-execution

- `AllowTopLevelPaidExecutionFrom<T>`: If the `origin` that sent the message is contained in `T`, this is a barrier that ensures the message contains weight payment instructions. In other words, it makes sure the first instruction puts asset into the holding register (`TeleportAsset`, `WithdrawAsset`, `ClaimAsset`, `ReserveAssetDeposit`), and then it checkes that there exists a `BuyExecution` instruction that is able to pay for the message weight. **Critical to avoid free DOS**.

---v
### 🚧 Configuring Barriers with xcm-builder

- `AllowUnpaidExecutionFrom<T>`: If the `origin` that sent the message is contained in `T`, allows free execution from such origin (i.e., does not check anything from the message). Useful for chains that "trust" each other (e.g., Statemine or any system parachain with the relay)

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

---v
### 🚧 Configuring Barriers with xcm-builder
- `AllowKnownQueryResponses<ResponseHandler>`: Allows the execution of the message if it is a `QueryResponse` message and the `ResponseHandler` is expecting such response

- `AllowSubscriptionsFrom<T>`: If the `origin` that sent the message is contained in `T`, it allows the execution of the message if it contains only a `SubscribeVersion` or `UnsubscribeVersion` instruction.

---v

### 🚧 Configuring Barriers with xcm-builder: XCMV3 notes!
- `WithComputedOrigin<(InnerBarrier, LocalUniversal, MaxPrefixes)>`: Scans up to `MaxPrefixes` instructions for origin-alterers (`DescendOrigin` or `UniversalOrigin`) and the evaluates the inner barriers with the new computed origin. **Key to distinguish between barriers for derivative locations or for origins trying to send a message for themselves**.

Notes: To meet our example usecase, we only need the relay to have free execution. Hence using `AllowUnpaidExecutionFrom` should be enough.

---

### ⚖️ Configuring WeightTrader with xcm-builder
- Specifies how to charge for weight inside the xcm execution.
- Used in the `BuyExecution` instruction
- Used in the `RefundSurplus` instruction

---v
### ⚖️ Configuring WeightTrader with xcm-builder

1. `FixedRateOfFungible`: Converts weight to fee at a fixed rate and charges in a specific fungible asset
```rust
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

### 🔃 Configuring LocationInverter with xcm-builder
- The concept of `LocationInverter` is simple: knowing how to go from a `source` location to a `target` location, it calculates how to go from `target` to `source`.

-  Xcm-builder provides an easy way of doing this with the `LocationInverter<Ancestry>` struct. The only thing we need to configure is the **Ancestry**, which indicates how to go from `root` (the top-level consensus system) to your chain.

Example:
- **Ancestry**: `para_1000`
- **Source to target**: `../../../para_2/account32_default`
- **Target to source**: `../../para_1000`

---v

### 🔃 Configuring LocationInverter with xcm-builder

**Important Notes!:**

**`LocationInverter` configuration will dissapear in XcmV3!**. Instead, xcmV3 has the notion of `UniversalLocation`, which is similar to the `Ancestry` concept. However, **`Ancestry` referred to the location of the chain within the top-level local consensus system**. `UniversalLocation` refers to the location of the chain within `Universal Consensus`, including the top-level consensus system:

Example for parachain 1000 in Kusama:
- **Ancestry**: `para_1000`
- **UniversalLocation**: `GlobalConsensus(Kusama)/para_1000`

---
### 🎨 Configuring pallet-xcm
Pallet-xcm is the main connection between the FRAME subsystem and the XCM subsystem. Essentially **pallet-xcm allows us to send/execute XCM and interact with the xcm-executor**. Pallet-xcm allows to configure the following items:
- It allows for users to interact with the xcm-executor by allowing them to execute xcm messages. These can be filtered through the `XcmExecuteFilter`.
- It provides an easier interface to do reserveTransferAssets and TeleportAssets. The origins from which these messages can be sent can also be filtered by `XcmTeleportFilter` and `XcmReserveTransferFilter`
- It handles XCM version negotiation duties
- It handles asset-trap/claim duties
- It allows sending arbitrary messages to other chains for certain origins. The origins that are allowed to send message can be filtered through `SendXcmOrigin`.

---v
### 🎨 Configuring pallet-xcm
```rust
impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	// Who can send XCM messages?
	// How are origins handled?
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	// How do we route messages?
	type XcmRouter = XcmRouter;
	// Who can execute XCMs/teleport assets/reserve-transfer assets?
	// How are origins handled?
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	// Who and what messages are allowed to be executed?
	type XcmExecuteFilter = Everything;
	// The XCM executor itself
	type XcmExecutor = XcmExecutor;
	// Who and what kind of assets are allowed to be teleported via the `teleport_asset` extrinsic?
	type XcmTeleportFilter = Everything;
	// Who and what kind of assets are allowed to be transferred as a reserve asset via the `reserve_transfer_assets` extrinsic?
	type XcmReserveTransferFilter = Everything;
	type Weigher = XcmWeigher;
	type LocationInverter = LocationInverter<Ancestry>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// What XCM version do we advertise as supported
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}
```
---

### 🛄 Configuring Asset Trap/Claims with PalletXcm
The `AssetTrap` configuration item allows us to decide what to do with assets that remain in the holding register after the XCM instructions are executed. Similarly `AssetClaim` allows us to decide how to reclaim assets that were trapped.

```rust
impl Config for XcmConfig {
	/* snip */
	type AssetClaim = PalletXcm;
	type AssetTrap = PalletXcm;
  /* snip */
}
```

---v
### 🛄 Configuring Asset Trap/Claims with PalletXcm

- **PalletXcm asset trapper**: PalletXcm stores Trapped assets in the `AssetTraps` storage item. For an efficient Trap management, traps are indexed by `BlakeTwo256((origin, assets))`, each holding a counter of how many times such origin has trapped such multiAsset.

- **PalletXcm asset claimer**: Similarly to how assets are trapped, PalletXcm also allows for claiming trapped assets, providing that:
	- the origin claiming the assets is identical to the one that trapped them.
	- the multiAsset being claimed is identical to the one that was trapped

---

## 🗣️ XCM Version Negotiation

XCM is an **extensible message format**.

Versioning allows chains to communicate as XCM evolves.

```rust
pub enum VersionedXcm {
    V0(v0::Xcm),
    V1(v1::Xcm),
    V2(v2::Xcm),
}
```
---v

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

---

### Configuring, ResponseHandler and SubscriptionService with pallet-xcm

The last 2 items will be set to be handled by pallet-xcm. 

```rust
 impl Config for XcmConfig {
	/* snip */
	type ResponseHandler = PalletXcm;
	type SubscriptionService = PalletXcm;
 }
```
But what do all these elements mean:

1. `ResponseHandler`: The component in charge of handling response messages from other chains

2. `SubscriptionService`: The component in charge of handling version subscription notifications from other chains.

