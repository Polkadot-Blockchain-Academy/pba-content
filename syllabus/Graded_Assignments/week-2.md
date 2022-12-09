# Week 2 Assignment - Blockchain and Substrate

This assignment is a project covering the material you've learned about writing Substrate Runtimes with FRAME. To complete this project, select one of the following options,

## Option 1: UTXO Runtime

- Do not use frame (Obviously because we haven't talked about it yet)
- No account model
- Runtime logic is similar to Bitcoin, Litecoin, Monero (not the privacy part)
- Users are able to spend and receive UTXOs
- Block reward is paid as a new UTXO
- Fees come from leftover amount of coins

## Option 2: Hybrid Consensus

- Hybrid consensus - Proof of Work Authoring with Grandpa Finality (PoA is fine)

## Option 3: Blockchain UI

Choose one fo the Frameless Runtimes that we built in class and design a user interface for it. Your user interface should allow users to read all relevant chain state and submit signed transactions. To achieve this you will need to make use of Substrate's RPC methods.

This user interface does not need to be a beautiful webapp, although that is certainly welcome). A simple CLI based UI is enough.

## Option 4: Block Explorer / Indexer

TODO