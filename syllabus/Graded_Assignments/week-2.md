# Week 2 Assignment - Simple On-Chain Bridge

This assignment covers material from Module 3: Blockchain and Module 4: Smart Contracts.
In it, you will build a btc-relay-like on-chain bridge smart contract.
You will build this bridge twice in two different smart contracting languages of your choosing.

## An Evm Language

For your first implementation, you need to use a language that targets the EVM.
This basically means Solidity or Vyper (although if you know some esoteric evm language, you are welcome to use that as well).

## A Wasm Language

For your second implementation, you need to use a language that targets wasm (the pallet-contracts interface).
This basically means either ink! or ask (although, again, more exotic ones are also welcome).

## The Bridge Spec

Major TODO here.
Basically we want staked users to be able to submit new headers.
The contract should have a challenge period during which challenges can result in slashed stake, details tbd.
The contract should track forks near the tip.
Contract should store all header hashes back to some starting point.
Does it need to track all headers?

Basically look at btc relay, see what it does, simplify it, and require that.