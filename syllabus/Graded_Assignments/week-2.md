# Week 2 Assignment - Blockchain and Substrate

This assignment covers the material from Module 3: Blockchain Fundamentals and Module 4: Substrate. In it, you will build a cryptocurrency Substrate runtime, a simple user interface, and gently touch Substrate's consensus layer.

Your project should include a README (roughly 200 - 2000 words) explaining how to use the project and what parts of code you want the grader to look at.

## Primary Task: The Runtime

Choose one of these two options to complete. You do not need to do both.

### Option 1: UTXO-based Cryptocurrency

- Do not use frame (Obviously because we haven't talked about it yet)
- No account model
- Runtime logic is similar to Bitcoin, Litecoin, Monero (not the privacy part)
- Users are able to spend and receive UTXOs
- Block reward is paid as a new UTXO
- Fees are implied by the difference between the consumed and produced UTXOs

### Option 2: Account-based Cryptocurrency

- Do not use frame (Obviously because we haven't talked about it yet)
- Must use account model
- Runtime logic is similar to Ethereum's Eth token (not including the EVM)
- Users are able to send and receive tokens
- Block reward is paid to the author's account
- Fees are specified explicitly by the user

## Secondary Task: User Interface

Build a user-interface for the blockchain you built above. Your user interface should allow users to read all relevant chain state and submit signed transactions. To achieve this you will need to make use of Substrate's RPC methods.

This user interface does not need to be a beautiful webapp, although that is certainly welcome. A simple CLI based UI is enough.

## Tertiary Task: Consensus

Make your blockchain use a hybrid Proof of Work / Proof of Authority consensus scheme. Use Proof of Work for block authoring. You may look at the code for the academy-pow chain that we ran in module three for inspiration. Make it use Proof of Authority Grandpa for deterministic finality. You may look at the Substrate node template for inspiration.
