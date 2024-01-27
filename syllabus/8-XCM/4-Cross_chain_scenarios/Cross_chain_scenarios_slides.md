---
title: Cross-chain scenarios
description: Exploring and end-to-end cross-chain use-case.
duration: 1 hour
---

# Cross-chain scenarios

---v

## What we've seen so far

<pba-flex center>

- What XCM is and its main concepts
- How XCM programs are written and executed
- The interface between FRAME and XCM

---v

## What we'll look into now

<pba-flex center>

- Cross-chain examples
- Cross-chain composability
- Attack scenarios

---v

## At the end of this lecture, you will be able to:

<pba-flex center>

- Build existing or new cross-chain use-cases
- Leverage services offered by the broader Polkadot ecosystem for your apps
- Mitigate risk during cross-chain interactions

---

# Cross-chain DEX

Notes:

Let's start with a cross-chain DEX.
Say we have a DEX chain working, with liquidity pools and users.
It's only working for accounts on that chain, but we'd like to extend it to the broader Polkadot ecosystem.

---v

## Requirements

<pba-flex center>

- Accounts on other chains can transfer tokens to our chain
- They can do swaps without an account on our chain
- They can add/remove liquidity without an account on our chain

---v

## Diagram

<diagram class="mermaid">
graph LR
A(A) --"Transfer"--> DEX(DEX)
DEX --"Swap/Liquidity"--> DEX
</diagram>

Notes:

Swaps and liquidity exist on our DEX chain.
However, we want an end user that already has an account on the A chain to simply use the funds it has there to interact with our chain.
No need for them to create an account on our chain.
How do we do that?

---

## Configuring XCM

The XCM configuration translates the XCVM instructions into actual native runtime functionality of our chain.

Notes:

The first step is to add the XCM executor to our chain.

The executor is highly configurable, we need to make sure to tailor it to our use-case.

---v

## AssetTransactor

<pba-flex center>

```rust
type AssetTransactor = FungiblesAdapter<
  Assets,
  Matcher,
  LocationToAccountId,
  ...
>;
```

Notes:

This is a type that exists in `xcm-builder` to help adapt existing FRAME pallets to the XCVM.
`Assets` is the assets pallet as configured in `construct_runtime`.
`Matcher` is a type that can convert from XCM `Location`s into the actual asset IDs our chain works with.
`LocationToAccountId` converts from XCM `Location`s to account ids on our local chain.

There are other adapters ready for other pallets, like:

- `FungibleAdapter` for the balances pallet
- `NonFungiblesAdapter` for the uniques pallet

---v

## Teleports or reserve asset transfers?

<pba-flex center>

```rust
type IsReserve = ();
type IsTeleporter = ();
```

Notes:

We can either use teleports or reserve asset transfers for moving assets between chains.
Which one should we use here?
In general we want reserve asset transfers since the real asset stays on the reserve and we just get a derivative of it, requires less trust.
Once we configure the locations we trust as reserves, we can start receiving transfers from them.

---v

## Reserve asset transfers

<pba-flex center>

```rust
Xcm(vec![
  WithdrawAsset(withdraw_amount),
  DepositReserveAsset {
    assets: All.into(),
    dest: (Parent, Parachain(dex_para_id)).into(),
    xcm: Xcm(vec![
      DepositAsset { assets, beneficiary },
    ]),
  },
])
```

Notes:

We could also use the XCM pallet's `transfer_assets` extrinsic, or `limited_reserve_asset_transfer`, to transfer assets over to our DEX chain.
`transfer_assets` will use our configuration to realize if it should use a teleport or a reserve asset transfer.
However, with this custom approach, we can add the rest of our program afterwards.

---v

## Sovereign account

<diagram class="mermaid">
graph LR
A(A) --"XCM"-->DEX(DEX)
A -.- ALocation("AccountId32(0x...)")
DEX -.- DEXLocation("../Parachain(A)/AccountId32(0x...)")
</diagram>

Notes:

The XCM Location on A is just a wrapper around the account on that chain.
The Location of the message that our DEX chain receives references the A chain.
Where do the funds go?

---v

## XCM Location -> AccountId

<pba-flex center>

```rust
type LocationToAccountId = (
  AccountId32Aliases<Network, AccountId>,
  HashedDescription<
    AccountId,
    DescribeFamily<DescribeAccountId32Terminal>
  >,
);
```

Notes:

The funds arrive to the sovereign account of the account that sent them.
`LocationToAccountId` transforms the location to an account id.
`AccountId32Aliases` just removes the wrapper for local accounts.

`HashedDescription` is more interesting.
Given an arbitrary location, it calculates the account by hashing the description of the parents and junctions.
This is used for figuring out the sovereign account.

Once the sovereign account over on the DEX chain is funded, we can use it for swapping.

---

## Swaps

<pba-flex center>

```rust
enum Instruction<Call> {
  // ...snip...
  ExchangeAsset {
    give: AssetFilter,
    want: Assets,
    maximal: bool,
  },
  // ...snip...
}
```

Notes:

This instruction can be used to express a swap.
You specify how much you're willing to give and how much you want in return.
If `maximal`, you use up all the `give` and accordingly receive more than `want`.
If not, you use the least amount of `give` and get at least `want`.
This can be paired up with other instructions to send the XCM we want.

---v

## Make the swap

<pba-flex center>

```rust
Xcm(vec![
  WithdrawAsset(withdraw_amount),
  BuyExecution { fees, weight_limit },
  ExchangeAsset { give, want, maximal },
  DepositAsset { assets, beneficiary },
])
```

Notes:

A program like this sent to the DEX chain will:

- take the assets from our sovereign account
- pays for execution
- swap the tokens we specified
- deposit them back into the sovereign account

What if we didn't include the `BuyExecution` instruction? We'd be able to exchange for more tokens.

---v

## Denial of Service attack

Imagine a million of these:

<pba-flex center>

```rust
Xcm(vec![
  WithdrawAsset(assets),
  DepositAsset { assets, beneficiary },
])
```

Notes:

If we don't enforce fee payment, then someone could send a ton of small messages to our chain, forcing our chain to execute a lot of work for free.

---v

## Barriers to the rescue

<pba-flex center>

```rust
type Barrier: ShouldExecute;
```

Notes:

The executor can be configured with barriers that are run for each message that wants to be executed.
The barriers determine whether the message can be executed or whether it's dropped.

---v

## Requiring fees to be paid

<pba-flex center>

```rust
type Barrier = AllowTopLevelPaidExecutionFrom<Everything>;
```

Notes:

This barrier is in `xcm-builder` and will reject any message that doesn't intend to pay fees.
This translates to not including the `BuyExecution` instruction.
With this, we exclude the messages that don't pay fees.

The `Everything` specifies we don't actually filter any origin of the messages, as long as they pay for their own execution.

---v

## Up until now

```rust
Xcm(vec![
  WithdrawAsset(withdraw_amount),
  DepositReserveAsset {
    assets: All.into(),
    dest: (Parent, Parachain(dex_para_id)).into(),
    xcm: Xcm(vec![
      BuyExecution { fees, weight_limit },
      ExchangeAsset { give, want, maximal },
      DepositAsset { assets, beneficiary },
    ]),
  },
])
```

Notes:

Until now, we do the transfer to the DEX, do the swap, and deposit the tokens in the sovereign account.

---

## Getting the tokens back

<pba-flex center>

- We've sent the tokens
- We made a swap
- Now how do we get them back into our chain?

---v

## Message to get them back

<pba-flex center>

```rust
InitiateReserveWithdraw {
  assets: All.into(),
  reserve: (Parent, Parachain(a_para_id)).into(),
  xcm: Xcm(vec![
    BuyExecution { fees, weight_limit },
    DepositAsset { assets, beneficiary },
  ]),
}
```

Notes:

The message is different because the reserve is not where it's executed, it's the destination.

---v

## Putting it all together

<pba-flex center>

```rust
Xcm(vec![
  WithdrawAsset(withdraw_amount),
  BuyExecution { fees, weight_limit },
  DepositReserveAsset {
    assets: All.into(),
    dest: (Parent, Parachain(dex_para_id)).into(),
    xcm: Xcm(vec![
      BuyExecution { fees, weight_limit },
      ExchangeAsset { give, want, maximal },
      InitiateReserveWithdraw {
        assets: All.into(),
        reserve: (Parent, Parachain(a_para_id)).into(),
        xcm: Xcm(vec![
          BuyExecution { fees, weight_limit },
          DepositAsset { assets, beneficiary },
        ]),
      }
    ]),
  },
])
```

---

## Summary

<pba-flex center>

In this lecture, we learnt:

- How chains interpret locations and turn them to accounts and FRAME origins
- How to set a barrier to protect our chain from attacks
- What adapters are available to translate XCM `Assets` to FRAME assets
