---
title: Smart contracts fundamentals
description: Introduction to fundamentals smart conctracts concepts
duration: 30min
---

## Smart contracts fundamentals

Notes:
The goal of the lecture is to define the basic definition of what a SC is
and explain the advantage and trade off of a smart contract chain

---

## Why

Notes:

- Always start with the why!
- 2 things to answers here:
  - One, The benefits of smart contracts as a technology: Smart Contracts are a huge improvement over traditional legal contracts, that are slow, expensive, and error-prone.
    With smart contracts, you can automate the execution of agreements, enforce them automatically, and remove the need for intermediaries. E.g crop insurance.
  - Two, For the Polkadot ecosystem, it's important to embrace a technology that is widely adopted by the Ethereum
    community, so that we can leverage the tooling, and easily onramp existing dapps, but also users and developers.

---

## Smart contracts & blockchain

<section>
  <pre><code>
State n  ----->  STF(State n, [Tx1, Tx2, ...])  ----->  State n+1
  </code></pre>
</section>

Notes:

Before defining what a smart contract (SC) is, we need to understand the environment in which it operates: the blockchain.
We won’t dive deep into what a blockchain is—that will be covered in detail during the PBA. For now, it's enough to define a blockchain as:
A distributed system where all participants (nodes) execute a common set of transactions contained in a block.
These transactions are processed using a state transition function, which updates the blockchain’s state from block n to block n+1.
The state transition function is defined by the protocol itself. Additionally, consensus mechanisms determine who gets to author the next block.

Since all nodes execute the same transactions with the same rules, they all derive the same resulting state. This ensures that block n+1 is identical across all honest nodes in the network.

---v

### Bitcoin

<section>
<pre><code>
State n                                                                     State n + 1
-------                                                                     -----------

                           STF(State n, [

Alice: 2ETH tx1: transfer(1BTC Alice -> BOB), Alice: 1BTC
Bob: 1BTC ──────▶ tx2: transfer(1BTC Charlie -> BOB), ─────▶ Bob: 3BTC
Charlie: 2BTC ... Charlie 1BTC
])

</code></pre>

</section>

Notes:

In Bitcoin, the state transition function primarily processes transactions, which are mostly transfer transactions.
Accounts submit transactions, transactions are gossiped over the network, and will eventually be included in a block.
The state transition function, will validate the transactions, and update the state of the ledger with these instructions ed ledger by moving ownership of coins from one address to

---v

### Polkadot

<section>
<pre><code>
State n                                                               State n + 1
-------                                                               -----------
                                     STF(State n, [
Accounts { Alice: 1DOT, ... }           tx1: transfer(...),           Accounts { <state_n+1 }
Assets:  { <state_n> }         ─────▶  tx2: stake(...),     ─────▶  Assets:  { <state_n+1> }
Staking: { <state_n> }                  tx3: vote(...),               Staking: { <state_n+1> }
OpenGov: { <state_n> }                  ...                           OpenGov: { <state_n+1> }
...                                   ])                              ...

</code></pre>

</section>

Notes:

Unlike Bitcoin, Polkadot's state transition function is highly flexible and programmable.
Blockchains built with Substrate run a WebAssembly (Wasm) runtime, which defines how the blockchain's state is updated.
This runtime is built using Rust modules, called pallets, each defining specific transaction types.
Because this state transition function is written in a Turing-complete language like Rust, it can execute complex logic beyond simple transfers.

Different pallets allow developers to introduce specialized logic into their blockchain:
Assets & NFT pallets → Define fungible and non-fungible tokens and their operations.
Democracy pallet → Enables on-chain governance, allowing proposals to be submitted, voted on, and enacted when approved.
Other pallets can introduce staking mechanisms, cross-chain messaging, DAOs, and more.

Another key feature of Polkadot is runtime upgrades. The runtime logic can be updated without requiring a hard fork.
A runtime upgrade is simply another transaction that updates the Wasm blob stored on-chain. Once this upgrade is applied, subsequent blocks execute the new logic.

Not just anyone can execute a runtime upgrade though, these transactions require root privileges, which are only granted through governance approval via OpenGov voting.

So to recap:
Polkadot's state transition function can execute any business logic defined in the runtime.
However, the runtime itself is not permissionless—it must be explicitly defined and deployed by the chain’s developers or governance
To allow arbitrary logic execution, a Smart Contracts module must be embedded in the runtime.
Otherwise, only predefined transaction types (e.g., assets, governance, staking) can be executed.

---

### Smart contracts chain

<section>
<pre><code>
State n                                                                                 State n + 1
-------                                                                                 -----------

Alice: 2ETH STF(State n, [ Alice: 2ETH
Bob: 1ETH tx1: call(Alice -> Bob, 1ETH), Bob: 1ETH
... ───▶ tx2: call(Alice -> 0x1, 1ETH, input: 0x1234), ───▶ ...
SC 0x1: 1ETH { <state_n> } tx3: call(Alice -> 0x2, 0ETH, input: 0x4567), SC 0x1: 1ETH { <state_n+1> }
SC 0x2: 0ETH { <state_n> } ... SC 0x2: 0ETH { <state_n+1> }
])

</code></pre>

</section>

Notes:

Now, we can finally define what a smart contract is.
A smart contract is a special type of account that is not controlled by a keypair, but instead by the code it defines.
A smart contract blockchain allows users to: - Deploy contracts on-chain.

- Call these contracts to execute their logic.

You can think of smart contracts as dormant programs stored on the blockchain at a specific address.
These programs remain inactive until they are triggered by a transaction.

So you can think of a contract as "code on chain". One important aspect of smart contracts is that they are immutable. The code that defines a contract cannot be changed once it is deployed. This ensures that the contract’s behavior remains predictable and that users can trust the contract to execute

Smart contracts are executed when:

- A regular account submits a call transaction to interact with them.
- Because smart contracts are highly composable, they can also interact with each other to execute complex workflows. This means that in addition to being called by regular accounts, they can also be triggered by other smart contracts, enabling powerful on-chain interoperability.

For example, the USDC smart contract, is an ERC-20 token that maintains a mapping of balances, associating each user’s address with the amount of USDC they hold.
When a user transfers USDC, the contract updates the sender’s balance by subtracting the amount, adds the same amount to the recipient’s balance, and stores the new state on-chain to ensure all nodes remain synchronized

---

## Bytecode & Virtual Machines

<pre><code>
             ┌────────────────┐
             │      Code      │
             │(.sol, .rs, ..) │
             │                │
             └────────┬───────┘
                      │
                      │ Compiler Produces
                      │ Bytecode, ABI
           ┌──────────┴─────────────┐
           │                        │
           │                        │
           ▼                        ▼
 ┌───────────────────┐    ┌───────────────────┐
 │                   │    │                   │
 │JSON-RPC           │    │JSON-RPC           │
 │                   │    │                   │
 │Deploy call        │    │Contract Call      │
 │- bytecode         │    │- address          │
 │- ABI encoded input│    │- ABI encoded input│
 │                   │    │                   │
 └───────────────────┘    └───────────────────┘
┌───────────────────────────────────────────────┐
│                                               │
│         Execute                               │
│                                               │
│  ┌─────────────────────────────────────────┐  │
│  │             Virtual Machine             │  │
│  │                                         │  │
│  └─────────────────────────────────────────┘  │
│ ┌──────────────────────────────────────────┐  │
│ │                                          │  │
│ │KV Store:                                 │  │
│ │                                          │  │
│ │- ...                                     │  │
│ │                                          │  │
│ │- address:   0x12345                      │  │
│ │- nonce:     0                            │  │
│ │- balance    1ETH                         │  │
│ │- bytecode   0x...                        │  │
│ │- key1       value1                       │  │
│ │- key2       value2                       │  │
│ │                                          │  │
│ └──────────────────────────────────────────┘  │
└───────────────────────────────────────────────┘
</code></pre>

Notes:

When we deploy a smart contract on-chain, we first compile it into bytecode, which can be executed by a virtual machine.
Different smart contract blockchains use different bytecode formats.
For example:

- On Ethereum contracts are compiled into EVM bytecode.
- On Solana, programs are compiled into BPF bytecode.
- On Polkadot Hub as we will explore later, we use PolkaVM bytecode

The state transition function of the blockchain runs the Virtual Machine to execute the instructions in this compiled bytecode, and update the state of the chain.

---

## Core Features of Smart Contracts

---h

### A simple Example

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract PiggyBank {
    address public owner;

    constructor() {
        owner = msg.sender;
    }

    function deposit() public payable {}

    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }

    function withdraw(uint256 withdrawAmount) public {
        require(msg.sender == owner, "You are not the owner");
        require(address(this).balance >= withdrawAmount, "Insufficient balance");

        (bool success, ) = payable(msg.sender).call{value: withdrawAmount}("");
        require(success, "Transfer failed");
    }
}
```

Notes:
Developers don’t typically write bytecode by hand. Instead, they write smart contracts in high-level languages, which are then compiled into bytecode. On EVM-compatible chains, the most widely used language is Solidity, that is compiled to bytecode using the `solc` compiler. Let's go through a simple `PiggyBank` solidity contract to illustrate it.

- When a contract is deployed, the constructor code is executed
- The owner can call the deposit function to deposit some balance
- The view function `GetBalance` is a 'readonly' function that can be queried through a JSON-RPC call
- If you send ETH directly to the contract (without calling a function like deposit), it will fail unless the contract has a receive or fallback function.
  Without one of these, any direct transfer (e.g., sendTransaction from a wallet) will be rejected with an error.

---v

### Immutability

Notes:
Contracts are immutable by design, however in some circumstances, you might want to upgrade to fix a bug or add or
improve existing features. There are several patterns to achieve this, one of the most common is the Proxy pattern.
Essentially, the proxy pattern involves creating a proxy contract that delegates calls to an implementation contract.
When you want to upgrade the contract, you deploy a new implementation contract and update the proxy to point to the new implementation.

---v

#### Smart Contract Upgrade

- **Upgradability**: Immutable, unless using proxies
- Governance Model: Typically managed by a contract owner or DAO governance
- Process:
  - Deploy a new contract implementation
  - Update the proxy contract to point to the new version
  - State Migration if needed
- Overhead:
  - Gas overhead due to delegate calls and proxy interactions
  - Gas costs for state migration & new contract deployment

---v

#### Substrate Runtime Upgrade (Polkadot)

- **Upgradability**: Achieved through a Wasm runtime upgrade
- **Governance Model**: On-chain governance (OpenGov)
- Process:
  - Proposal submitted through OpenGov
  - Once approved, a runtime upgrade transaction is dispatched, and state is migrated as part of the upgrade
    Overhead:
  - Gas Costs: No gas cost (upgrade happens at the protocol level)
  - No performance overhead (new runtime code replaces the old one)

---

### Composability

Notes:
Smart contracts on EVM-based chains are highly composable, meaning they can interact with each other to execute complex workflows. A contract call is always initiated by an Externally Owned Account (EOA) through a transaction.

Methods for contract interaction include:

- **Normal Call**:

  - Contract A calls Contract B.
  - Contract B is pushed to the call stack, executes, potentially updates its state, and may call other contracts.
  - Contract B returns control to Contract A along with the result and status (reverted or successful).
    An example for this would be a contract that calls another contract to transfer tokens.

- **Static Call**:

  - Contract A calls Contract B.
  - Contract B executes but cannot make state changes.
    An example for this would be a contract that reads data from another contract, e.g a price feed.

- **Delegate Call**:
  - Contract A calls Contract B in its execution context.
  - Contract B can read and write to A's storage, akin to a library executing within A.
    An example for this would be to use a Math library to perform calculations.

A good mental model is to think of smart contracts as the API layer of web3. Your contract can tap into other contracts
to access their functionality, read and write data and execute complex workflows. A good example of that are flash loans
on the Aave protocol, where a contract can borrow funds from the protocol, execute a series of transactions and repay
the loan in the same transaction.

---v

### Precompiled Contracts

Notes:

Another important feature of smart contracts is the ability to interact with precompiled contracts.
A precompile contract is a contract whose code is defined in the client software directly.
On Ethereum, that has a slow VM, it's used to perform computation intensive operations like elliptic curve cryptography,
outside the EVM, to improve performance.
In Substrate, a Smart-Contract pallet, can leverage this to expose other features of the runtime (like staking, xcm, governance, assets) to smart contracts

---

### Gas

Notes:

You might be wondering—if a smart contract can execute any arbitrary logic, what prevents it from defining an infinite loop that could stall the entire blockchain?
To prevent this and to protect the network from spam, virtual machines are metered. Every instruction executed by the VM has an associated gas cost, which represents the computational resources required to process it.

When you submit a transaction or when a contract calls another contract, you must specify the maximum amount of gas you’re willing to pay for execution. The contract’s code will either:

- Run to completion if there is enough gas.
- Run out of gas and revert, undoing any changes made to the contract storage, but you will still pay for the gas that was consumed before the failure.

Additionally, the blockchain itself imposes limits on gas usage:
It defines a block gas limit, which sets the maximum amount of gas that can be used across all transactions in a single block.
It also defines a gas price, which determines how much fees will be paid for a given amount of gas.
blockchains usually adjust gas prices dynamically based on network demand, ensuring fees reflect current congestion levels
This system ensures that no contract can consume unlimited resources, execution remains bounded, and transaction fees dynamically adjust based on network demand.

---v

```solidity
// TODO slide with infinite loop
    while (true) {
        // ...
        // This loop will consume all gas and revert
    }
}
```

---v

#### Metered calls in EVM

<img style="width: 1200px" src="./img/frontier/GasometerDiagram.png" />

- Checks before each opcode to make sure gas can be paid
- Safe: prevents unpaid work from being done
- Deterministic: results are unambiguous
- Very inefficient: lots of branching and extra work

---v

#### Weighted calls in substrate

```rust
#[pallet::weight(T::WeightInfo::set_metadata(name.len() as u32, symbol.len() as u32))]
pub fn set_metadata(
    origin: OriginFor<T>,
    id: T::AssetIdParameter,
    name: Vec<u8>,
    symbol: Vec<u8>,
    decimals: u8,
) -> DispatchResult {
    let origin = ensure_signed(origin)?;
    let id: T::AssetId = id.into();
    Self::do_set_metadata(id, &origin, name, symbol, decimals)
}
```

Notes:
In Substrate, each call defines a pre-dispatch weight, which can depend on the input parameters. Accounts must pay the estimated execution fee upfront, and any excess is refunded after execution.

---v

#### Metered VM Execution vs. Weighted Calls in Substrate

| Feature              | EVM Chains                     | Substrate Chains                         |
| -------------------- | ------------------------------ | ---------------------------------------- |
| **Execution Model**  | Metered at runtime (Gas)       | Pre-weighted calls                       |
| **Cost Calculation** | Dynamic, based on execution    | Static, determined pre-dispatch          |
| **Performance**      | Runtime overhead               | More predictable, optimized execution    |
| **Flexibility**      | Supports arbitrary computation | Requires (benchmnarked) weights per call |

Notes:

In VM-based blockchains, execution is metered using gas.
This makes execution flexible but introduces runtime overhead due to dynamic metering.

Contract execution performance is **less predictable**, as total costs depend on actual execution flow.
Wallet usually need to dry-run the execution to define how much gas is required for the execution.

In Substrate-based chains, execution is handled differently

- Instead of metering each instruction at runtime, calls have predefined weights based on computational complexity.
- This approach enables more efficient execution compared to metered VM, as the chain doesn’t need to meter each instruction dynamically, reducing runtime overhead.

---

## Security

Notes:

Permissionless Deployment is Risky

Anyone can deploy a smart contract, but if "code is law," any bug or vulnerability can be exploited.
Attackers actively search for vulnerabilities in deployed contracts.
Even small logic errors or gas inefficiencies can be exploited for financial gain.

- Security audits are essential before deploying contracts that manage funds.
- Use battle-tested smart contract libraries (e.g., OpenZeppelin).
- Follow established design patterns to avoid common vulnerabilities (e.g., reentrancy).
- Implement proper access control to prevent unauthorized actions.

---v

### The famous reentrency DAO hack

```solidity
contract Dao {
    // ...
    mapping(address => uint256) public balances;

    function deposit() public payable {
        require(msg.value >= 1 ether, "Deposits must be no less than 1 Ether");
        balances[msg.sender] += msg.value;
    }

    function withdraw() public {
        uint256 amount = balances[msg.sender];
        require(amount > 0, "No balance to withdraw");

        // 🔴 Sends ETH before updating balance
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");

        // 🔴 Balance Update after Transfer - Allows Reentrancy!
        balances[msg.sender] = 0;
    }
}

interface IDao {
    function withdraw() external ;
    function deposit()external  payable;
 }

contract Hacker{
    IDao dao;

    constructor(address _dao){
        dao = IDao(_dao);
    }

    function attack() public payable {
        require(msg.value >= 1 ether, "Need at least 1 ether to commence attack.");
        dao.deposit{value: msg.value}();
        dao.withdraw();
    }

    fallback() external payable{
        if(address(dao).balance >= 1 ether){
            dao.withdraw();
        }
    }
}
```

Note:
see https://blog.chain.link/reentrancy-attacks-and-the-dao-hack/

---

## JSON-RPC

- JSON-RPC is a **remote procedure call (RPC) protocol** using JSON for encoding requests and responses.
- It allows **external applications, wallets, and scripts** to interact with blockchain nodes.
- Most chains expose a **JSON-RPC API** for querying blockchain data and sending transactions.

---v

### Common Ethereum JSON-RPC Methods

| Method                      | Description                                     |
| --------------------------- | ----------------------------------------------- |
| `eth_call`                  | Executes a read-only contract call.             |
| `eth_estimateGas`           | Estimates the gas required for a transaction.   |
| `eth_sendRawTransaction`    | Sends a raw, signed transaction to the network. |
| `eth_getTransactionReceipt` | Retrieves transaction execution details.        |

Note:

- There are two types of transactions in Ethereum: read-only and state-changing transactions.
  When you want to read data from a contract, you use `eth_call` to execute a read-only contract call.

- When you want to send a transaction to the network, you will usually follow this flow:
- Estimate the gas required for the transaction using `eth_estimateGas`.
- Sign and submit the transaction using `eth_sendRawTransaction`.
- Finally, you can poll `eth_getTransactionReceipt` with the transaction hash to retrieve the transaction execution details.

The receipt is an important object, used by wallet and JS libraries, it will contain

- The transaction status
- Gas used, and logs generated during execution.

---v

### Common Substrate JSON-RPC Methods

| Method                   | Description                              |
| ------------------------ | ---------------------------------------- |
| `author_submitExtrinsic` | Submits a signed transaction             |
| `state_call`             | Calls a runtime API exposed by a pallet. |

---h

### Example sending a raw transaction

```json
curl
-H 'content-type: application/json'
https://westend-asset-hub-eth-rpc.polkadot.io/
-d '{
  "method":"eth_sendRawTransaction",
  "params" ["0x02f8b3018313c1..."],
  "id":2
  ,"jsonrpc":"2.0"
}'
```

---v

## Structure of a Transaction

```sh
# https://etherscan.io/getRawTx?tx=0xcd58fbee0f90c4b7136a5af85876090dd1593e4580f840bcf0a7b9219772a5d4
❯ cast decode-tx 0x02f8b3018313c1748387841585746a528800830249f094a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880b844a9059cbb000000000000000000000000ba04f1c1e4577165dd2297d3fbedf956b0e4c8a70000000000000000000000000000000000000000000000000000000004cc7c30c080a0c330502a046982553df56842433dfb1f318c980724bfd30be53e6461cea620aca025217d80ae9538009b3b24ab83fdac6df67982b433f74488d2c14fee41ca2d79
{
  "type": "0x2",
  "chainId": "0x1",
  "nonce": "0x13c174",
  "gas": "0x249f0",
  "maxFeePerGas": "0x746a528800",
  "maxPriorityFeePerGas": "0x878415",
  "to": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
  "value": "0x0",
  "input": "0xa9059cbb000000000000000000000000ba04f1c1e4577165dd2297d3fbedf956b0e4c8a70000000000000000000000000000000000000000000000000000000004cc7c30",
  "r": "0xc330502a046982553df56842433dfb1f318c980724bfd30be53e6461cea620ac",
  "s": "0x25217d80ae9538009b3b24ab83fdac6df67982b433f74488d2c14fee41ca2d79",
  "yParity": "0x0",
  "v": "0x0",
}
```

Notes:
A few things to note in the transaction:

- First of all the transaction is encoded using RLP (Recursive Length Prefix) encoding, the first byte is the type of the
  transaction, and defines the format of the transaction, that can be decoded using rlp-decode.
- The type here is 0x2, which means it is an EIP-1559 transaction, new fork can sometimes introduce new transaction type
  to define new transaction format, this type field is used to distinguish between them.
- The chainId defines the network on which the transaction is being sent, this makes sure that the transaction is not replayed on another network.
- The nonce is a very important component of the transaction, it is used to prevent replay attacks, it is incremented for each transaction sent by an account, your transaction will only be executed if the nonce is the next in line.
- gas, maxFeePerGas, maxPriorityFeePerGas are used to define the cost and fees generated for the transaction.
- value is the amount of ether being sent in the transaction.
- input is the ABI encoded data of the function being called, in this case, it is the transfer function of an ERC20 token, we will see how to decode this later.
- r, s, v are the signature of the transaction, used to verify the transaction was signed by the sender.

You will notice that the transaction does not have a 'from' field, this is because the origin can be recovered from the signature.

---v

### ABI decoding

```sh
INPUT="0xa9059cbb000000000000000000000000ba04f1c1e4577165dd2297d3fbedf956b0e4c8a70000000000000000000000000000000000000000000000000000000004cc7c30"

# Get the first 4 bytes (8 characters after '0x')
FIRST_4_BYTES="0x${INPUT:2:8}"

# Get the function signature using https://openchain.xyz/signatures
cast 4byte $FIRST_4_BYTES
transfer(address,uint256)

# Decode the input data using the function signature
ALL_BUT_FIRST_4_BYTES=${INPUT:10}
cast abi-decode -i "transfer(address,uint256)" $ALL_BUT_FIRST_4_BYTES
```

```sh
0xBA04f1c1E4577165dD2297D3FbEdF956B0e4C8a7
80510000 [8.051e7]
```

Notes:
Now that we have decoded the transaction, we can try to decode the input data

- the first 4 bytes of the input data are the function signature, which is used to identify the function being called, in this case, it is the transfer function of an ERC20 token.
- The rest of the input data is the parameters of the function, using the transfer function signature we can decode the input data to get the address and the amount being sent.

---v

### Encoding ABI parameters

```sh
cast abi-encode "test((bool, string, address))" "(true, hello, 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48)" | xxd -r -p | xxd -c 32

```

```hexdump

00000000: 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0020  ...............................
00000020: 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0001  ................................
00000040: 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0060  ...............................`
00000060: 0000 0000 0000 0000 0000 0000 a0b8 6991 c621 8b36 c1d1 9d4a 2e9e b0ce 3606 eb48  ..............i..!.6...J....6..H
00000080: 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0005  ................................
000000a0: 6865 6c6c 6f00 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000  hello...........................
```

Notes:

We’ve covered decoding ABI-encoded data—now let’s look at the opposite process: encoding function call parameters.
Using the function's ABI signature, we can encode the parameters using cast abi-encode.

A few key things to note:

- We are encoding a tuple containing a boolean, a string, and an address.
- Since a tuple is dynamic, the first 32 bytes store an offset to where the actual data starts—here, 0x20.
- The first tuple element (boolean) is at offset 0x20. Even though a bool is just 1 byte, like everything in the EVM, it is padded to 32 bytes.
- The string is another dynamic type, so instead of being stored inline, its offset (0x60) is stored at 0x40.
- The address is a fixed-size type, so it’s stored inline immediately after the offsets.
- At offset 0x60, we find the length of the string, and right after that, the string content itself.

## Smart Contract Ecosystem

---v

### Block Explorers

- Track transactions and smart contract states
- Enable transparency & debugging tools for developers
- Examples: Etherscan, Subscan, Blockscout

---v

### Oracles – External Data Integration

- Enable hybrid on-chain/off-chain contracts
- Examples: Chainlink, Redstone
- Deliver real-world data on-chain for smart contracts (e.g., price feeds, weather, events)
- Two models
  - Push: Data is pushed by node operators at specific interval and made available to contracts
  - Pull: Signed data package is attached to the transaction and verified by a contract on chain

---v

### Indexers

- Blockchain are write optimized, and querying data can be slow
- Indexers subscribe to the blockchain and store the data in a more queryable format

* Allow fast and structured access to blockchain records
* Improve UX for dApps by reducing raw node queries
* Examples: The Graph, Subsquid

---
