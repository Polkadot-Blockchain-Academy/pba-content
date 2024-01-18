---
title: Interacting With a Substrate Blockchain
duration: 60 minutes
---

# Interacting With a Substrate Blockchain

---

## Before we start

Find all the commands that will be used in this workshop:
[tinyurl.com/hk24-substrate](https://hackmd.io/@ak0n/hk24-substrate-interaction)

---

## Before we start

- Clone polkadot-sdk

```sh
git clone https://github.com/paritytech/polkadot-sdk.git
```

<br/>

- Compile your node

```sh
cargo build --release -p minimal-node
```

---

## Interacting With a Substrate Blockchain

> How does a user or an application interact with a blockchain?

Notes:

- Wait for 1 answer from students or at least for 10 seconds.

---v

## Interacting With a Substrate Blockchain

- Usually they connect to a public RPC server, i.e. a substrate node that exposes its RPC interface publicly.

<br/>

- Run their own node.

<!-- .element: class="fragment" -->

---v

## Interacting With a Substrate Blockchain

<img style="width: 1200px;" src="../../assets/img/5-Substrate/dev-4-1-json.svg" />

---

## JSON-RPC

> JSON-RPC is a remote procedure call protocol encoded in JSON. It is similar to the XML-RPC
> protocol, defining only a few data types and commands.

---v

### JSON-RPC

```json
{
  "jsonrpc": "2.0",
  "method": "subtract",
  "params": {
    "minuend": 42,
    "subtrahend": 23
  },
  "id": 3
}
```

<br/>

```json
{
  "jsonrpc": "2.0",
  "result": 19,
  "id": 3
}
```

<!-- .element: class="fragment" -->

---v

### JSON-RPC

- Entirely transport agnostic.
- Substrate based chains expose both `websocket` and `http` (or `wss` and `https`, if desired).

Notes:

- You could choose which port to run the ws or http server on by using the flags `--ws-port` and `--rpc-port`
  respectively. By default, port 9944 is used.

---v

### JSON-RPC

The RPC methods that a substrate node exposes are scoped and has the pattern `"<scope>_<method>"`.

```sh
 wscat \
  -c ws://localhost:9944 \
  -x '{"jsonrpc":"2.0", "id": 42, "method":"rpc_methods" }' \
  | jq
```

---v

### JSON-RPC: Scopes

- &shy;<!-- .element: class="fragment" --> `author`: for submitting extrinsic to the chain.
- &shy;<!-- .element: class="fragment" --> `chain`: for retrieving information about the _blockchain_ data.
- &shy;<!-- .element: class="fragment" --> `state`: for retrieving information about the _state_ data.
- &shy;<!-- .element: class="fragment" --> `system`: information about the chain.
- &shy;<!-- .element: class="fragment" --> `rpc`: information about the RPC endpoints.

Notes:
recall:

https://paritytech.github.io/substrate/master/sc_rpc_api/index.html
https://paritytech.github.io/substrate/master/sc_rpc/index.html

- The full list can also be seen here: https://polkadot.js.org/docs/substrate/rpc/
- Specs: https://paritytech.github.io/json-rpc-interface-spec/introduction.html
- Upcoming changes to JSON-RPC api: https://forum.polkadot.network/t/new-json-rpc-api-mega-q-a/3048

---

### Workshop: Intro

- Transfer tokens from Alice to Bob.

<br/>

- We will cheat a bit and take help sometimes from [Polkadot.js app](https://polkadot.js.org/apps/#/explorer).

<!-- .element: class="fragment" -->

Notes:

- When we start up a dev chain, some well known accounts are already minted some balance at genesis. We will use Alice
  and Bob which are well known accounts.
- The parts we cheat is because we will need to know more about FRAME to be able to calculate some storage keys.

---v

### Workshop: Spin up your node

- Check out cli docs

```sh
./target/release/minimal-node --help
```

<br/>

- Spin up your dev node.

```sh
./target/release/minimal-node --chain=dev --tmp
```

<!-- .element: class="fragment" -->

Notes:

- What does --chain=dev and --tmp do? What other flag can you use?

---v

### Workshop: Check balance

- Query current balance of Alice and Bob.

```sh
wscat \
  -c ws://localhost:9944 \
  -x '{"jsonrpc":"2.0", "id": 42, "method":"state_getStorage", "params": [""] }' \
  | jq
```

Notes:

- You will learn how the storage key is calculated in FRAME based substrate chains in the FRAME module.
- What do you get?

---v

### Workshop: Metadata

- Recall type information is lost in SCALE encoded data.
- Substrate exposes type information using metadata.

```sh
wscat \
  -c ws://localhost:9944 \
  -x '{"jsonrpc":"2.0", "id": 42, "method":"state_getMetadata" }' \
  | jq
```

<br/>

- This itself is Scale Encoded. See [frame-metadata](https://github.com/paritytech/frame-metadata).
- Derive type of AccountInfo using this metadata.

<!-- .element: class="fragment" -->

Notes:

- Use PJS app to get frame-metadata: Developer > RPC Calls > state > getMetadata.
- [Metadata](https://hackmd.io/@ak0n/rJUhmXmK6) with most details not relevant stripped off.
- Read more about
  metadata: https://docs.substrate.io/build/application-development/#exposing-runtime-information-as-metadata.

---v

### Workshop: Decoding balance

- Use [scale decoder](https://www.shawntabrizi.com/substrate-js-utilities/codec/) to decode balance.
- Use the following type information for AccountInfo.

```json
{
  "info": {
    "nonce": "u32",
    "ignore": "(u32, u32, u32)",
    "balance": {
      "free": "u64",
      "ignore": "(u64, u64, u128)"
    }
  }
}
```

Notes:
The actual type is:

```json
{
  "info": {
    "nonce": "u32",
    "ignore": "u32",
    "providers": "u32",
    "sufficients": "u32",
    "balance": {
      "free": "u64",
      "reserved": "u64",
      "frozen": "u64",
      "flags": "u128"
    }
  }
}
```

---v

### Workshop: Transfer some tokens

- &shy;<!-- .element: class="fragment" --> Take PJS help to get the signed extrinsic.
- &shy;<!-- .element: class="fragment" --> Use the following command to submit the extrinsic.

```sh
wscat \
  -c ws://localhost:9944 \
  -x '{"jsonrpc":"2.0", "id": 42, "method":"author_submitExtrinsic", "params": [""] }' \
  | jq
```

- &shy;<!-- .element: class="fragment" --> Check balance again for both accounts.
- &shy;<!-- .element: class="fragment" --> What happens to nonce of Alice?

Notes:

- Students will learn how to build the signed extrinsic themselves in their assignment.
- Let students do the second part themselves.

---v

### Workshop: Versions

- Find runtime version of the polkadot and westend chain `state_getRuntimeVersion`.
- Find node version of the polkadot and westend chain `system_version`.
- Change RPC provider and see if any of the above value changes?

<br/>

- For runtime version, you read `specVersion` : `1,005,000` as `1.5.0`.

<!-- .element: class="fragment" -->

Notes:

- `wscat -c wss://polkadot-rpc.dwellir.com -x '{"jsonrpc":"2.0", "id":1, "method":"state_getRuntimeVersion"}' | jq`.
- `wscat -c wss://polkadot-rpc.dwellir.com -x '{"jsonrpc":"2.0", "id":1, "method":"system_version"}' | jq`.
- Show polkadot telemetry: https://telemetry.polkadot.io/.

---

### JSON-RPC: Application

- On top of `SCALE` and `JSON-RPC`, a large array of libraries have been built.

- &shy;<!-- .element: class="fragment" --> `PJS-API` / `PJS-APPS`
- &shy;<!-- .element: class="fragment" --> `subxt`
- &shy;<!-- .element: class="fragment" --> Any many more!

Notes:

https://github.com/JFJun/go-substrate-rpc-client
https://github.com/polkascan/py-substrate-interface
more here: https://project-awesome.org/substrate-developer-hub/awesome-substrate
Listen to James Wilson introducing subxt: https://www.youtube.com/watch?v=aFk6We_Ke1I

---

## Additional Resources! 😋

> Check speaker notes (click "s" 😉)

Notes:

- see "Client Libraries" here: https://project-awesome.org/substrate-developer-hub/awesome-substrate
- https://paritytech.github.io/json-rpc-interface-spec/introduction.html
- Full subxt guide: https://docs.rs/subxt/latest/subxt/book/index.html

---

### JSON-RPC: Mini Activity

In Kusama:

- Find the genesis hash..
- Number of extrinsics at block 10,000,000.
- The block number is stored under `twox128("System") ++ twox128("Number")`.
- Find it now, and at block 10,000,000.

<br/>

- Refer to the "Substrate; Show Me The Code" lecture to find the right RPC endpoints.
- You have 15 minutes!

Notes:

```sh
# 10,000,000 in hex
printf "%x\n" 10000000
# Genesis hash
wscat -c wss://kusama-rpc.polkadot.io -x '{"jsonrpc":"2.0", "id":72, "method":"chain_getBlockHash", "params": ["0x0"] }' | jq
# Hash of the block at height 10,000,000
wscat -c wss://kusama-rpc.polkadot.io -x '{"jsonrpc":"2.0", "id":72, "method":"chain_getBlockHash", "params": ["0x989680"] }' | jq
# The block at height 1,000,000
wscat -c wss://kusama-rpc.polkadot.io -x '{"jsonrpc":"2.0", "id":72, "method":"chain_getBlock", "params": ["0xdcbaa224ab080f2fbf3dfc85f3387ab21019355c392d79a143d7e50afba3c6e9"] }' | jq

# `0x26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac` now.
wscat -c wss://kusama-rpc.polkadot.io -x '{"jsonrpc":"2.0", "id":72, "method":"state_getStorage", "params": ["0x26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac"] }' | jq
# `0x26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac` at block 1,000,000.
wscat -c wss://kusama-rpc.polkadot.io -x '{"jsonrpc":"2.0", "id":72, "method":"state_getStorage", "params": ["0x26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac", "0xdcbaa224ab080f2fbf3dfc85f3387ab21019355c392d79a143d7e50afba3c6e9"] }' | jq
```

Notice that this number that we get back is the little endian (SCALE) encoded value that we passed in at first.

---

## `subxt`

- Something analogous to `PJS api` for Rust.
- The real magic is that it generates the types by fetching the metadata at compile time, or linking
  it statically.
- ..It might need manual updates when the code, and therefore the metadata changes.
