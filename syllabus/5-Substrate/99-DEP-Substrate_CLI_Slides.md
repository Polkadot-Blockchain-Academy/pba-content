---
title: (Deprecated)Substrate CLI
description: A brief look at the Substrate CLI.
duration: 1 hour
---

# Substrate CLI

---

## Substrate CLI

In this section, we will quickly go over some of the common CLI commands you should be familiar with when working with Substrate.

---

## Build

Least Time to Execute

```sh
cargo check
```

```sh
cargo build
```

```sh
cargo build --release
```

---

## Build

Which crate is it exactly being executed when I type `cargo run`? Where's the damn `fn main() -> {}`.

```toml
[[bin]]
name = "substrate"
path = "bin/main.rs"
required-features = ["cli"]
```

- Thus, `cargo b --release -p node-cli` is slightly faster.
- Or `cargo b --release -p node-template`.

---

## Testing Specific Packages

From: `substrate/bin/node-template/runtime/Cargo.toml`

```toml
[package]
name = "node-template-runtime"
```

```sh
cargo test -p node-template-runtime
```

This will run the tests just in the `node-template-runtime` library.

---

## Wasm Builder Environment Variables

Some useful environment variables from the Wasm builder:

- `SKIP_WASM_BUILD` - Skips building any Wasm binary. This is useful when only native should be recompiled.
- `FORCE_WASM_BUILD` - Can be set to force a Wasm build.

For example, an even faster way to build just the client:

```sh
SKIP_WASM_BUILD=1 cargo build --release
```

---

## Substrate Master Doesn't Compile?

Probably just need to:

```sh
rustup update
```

Generally, we ensure that Substrate master always works with the latest Rust compiler.

---

## CLI Commands

```sh
./target/release/substrate --help
```

````sh
Commands:

inspect, benchmark, try-runtime, key, verify, vanity, sign,
build-spec, check-block, export-blocks, export-state,
import-blocks, purge-chain, revert, chain-info, help

Options:

...LOTs of these!!!
```

---

## Chain Specification

- A JSON object which uniquely defines a blockchain.
- Contains various metadata and the genesis state.

---

## Opinionated FRAME based JSON

```sh
./target/release/substrate build-spec --chain=dev > spec.json
````

```json
{
  "name": "Development",
  "id": "dev",
  "chainType": "Development",
  "bootNodes": ["/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWNsFq6B97vcYvjtx9RTQrptYqVFN8Nvtg7Nr2fD3jBUMz"],
  "telemetryEndpoints": null,
  "protocolId": null,
  "properties": null,
  "forkBlocks": null,
  "badBlocks": null,
  "lightSyncState": null,
  "codeSubstitutes": {},
  "genesis": {
    "runtime": {
      "system": {
        "code": "<REALLY BIG WASM BLOB>"
      },
      "babe": {
        "authorities": [],
        "epochConfig": {
          "c": [1, 4],
          "allowed_slots": "PrimaryAndSecondaryPlainSlots"
        }
      },
      "indices": {
        "indices": []
      },
      "balances": {
        "balances": [
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000000000],
          ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 1000000000000000000000],
          ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 1000000000000000000000],
          ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 1000000000000000000000],
          ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", 1000000000000000000000],
          ["5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL", 1000000000000000000000],
          ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", 1000000000000000000000],
          ["5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc", 1000000000000000000000],
          ["5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT", 1000000000000000000000],
          ["5HKPmK9GYtE1PSLsS1qiYU9xQ9Si1NcEhdeCq9sw5bqu4ns8", 1000000000000000000000],
          ["5FCfAonRZgTFrTd9HREEyeJjDpT397KMzizE6T3DvebLFE7n", 1000000000000000000000],
          ["5CRmqmsiNFExV6VbdmPJViVxrWmkaXXvBrSX8oqBT8R9vmWk", 1000000000000000000000]
        ]
      },
      "transactionPayment": null,
      "staking": {
        "historyDepth": 84,
        "validatorCount": 1,
        "minimumValidatorCount": 1,
        "invulnerables": ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"],
        "forceEra": "NotForcing",
        "slashRewardFraction": 100000000,
        "canceledPayout": 0,
        "stakers": [
          [
            "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            1000000000000000000,
            "Validator"
          ]
        ],
        "minNominatorBond": 0,
        "minValidatorBond": 0,
        "maxValidatorCount": null,
        "maxNominatorCount": null
      },
      "session": {
        "keys": [
          [
            "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
            "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
            {
              "grandpa": "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu",
              "babe": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
              "im_online": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
              "authority_discovery": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
            }
          ]
        ]
      },
      "democracy": {
        "phantom": null
      },
      "council": {
        "phantom": null,
        "members": []
      },
      "technicalCommittee": {
        "phantom": null,
        "members": [
          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
          "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
          "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
          "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
          "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL"
        ]
      },
      "elections": {
        "members": [
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000000],
          ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 1000000000000000000],
          ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 1000000000000000000],
          ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 1000000000000000000],
          ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", 1000000000000000000],
          ["5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL", 1000000000000000000]
        ]
      },
      "technicalMembership": {
        "members": [],
        "phantom": null
      },
      "grandpa": {
        "authorities": []
      },
      "treasury": null,
      "sudo": {
        "key": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
      },
      "imOnline": {
        "keys": []
      },
      "authorityDiscovery": {
        "keys": []
      },
      "society": {
        "pot": 0,
        "members": [
          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
          "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
          "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
          "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
          "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL"
        ],
        "maxMembers": 999
      },
      "vesting": {
        "vesting": []
      },
      "assets": {
        "assets": [],
        "metadata": [],
        "accounts": []
      },
      "gilt": null,
      "transactionStorage": {
        "byteFee": 10,
        "entryFee": 1000,
        "storagePeriod": 100800
      },
      "allianceMotion": {
        "phantom": null,
        "members": []
      },
      "alliance": {
        "founders": [],
        "fellows": [],
        "allies": [],
        "phantom": null
      },
      "nominationPools": {
        "minJoinBond": 100000000000000,
        "minCreateBond": 1000000000000000,
        "maxPools": 16,
        "maxMembersPerPool": 32,
        "maxMembers": 512
      }
    }
  }
}
```

Notes:

We will learn how specifically keys are generated in the FRAME section.

But you can use tools like:

https://github.com/shawntabrizi/substrate-known-keys/blob/master/known-keys.json

Some simple ones:

```js
// The Wasm Blob.
// Just hex representation of UTF-8 String.
":code" = 0x3a636f6465

// System Accounts
0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9

// Block Number
0x26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac
```

---

## Raw Chain Spec

Totally un-opinionated. Just raw key-value pairs.

```sh
./target/release/substrate build-spec --chain=dev --raw > spec.json
```

```json
{
  "name": "Development",
  "id": "dev",
  "chainType": "Development",
  "bootNodes": ["/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWNsFq6B97vcYvjtx9RTQrptYqVFN8Nvtg7Nr2fD3jBUMz"],
  "telemetryEndpoints": null,
  "protocolId": null,
  "properties": null,
  "forkBlocks": null,
  "badBlocks": null,
  "lightSyncState": null,
  "codeSubstitutes": {},
  "genesis": {
    "raw": {
      "top": {
        "0x074b65e262fcd5bd9c785caf7f42e00a4e7b9012096b41c4eb3aaf947f6ea429": "0x0000",
        "0x0e7b504e5df47062be129a8958a7a1271689c014e0a5b9a8ca8aafdff753c41c": "0xe8030000000000000000000000000000",
        "LOTS more hex keys": "0x0000"
      },
      "childrenDefault": {}
    }
  }
}
```

---

## Loading Chain Spec

```sh
./target/release/substrate --chain=spec.json
```

---

## Loading Chain Spec

Some specs are included in the client binary:

From: `substrate/bin/node/cli/src/command.rs`

```
      --chain <CHAIN_SPEC>
          Specify the chain specification.

          It can be one of the predefined ones (dev, local, or staging) ..
```

---

## Development Node

```sh
--dev
	Specify the development chain.

	This flag sets `--chain=dev`, `--force-authoring`, `--rpc-cors=all`, `--alice`, and
	`--tmp` flags, unless explicitly overridden.
```

The most common way to quickly launch a test network:

```sh
./target/release/substrate --dev
```

---

## `--dev` Implications

- `--chain=dev`: Use the dev chain specification.
- `--force-authoring`: Make blocks even if you are offline. (when you are not connected to any peers)
- `--rpc-cors=all`: Allow any browser Origin to access the HTTP & WS RPC servers.
- `--alice`: Shortcut for `--name Alice --validator` with session keys for `Alice` added to keystore.
- `--tmp`: Run a temporary node where blockchain data is stored in a temporary directory.

---

## Development Accounts

- `--alice`
- `--bob`
- `--charlie`
- `--dave`
- `--eve`
- `--ferdie`

On a `--dev` node, these accounts are configured as validators and also given funds.

```sh
--validator
	Enable validator mode.

	The node will be started with the authority role and actively participate in any
	consensus task that it can (e.g. depending on availability of local keys).
```

---

## Control Where Data is Stored

```sh
./target/release/substrate --dev --base-path=./tmp/
```

This will result in:

```
tmp/
├─ chains/
│  ├─ dev/
│  │  ├─ db/
│  │  ├─ network/
│  │  ├─ keystore/
```

---

## Purging Chain Data

Without specifying a `--base-path`, you may not know where the blockchain data is being stored.

`purge-chain` sub-command can help with that.

```sh
./target/release/substrate purge-chain --dev

Are you sure to remove "/Users/shawntabrizi/Library/Application Support/substrate/chains/dev/db/full"? [y/N]:

y

"/Users/shawntabrizi/Library/Application Support/substrate/chains/dev/db/full" removed.
```

---

## Database Stuff

```sh
--database <DB>
	Select database backend to use

	[possible values: rocksdb, paritydb, paritydb-experimental, auto]

--db-cache <MiB>
	Limit the memory the database cache can use
```

---

## Execute as Native

```sh
--execution <STRATEGY>
	The execution strategy that should be used by all execution contexts

	[possible values: native, wasm, both, native-else-wasm]
```

```sh
./target/release/substrate --dev --execution=native
```

---

## Type of Wasm Execution

```
--wasm-execution <METHOD>
    Method for executing Wasm runtime code

    [default: compiled]

    Possible values:
    - interpreted-i-know-what-i-do: Uses an interpreter
    - compiled:                     Uses a compiled runtime
```

---

## Block Pruning

- You can control how many finalizes blocks to keep and the underlying block state.
- Note that changing this usually is not allowed.

```
--state-pruning <PRUNING_MODE>
    Specify the state pruning mode.

    Possible values:
    - archive:
    - 'archive-canonical'
    - number
    [default: 256]

--blocks-pruning <PRUNING_MODE>
    Specify the blocks pruning mode.
    Possible values:
    - 'archive'
    - 'archive-canonical'
    - number
    [default: archive-canonical]
```

---

## RPC and WS

You probably want to control RPC and WS ports if you are launching multiple nodes on the same machine.

```sh
--rpc-port <PORT>
	Specify HTTP RPC server TCP port
  Default is 9934

--ws-port <PORT>
	Specify WebSockets RPC server TCP port
  Default is 9944
```

Bandwidth configurations:

```sh
--rpc-max-request-size <RPC_MAX_REQUEST_SIZE>
	Set the the maximum RPC request payload size for both HTTP and WS in megabytes. Default is 15MiB

--rpc-max-response-size <RPC_MAX_RESPONSE_SIZE>
	Set the the maximum RPC response payload size for both HTTP and WS in megabytes. Default is 15MiB
```

---

## "Unsafe" RPC and WS Stuff

```sh
--rpc-methods <METHOD SET>
	RPC methods to expose.

	- `unsafe`: Exposes every RPC method.
	- `safe`: Exposes only a safe subset of RPC methods, denying unsafe RPC methods.
	- `auto`: Acts as `safe` if RPC is served externally, e.g. when `--{rpc,ws}-external` is
		passed, otherwise acts as `unsafe`.

	[default: auto]
	[possible values: auto, safe, unsafe]

--rpc-external
	Listen to all RPC interfaces.

	Default is local. Notes: not all RPC methods are safe to be exposed publicly. Use an RPC
	proxy server to filter out dangerous methods. More details:
	<https://docs.substrate.io/v3/runtime/custom-rpcs/#public-rpcs>. Use
	`--unsafe-rpc-external` to suppress the warning if you understand the risks.

--ws-external
	Listen to all Websocket interfaces.

	Default is local. Notes: not all RPC methods are safe to be exposed publicly. Use an RPC
	proxy server to filter out dangerous methods. More details:
	<https://docs.substrate.io/v3/runtime/custom-rpcs/#public-rpcs>. Use
	`--unsafe-ws-external` to suppress the warning if you understand the risks.
```

---

## Enabling Logs

```rust
-l, --log <LOG_PATTERN>...
		Sets a custom logging filter. Syntax is <target>=<level>, e.g. -lsync=debug.

		Log levels (least to most verbose) are error, warn, info, debug, and trace. By default,
		all targets log `info`. The global log level can be set with -l<level>.
```

Same as `RUST_LOG` environment variable.

All `runtime` prefixed logs:

```sh
RUST_LOG=runtime=debug ./target/release/substrate --dev
./target/release/substrate --dev --log=runtime=debug
```

All `runtime::staking` prefixed logs:

```sh
./target/release/substrate --dev --log=runtime::staking=debug
```

Notes:

Just an example of how you can add logs to your code:

```rust
pub(crate) const LOG_TARGET: &str = "runtime::staking";

// syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::LOG_TARGET,
			concat!("[{:?}] 💸 ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}
```

Then to log:

```rust
log!(warn, "genesis election provider failed due to {:?}", e);
```

---

## Sync Flags

You can control how you sync your node.

```sh
--sync <SYNC_MODE>
	Blockchain syncing mode.

	- `full`: Download and validate full blockchain history.
	- `fast`: Download blocks and the latest state only.
	- `fast-unsafe`: Same as `fast`, but skip downloading state proofs.
	- `warp`: Download the latest state and proof.

	[default: full]
	[possible values: full, fast, fast-unsafe, warp]
```

---

## Transaction Pool Size

```sh
--pool-kbytes <COUNT>
	Maximum number of kilobytes of all transactions stored in the pool

	[default: 20480]

--pool-limit <COUNT>
	Maximum number of transactions in the transaction pool

	[default: 8192]
```

---

## Sub Commands

```sh
SUBCOMMANDS:
	benchmark
			Sub-commands concerned with benchmarking. The pallet benchmarking moved to the `pallet`
			sub-command
	build-spec
			Build a chain specification
	chain-info
			Db meta columns information
	check-block
			Validate blocks
	export-blocks
			Export blocks
	export-state
			Export the state of a given block into a chain spec
	help
			Print this message or the help of the given subcommand(s)
	import-blocks
			Import blocks
	inspect
			Decode given block or extrinsic using current native runtime.
	key
			Key management cli utilities
	purge-chain
			Remove the whole chain
	revert
			Revert the chain to a previous state
	sign
			Sign a message, with a given (secret) key
	try-runtime
			Try some command against runtime state. N o t e: `try-runtime` feature must be enabled
	vanity
			Generate a seed that provides a vanity address
	verify
			Verify a signature for a message, provided on STDIN, with a given (public or secret) key
```

---

## Additional Resources! 😋

> Check speaker notes (click "s" 😉)

<img width="300px" rounded src="../../assets/img/5-Substrate/thats_all_folks.png" />

Notes:

Both Substrate and IPFS use LibP2P.

It is pretty easy to turn a Substrate node into a IPFS client.

```sh
--ipfs-server
	Join the IPFS network and serve transactions over bitswap protocol
```

Additional ideas:

1. Sync a new polkadot network with fast and warp
1. Then check yourself out in telemetry.
