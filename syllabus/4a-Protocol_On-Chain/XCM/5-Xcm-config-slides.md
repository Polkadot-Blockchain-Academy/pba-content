---
title: XCM Configuration
description: Introduction to configuring the XCM executor
duration: 1 hour
---

# XCM Config

Notes:

We mentioned the executor is highly configurable.
What can we configure?

---v

## What you'll learn

- Different configuration items available in the XCM executor.
- The tools available to help you configure them.

---

# Config items

<pba-flex center>

- AssetTransactor
- Barrier
- LocationConverter
- OriginConverter
- IsReserve
- IsTeleporter
- Weigher

Notes:

We'll briefly introduce you to all these concepts.
It might be a bit dry, but the idea is to jump straight to the workshop afterwards.

These are all associated types on an `XcmConfig` struct.

---

# Asset Transactor

<pba-flex center>

- Withdrawing
- Depositing

Notes:

Lets the executor handle withdrawing and depositing assets.

---v

## Available adapters

- Fungible(s) adapter: pallet-balances/pallet-assets
- Nonfungible(s) adapter: pallet-uniques/pallet-nfts

Notes:

Each can be configured to support different assets.
Of course you can write your own as well.

---

# Barrier

We can receive any message from chains we are connected to.

Do we want to?

Notes:

The barrier lets us filter incoming messages based on their structure.
It's a firewall.

---v

## Available barriers

<pba-flex center>

- TakeWeightCredit
- AllowTopLevelPaidExecutionFrom
- WithComputedOrigin
- AllowUnpaidExecutionFrom
- AllowExplicitUnpaidExecutionFrom
- AllowKnownQueryResponses
- AllowSubscriptionsFrom

Notes:

These are some of the most relevant barriers provided.
You can of course write your own and filter whatever you want.
They come with some degree of customizability.

---

# LocationConverter

Way to convert from `Location` to `AccountId`.

Notes:

We always deal with locations on XCM, never accounts.
This config item converts locations to accounts.
This means pallets can have account, external locations can also have accounts.

---v

## Available converters

<pba-flex center>

- AccountId32Aliases
- HashedDescription

Notes:

The first just grabs an AccountId32 junction and gets the local account from it.
The second is very generic and can be used to generate accounts for any type of junction.

---

## OriginConverter

Used for converting locations to FRAME origins.

---v

## Available converters

<pba-flex center>

- SovereignSignedViaLocation
- ParentAsSuperuser

Notes:

SovereignSignedViaLocation just creates a signed origin from the account derived by
the location converter.

ParentAsSuperuser creates a root origin if the message came from the parent.

---

## IsReserve and IsTeleporter

Specifies reserve locations for particular assets.

---v

## IsReserve and IsTeleporter: helpers

<pba-flex center>

- NativeAsset
- Case<T>

Notes:

NativeAsset accepts any chain as a reserve or teleporter for their own native asset.

Case<T> just takes a tuple of an `AssetFilter` and a `Location`.
It specifies that location is a reserve or teleporter for those assets.

---

## Weigher

The weigher weighs XCMs, it assigns a weight to each instruction.

---v

## Available weighers

<pba-flex center>

- FixedWeightBounds (testing)
- WeightInfoBounds (production)

Notes:

FixedWeightBounds is only for testing, it assigns a constant weight to each instruction.

WeightInfoBounds uses benchmarks for assigning different weights to different instructions.

---v

## Weighing

XCM instructions need to be weighed by each runtime!

Different configuration items change the benchmarks.

---

# Next steps

We'll look at the XCM simulator, a way to let us experiment configuring XCM and sending and executing messages.
