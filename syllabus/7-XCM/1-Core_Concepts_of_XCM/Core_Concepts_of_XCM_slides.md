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
- Navigate existing resources that relate to XCM
- Differentiate between XCM and message-passing protocols like XCMP

---

## What cross-chain use cases exist?

Performing operations on different blockchains?

<!--
- Transfers
- One contract calling another contract
- Credential checking
- Voting
Engage the audience and expect them to say transfers as the use case, then show them the other problems worth solving
-->

How might you go about designing a _system_ to facilitate them?

Notes:

EXERCISE: ask the class to raise hands and postulate on generally what one might do.

---

## 🎬 Some Concrete Use-cases

<pba-flex center>

1. Cross-consensus asset transfers
2. Execute platform-specific actions (extrinsics) such as governance voting
3. Enables single use-case chains e.g. [Collectives](https://github.com/paritytech/cumulus/tree/master/parachains/runtimes/assets) as asset parachains, Identity

Notes:

While the goal of XCM is to be general, flexible and future-proof, there are of course practical needs which it must address, not least the transfer of tokens between chains.
We need a way to reason about, and pay for, any required fees on the receiving CS.
Platform-specific action; for example, within a Substrate chain, it can be desirable to dispatch a remote call into one of its pallets to access a niche feature.
XCM enables a single chain to direct the actions of many other chains, which hides the complexity of multi-chain messaging behind an understandable and declarative API.

---

### XCM aims to be a _language communicating intentions between consensus systems._

---

## Cross _Consensus_?

**Consensus systems**: A chain, contract or other global, encapsulated, state machine singleton.

<pba-flex center>

<!-- - Can be any programmatic state-transition system that exists within consensus which can send/receive datagrams. -->

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
- Consensus systems may evolve over time

Notes:

- A system which intends to send messages to more than one destination would need to understand how to author a message for each.
  On that note, even a single destination may alter its native transaction/message format over time.
  Smart contracts might get upgrades, blockchains might introduce new features or alter existing ones and in doing so change their transaction format.
- Special tricks may be required to withdraw funds, exchange them and then deposit the result all inside a single transaction.
  Onward notifications of transfers, needed for a coherent reserve-asset framework, do not exist in chains unaware of others.
  Some use-cases don't require accounts.
- Some systems assume that fee payment had already been negotiated, while some do not.
- If a consensus system changes a pallet from one place to another, it breaks the native message but not XCM

TODO: Why not just send EVM programs. Why XCVM instead of EVM?
Add Shawn's picture.
It's up to the interpreter to interpret the intention how it makes sense.

---

## XCM Communication Model

XCM is designed around four 'A's:

<pba-flex center>

- **Agnostic**: No assumptions about Consensus System messaged
- **Absolute**: Guaranteed delivery, interpretation, and ordering
- **Asynchronous**: No assumption of blocking for sender/receiver
- **Asymmetric**: No results or callbacks (separately communicated!)

Notes:

The 4 'A's are assumptions XCM makes over the transport protocol.
These are things it assumes.
Context: consensus systems are deterministic.

TODO: "Relies on" is better to use.

TODO: If you compare it with IBC, they don't make these assumptions. You factor in fallibility of the transport protocol.

TODO: Further down the line, reference XCMP.

TODO: Break into 4 slides to explain all of them .

- **Agnostic**: XCM makes no assumptions about the nature of the Consensus System between which messages are being passed.
- **Absolute**: XCM messages are guaranteed to be delivered and interpreted accurately, in order and in a timely fashion. The message format does not do much about the message possibly not being delivered.
- **Asynchronous**: XCM messages in no way assume that the sender will be blocking on its completion. You can't just block execution in the middle of a block, it has to be asynchronous. Different systems have different ways of tracking time.
- **Asymmetric**: XCM messages do not have results.
  Any results must be separately communicated to the sender with an additional message.

---v

## Async vs Sync

XCM crossing the barrier between a single consensus system<br/>cannot generally be synchronous.

No guarantees on delivery time.

Notes:

Generally, consensus systems are not designed to operate in sync with external systems.
They intrinsically need to have a uniform state to reason about and do not, by default, have the means to verify states of other consensus systems.
Thus, each consensus system cannot make any guarantees on the expected time required to deliver results; doing so haphazardly would cause the recipient to be blocked waiting for responses that are either late or would never be delivered, and one of the possible reasons for that would be an impending runtime upgrade that caused a change in how responses are delivered.

---v

## XCM is "fire and forget"

XCM has no results:

<pba-flex center>

- No errors reported to sender
- No callbacks for sender

Notes:

The receiver side can and does handle errors, but the sender will not be notified, unless the error handler specifically tries to send back an XCM that makes some sort of XCM that notifies status back to the origin, but such an action should be considered as constructing a separate XCM for the sole purpose of reporting information, rather than an intrinsic functionality built into XCM, akin to how UDP can also create a "response" to an incoming datagram, yet the response is too considered as a separate UDP datagram instance.

TODO: XCM is a bit like REST. XCMP is a bit like TCP/IP. Not quite. Analogies can often hurt more than they help.

---

## 📍 Locations in XCM

How does one consensus system address another?

Location in consensus might be the whole system or an isolatable part in the system.

Examples:

- Accounts
- Smart contracts
- Pallets
- Blockchains

Notes:

The `MultiLocation` type identifies any single _location_ that exists within the world of consensus.
Representing a scalable multi-shard blockchain such as Polkadot, a lowly ERC-20 asset account on a parachain, a smart contract on some chain, etc.
It is always represented as a location _relative_ to the current consensus system, and never as an absolute path, due to the fact that the network structure can always change, and so absolute paths can quickly go out of date.

---v

## Location hierarchy

There's a hierarchy of locations within consensus.

TODO: Add diagrams

---v

## Universal location

We can imagine a hypothetical location that contains all top-level consensus systems.

---v

## Relative vs absolute

We express locations in two ways, `MultiLocation` is relative and `InteriorMultiLocation` is absolute.
We need relative locations because some consensus systems might change their location.
If a system moves, relative is better.
In general, you want to work with relative locations since you don't care which particular relay chain you are on.
In the case of bridges, you go between consensus systems, so you use absolute locations.

```rust
pub type MultiLocation = RelativeMultiLocation;
pub type InteriorMultiLocation = AbsoluteMultiLocation;
// TODO: Rename them to just Location

pub struct RelativeMultiLocation {
    pub parents: u8,
    pub interior: Junctions,
}

pub struct AbsoluteMultiLocation; // TODO
```

---v

## MultiAsset

We identify assets with locations.
We identify the native token of a chain to its location.

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

## MultiLocation Examples

TODO: I would use diagrams and split it into more slides.

- `../Parachain(1000)`: Evaluated within a parachain, this would identify our sibling parachain of index 1000. (In Rust we would write `MultiLocation { parents: 1, junctions: X1(Parachain(1000)) }` or alternatively `ParentThen(Parachain(1000)).into()`.)

- `../AccountId32(0x1234...cdef)`: Evaluated within a parachain, this would identify the 32-byte account 0x1234…cdef on the relay chain.

- `Parachain(42)/AccountKey20(0x1234...abcd)`: Evaluated on a relay chain, this would identify the 20-byte account 0x1234…abcd on parachain number 42 (presumably something like Moonbeam which hosts Ethereum-compatible accounts).

---

## MultiLocation Examples

<!-- TODO DESIGN: use multilocation graphic from above and add labels in fragment / new slide here -->
<!-- Base on this set of slides: https://docs.google.com/presentation/d/18qRqqw73L9NTWOX1cfGe5sh484UgvlpMHGekQHu9_8M/edit#slide=id.g8063ab3d6f_0_1418 . If hard, just make these into images via screenshot & use full screen -->

TODO: Too many things going on. Circle the different examples.
Add a "You are here" marker instead of "Origin: Polkadot".
Add a bridge example with Polkadot<>Kusama.

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

TODO: Used for privileges.

Notes:

Since `MultiLocation`s are relative, when an XCM gets sent over to another chain, the origin location needs to be rewritten from the perspective of the receiver, before the XCM is sent to it.
This is calling re-anchoring.

TODO (remove): Native is direct mapping, when you happen to have the exact equivalent of a runtime origin.
Happens in parachains in the polkadot parachain pallet.
You usually want the sovereign account.

---

## `MultiLocation` established!

Now we know how to describe the destination, what _do we want to send_?

Let's start with messages (XCVM Programs!) about **_assets_**.

Notes:

---

<pba-col>

### 💰 `MultiAsset` in XCM

There are many _classes_ of assets (fungible, NFTs,...)

TODO: Rename to `Asset` and rename `VersionedMultiAsset` to `MultiAsset`.

```rust
struct MultiAsset {
   pub id: AssetId,
   pub fun: Fungibility,
}
```

The datatype `MultiAsset` describes them all.

---

## Asset Representation

<div style="font-size: smaller">

```rust
struct Asset {
    pub id: AssetId,
    pub fun: Fungibility,
}

struct AssetId(Location);

enum Fungibility {
    Fungible(u128),
    NonFungible(AssetInstance),
}

// enum AssetInstance {
//     Undefined,
//     Index(u128),
//     Array4([u8; 4]),
//     Array8([u8; 8]),
//     Array16([u8; 16]),
//     Array32([u8; 32]),
// }
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

/// Creates an NFT with an Undefined asset instance
let nft: MultiAsset = ([0; 32], [TODO...]).into();
```

Notes:

In Polkadot, a unit of native token = 1 planck, and 10 billion plancks = 1 DOT

---

## Asset Wildcards and Filters

TODO: More pictures.
Use the filters to select assets from an image with a grid of lots of assets.

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

TODO: Add a table of the location interpretations by the different parachains.
Also add UniversalLocation.

`Location`s are relative.

**Scenario:**<br/>
Current consensus system is `AssetHub` on Polkadot.

Destination consensus system is `Collectives` on Polkadot.

We want to send USDT to collectives.

<pba-flex center>

- Where is the universal location of `Here`? (Poladot or Kusama or what)
- What happens when I send an `Asset`<br/>with an `AssetId` of `Here` to `../Collectives`?

Misinterpretation ensues.

Notes:

MultiLocations are relative, so they must be updated and rewritten when sent to another chain.

---

## 🤹 Multiple models for transferring assets

<pba-flex center>

1. "Remote control" an account on another system
2. Reserve transfers
3. Teleport transfers

Notes:

We might want to simply control an account on a remote chain, allowing the local chain to have an address on the remote chain for receiving funds and to eventually transfer those funds it controls into other accounts on that remote chain.
Accounts that are controllable by a remote chain are often referred to as **Sovereign accounts**.

---

## 🤹 Multiple models for transferring assets

<pba-cols>
<pba-col>

TODO: Divide into two slides.
TODO: Talk about sovereign accounts before? I don't think we have.

<!-- <img rounded style="width: 500px;" src="../../../assets/img/7-XCM/rm-tx.png" alt="Remote Transfer" /> -->
<img rounded style="width: 500px;" src="../../../assets/img/7-XCM/teleport.png" alt="Teleport" />

</pba-col>
<pba-col>

TODO: Add case where A = R.

<img rounded style="width: 400px;" src="../../../assets/img/7-XCM/reserve-tx.png" alt="Reserve Transfer" />

</pba-col>
</pba-cols>

Notes:

TODO: use examples from here https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392 to describe the images

TODO: You can use XCM with UTXO-based models.

---

## Next steps

<pba-flex center>

1. Blog series introducing XCM: Parts [1](https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392), [2](https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83), and [3](https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166).
2. XCM Format [repository](https://github.com/paritytech/xcm-format)
3. XCM [Docs](https://paritytech.github.io/xcm-docs/)
<!-- 1. TODO: fill this in - polkadot / cumulus / parachains repos?  -->

---

## Glossary

TODO: Move this to the docs and link there.

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
