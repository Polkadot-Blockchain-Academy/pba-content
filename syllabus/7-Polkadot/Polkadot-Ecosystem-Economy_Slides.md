---
title: Polkadot Ecosystem and Economy
description: A review of the parts of Polkadot which develop its ecosystem and economy.
duration: 1 hour
---

topics to cover


- Token Functionality
	- Reserved Tokens
	- Locked Tokens
- Token Utility
	- Staking
	- Bonding
	- Free Balances
	- Governance
- Council (skip?)
- Open Governance
- Treasury
- Fellowship
- Society (skip?)

---

# The DOT Token

---

## Token State

The DOT token can be in one of the following states:

1. Transferable
2. Locked (Frozen)
2. Reserved (Held)

---

## Reserved vs Locked Balance

- New terms "Frozen" and "Held" are not quite used in Polkadot yet...
- Both states belong to the user... but cannot be spent / transferred.
- Reserved balances stack on top of one another.
	- Useful for user deposits, or other use cases where there is sybil concerns.
	- Ex: Deposit for storing data on-chain,
- Locked balances can overlap each other.
	- Useful when you want to use the same tokens for multiple use cases.
	- Ex: Using the same tokens for both staking and voting in governance.

---

## Storage Bloat

One blockchain scaling problem is storage bloat over time.

<br />

Consider the "cost" of storing data on Ethereum:

- A one time gas fee based on the amount of data stored.
- Once is it placed on the network, it lives there forever, with no additional costs.
- Over a long enough period of time, the cost of storage per time will reduce to zero.

---

## Storage Deposits

To solve this problem, Polkadot additionally takes a storage deposit (in the form of Reserved Balance) for any data stored in the blockchain.

- This deposit is returned to the user when the user removes the data from the chain.

- This deposit can be quite extreme, since it is returned to the user, and can represent the impermanence or lack of "importance" of the data.

---

## Dust Accounts & Existential Deposit

The most bloat-ful storage on most blockchains are user accounts:

- Both Ethereum and Bitcoin are riddled with "dust accounts" which have such a small balance, they are not worth "cleaning up".

- Polkadot solves this by having an "existential deposit" that all users must hold a minimum amount of DOT, else their account data will be cleaned up.

- Existential deposit can be thought of as a storage deposit for account data.

---

## DOT is a Utility Token

The DOT token serves multiple purposes to help the Polkadot network function:

- Staking
- Bonding for Parachain Slots / Execution Cores
- On-Chain Decision Making
- Value Bearing for Trading / Using

---

## Ideal Usage of DOT Tokens

<image src="../../../assets/img/7-Polkadot/ideal-token-distribution.svg" style="width: 1000px;">


Notes:

- 50% Staking / Governance
- 30% Parachains
- 20% Tradable / Useable

---

## DOT Inflation

<div class="grid grid-cols-2">

<div>

<image src="../../../assets/img/7-Polkadot/eco/inflation.svg" style="width: 500px;">

</div>

<div>

DOT is currently configured to have a fixed inflation rate of 10% per year.

Newly minted tokens are distributed to stakers (validators / nominators) and the treasury.

</div>

</div>

---

## Ideal Staking Rate

We cannot force / tell users how to use their tokens, so we encourage "ideal" behavior by associating DOT token usage to how inflation is distributed.

There’s a function that redirects some of the 10% inflation to the Treasury, instead of the stakers, when `ideal_rate != staking_rate`.

Token holders are financially incentivized to maximize their staking returns, and thus distribute their tokens appropriately.

---

## DOT Inflation vs Staking

<image src="../../../assets/img/7-Polkadot/staking-rate.png" style="width: 900px;">


> Blue: Inflation vs Staking Rate
>
> Green: APY of Stakers vs Staking Rate
>
> Black: Total Inflation vs Staking Rate

---

## DOT Utility: Parachains

Polkadot provides many utilities, but arguably its most important utility is providing flexible, secure, and scalable blockspace.

Developers can purchase this blockspace as fixed-term or on-demand Parachains, **only** with the DOT token.

<br />

> If you believe that flexible and secure blockspace has value, then you agree that DOT also has value.

---

## Expected Parachain Costs

Back of the napkin math:

- ~1 Billion DOT
- 30% Locked Up for Parachains = 300 Million
- ~100 Parachain = 3 Million DOT per Parachain Slot

At equilibrium...

---

## Ongoing Economics Updates

There is a lot of ongoing discussion about updating the economics of Parachains.

Likely, these mechanics will update pretty soon, and continually over time.

---

## DOT Utility: Staking

<div class="grid grid-cols-3">

<div class="col-span-2 text-left">

Given the existence of a value bearing token, it can be used to provide security to Polkadot:

- If users want to provide security to the network, they can stake their tokens.

- Stakers are rewarded for good behavior, and punished for bad behavior.

- It is assumed that punishments are aggressive enough that rational actors would never act maliciously.

</div>

<div>

<image src="../../../assets/img/7-Polkadot/eco/staking.svg" style="width: 400px;">

</div>

</div>

---

## Staking: Validators and Nominators

<div class="grid grid-cols-3">

<div class="col-span-2 text-left">

In the staking system, there are two roles:

- Validators: Those who run block producing / parachain validating nodes for Polkadot.
- Nominators: Users who place their tokens behind validators they think will perform their job well.

Validators (and their nominators) are rewarded based on work done for the network. Rewards may vary day to day, but should be consistent over long periods of time.

</div>

<div>

<image src="../../../assets/img/7-Polkadot/eco/collab.svg" style="width: 400px;">

</div>

</div>

---
