# Substrate Frameless Node Template

A stripped down version of the [node template](https://github.com/substrate-developer-hub/substrate-node-template), ready for hackin'.

## Coding Activity 1 - Flipper Runtime

We have prepared a FRAME-less runtime.
It does, by all means nothing as it is now.

Try and do as many of the following, in the same order:

1. Make it a flipper: it stores one bool value, and per each transaction, flips that value.
1. Make it an adder: it stores one u32 value, and each transaction specifies a number, which is added to this value and stored onchain again.
1. Extend your transaction so that the runtime can be either an adder or a multiplier.
1. Add a kill-switch to this runtime. Look into well_known_keys to see which key you have to wipe.
1. Make this runtime upgradable! The upgrade operation can simply be protected by a "password" as you don't have any notion of accounts yet.
1. Add a notion of accounts and nonces and signatures.
1. Add a notion of balances
1. Write a custom runtime API, and try to call it over the RPC.
1. Implement a tx-pool api, implement tipping, priority, longevity etc.

.. you can virtually do endless experiments on top of the frameless runtime. Make good ues of it, and you will learn a lot about the fundamentals of substrate!

Moreover, this is the perfect intro to FRAME. You feel the pain of how hard it is to code your own blockchain from scratch 😈.

## Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo b -r
```

### Embedded CLI Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-template -h
```

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

In case of being interested in maintaining the chain's state between runs a base path must be added:

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/node-template --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```
