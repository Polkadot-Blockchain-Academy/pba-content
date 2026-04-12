---
title: Interacting With a Substrate Blockchain
duration: 60 minutes
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# Interacting With a Substrate Blockchain

---

## Before We Start

Run the following commands, if you have not done already:

```
cargo install staging-chain-spec-builder
cargo install polkadot-omni-node
# in your polkadot-sdk clone
cargo build --release -p minimal-template-runtime
```

---

## Substrate Node's Little Secret

<img src="../../../assets/img/5-Substrate/dev-4-3-omni.svg" />


---v

## Nodes With Hardcoded Runtime

```
# valid, runs the polkadot chain-spec
polkadot --chain polkadot --tmp
# runs the kusama chain-spec
polkadot --chain kusama --tmp
```

---v

## Omni-Nodes

```
# Any chain-spec file generated with chain-spec-builder
polkadot-omni-node --chain spec.json --tmp
```

---

## Chain Specification Files

* JSON File containing initial specification of the chain:
  * What is in the genesis state?
  * What is the native token of the chain?
  * Genesis block hash
  * Bootnodes
  * And more!

---

## Genesis WASM File

* Initial WASM file must be in the chain-spec as well!

```
chain-spec-builder create --runtime ./minimal_template_runtime.wasm
```

---

## Genesis State

* The WASM file and chain-spec-builder have a protocol to communicate _WHAT_ the genesis state should be.

```
chain-spec-builder create --runtime ./minimal_template_runtime.wasm default
                                                                    ^^^^^^^
```

---

## JSON RPC

<img style="width: 800px;" src="../../../assets/img/5-Substrate/dev-4-1-json.svg" />

---

## JSON RPC

* The interface available to you once you run any substrate-based chain.
  * `wscat`, `curl` etc.
  * client libraries, like `polkadot-js`, `papi`, `subxt`
  * UIs like `polkadot-js-apps`, `papi-console`
  * Any much more, that is ultimately built on top of JSON RPC
* (needless to say you may also connect to a public RPC server, not just your local one)

---

## JSON-RPC

The RPC methods that a substrate node exposes are scoped and has the pattern `"<scope>_<method>"`.

```sh
 wscat \
  -c ws://localhost:9944 \
  -x '{"jsonrpc":"2.0", "id": 42, "method":"rpc_methods" }' \
  | jq
```

---

## JSON-RPC: Scopes

- &shy;<!-- .element: class="fragment" --> `author`: for submitting extrinsic to the chain.
- &shy;<!-- .element: class="fragment" --> `chain`: for retrieving information about the _blockchain_ data.
- &shy;<!-- .element: class="fragment" --> `state`: for retrieving information about the _state_ data.
- &shy;<!-- .element: class="fragment" --> `system`: information about the chain.
- &shy;<!-- .element: class="fragment" --> `rpc`: information about the RPC endpoints.

Notes:
recall:

https://docs.rs/sc-rpc-api/latest/sc_rpc_api/

- The full list can also be seen here: https://polkadot.js.org/docs/substrate/rpc/
- Specs: https://paritytech.github.io/json-rpc-interface-spec/introduction.html
- Upcoming changes to JSON-RPC api: https://forum.polkadot.network/t/new-json-rpc-api-mega-q-a/3048

---


---

## QUICK PAPI Demo


---

## Your Workshop

https://hackmd.io/@kizi/B1UU4HI2Wx


---

## Additional Resources! 😋

> Check speaker notes (click "s" 😉)

Notes:

- see "Client Libraries" here: https://project-awesome.org/substrate-developer-hub/awesome-substrate
- https://paritytech.github.io/json-rpc-interface-spec/introduction.html
- Full subxt guide: https://docs.rs/subxt/latest/subxt/book/index.html
- https://github.com/JFJun/go-substrate-rpc-client
- https://github.com/polkascan/py-substrate-interface
- https://github.com/polkadot-api/polkadot-api
- more here: https://project-awesome.org/substrate-developer-hub/awesome-substrate
- Listen to James Wilson introducing subxt: https://www.youtube.com/watch?v=aFk6We_Ke1I
