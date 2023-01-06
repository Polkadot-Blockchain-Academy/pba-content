---
title: Writing, Sending, and Execution of XCM
description: Writing, Sending, and Execution of XCM for Web3 Engineers
duration: 1 hour
instructors: ["Keith Yeung"]
teaching-assistants: ["Dan Shields"]
---

# Writing, Sending, and Execution of XCM

---

## _At the end of this lecture, you will be able to:_

- Construct XCMs via a combination of basic instructions
- Contribute to the discussions on upcoming changes to the XCM format
- Understand the basic structure and architecture behind cross-consensus asset transfers

---

## XCM is an evolving format!

We are presently in V2 with V3 on the way!

Notes:

XCM is a work-in-progress, versioned format!
A goal of this module is to make you capable of contributing to it.

<!-- TODO: activity to look at proposed but not finalized addition (something like a ZK instruction set) -->

---

## The XCM format

The specification document for the XCM format:

<br>

https://github.com/paritytech/xcm-format

---

## XCM Specification Main Sections

1. [XCM Communication Model](#xcm-communication-model)
1. [Basic Top-Level Format](#basic-top-level-format)
1. XCVM Registers\*
1. [Basic XCVM Operation](#basic-top-level-format)
1. [XCVM Instruction Set](#instruction)
1. Universal Asset Identifiers (`MultiAsset`)\*
1. Universal Consensus Location Identifiers (`MultiLocation`)\*
1. [XCM Error Types](#xcm-errors)

<br>

_\*covered in lesson 1_

Notes:

1. Talks about terms used in XCM, the entire architecture around XCM (namely the communication model and XCVM)
1. Is about the `VersionedXcm` type and how XCM versioning is done
1. All registers (states) of the XCVM
1. The steps undertaken by the XCVM to execute one single instruction
1. All instructions and their semantics, the registers that they may change, and their behavior
1. The kind of assets that can be represented using `MultiAsset`
1. How locations are represented in XCM via `MultiLocation`
1. The kinds of errors that one can encounter during the execution of an XCM

---

## XCM Communication Model

XCM is designed around four 'A's:

<widget-text center>

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

XCM crossing the barrier between a single consensus system cannot generally be synchronous.

<br>

No guarantees on delivery time.

Notes:

Generally, consensus systems are not designed to operate in sync with external systems.
They intrinsically need to have a uniform state to reason about and do not, by default, have the means to verify states of other consensus systems.
Thus, each consensus system cannot make any guarantees on the expected time required to deliver results; doing so haphazardly would cause the recipient to be blocked waiting for responses that are either late or would never be delivered, and one of the possible reasons for that would be an impending runtime upgrade that caused a change in how responses are delivered.

---

## XCM is "fire and forget"

<br>

XCM has no results:

<widget-text center>

- No errors reported to sender
- No callbacks for sender

Similar to UDP

Notes:

The receiver side can and does handle errors, but the sender will not be notified, unless the error handler specifically tries to send back an XCM that makes some sort of XCM that notifies status back to the origin, but such an action should be considered as constructing a separate XCM for the sole purpose of reporting information, rather than an intrinsic functionality built into XCM, akin to how UDP can also create a "response" to an incoming datagram, yet the response is too considered as a separate UDP datagram instance.

---

## Async XCM

We _could_ have XCM describe async behavior but do not because:

<widget-text center>

- Complexity, custom per sender/receiver pair
- Expense of operating in fee-based systems

Notes:

Asynchronous systems vary widely by implementation, and as a protocol that attempts to bridge between disparate consensus systems, XCM does not attempt to define the behavior or architecture of its interlocutors.
Rather, XCM defines and standardizes the interface and semantics that two or more consensus systems can use to interact with each other, but leaves the ultimate implementation details to its participating systems.

---

## Basic Top-Level Format

```rust
pub enum VersionedXcm {
    /*
    Obsolete versions
    #[codec(index = 0)]
    V0(v0::Xcm),
    #[codec(index = 1)]
    V1(v1::Xcm),
    */
    #[codec(index = 2)]
    V2(v2::Xcm),
    #[codec(index = 3)]
    V3(v3::Xcm),
}
```

Notes:

All the XCM format documentation does is state that the SCALE encoding index for the enums are in `u8`, and is defined by the `#[codec(index)]` attribute as we see in the code.
It also concretely states that in XCMv2, the `v2::Xcm` struct is simply defined as a wrapper type around a `Vec<Instruction>`, where `Instruction` is the enum representing the XCM instruction set that exists in version 2.

---

## Basic XCVM Operation

XCVM operates as a fetch-dispatch loop

<widget-text center>

- Common in state machines

<!-- TODO: Graphics about a state machine similar to how the XCVM operates -->

---

## XCM vs. Standard State Machine

<widget-text center>

1. Error register
1. Error _handler_ register
1. Appendix register

Notes:

1. The error register is _not_ cleared when an XCM program completes successfully.
   This allows the code in the Appendix register to use its value.
1. Code that is run in the case where the XCM program fails or errors.
   Regardless of the result, when the program completes, the error handler register is cleared.
   This ensures that error handling logic from a previous program does not affect any appended code (i.e. the code in the error handler register does not loop infinitely, the code in the Appendix register cannot access the result of the code execution in the error handler).
1. Code that is run regardless of the execution result of the XCM program.

---

## Kinds of XCM Instructions Recap

Four kinds of XCM instructions:

<widget-text center>

- Instruction
- Trusted Indication
- Information
- System Notification

Notes:

Instruction - instructions that result in a state change in the local consensus system, or instruct the local consensus system to achieve some desired behavior.
Trusted Indication - instructions that notify the recipient that the sender wants the recipient to trust that the sender has performed some sort of (usually) state-altering action.
Information - instructions that report information that the recipient has requested for.
System Notification - instructions that notify about transport protocol events.
These instructions usually originate from the relay chain.

---

## Instruction

Split into a few categories. Instructions that:

<widget-text center>

- **handle assets**
- changes the state of the XCVM registers
- report information back to the sender
- deal with version negotiation
- assert a certain condition of the XCVM
- relates to weight fee payment
- interact with other subsystems of the blockchain

---

## Instruction

<widget-text center>

handle assets:

<br>

- handle an Asset local assets only
- transfer an Asset assets to another consensus system
- deal an Asset with asset locking
- burn an Asset, recover or exchange specified assets

---

## Asset Instructions

```rust
enum Instruction {
    // instructions that handle local assets only
    WithdrawAsset(MultiAssets),
    DepositAsset { assets: MultiAssetFilter, beneficiary: MultiLocation },
    TransferAsset { assets: MultiAssets, beneficiary: MultiLocation },
    // instructions that transfer assets to another consensus system
    InitiateReserveWithdraw { assets: MultiAssetFilter, reserve: MultiLocation, xcm: Xcm },
    DepositReserveAsset { assets: MultiAssetFilter, dest: MultiLocation, xcm: Xcm },
    TransferReserveAsset { assets: MultiAssets, dest: MultiLocation, xcm: Xcm },
    InitiateTeleport { assets: MultiAssetFilter, dest: MultiLocation, xcm: Xcm },
    // instructions that deal with asset locking
    LockAsset { asset: MultiAsset, unlocker: MultiLocation },
    UnlockAsset { asset: MultiAsset, target: MultiLocation },
    RequestUnlock { asset: MultiAsset, locker: MultiLocation },
    // instructions that burn, recover or exchange specified assets
    BurnAsset(MultiAssets),
    ClaimAsset { assets: MultiAssets, ticket: MultiLocation },
    ExchangeAsset { give: MultiAssetFilter, want: MultiAssets, maximal: bool },
}
```

Notes:

The withdrawal destination is always the holding register if the instruction does not have parameters specifying the beneficiary.
Similarly, the deposit source is always from the holding register.

---

## Register State Change Instructions

```rust
enum Instruction {
    // instructions that modify the origin register
    AliasOrigin(MultiLocation),
    DescendOrigin(InteriorMultiLocation),
    UniversalOrigin(Junction),
    // instructions that provide additional instructions to execute upon certain conditions
    SetErrorHandler(Xcm),
    SetAppendix(Xcm),
    // instructions that modify the topic register
    SetTopic([u8; 32]),
    // instructions that modify the error register
    Trap(u64),
    // instructions that modify the fees mode register
    SetFeesMode { jit_withdraw: bool },
    // instructions that clears the specified register
    ClearTopic,
    ClearError,
    ClearOrigin,
    ClearTransactStatus,
}
```

Notes:

The origin register is used for access control, hence it has the most number of instructions to modify the state of it.

---

## Information Reporting Instructions

```rust
enum Instruction {
    ReportError(QueryResponseInfo),
    ReportHolding { response_info: QueryResponseInfo, assets: MultiAssetFilter },
    ReportTransactStatus(QueryResponseInfo),
    QueryPallet { module_name: Vec<u8>, response_info: QueryResponseInfo },
}
```

```rust
pub struct QueryResponseInfo {
    pub destination: MultiLocation,
    pub query_id: QueryId,
    pub max_weight: Weight,
}
```

Notes:

All of these instructions contain a `QueryResponseInfo` struct, which contains information about the intended destination of the response, the ID of the query, and the maximum weight that the dispatchable call function can use.
The dispatchable call function is an optional operation that XCM author can specify, and is executed upon receiving the response, effectively acting as a lifecycle hook on response.

---

## Version Negotiation Instructions

```rust
enum Instruction {
    SubscribeVersion { query_id: QueryId, max_response_weight: Weight },
    UnsubscribeVersion,
}
```

Notes:

SubscribeVersion - instructs the local system to notify the sender whenever the former has its XCM version upgraded or downgraded.
UnsubscribeVersion - if the sender was previously subscribed to XCM version change notifications for the local system, then this instruction tells the local system to stop notifying the sender on version changes.

---

## Assertion Instructions

```rust
enum Instruction {
    ExpectAsset(MultiAssets),
    ExpectOrigin(Option<MultiLocation>),
    ExpectError(Option<(u32, Error)>),
    ExpectPallet { index: u32, name: Vec<u8>, module_name: Vec<u8>, crate_major: u32, min_crate_number: u32 },
}
```

Notes:

Upon failure, these instructions will throw an `ExpectationFalse` error.

---

## Weight Payment Instructions

```rust
enum Instruction {
    RefundSurplus,
    BuyExecution { fee: MultiAsset, weight_limit: WeightLimit },
}
```

Notes:

For consensus systems that require senders to pay for executing XCMs, the `BuyExecution` instruction specifies which asset will be used to pay for it and the maximum weight that the sender is willing to pay.
One important point to understand is that `BuyExecution` itself doesn't contain the fees -- rather, it contains a `fee` parameter which is used to indicate which of the assets in the holding register can be used to pay for fees.
If the specified asset does not exist in holding, or if the local system does not recognize the `fee` asset as an eligible asset to pay for fees, then an error is thrown.
If the maximum weight specified by the sender in `weight_limit` is too low, then the asset up to the limit is deducted from the holding and an error is thrown.

---

## Subsystem Interaction Instructions

```rust
enum Instruction<Call> {
    ExportMessage { network: NetworkId, destination: InteriorMultiLocation, xcm: Xcm<()> },
    Transact { origin_kind: OriginKind, require_weight_at_most: u64, call: DoubleEncoded<Call> },
}
```

Notes:

We finally introduce the `Call` type parameter here to indicate where it is actually used within the `Instruction` enum, which is by the `Transact` instruction to encode the destination's runtime `Call` type.
XCMs that do not make use of the `Transact` instruction can simply use the `()` (unit) type as the type parameter to `Instruction` or `Xcm`, as evidenced by the `xcm` field in `ExportMessage`.
The result of executing the decoded `Call` parameter in `Transact` is stored in the transaction status register.

---

## Trusted Indication

Sender must have state-altering action _prior_ to sending

```rust
enum Instruction {
    ReserveAssetDeposited(MultiAssets),
    ReceiveTeleportedAsset(MultiAssets),
    NoteUnlockable { asset: MultiAsset, owner: MultiLocation },
}
```

Notes:

ReserveAssetDeposited - The specified assets must have been transferred/deposited to the recipient's sovereign account on the sender.
The sender is trusted to be acting as a reserve for the specified assets -- such a trust is configured via the `IsReserve` XCM configuration item on the recipient.
ReceiveTeleportedAsset - The specified assets must have been removed out of total circulation by the sender.
Like `ReserveAssetDeposited`, the sender needs to configured as a trusted teleport location via `IsTeleport`.
NoteUnlockable - The specified assets must have been locked by the sender.
The exact definition of "asset locking" is defined by the sender; however this would not cause any incompatibilities as long as "asset unlocking" as defined by the sender undoes the asset lock.

---

## Information

Offering some requested information that the local system is expecting

```rust
enum Instruction {
    QueryResponse { query_id: QueryId, response: Response, max_weight: Weight, querier: Option<MultiLocation> },
}
```

Notes:

This instruction is generally safe to execute, the only tidbit here is that the local system may not be expecting the response from the sender.
Therefore, the `querier` parameter should be checked to ensure that the system that requested the information matches with what is expected.

---

## System Notification

Handling operations for the underlying transport layer

```rust
enum Instruction {
    HrmpNewChannelOpenRequest { sender: u32, max_message_size: u32, max_capacity: u32 },
    HrmpChannelAccepted { recipient: u32 },
    HrmpChannelClosing { initiator: u32, sender: u32, recipient: u32 },
}
```

Notes:

<!-- TODO: Get someone familiar with XCMP to comment on whether or not these XCM instructions would still exist when we switch over to XCMP -->

---

## Common XCM patterns

Most systems expect execution fee payment

```rust
Xcm(vec![
    WithdrawAsset(some_asset), // ReceivedTeleportedAsset(..) | ReserveAssetDeposited(..) | ClaimAsset { .. }
    ClearOrigin, // optional
    BuyExecution { fee: some_asset, weight_limit: Unlimited },
    // ... the rest of the instructions
 ])
```

Notes:

This pattern is a requirement from the `AllowTopPaidExecutionsFrom` barrier condition, which we expect most systems that collect fees for execution to use.
The aim of the first instruction is to fill up the holding register with assets, so that `BuyExecution` can use the holding register to pay for execution.
`ClearOrigin` is optional here, but if it exists, it is to prevent the subsequent instructions from gaining the privileges of the origin system.

---

## XCM Errors

Error register - set upon execution failure with `XcmError` variant
Error handler register - if set, the XCM in the register is executed upon error

Notes:

The error register also includes a `u32` which denotes the instruction index that can be used to locate which XCM instruction failed.
The code in the error handler register can only be used once, otherwise the executor can go into an infinite loop if it too errors during execution.
