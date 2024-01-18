---
title: Parachain XCM Configuration # Also update the h1 header on the first slide to the same name
description: XCM configuration overview and considerations, for parachains.
duration: 1 hour
---

# Parachain XCM Configuration

---v

## _At the end of this lecture, you will be able to:_

<pba-flex center>

- Understand the different XCM configurable parts of a chain
- Construct different XCM configurations for chains with different needs

---

## 🛠️ `XcmConfig`

<pba-flex center>

The XCM executor is very configurable.

This is because different systems can/need to interpret some instructions differently than others.

For example, a DEX chain might implement the `ExchangeAsset` instruction, but perhaps not an identity chain.

Notes:

The XCM Configuration has many configurable items

EXERCISE: ask the class to raise hands and postulate on what they think should be configurable.

---v

## 🛠️ Configurables in `XcmConfig`

```rust [1-2|6-7|8-9|10-11|12-13|16-31]
// How we convert locations into account ids
type SovereignAccountOf = SovereignAccountOf;

pub struct XcmConfig;
impl Config for XcmConfig {
  // Pre-execution filters
  type Barrier = Barrier;
  // How we withdraw/deposit assets
  type AssetTransactor = LocalAssetTransactor;
  // How we convert a Location to a FRAME dispatch origin
  type OriginConverter = LocalOriginConverter;
  // How we route the XCM outside this chain
  type XcmSender = XcmRouter;
  // Who we trust as reserve chains
  type IsReserve = ?;
  // Who do we trust as teleporters
  type IsTeleporter = ?;
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
  // The absolute Location of the current system
  type UniversalLocation = ?;
}
```

Notes:

- `SovereignAccountOf`: Means of converting a `Location` into an account ID
  Used later for: `OriginConverter` , `AssetTransactor`

- `xcm-pallet` is a pallet that not only allows sending and executing XCM messages, but rather it also implements several of the configuration traits and thus can be used perform several XCM configuration actions.

---v

## 🛠️ `xcm-builder`

`xcm-builder` is a crate containing common configuration shims to facilitate XCM configuration.

Most pre-built configuration items can be found in `xcm-builder`.

---

### 🤔 Some useful questions when configuring XCM

- _Is my chain going to transfer just the native token? or multiple kinds of assets?_

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

### 📁 `SovereignAccountOf`

- Defines how we convert a `Location` into a local account ID.
- Used when we want to withdraw/deposit tokens from a `Location` defined origin
- Used when we want to dispatch as signed origins from a `Location` defined origin.

<diagram class="mermaid">
graph TD;
  Location("AccountId32 { id: [18, 52, ..., 205, 239], network: Some(Rococo) }")-- SovereignAccountOf -->Account("0x123..def (Alice)")
</diagram>

Notes:

- This will define how we convert a `Location` into a local account ID.
- This is useful when we want to withdraw/deposit tokens from a `Location` defined origin or when we want to dispatch as signed origins from a `Location` defined origin.

---v

### `HashedDescription`

Hashes the description of a `Location` and converts that into an `AccountId`.

```rust

pub struct HashedDescription<AccountId, Describe>(PhantomData<(AccountId, Describe)>);
impl<
  AccountId: From<[u8; 32]> + Clone,
  Describe: DescribeLocation
> ConvertLocation<AccountId> for HashedDescription<AccountId, Describe>
{
	fn convert_location(value: &Location) -> Option<AccountId> {
		Some(blake2_256(&Describe::describe_location(value)?).into())
	}
}
```

---v

### `DescribeLocation`

Means of converting a location into a stable and unique descriptive identifier.

```rust
pub trait DescribeLocation {
	/// Create a description of the given `location` if possible. No two locations should have the
	/// same descriptor.
	fn describe_location(location: &Location) -> Option<Vec<u8>>;
}
```

Notes:

[Impl for Tuple](https://github.com/paritytech/polkadot/blob/c7f58c17f906467634a5b236d7b3c1df24057419/xcm/xcm-builder/src/location_conversion.rs#L34)

---v

### `DescribeTerminus`

```rust
fn describe_location(l: &Location) -> Option<Vec<u8>> {
	match (l.parents, &l.interior) {
		(0, Here) => Some(Vec::new()),
		_ => return None,
	}
}
```

---v

### `DescribePalletTerminal`

```rust
fn describe_location(l: &Location) -> Option<Vec<u8>> {
	match (l.parents, &l.interior) {
		(0, X1(PalletInstance(i))) =>
			Some((b"Pallet", Compact::<u32>::from(*i as u32)).encode()),
		_ => return None,
	}
}
```

---v

### `AccountId32Aliases`

- Converts a local `AccountId32` `Location` into an account ID of 32 bytes.
- Needed for dealing with local accounts on the same system.

---v

### Example converter

An example of a converter definition:

<pba-flex center>

```rust
pub type LocationToAccount = (
  AccountId32Aliases<Network, AccounId>, // For dealing with local accounts
  HashedDescription<AccountId, (
    LegacyDescribeForeignChainAccount, // Legacy conversion - MUST BE FIRST!
    DescribeTerminus,
    DescribePalletTerminal
  )>,
);
```

---

# 🚧 `Barrier`

- Barriers specify whether or not an XCM is allowed to be executed on the local consensus system.
- They are checked before the actual XCM instruction execution.
- **Barriers should not involve any heavy computation.**

Notes:

**At the point at which barriers are checked nothing has yet been paid for its execution**.

---v

## `AllowTopLevelPaidExecutionFrom<T>`

- For origins contained in `T`, it makes sure the first instruction puts asset into the holding register, followed by a `BuyExecution` instruction capable of buying sufficient weight.
- **Critical to avoid free DOS**.

Notes:

- A chain without `AllowTopLevelPaidExecutionFrom` could potentially receive several heavy-computation instructions without paying for it.
  Checking that the first instructions are indeed paying for execution helps to quick-discard them.

- While `BuyExecution` is crucial for messages coming from other consensus systems, local XCM execution fees are paid as any other substrate extrinsic.

---v

## `AllowExplicitUnpaidExecutionFrom<T>`

- Allows free execution if `origin` is contained in `T` and the first instruction is `UnpaidExecution`.

---v

# Attack scenario

Say we don't have a barrier, then we accept all incoming XCMs.

A message that doesn't pay fees can be sent repeatedly to cause a denial of service attack.

<pba-flex center>

```rust
Xcm(vec![
  WithdrawAsset(assets),
  DepositAsset { assets, beneficiary },
])
```

---v

## Requiring fees to be paid

The first thing we can do to protect ourselves from that sort of attack is use the `AllowTopLevelPaidExecutionFrom<T>` barrier:

<pba-flex center>

```rust
type Barrier = AllowTopLevelPaidExecutionFrom<Everything>;
```

We can accept messages from `Everything` since they'll be dropped before execution if they don't intend to pay for fees.

---v

## Physical vs Computed origin

- Physical origin: the consensus system that built this particular XCM and sent it to the recipient. E.g. Parachain A
- Computed origin: the entity that ultimately instructed the consensus system to build the XCM. E.g. A user account in parachain A

Notes:

If an EOA transfers some funds via XCM, then the computed origin would be its account, but the physical origin would be the platform that was used (e.g. parachain).

---v

## `WithComputedOrigin`

Allows for origin altering instructions at the start.

This allows messages from user accounts instead of only chains.

<pba-flex center>

```rust
pub struct WithComputedOrigin<InnerBarrier, LocalUniversal, MaxPrefixes>;
```

---v

## `TakeWeightCredit`

- Subtracts the maximum weight the message can consume from the available weight credit.
- Usually configured for local XCM execution

---

# 🪙 `AssetTransactor`

- Defines how to withdraw and deposit assets
- Heavily dependant on the assets we want our chain to transfer

<diagram class="mermaid">
graph LR
  Withdraw("WithdrawAsset((Here, 100u128).into())")-->DOT(100 tokens from e.g. pallet-balances)
</diagram>

Notes:

- The relay chain is a clear example of a chain that handles a **single token**.
- AssetHub on the contrary acts as an asset-reserve chain, and it needs to handle **several assets**

---v

## `xcm-builder` adaptersk

|Fungibility/Number|Single|Multiple|
|--------|------|--------|
|Fungible|FungibleAdapter|FungiblesAdapter|
|Non-fungible|NonFungibleAdapter|NonFungiblesAdapter|

---

## Summary

<pba-flex center>

In this lecture, we learnt:

- How chains interpret locations and turn them to accounts and FRAME origins
- How to set a barrier to protect our chain from attacks
- What adapters are available to translate XCM `Assets` to FRAME assets
