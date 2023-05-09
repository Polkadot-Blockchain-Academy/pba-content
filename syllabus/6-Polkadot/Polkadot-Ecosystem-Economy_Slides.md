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

# Mechanics of the DOT Token

---

## Token State

The DOT token can be in one of the following states:

1. Free Balance
	- Spendable Balance
	- Locked Balance
2. Reserved Balance

---

## Reserved vs Locked Balance

- Both states belong to the user... but cannot be spent / transferred.
- Reserved balances stack on top of one another.
	- Useful for user deposits, or other use cases where there is sybil concerns.
	- Ex: Deposit for storing data on-chain,
- Locked balances can overlap each other.
	- Useful when you want to use the same tokens for multiple use cases.
	- Ex: Using the same tokens for both staking and voting in governance.


---

## Reserved Balances

<image src="../../../assets/img/5-Polkadot/reserved-balance.svg" style="width: 1000px">

---

## Reserved Balance Example

<image src="../../../assets/img/5-Polkadot/reserved-balance-example.svg" style="width: 1000px">

---

## Locked Balances

<image src="../../../assets/img/5-Polkadot/locked-balance.svg" style="width: 1000px">

---

## Locked Balance Example

<image src="../../../assets/img/5-Polkadot/locked-balance-example.svg" style="width: 1000px">

---

## Storage Bloat

One blockchain scaling problem is storage bloat over time.

Consider the "cost" of storing data on Ethereum: a one time gas fee based on the amount of data stored.

Once is it placed on the network, it lives there forever, with no additional costs.

Over a long enough period of time, the cost of storage per time will reduce to zero.

---

## Storage Deposits

To solve this problem, Polkadot additionally takes a storage deposit (in the form of Reserved Balance) for any data stored in the blockchain.

This deposit is returned to the user when the user removes the data from the chain.

This deposit can be quite extreme, since it is returned to the user, and can represent the impermanence or lack of "importance" of the data.

---

## Dust Accounts & Existential Deposit

The most bloat-ful storage on most blockchains are user accounts.

Both Ethereum and Bitcoin are riddled with "dust accounts" which have such a small balance, they are not worth "cleaning up".

Polkadot solves this by having an "existential deposit" that all users must hold a minimum amount of DOT, else their account data will be cleaned up.

---

# DOT Token Utilities

---

## DOT is a Utility Token

The DOT token serves multiple purposes to help the Polkadot network function:

- Staking
- Bonding for Parachain Slots
- On-Chain Decision Making
- Value Bearing for Trading / Using

---

## Ideal Usage of DOT Tokens

TODO Pie Chart

- 50% Staking / Governance
- 30% Parachains
- 20% Tradable

---

## Ideal Inflation Rate

We cannot force / tell users how to use their tokens, so we encourage "ideal" behavior by associating DOT token usage to the inflation rate of the token.

Token holders are financially incentivized to maximize inflation (to maximize their staking returns), and thus distribute their tokens appropriately.

---

## DOT Inflation vs Staking

TODO create a graph which shows the inflation rate from 0% to 100% of DOT tokens being staked.

---
