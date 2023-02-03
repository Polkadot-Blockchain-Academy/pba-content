---
title: Cross Consensus Messaging (XCM)
description: XCM Core Concepts, Terms, and Logic for Web3 Engineers
duration: 1 hour
---

# Cross Consensus Messaging (XCM)

## _Core Concepts, Terms, and Logic_

Notes:

**Pre-requisites**

- FRAME (Storage Items, Dispatchables, Event, Errors, etc.)
- Polkadot & parachains conceptually
- Assets (NFTs and fungibles)

---

## _At the end of this lecture, you will be able to:_

<pba-flex center>

- Define the concepts, syntax, and terms of XCM
- Navigate exciting resources that relate to XCM
- Differentiate between XCM and message-passing protocols like XCMP

---

## What cross-chain use cases exist?

Performing operations on different blockchains?

How might you go about designing a _system_ to facilitate them?

Notes:

EXERCISE: ask the class to raise hands and postulate on generally what one might do.

---

## 🎬 Some Concrete Use-cases

<pba-flex center>

1. Cross-consensus asset transfers
1. Execute platform-specific actions (extrinsics) such as governance voting
1. Enables single use-case chains e.g. [Statemint/e](https://github.com/paritytech/cumulus/tree/master/parachains/runtimes/assets) as asset parachains

Notes:

While the goal of XCM is to be general, flexible and future-proof, there are of course practical needs which it must address, not least the transfer of tokens between chains.
We need a way to reason about, and pay for, any required fees on the receiving CS.
Platform-specific action; for example, within a Substrate chain, it can be desirable to dispatch a remote call into one of its pallets to access a niche feature.
XCM enables a single chain to direct the actions of many other chains, which hides the complexity of multi-chain messaging behind an understandable and declarative API.

---

### XCM aims to be a _language communicating ideas between consensus systems._

---

## Cross _Consensus_?

**Consensus systems**: A chain, contract or other global, encapsulated, state machine singleton.

<pba-flex center>

- Can be any programmatic state-transition system that exists within consensus which can send/receive datagrams.
- It does not even have to be a _distributed_ system, only that it can form _some_ kind of consensus.

Notes:

A consensus system does not necessarily have to be a blockchain or a smart contract, it can be something that already exists in the Web 2.0 world, such as an AWS server.

---

## 🤟 A Format, not a Protocol

XCM is a **_messaging format_**.

It is akin to the post card from the post office

It is _not_ a messaging protocol!

A post card doesn't send itself!

Notes:

It cannot be used to actually “send” any message between systems; its utility is only in expressing what should be done by the receiver.
like many aspects core to Substrate, this separation of concerns empowers us to be far more generic and enable much more.
A post card relies on the postal service to get itself sent towards its receivers, and that is what a messaging protocol does.

---

## 😬 Why not _native_ messages?

Drawbacks of relying on native messaging or transaction format:

<pba-flex center>

- Lack of uniformity between consensus systems on message format
- Common cross-consensus use-cases do not map one-to-one to a single transaction
- Operations on consensus systems have different assumptions e.g. fee payment

Notes:

- A system which intends to send messages to more than one destination would need to understand how to author a message for each.
  On that note, even a single destination may alter its native transaction/message format over time.
  Smart contracts might get upgrades, blockchains might introduce new features or alter existing ones and in doing so change their transaction format.
- Special tricks may be required to withdraw funds, exchange them and then deposit the result all inside a single transaction.
  Onward notifications of transfers, needed for a coherent reserve-asset framework, do not exist in chains unaware of others.
- Some systems assume that fee payment had already been negotiated, while some do not.

---

## XCM Communication Model

XCM is designed around four 'A's:

<pba-flex center>

- **Agnostic**: No assumptions about Consensus System messaged
- **Absolute**: Guaranteed delivery, interpretation, and ordering
- **Asynchronous**: No assumption of blocking for sender/receiver
- **Asymmetric**: No results or callbacks (separately communicated!)

Notes:

- **Agnostic**: XCM makes no assumptions about the nature of the Consensus System between which messages are being passed.
- **Absolute**: XCM messages are guaranteed to be delivered and interpreted accurately, in order and in a timely fashion.
- **Asynchronous**: XCM messages in no way assume that the sender will be blocking on its completion.
- **Asymmetric**: XCM messages do not have results.
  Any results must be separately communicated to the sender with an additional message.

---

## Async vs Sync

XCM crossing the barrier between a single consensus system<br/>cannot generally be synchronous.

No guarantees on delivery time.

Notes:

Generally, consensus systems are not designed to operate in sync with external systems.
They intrinsically need to have a uniform state to reason about and do not, by default, have the means to verify states of other consensus systems.
Thus, each consensus system cannot make any guarantees on the expected time required to deliver results; doing so haphazardly would cause the recipient to be blocked waiting for responses that are either late or would never be delivered, and one of the possible reasons for that would be an impending runtime upgrade that caused a change in how responses are delivered.

---

## XCM is "fire and forget"

XCM has no results:

<pba-flex center>

- No errors reported to sender
- No callbacks for sender

Similar to UDP

Notes:

The receiver side can and does handle errors, but the sender will not be notified, unless the error handler specifically tries to send back an XCM that makes some sort of XCM that notifies status back to the origin, but such an action should be considered as constructing a separate XCM for the sole purpose of reporting information, rather than an intrinsic functionality built into XCM, akin to how UDP can also create a "response" to an incoming datagram, yet the response is too considered as a separate UDP datagram instance.

---

## Async XCM

We _could_ have XCM describe async behavior but do not because:

<pba-flex center>

- Complexity, custom per sender/receiver pair
- Expense of operating in fee-based systems

Notes:

Asynchronous systems vary widely by implementation, and as a protocol that attempts to bridge between disparate consensus systems, XCM does not attempt to define the behavior or architecture of its interlocutors.
Rather, XCM defines and standardizes the interface and semantics that two or more consensus systems can use to interact with each other, but leaves the ultimate implementation details to its participating systems.

---

## 📍 Locations in XCM

`MultiLocation` = a **_relative_** location in the consensus multiverse.

All entities are addressed as paths to them, _relative_ to the current consensus system.
pub struct MultiLocation {
pub parents: u8,
pub interior: Junctions,
}

````

Notes:

The `MultiLocation` type identifies any single _location_ that exists within the world of consensus.
Representing a scalable multi-shard blockchain such as Polkadot, a lowly ERC-20 asset account on a parachain, a smart contract on some chain, etc.
It is always represented as a location _relative_ to the current consensus system, and never as an absolute path, due to the fact that the network structure can always change, and so absolute paths can quickly go out of date.

---

## Junction

An item in a path to describe the<br/>relative location of a consensus system:

<pba-flex center>

- `Parachain`
- `AccountId32`
- `PalletInstance`
- `GeneralKey`

Notes:

This is akin to a directory on a file path, e.g. the `foo` in `/foo/bar`.

---

## Junction*s*\*

```rust
enum Junctions {
    X1(Junction),
    X2(Junction, Junction),
    X3(Junction, Junction, Junction),
    // ...
    X8(Junction, /*...*/),
}
````

Enum containing multiple `Junction`s

Notes:

An array like `[Junction; 8]` or a `Vec` is explicitly not used in place of the `Junctions` enum.
This is because `Vec`s cannot be pattern-matched, and arrays have a fixed size at compilation time, and thus unused `Junction` "element slots" will always be required to be filled in, bloating the _encoded_ size of a `Junctions` data structure.

---

## MultiLocation Examples
- `../Parachain(1000)`: Evaluated within a parachain, this would identify our sibling parachain of index 1000. (In Rust we would write `MultiLocation { parents: 1, junctions: X1(Parachain(1000)) }` or alternatively `ParentThen(Parachain(1000)).into()`.)

- `../AccountId32(0x1234...cdef)`: Evaluated within a parachain, this would identify the 32-byte account 0x1234…cdef on the relay chain.

- `Parachain(42)/AccountKey20(0x1234...abcd)`: Evaluated on a relay chain, this would identify the 20-byte account 0x1234…abcd on parachain number 42 (presumably something like Moonbeam which hosts Ethereum-compatible accounts).

---

## MultiLocation Examples

<!-- TODO DESIGN: use multilocation graphic from above and add labels in fragment / new slide here -->
<!-- Base on this set of slides: https://docs.google.com/presentation/d/18qRqqw73L9NTWOX1cfGe5sh484UgvlpMHGekQHu9_8M/edit#slide=id.g8063ab3d6f_0_1418 . If hard, just make these into images via screenshot & use full screen -->

<img rounded style="width: 650px;" src="../../../assets/img/7-XCM/mod1-multilocation-picture.png" alt="MultiLocation Example" />

Notes:
speak to an example of non-parachain multi-location that would use a bridge
XCM reasons about addressing (as in a postal address) that must include understanding where you are, not just where you are going!
This will be very powerful later on (Origins)

<!-- TODO: does XCM explicitly need to know the Origin of the message? Could there be anonymous XCM? (no "return to sender" field on mail) -->

---

## Cross-Consensus Origins

A `MultiLocation` denoting where an XCM originated from

_Relative_ to the current location

Can be converted into a pallet origin in a FRAME runtime

Notes:

Since `MultiLocation`s are relative, when an XCM gets sent over to another chain, the origin location needs to be rewritten from the perspective of the receiver, before the XCM is sent to it.
This is calling re-anchoring.

---

## `MultiLocation` established!

Now we know how to describe the destination, what _do we want to send_?

Let's start with messages (XCVM Programs!) about **_assets_**.

Notes:

---

<pba-col>

### 💰 `MultiAsset` in XCM

There are many _classes_ of assets (fungible, NFTs,...)

```rust
struct MultiAsset {
   id: AssetId,
```

The datatype `MultiAsset` describes them all.

---

## Asset Representation

<div style="font-size: smaller">

```rust
struct MultiAsset {
    pub id: AssetId,
    pub fun: Fungibility,
}

enum AssetId {
    Concrete(MultiLocation),
    Abstract([u8; 32]),
}

enum Fungibility {
    Fungible(u128),
    NonFungible(AssetInstance),
}

enum AssetInstance {
    Undefined,
    Index(u128),
    Array4([u8; 4]),
    Array8([u8; 8]),
    Array16([u8; 16]),
    Array32([u8; 32]),
}
```

</div>

Notes:

A MultiAsset is composed of an asset ID and an enum representing the fungibility of the asset.
Asset IDs can either be Concrete or Abstract:
Concrete assets - can be identified by a `MultiLocation` path that leads to the system that issues it
Abstract assets - can be identified only by a label/name

Assets can also either be fungible or non-fungible:
Fungible - each token of this asset has the same value as any other
NonFungible - each token of this asset is unique and cannot be seen as having the same value as any other token under this asset

Non-fungible assets will then also need to further specify which exact token it represents under the same asset ID, and we use the AssetInstance enum to express the uniqueness of such a token.

---

## Convenience methods to create `MultiAsset`

```rust
/// Creates 10 billion units of fungible native tokens
let fungible_asset: MultiAsset = (Here, 10_000_000_000u128).into();
//          or MultiAsset::from((Here, 10_000_000_000u128)) ^^^^

/// Creates an abstract NFT with an undefined asset instance
let nft_asset: MultiAsset = ([0; 32], ()).into();
```

Notes:

In Polkadot, a unit of native token = 1 planck, and 10 billion plancks = 1 DOT

---

## Asset Wildcards and Filters

```rust
enum WildMultiAsset {
    All,
    AllOf { id: AssetId, fun: WildFungibility },
    AllCounted(u32),
    AllOfCounted { id: AssetId, fun: WildFungibility, count: u32 },
}

enum WildFungibility {
    Fungible,
    NonFungible,
}

enum MultiAssetFilter {
    Definite(MultiAssets),
    Wild(WildMultiAsset),
}

struct MultiAssets(Vec<MultiAsset>);
```

Notes:

These are types used by various instructions that want to express the idea to select all of one kind of assets in the holding register, but do not know the exact amount of assets that already exists in holding.
"Wild" in this context has the same meaning as the "wild" in "wildcard".
The "counted" variants is used when we want to limit the amount of assets that the wildcard can select.
This is very useful in cases where we want to give an upper limit to the execution time required to select the assets within the holding register, or that we simply just want the specified number of types of assets within the specified class of assets.

---

## Reanchoring

`MultiLocation`s are relative.

**Scenario:**<br/>
Current consensus system is `Para(1337)`.<br/>
Destination consensus system is `Para(6969)`.

<pba-flex center>

- Where is `Here`?
- What happens when I send a `MultiAsset`<br/>with an `AssetId` of `Concrete(Here)` to `Para(6969)`?

Notes:

MultiLocations are relative, so they must be updated and rewritten when sent to another chain.

---

## 🤹 Many models for <br/> transferring assets

<pba-flex center>

1. "Remote control" an account on another system
1. Reserve transfers
1. Teleport transfers

Notes:

We might want to simply control an account on a remote chain, allowing the local chain to have an address on the remote chain for receiving funds and to eventually transfer those funds it controls into other accounts on that remote chain.
Accounts that are controllable by a remote chain are often referred to as **Sovereign accounts**.

---

## 🤹 Many models for <br/> transferring assets

<pba-cols>
<pba-col>

<img rounded style="width: 500px;" src="../../../assets/img/7-XCM/rm-tx.png" alt="Remote Transfer" />
<img rounded style="width: 500px;" src="../../../assets/img/7-XCM/teleport.png" alt="Teleport" />

</pba-col>
<pba-col>

<img rounded style="width: 400px;" src="../../../assets/img/7-XCM/reserve-tx.png" alt="Reserve Transfer" />

</pba-col>
</pba-cols>

Notes:

TODO: use examples from here https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392 to describe the images

---

## Next steps

<pba-flex center>

1. Blog series introducing XCM: Parts [1](https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392), [2](https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83), and [3](https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166).
1. XCM Format [repository](https://github.com/paritytech/xcm-format)
<!-- 1. TODO: fill this in - polkadot / cumulus / parachains repos?  -->

---

## Glossary

<!-- TODO: ensure these are in the class glossary! Remove this slide and simply reference in the slides -->

- UMP (Upward Message Passing) allows parachains to send messages to their relay chain.
- DMP (Downward Message Passing) allows the relay chain to pass messages down to one of their parachains.
- HRMP (Horizontal Message Passing)
- XCM
- XCVM
- XCMP (Cross-Consensus Message Passing), which is perhaps the best known of them, allows the parachains to send messages between themselves.
- {XCM} Junctions
- MultiLocations
- Sovereign account(s)
- Holding register
- Consensus system
- {XCM} Instructions
- {XCM config} Barriers
- {XCM config} Filters
- UDP {networking}
- TTL {networking}

---

## Polkadot Network Diagram

<img rounded src="../../../assets/img/0-Shared/parachains/relay-network-diagram.png" alt="Relay Network Diagram" style="width:800px;" />
