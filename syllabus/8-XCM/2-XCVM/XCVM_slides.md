## 🫀 The XCVM

At the core of XCM lies the **Cross-Consensus Virtual Machine (XCVM)**.

A “message” in XCM is an XCVM program.

The XCVM is a state machine, state is kept track in **Registers**.

Notes:

It’s an ultra-high level non-Turing-complete computer whose instructions are designed to be roughly at the same level as transactions.
Messages are one or more XCM instructions.
The program executes until it either runs to the end or hits an error, at which point it finishes up and halts.
An XCM executor following the XCVM specification is provided by Parity, and it can be extended or customized, or even ignored altogether and users can create their own construct that follows the XCVM spec.
---
### XCVM Instructions

XCM Instructions might change a register, they might change the state of the consensus system or both.
   
One example of such an instruction would be `TransferAsset` which is used to transfer an asset to some other address on the remote system.
It needs to be told which asset(s) to transfer and to whom/where the asset is to be transferred.

```rust
enum Instruction {
    TransferAsset {
        assets: MultiAssets,
        beneficiary: MultiLocation,
    }
    /* snip */
}
```
---
## XCVM Instructions preview

Four kinds of instructions:

- Instruction
- Trusted Indication
- Information
- System Notification

Notes:

`Instruction` is a bad name for the kind of XCVM instructions that we have, but it means instructions that result in a state change in the local consensus system, or instruct the local consensus system to achieve some desired behaviour.

<!-- TODO example of XCM message that intuitively makes sense for students that can reason about assets and fees, highlight lines in code block and talk to them. Highlight LOCATION and ASSET instructions, that we will go into next -->  
---
## XCVM Registers

```rust
pub struct XcmExecutor<Config: config::Config> {
    holding: Assets,
    holding_limit: usize,
    context: XcmContext,
    original_origin: MultiLocation,
    trader: Config::Trader,
    error: Option<(u32, XcmError)>,
    total_surplus: u64,
    total_refunded: u64,
    error_handler: Xcm<Config::Call>,
    error_handler_weight: u64,
    appendix: Xcm<Config::Call>,
    appendix_weight: u64,
    transact_status: MaybeErrorCode,
    fees_mode: FeesMode,
    topic: Option<[u8; 32]>,
    _config: PhantomData<Config>,
}
```

- Registers _are_ the state of XCVM
- Note that XCVM registers are temporary/transient

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

<widget-columns>
<widget-column>

### 💁 The Holding Register

`WithdrawAsset` has no location specified for assets.

They are _temporarily_ held in what in the Holding Register.

</widget-column>
<widget-column>

```rust
WithdrawAsset(MultiAssets),

// There are a number of instructions
// which operate on the Holding Register.
// One very simple one is the
// `DepositAsset` instruction.

enum Instruction {
    DepositAsset {
        assets: MultiAssetFilter,
        max_assets: u32,
        beneficiary: MultiLocation,
    }, 
    /* snip */
}
```

</widget-column>
</widget-columns>

Notes:

Let’s take a look at another XCM instruction: `WithdrawAsset`. On the face of it, this is a bit like the first half of `TransferAsset`: it withdraws some assets from the account of the place specified in the Origin Register.
But what does it do with them? — if they don’t get deposited anywhere then it’s surely a pretty useless operation.

---

<!--- TODO Add more things relating to XCM discuss with Keith and Gorka --->
