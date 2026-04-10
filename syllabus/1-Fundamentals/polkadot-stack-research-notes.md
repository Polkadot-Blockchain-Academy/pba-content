# Polkadot Stack - Research Notes

## Three-Layer Architecture

### 1. Polkadot Triangle (User Interfaces)
The "Triangle" is a three-party architecture for user-facing apps:
- **Host** (Triangle User Agent) - trusted application shell
- **Product** - sandboxed dApp running inside the host
- **Blockchain** - Polkadot chains providing crypto operations

**Concrete Applications:**
- **Polkadot Desktop** - Tauri-based desktop app (via host-sdk Rust crates)
- **Polkadot Mobile** - iOS (Swift via UniFFI) and Android (Kotlin) apps
- **Polkadot Website** - dot.li webapp (browser with JS smoldot, Helia IPFS, polkadot-api)
- **Host SDK (UserAgentKit)** - The SDK for building all the above

**Key host-sdk capabilities:**
- Light clients (smoldot for Substrate, Helios for Ethereum, Kyoto for Bitcoin)
- BIP-39 wallet with platform keychain (Touch ID, Android Keystore)
- DOTNS `.dot` name resolution (on-chain lookup -> IPFS content fetch)
- Product isolation (sandboxed iframes/webviews, no network access)
- Extensions: data channels, media calls, mesh storage, CRDT, VRF

**Product SDK** (`@novasamatech/product-sdk` or `@polkadot-apps/product-sdk`):
- `window.host.getAddress()` - get current account
- `window.host.storage.get/set/remove()` - scoped key-value storage
- `window.host.statements.subscribe/write()` - pub/sub via statement store
- Extensions: `ext.data`, `ext.media`, `ext.files`, `ext.mesh`, `ext.crdt`, `ext.vrf`

**DotNS (.dot Name Service):**
- Solidity contracts on Asset Hub (via pallet-revive)
- Commit-reveal registration with PoP-aware pricing
- Content resolver stores IPFS CIDs
- Resolution: namehash -> ReviveApi::call -> decode CID -> fetch from IPFS

### 2. Polkadot Stack (Developer Experience)

**Polkadot SDK** (monorepo: Substrate + Polkadot + Cumulus + Bridges):
- **Substrate** - blockchain framework (node infrastructure)
- **FRAME** - runtime development framework (120+ pallets)
- **Cumulus** - parachain development toolkit
- **Templates** - minimal, solochain, parachain starters

**Smart Contracts:**
- **pallet-revive** - EVM-compatible smart contracts on PolkaVM (RISC-V)
- **Solidity** compiled via `resolc` to PVM bytecode, or standard `solc` for EVM
- **eth-rpc** sidecar - bridges Ethereum JSON-RPC to Substrate
- Standard Ethereum tooling works: MetaMask, Hardhat, Foundry, viem, ethers.js

**Client Libraries:**
- **PAPI** (TypeScript) - light-client-first, typed API from metadata, native BigInt
- **subxt** (Rust) - typed Rust client, proc-macro metadata generation
- **alloy/viem** - Ethereum-compatible clients through eth-rpc

**polkadot-stack-template** - Full teaching template:
- FRAME pallet (Proof of Existence)
- Solidity contracts (EVM + PVM targets)
- React frontend (PAPI + viem + product-sdk)
- Rust CLI (subxt + alloy)
- Scripts for local dev, deployment, IPFS upload

**Two Parallel Developer Access Paths:**
1. **Native Substrate**: PAPI/subxt -> WebSocket -> Substrate node
2. **Ethereum-compatible**: viem/alloy -> HTTP -> eth-rpc sidecar -> Substrate node

### 3. Polkadot Platform (Web3 Cloud Infrastructure)

**Relay Chain:**
- Shared security (NPoS validators)
- Parachain validation (ELVES protocol)
- Being slimmed post-AHM (Asset Hub Migration)

**System Parachains:**
- **Asset Hub** - PRIMARY user chain (DOT, assets, NFTs, DEX, governance, staking, pallet-revive smart contracts)
- **Bridge Hub** - Kusama bridge (GRANDPA light client) + Ethereum bridge (Snowbridge beacon client)
- **Coretime** - Broker pallet for agile coretime sales (bulk + on-demand)
- **People Chain** - On-chain identity system with registrars
- **Collectives** - Technical Fellowship, Ambassador Program, Secretary

**Bulletin Chain:**
- Persistent data storage (IPFS-compatible, Blake2b CIDs)
- Transaction-indexed storage with authorization model
- 2 MiB per-tx, auto-chunking SDK, 2-week default retention
- SDKs in Rust and TypeScript (BYOC pattern)

**Custom Parachains:**
- Teams can build their own using Polkadot SDK
- Deploy via coretime (bulk or on-demand)
- Full heterogeneous runtime customization

## The Complete Flow

Developer builds dApp using polkadot-stack-template patterns:
1. Write FRAME pallet or Solidity contracts
2. Build frontend with PAPI + viem + product-sdk
3. Deploy frontend to IPFS
4. Register `.dot` name via DotNS
5. Users access via Triangle User Agent (desktop/mobile/web)
6. Host resolves `.dot` name -> fetches from IPFS -> renders in sandbox
7. dApp talks to chains through host bridge (signing, queries, transactions)
