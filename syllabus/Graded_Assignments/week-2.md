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

Take the <TODO> Runtime that we built in class and design a User interface for it. Your user interface should allow users to see <Some Chainstate> and submit signed transactions. To achieve this you will need to make use of Substrate's RPC methods.

## Option 4: Block Explorer / Indexer