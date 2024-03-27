---
title: XCVM
description: Learning about the XCVM state machine
duration: 1 hour
---

# XCVM

---

# 🫀 The XCVM

At the core of XCM lies the **Cross-Consensus Virtual Machine (XCVM)**.

A “message” in XCM is an XCVM program, which is a sequence of instructions.

The XCVM is a state machine, state is kept track in **registers**.

Notes:

It’s an ultra-high level non-Turing-complete computer.
Messages are one or more XCM instructions.
The program executes until it either runs to the end or hits an error, at which point it finishes up and halts.
An XCM executor following the XCVM specification is provided by Parity, and it can be extended or customized, or even ignored altogether and users can create their own construct that follows the XCVM spec.

---

# XCVM Registers

<diagram class="mermaid">
graph LR
    subgraph Registers[ ]
        Holding(Holding)
        Origin(Origin)
        More(...)
    end
</diagram>

Notes:

Registers _are_ the state of XCVM.
Note that they are temporary/transient.
We'll talk about are the `holding` and `origin` registers, but there are more.

---v

## 📍 The Origin Register

Contains an `Option<Location>` of the cross-consensus origin where the message originated from.

Notes:

This `Location` can change over the course of program execution.

It might be `None` because some instructions clear the origin register.

---v

### 💸 The Holding Register

Expresses a number of assets in control of the xcm execution that have no on-chain representation.

They don't belong to any account.

It can be seen as the register holding "unspent assets".

---

# 📜 XCVM Instructions

XCVM Instructions might change a register, they might change the state of the consensus system or both.

---v

## Kinds of instructions

<pba-flex center>

- Command
- Trusted Indication
- Information
- System Notification

---v

## Example: WithdrawAsset

An instruction used to get assets from an account and put them into the holding register.

<pba-flex center>

```rust
WithdrawAsset(Assets)
```

Notes:

This instruction is a command.
It takes the assets from the account specified in the origin register and puts them in the holding register.

---v

## Example: ReceiveTeleportedAsset

<pba-flex center>

Used for teleporting assets between two systems.

```rust
ReceiveTeleportedAsset(Assets)
```

Notes:

This instruction is a trusted indication.
It tells the receiver that the sender has burnt some assets and they should be minted here.
This is used for teleports, which we'll look into in the next lecture.
A lot of trust is needed between both systems.

---v

## Example: QueryResponse

Used for reporting information back to another system.

<pba-flex center>

```rust
QueryResponse {
    #[codec(compact)]
    query_id: QueryId,
    response: Response,
    max_weight: Weight,
    querier: Option<Location>,
}
```

Notes:

This instruction is reporting back information.
Different things can be reported, like a certain pallet, the result of an operation, etc.

---

# Basic XCVM Operation <!-- Presenting the example program -->

<diagram class="mermaid">
graph LR
    subgraph Program
        direction LR
        WithdrawAsset-->BuyExecution
        BuyExecution-->DepositAsset
    end
</diagram>

Notes:

Here we have a very simple program, which if executed locally would result in a regular asset transfer within a single consensus system.

---v

## XCVM Operation <!-- Fetching WithdrawAsset instruction -->

<diagram class="mermaid">
graph LR
    subgraph Program
        WithdrawAsset-->BuyExecution:::disabled
        BuyExecution-->DepositAsset:::disabled
    end
    Executor(Executor)--"Fetch"-->WithdrawAsset
    subgraph Registers
        Origin(Origin)
        Holding(Holding)
    end
    Registers-.-Executor
    linkStyle 0 opacity:0.3
    linkStyle 1 opacity:0.3
    classDef disabled opacity:0.3
</diagram>

Notes:

The XCVM, or executor, fetches instruction from the program and executes them one by one.
It starts with the `WithdrawAsset` instruction, which loads the holding register using assets from the location specified in the origin register.

---v

## XCVM Operation <!-- Getting assets from Location specified by the Origin register -->

<diagram class="mermaid">
graph LR
    subgraph Program
        WithdrawAsset-->BuyExecution:::disabled
        BuyExecution-->DepositAsset:::disabled
    end
    Executor(Executor)--"Fetch"-->WithdrawAsset
    Executor--"Get assets"-->Origin
    subgraph Registers
        Origin(Origin)
        Holding(Holding)
    end
    Registers-.-Executor
    linkStyle 0 opacity:0.3
    linkStyle 1 opacity:0.3
    classDef disabled opacity:0.3
</diagram>

---v

## XCVM Operation <!-- Putting assets in Holding register -->

<diagram class="mermaid">
graph LR
    subgraph Program
        WithdrawAsset-->BuyExecution:::disabled
        BuyExecution-->DepositAsset:::disabled
    end
    Executor(Executor)--"Fetch"-->WithdrawAsset
    Executor--"Put assets"-->Holding
    subgraph Registers
        Origin(Origin)
        Holding(Holding)
    end
    Registers-.-Executor
    linkStyle 0 opacity:0.3
    linkStyle 1 opacity:0.3
    classDef disabled opacity:0.3
</diagram>

---v

## XCVM Operation <!-- Fetching BuyExecution instruction -->

<diagram class="mermaid">
graph LR
    subgraph Program
        WithdrawAsset:::disabled-->BuyExecution
        BuyExecution-->DepositAsset:::disabled
        DepositAsset
    end
    Executor(Executor)--"Fetch"-->BuyExecution
    linkStyle 0 opacity:0.3
    linkStyle 1 opacity:0.3
    classDef disabled opacity:0.3
</diagram>

---v

## XCVM Operation

<diagram class="mermaid"> <!-- Fetching DepositAsset instruction -->
graph LR
subgraph Program
WithdrawAsset:::disabled-->BuyExecution:::disabled
BuyExecution-->DepositAsset
DepositAsset
end
Executor(Executor)--"Fetch"-->DepositAsset
subgraph Registers
Holding(Holding)
Origin(Origin)
end
Registers-.-Executor
linkStyle 0 opacity:0.3
linkStyle 1 opacity:0.3
classDef disabled opacity:0.3
</diagram>

---v

## XCVM Operation

<diagram class="mermaid"> <!-- Getting assets from Holding register -->
graph LR
subgraph Program
WithdrawAsset:::disabled-->BuyExecution:::disabled
BuyExecution-->DepositAsset
DepositAsset
end
Executor(Executor)--"Fetch"-->DepositAsset
subgraph Registers
Holding(Holding)
Origin(Origin)
end
Registers-.-Executor
Executor--"Get assets"-->Holding
linkStyle 0 opacity:0.3
linkStyle 1 opacity:0.3
classDef disabled opacity:0.3
</diagram>

---v

## XCVM Operation

<diagram class="mermaid"> <!-- Putting assets in Location specified by the beneficiary field -->
graph LR
subgraph Program
WithdrawAsset:::disabled-->BuyExecution:::disabled
BuyExecution-->DepositAsset
DepositAsset
end
Executor(Executor)--"Fetch"-->DepositAsset
subgraph Registers
Holding(Holding)
Origin(Origin)
end
Registers-.-Executor
DepositAsset-.-Beneficiary(Beneficiary)
Executor--"Put assets"-->Beneficiary
linkStyle 0 opacity:0.3
linkStyle 1 opacity:0.3
classDef disabled opacity:0.3

</diagram>

---v

## XCVM vs. Standard State Machine

<pba-flex center>

1. Error _handler_ register
2. Appendix register

Notes:

1. Code that is run in the case where the XCM program fails or errors.
   Regardless of the result, when the program completes, the error handler register is cleared.
   This ensures that error handling logic from a previous program does not affect any appended code (i.e. the code in the error handler register does not loop infinitely, the code in the Appendix register cannot access the result of the code execution in the error handler).
2. Code that is run regardless of the execution result of the XCM program.

---

## Reanchoring

How do different locations reference the same asset?

<diagram class="mermaid limit size-70">
graph TD
    Polkadot(Polkadot)-->AssetHub("Asset Hub (1000)")
    Polkadot-->BridgeHub("Bridge Hub (1002)")
    AssetHub-->Alice(Alice)
    AssetHub-->AssetsPallet(Pallet Assets)
    AssetsPallet-->Asset(USDT)
</diagram>

Notes:

Locations are relative, so they must be updated and rewritten when sent to another chain, for them to be interpreted correctly.

Native tokens are referenced by the location to their system.

---v

### USDT from Asset Hub

`PalletInstance(50)/GeneralIndex(1984)`

<diagram class="mermaid limit size-70">
graph TD
    Polkadot(Polkadot):::disabled-->AssetHub("📍 Asset Hub (1000)")
    Polkadot-->BridgeHub("Bridge Hub (1002)"):::disabled
    AssetHub-->Alice(Alice):::disabled
    AssetHub-->AssetsPallet(Pallet Assets)
    AssetsPallet-->Asset(USDT)
    linkStyle 0 opacity:0.3
    linkStyle 1 opacity:0.3
    linkStyle 2 opacity:0.3
    classDef disabled opacity:0.3
</diagram>

---v

### USDT from Bridge Hub

`../Parachain(1000)/PalletInstance(50)/GeneralIndex(1984)`

<diagram class="mermaid limit size-70">
graph TD
    Polkadot(Polkadot)-->AssetHub("Asset Hub (1000)")
    Polkadot-->BridgeHub("📍 Bridge Hub (1002)")
    AssetHub-->Alice(Alice):::disabled
    AssetHub-->AssetsPallet(Pallet Assets)
    AssetsPallet-->Asset(USDT)
    BridgeHub-->Polkadot
    linkStyle 1 opacity:0.3
    linkStyle 2 opacity:0.3
    linkStyle 5 stroke-dasharray:5
    classDef disabled opacity:0.3
</diagram>

---v

### Reanchoring to the rescue

<diagram class="mermaid">
graph LR
    subgraph OutgoingMessage[Outgoing message from Bridge Hub]
        USDTBridgeHub(USDT from Bridge Hub's perspective)
    end
    USDTBridgeHub--Reanchoring-->USDTAssetHub
    subgraph IncomingMessage[Incoming message in Asset Hub]
        USDTAssetHub(USDT from Asset Hub's perspective)
    end
</diagram>

---

## 🤹 Cross-consensus transfers

Notes:

The two ways of transferring assets between consensus systems are teleports and reserve transfers.

---v

### 1. Asset teleportation

<img rounded style="width: 500px;" src="../img/teleport.png" />

Notes:

Teleportation works by burning the assets on the source chain and minting them on the destination chain.
This method is the simplest one, but requires a lot of trust, since failure to burn or mint on either side will affect the total issuance.

---v

### 1.1. Example: System parachains?

<diagram class="mermaid">
graph LR
    BridgeHub(Bridge Hub)--"Trust"-->AssetHub(Asset Hub)
</diagram>

---v

### 1.2. Example: Polkadot and Kusama?

<diagram class="mermaid">
graph LR
    Polkadot(Polkadot)--"No trust"-->Kusama(Kusama)
</diagram>

---v

### 2. Reserve asset transfers

<img rounded style="width: 400px;" src="../img/reserve-tx.png" />

Notes:

Reserve asset transfers are more complicated, since they bring in a third actor called the reserve chain.
Chain A and B needn't trust each other, they only need to trust the reserve chain.
The reserve chain holds the real assets, A and B deal only with derivatives.
The transfer is made by burning derivatives from A, moving them from A's SA to B's SA in R, then minting on B.

In some cases, the sender, A, can also be the reserve for a particular asset, in which case the process is simplified, there's no burning of derivatives.
This usually happens with parachains' native tokens.

You always trust the issuer of the token to not mint infinite tokens.

---v

### 2.1. Example: Parachain native tokens

<diagram class="mermaid">
graph LR
    subgraph A [A = R]
        Sender(Sender account)--"Move X real asset"-->BSovereignAccount(B's Sovereign Account)
    end
    A--"Mint X derivatives"-->B(B)
</diagram>

Notes:

Most parachains act as the reserve for their own token.
To transfer their token to other chains, they move the real assets to a sovereign account and then tell the chain to mint equivalent derivatives.

---v

### 2.2. Example: Polkadot to Kusama

<diagram class="mermaid">
graph LR
    Polkadot(Polkadot)-->AssetHubP
    subgraph AssetHubP [Asset Hub Polkadot]
        Sender(Sender account)--"Move X real DOT"-->KusamaSovereignAccount("Kusama's sovereign account")
    end
    AssetHubP--"Mint X DOT derivatives"-->Kusama(Kusama)
</diagram>

Notes:

AssetHub Kusama acts as the reserve for KSM.
Kusama doesn't trust Polkadot to teleport KSM to it, but it does trust its own reserve, the AssetHub.
Polkadot has a sovereign account in Kusama's AssetHub with some amount of KSM.
Whenever some user in Polkadot wants to get KSM on Kusama, they just give the DOT to Polkadot and the KSM are moved from one sovereign account to another.
No new trust relationships are added.

---

# 💁 XCM by example

---v

## The `WithdrawAsset` instruction

<pba-flex center>

```rust
enum Instruction {
    /* snip */
    WithdrawAsset(Assets),
    /* snip */
}
```

Notes:

There are a number of instructions
which place assets on the Holding Register.
One very simple one is the
`WithdrawAsset` instruction.

It withdraws some assets from the account of the location specified in the Origin Register.
But what does it do with them?
If they don’t get deposited anywhere then it's a pretty useless operation.
These assets are held in the holding register until something is done with them, for example, using the following instruction.

---v

## The `BuyExecution` instruction

<pba-flex center>

```rust
enum Instruction {
    /* snip */
    BuyExecution {
        fees: Asset,
        weight_limit: WeightLimit,
    },
    /* snip */
}
```

Notes:

This instruction uses the specified assets in the Holding register to buy weight for the execution of the following instructions.
It's used in systems that pay fees.

`weight_limit` is a sanity check, to make sure that the execution errors if you would buy more than that weight.
The estimate for the weight has to come from using the recipient's weigher, not the sender's.
The recipient is the one who actually executes the message.

---v

## The `DepositAsset` instruction

<pba-flex center>

```rust
enum Instruction {
    /* snip */
    DepositAsset {
        assets: AssetFilter,
        beneficiary: Location,
    },
    /* snip */
}
```

Notes:

Takes assets from the holding register and deposits them in a beneficiary.
Typically an instruction that places assets into the holding register would have been executed previously.

---v

## Putting it all together

<pba-flex center>

```rust
Xcm(vec![
    WithdrawAsset((Here, amount).into()),
    BuyExecution {
        fees: (Here, amount).into(),
        weight_limit: Limited(sanity_check_weight_limit)
    },
    DepositAsset { assets: All.into(), beneficiary: AccountId32 { ... }.into() },
])
```

Notes:

All examples in these slides use the latest xcm version.

---v

## Good pattern

<pba-flex center>

```rust
Xcm(vec![
    WithdrawAsset((Here, amount).into()),
    BuyExecution {
        fees: (Here, amount).into(),
        weight_limit: Limited(sanity_check_weight_limit)
    },
    DepositAsset { assets: All.into(), beneficiary: AccountId32 { ... }.into() },
    RefundSurplus,
    DepositAsset { assets: All.into(), beneficiary: sender }
])
```

---

# Reserve asset transfer

<pba-flex center>

```rust
Xcm(vec![
    WithdrawAsset(asset),
    InitiateReserveWithdraw {
        assets: All.into(),
        reserve: reserve_location,
        xcm: /* ...what to do with the funds in the reserve... */,
    },
])
```

Notes:

This message is executed locally.
Then, a message is sent to the `reserve` location.
That message contains the custom `xcm` provided along with other instructions.

---v

## Message received in reserve

<pba-flex center>

```rust
Xcm(vec![
    WithdrawAsset(reanchored_asset),
    ClearOrigin, // <- Why is this needed?
    /* ...custom instructions... */
])
```

Notes:

This is the message the reserve receives.

The `ClearOrigin` instruction deletes the content of the origin register.
This is needed because we don't trust the origin to do anything other than move its own assets.

---v

## Custom XCM

<pba-flex center>

```rust
let xcm_for_reserve = Xcm(vec![
    DepositReserveAsset {
        assets: All.into(),
        dest: location,
        xcm: Xcm(vec![
            DepositAsset {
                assets: All.into(),
                beneficiary: AccountId32 { ... }.into(),
            },
        ]),
    },
]);
```

Notes:

For a simple reserve asset transfer, this message will work.

---v

## Message received in destination

<pba-flex center>

```rust
Xcm(vec![
    ReserveAssetDeposited(reanchored_asset),
    ClearOrigin, // <- Why is this needed?
    /* ...custom instructions... */
])
```

Notes:

A very clear exploit in not having `ClearOrigin` here is syphoning all funds from
the reserve's sovereign account in the destination.
The destination can't trust the reserve to totally speak for the source, only for the assets.

---

# Summary

<pba-flex center>

- XCVM
- Kinds of instructions
- Registers: Origin, Holding, Error handler, Appendix
- Reanchoring
- Cross-consensus transfers
  - Teleports
  - Reserve asset transfers
- Instructions
  - WithdrawAsset, BuyExecution, DepositAsset
  - RefundSurplus
  - InitiateReserveWithdraw, ReserveAssetDeposited

---v

## Next steps

<pba-flex center>

1. Blog series introducing XCM: Parts [1](https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392), [2](https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83), and [3](https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166).
2. XCM Format [repository](https://github.com/paritytech/xcm-format)
3. XCM [Docs](https://paritytech.github.io/xcm-docs/)
