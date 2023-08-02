---
title: (Deprecated)JSON-RPC Protocol
description: Substrate Overview for Web3 engineers
duration: 45 minutes
---

# JSON-RPC Protocol

### _..and its usage in Substrate._

---

### The Need For a Common Language

<img style="width: 1200px;" src="../../assets/img/5-Substrate/dev-4-1-json.svg" />

---

## JSON-RPC

> JSON-RPC is a remote procedure call protocol encoded in JSON. It is similar to the XML-RPC protocol, defining only a few data types and commands.

---v

### JSON-RPC

- Nowadays, mostly version 2 is used.
- Request

```json
{ "jsonrpc": "2.0", "method": "subtract", "params": { "minuend": 42, "subtrahend": 23 }, "id": 3 }
```

- Response, if `id` is provided

```json
{ "jsonrpc": "2.0", "result": 19, "id": 3 }
```

---v

### JSON-RPC

- Entirely transport agnostic. Deliver the packet to the server, and it will reply.
- Substrate based chains expose both `ws` and `http` (or `wss` and `https`, if desired).

> with `--ws-port` and `--rpc-port`, 9944 and 9934 respectively.

Notes:

```sh
# Kusama endpoint
echo '{ "jsonrpc":"2.0", "id":1,  "method":"system_chain" }' | websocat -B 99999999 ws://34.79.74.54:9924
```

---v

### JSON-RPC

- JSON-RPC methods are conventionally written as `scope_method`

  - e.g. `rpc_methods`, `state_call`

- `author`: for submitting stuff to the chain.
- `chain`: for retrieving information about the _blockchain_ data.
- `state`: for retrieving information about the _state_ data.
- `system`: information about the chain, not to be confused with `frame-system`.
- `rpc`: information about the RPC endpoints.

---v

### JSON-RPC

- The full list of substrate RPC can be seen here: https://polkadot.js.org/docs/substrate/rpc/

---v

### JSON-RPC

- Let's look at a few examples:

- `system_name`, `system_chain`, `system_chainType`, `system_health`, `system_version`, `system_nodeRoles`, `rpc_methods`, `state_getRuntimeVersion`, `state_getMetadata`

```sh
# Polkadot public endpoint
echo '{"jsonrpc":"2.0", "id":1, "method":"system_chain" }' | websocat -B 99999999 wss://rpc.polkadot.io | jq
```

---v

### JSON-RPC

- The whole point of JSON-RPC is to abstract away programming languages and allow a client and server to talk to each other.
- That being said, the following are the noteworthy "libraries" examples that you can use:
  - `polkadot.js API`: low and high level library in JS/TS
  - `JSONRPSee` (low level Rust library)
  - `subxt` (high level Rust library)
  - https://polkadot.js.org/apps/#/rpc

---v

### JSON-RPC: Activity

- Prepare `websocat` (and optionally `jq` to pretty display JSON outputs) as your tool of
  choice.

1. Find the genesis hash of the given chain via RPC.
1. Find the first 4 bytes of the code of the given chain.
1. Find the first 4 bytes of the code of the given chain at block 1 million.
1. The block number is stored under `twox(System) ++ twox(Number)`. Find it!

Notes:

```sh
# Kusama
echo '{"jsonrpc":"2.0", "id":72, "method":"chain_getBlockHash", "params": ["0x0"] }' | websocat -B 99999999 ws://34.79.74.54:9944 | jq
echo '{"jsonrpc":"2.0", "id":72, "method":"chain_getBlock", "params": ["0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"] }' | websocat -B 99999999 ws://34.79.74.54:9944 | jq

echo '{"jsonrpc":"2.0", "id":72, "method":"state_getStorage", "params": ["0x3a636f6465"] }' | websocat -B 99999999  ws://34.79.74.54:9944 | head -c 100

System: 0x26aa394eea5630e07c48ae0c9558cef7
Number: 0x02a5c1b19ab7a04f536c519aca4983ac
Key: 0x26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac
```

---v

### JSON-RPC: Code Activity

- Try and find all the RPC endpoints in the Rust code!
- The runtime can expose custom RPCs as well, try and find them!
- You have 15 minutes!

---

## Additional Resources! 😋

> Check speaker notes (click "s" 😉)

Notes:

- see "Client Libraries" here: https://project-awesome.org/substrate-developer-hub/awesome-substrate
- https://paritytech.github.io/json-rpc-interface-spec/introduction.html

TODO:

subxt, especially usage of light clients.

safe and unsafe RPCs
