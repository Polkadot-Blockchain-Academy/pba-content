---
title: The Polkadot Stack Template
description: A hands-on introduction to the polkadot-stack-template project.
duration: 1 hour
owner: Shawn Tabrizi
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# The Polkadot Stack Template

---

## Your Course Project

Throughout this course, you will build a project using the **polkadot-stack-template**.

This template demonstrates the full Polkadot stack through a single concept: **Proof of Existence**.

The same idea (claim a file hash on-chain) is implemented three ways:

- A **FRAME pallet** (native Substrate)
- A **Solidity contract on EVM** (Ethereum bytecode)
- A **Solidity contract on PVM** (PolkaVM / RISC-V bytecode)

Notes:

The polkadot-stack-template is your hands-on project for this course. It's a full-stack application that touches every layer of the Polkadot ecosystem. The idea is simple: prove that a file existed at a certain time by recording its hash on-chain. But we implement it three different ways to show you the different development paths available on Polkadot. You'll also build a React frontend and a Rust CLI to interact with all three implementations.

---

## Template Overview

<div class="text-small">

| Component | Path | Tech Stack |
|-----------|------|------------|
| **FRAME Pallet** | `blockchain/` | Rust, FRAME, Cumulus, polkadot-omni-node |
| **EVM Contract** | `contracts/evm/` | Solidity, Hardhat, solc, viem |
| **PVM Contract** | `contracts/pvm/` | Solidity, Hardhat, resolc, `@parity/hardhat-polkadot` |
| **React Frontend** | `web/` | React, Vite, TypeScript, PAPI, viem, Zustand |
| **Rust CLI** | `cli/` | Rust, subxt, alloy, clap |
| **Dev Scripts** | `scripts/` | Zombienet, docker-compose |

</div>

Every component is **optional and removable**. Teams keep the slices they need.

Notes:

Here's the complete layout of the stack template. Five major components, each demonstrating a different part of the Polkadot developer experience. The blockchain directory contains a full parachain runtime with a custom pallet. The contracts directory has the same Solidity contract compiled to two targets. The web directory is a React frontend that talks to all three implementations. The CLI is a Rust tool that does the same. And the scripts directory orchestrates local development with zombienet.

---

## The Complete Flow

<diagram class="mermaid">
graph TB
    subgraph Dev["Developer Builds"]
        D1["FRAME Pallet<br/>(Rust)"]
        D2["Solidity Contract"]
        D3["React Frontend<br/>(PAPI + viem)"]
    end

    subgraph Deploy["Deploy To"]
        E1["Parachain Runtime<br/>(via Coretime)"]
        E2["Asset Hub<br/>(via eth-rpc)"]
        E3["IPFS<br/>(via Bulletin Chain)"]
        E4["DotNS<br/>(myapp.dot)"]
    end

    subgraph User["User Accesses"]
        U1["dot.li / Desktop / Mobile"]
        U2["Resolves myapp.dot"]
        U3["Loads from IPFS"]
        U4["Interacts via Host"]
    end

    D1 --> E1
    D2 --> E2
    D3 --> E3
    E3 --> E4

    E4 --> U2
    U1 --> U2
    U2 --> U3
    U3 --> U4
    U4 -->|"PAPI / viem"| E1
    U4 -->|"PAPI / viem"| E2
</diagram>

Notes:

Here's the end-to-end flow. A developer writes their pallet in Rust, their contracts in Solidity, and their frontend in React. The pallet deploys as part of a parachain runtime via coretime. The contracts deploy to Asset Hub through eth-rpc. The frontend uploads to IPFS via the Bulletin Chain and gets a .dot domain via DotNS. A user opens a Triangle host, types the .dot name, and the host resolves it, fetches the frontend, and renders it in a sandbox. The frontend talks to the blockchain through the host's bridge, using PAPI for pallets and viem for contracts. No centralized servers anywhere in the chain.

---

## The Pallet

```rust
#[pallet::storage]
pub type Claims<T: Config> =
    StorageMap<_, Blake2_128Concat, H256, Claim<T>>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(T::WeightInfo::create_claim())]
    pub fn create_claim(
        origin: OriginFor<T>,
        hash: H256,
    ) -> DispatchResult {
        let sender = ensure_signed(origin)?;
        ensure!(!Claims::<T>::contains_key(hash), Error::<T>::AlreadyClaimed);
        let claim = Claim { owner: sender.clone(), block: frame_system::Pallet::<T>::block_number() };
        Claims::<T>::insert(hash, claim);
        Self::deposit_event(Event::ClaimCreated { who: sender, hash });
        Ok(())
    }
}
```

Notes:

Here's the actual pallet code from the template. It's a clean, minimal FRAME pallet. A StorageMap maps 32-byte blake2 hashes to Claims, which record the owner and block number. The create_claim function checks that the hash isn't already claimed, creates a new claim, inserts it into storage, and emits an event. This is the pattern you'll learn throughout the FRAME module of this course.

---

## The Contract

```solidity
contract ProofOfExistence {
    struct Claim {
        address owner;
        uint256 blockNumber;
    }

    mapping(bytes32 => Claim) public claims;

    function createClaim(bytes32 documentHash) external {
        require(claims[documentHash].owner == address(0), "Already claimed");
        claims[documentHash] = Claim(msg.sender, block.number);
        emit ClaimCreated(msg.sender, documentHash);
    }
}
```

Same logic, same ABI. Compiles to **EVM** (solc) and **PVM** (resolc).

Notes:

And here's the Solidity version of the same logic. Same concept, same structure, just in Solidity instead of Rust. The remarkable thing is that this exact same source file gets compiled to two different bytecode formats. solc produces traditional EVM bytecode, and resolc produces PolkaVM RISC-V bytecode. Both are deployed to the same chain through pallet-revive, and both are accessible through the eth-rpc sidecar. The frontend and CLI can interact with both using the same ABI.

---

## The Frontend

<div class="grid grid-cols-2">
<div class="text-left">

**Six pages:**
1. Home
2. Pallet PoE (via PAPI)
3. EVM PoE (via viem)
4. PVM PoE (via viem)
5. Statements
6. Accounts

**Smart host detection:**
- Running inside Triangle host? Use host wallet
- Standalone browser? Use browser extensions

</div>
<div class="text-left">

```typescript
// Pallet interaction (PAPI)
const api = client.getTypedApi(descriptors);
await api.tx.TemplatePallet
  .create_claim({ hash })
  .signAndSubmit(signer);

// Contract interaction (viem)
const tx = await walletClient
  .writeContract({
    address: evmContract,
    abi,
    functionName: "createClaim",
    args: [hash],
  });
```

</div>
</div>

Notes:

The React frontend demonstrates both access paths side by side. For pallet interactions, it uses PAPI with typed descriptors. For contract interactions, it uses viem. Both are talking to the same chain. The frontend also has smart host detection: if it's running inside a Triangle User Agent like Polkadot Desktop, it uses the host's wallet for accounts. If it's running standalone in a browser, it falls back to browser extension wallets like Polkadot.js or Talisman.

---

## The CLI

<div class="grid grid-cols-2">
<div class="text-left">

**Commands:**

```
stack-cli pallet create-claim <hash>
stack-cli pallet get-claim <hash>

stack-cli contract create-claim evm <hash>
stack-cli contract create-claim pvm <hash>

stack-cli chain info
stack-cli chain blocks

stack-cli prove <file>
```

</div>
<div class="text-left">

**Tech mapping:**

| Target | Library |
|--------|---------|
| Pallets | subxt (Substrate WS) |
| Contracts | alloy (eth-rpc HTTP) |
| Chain info | subxt |
| Prove | both + bulletin chain |

The `prove` command does it all: hash, claim, and optionally upload to the Bulletin Chain.

</div>
</div>

Notes:

The Rust CLI mirrors everything the frontend does, but from the command line. Pallet commands use subxt for native Substrate interaction. Contract commands use alloy through the eth-rpc adapter. The prove command is the all-in-one: it hashes a file, creates a claim on either the pallet or contract, and optionally uploads the file to the Bulletin Chain for persistent storage. This is a great reference for building Rust backend services.

---

## What You Will Build

<div class="text-left">

During this course, using the **polkadot-stack-template**, you will:

1. Build a **FRAME pallet** with custom storage, calls, events, and errors
2. Write and deploy **Solidity contracts** to both EVM and PVM targets
3. Build a **React frontend** using PAPI and viem
4. Build a **Rust CLI** using subxt and alloy
5. Run a **local test network** with zombienet
6. Deploy to **IPFS** and register a **.dot domain**
7. Optionally integrate with the **Bulletin Chain** for data storage

</div>

Notes:

So what does this mean for you? Over the course of this program, you'll get hands-on experience with every layer of the stack. You'll learn FRAME deeply in the FRAME module, build smart contracts, create frontends, and deploy the full application. The stack template is designed so you can keep the pieces you need and discard the rest. Some teams will focus on the pallet, others on contracts, others on the full stack. The goal is that by the end, you can build and deploy a complete Polkadot application from scratch.

---

## Key Resources

<div class="grid grid-cols-2">
<div class="text-left">

**Repositories:**
- `polkadot-sdk` - Core SDK
- `polkadot-stack-template` - Your project
- `host-sdk` - Triangle host SDK
- `dotns` - .dot name service
- `polkadot-bulletin-chain` - Data storage
- `polkadot-api` - PAPI

</div>
<div class="text-left">

**Documentation:**
- papi.how - PAPI docs
- paritytech.github.io/polkadot-sdk - SDK docs
- Blockscout explorer
- Polkadot Wiki

**Tools:**
- Polkadot.js Apps - Chain explorer
- Zombienet - Local networks
- psvm - SDK version manager

</div>
</div>

Notes:

Here are the key resources you'll need throughout the course. The polkadot-stack-template is your starting point. The PAPI docs at papi.how are excellent for frontend development. The Polkadot SDK docs cover everything about pallets and runtimes. Blockscout is your go-to block explorer, especially for smart contracts. And zombienet is essential for local development and testing.

---

<!-- .slide: data-background-color="#000000" -->

# Questions
