---
title: JSON-RPC Spec
description: New JSON-RPC Spec in depth
---

# JSON-RPC Spec

---

# Recap

## Block States

<img rounded src="./img/block-states.png" />

Notes:

https://excalidraw.com/#json=rV-hfREHgg-bLmhQtMb0o,jc3goNcep8O8tg_HANnz5A

Concepts: Finalized, Pruned, Best, Fork

---

# Recap

## JSON-RPC

- Client - Server RPC protocol with JSON
- Request
```js
{ jsonrpc: "2.0", id: "{req_id}", method: "method_name", params: {…} }
```

- Response
```js
{ jsonrpc: "2.0", id: "{req_id}", result: {…} }
```

- Notification
```js
{ jsonrpc: "2.0", method: "method_name", params: {…} }
```

---

# Recap

## JSON-RPC Spec

- Spec build for polkadot that uses JSON-RPC
- Group of functions
  - rpc
  - chainSpec
  - <span class="fragment highlight-red">chainHead</span>
  - transaction
  - transactionWatch
  - archive

Notes:

We will focus on chainHead

---

# JSON-RPC Spec

```js
>> { jsonrpc: "2.0", id: "1", method: "rpc_methods", params: {} }

<< { jsonrpc: "2.0", id: "1", result: [
     "rpc_methods",
     "chainSpec_v1_chainName",
     "chainSpec_v1_genesisHash",
     "chainSpec_v1_properties",
     "chainHead_v1_call",
     "chainHead_v1_follow",
     …
   ]}
```

Notes:

Mention `id` field can be any string, used to correlate messages.

---

# Chain Head

- Subscription
  - `chainHead_v1_follow`
  - `chainHead_v1_unfollow`

---

# Chain Head

- Operations
  - `chainHead_v1_header`
  - `chainHead_v1_body`
  - `chainHead_v1_storage`
  - `chainHead_v1_call`
- Control <!-- .element: class="fragment" -->
  - `chainHead_v1_continue`
  - `chainHead_v1_stopOperation`
  - `chainHead_v1_unpin`

---

# Follow

- Parameters
  withRuntime: boolean
  ```ts
  { jsonrpc: "2.0", id: "1", method: "chainHead_v1_follow", params: [true] }
  ```
  <span class="fragment" style="font-size: 0.7em; opacity: 0.7">(Pro tip: Always use `true`)</span>
- Result: Subscription ID <!-- .element: class="fragment" -->
  ```ts
  { jsonrpc: "2.0", id: "1", result: "B4GEopiw1w38Wkr…MxpkWH4JPd4S" }
  ```
- Notifications <!-- .element: class="fragment" -->
  ```js
  {
      jsonrpc: "2.0",
      method: "chainHead_v1_followEvent",
      params: {
        subscription: "B4GEopiw1w38Wkr…MxpkWH4JPd4S",
        result: {…}
      }
  }
  ```

Notes:

Dig a bit into the `withRuntime` parameter, and why using `false` it's a bad idea 99% of the time.

The notifications we will see will be the value of `result`, we omit the JSON-RPC wrapper and subscription parameter for simplicity.

---

## Initialized

```ts
{
  event: "initialized",
  finalizedBlockHashes: [
    "0x00…000",
    …
    "0x00…000"
  ],
  finalizedBlockRuntime: {…}
}
```

---

## New Block

```ts
{
  event: "newBlock",
  blockHash: "0x00…000",
  parentBlockHash: "0x00…000",
  newRuntime: {…}
}
```

---

## Best Block Changed

```ts
{
  event: "bestBlockChanged",
  bestBlockHash: "0x00…000"
}
```

---

## Finalized

```ts
{
  event: "finalized",
  finalizedBlockHashes: [
    "0x00…000",
    …
    "0x00…000"
  ],
  prunedBlockHashes: [
    "0x00…000",
    …
    "0x00…000"
  ],
}
```

---

# Follow

<img rounded src="./img/block-states.png" />

Notes:

Example of notifications emitted by a follow subscription based on this image.

---

# Block Pinning

- Light-client friendly
- Initialized event `finalizedBlockHashes`
- New Block event `blockHash`
- `chainHead_v1_unpin`

---

## Stop

```ts
{
  event: "stop"
}
```

---

# Operations

- Query the chain

- Send an operation request
- Receive an Operation ID as a result
- Receive a Follow Event with the Operation ID with the actual result

---

## Header

- Parameters
  ```ts
  [followSubscription: string, hash: string]

  { jsonrpc: "2.0", id: "1",
    method: "chainHead_v1_header",
    params: ["B4GEopiw1w38Wkr…MxpkWH4JPd4S", "0x00…00"]
  }
  ```
- Result: SCALE-encoded header <!-- .element: class="fragment" -->
  ```ts
  { jsonrpc: "2.0", id: "1", result: "0x00…00" }
  ```

Notes:

The only exception, as the header is guaranteed to be already in the node.

---

## Body

- Parameters
  ```ts
  [followSubscription: string, hash: string]

  { jsonrpc: "2.0", id: "1",
    method: "chainHead_v1_body",
    params: ["B4GEopiw1w38Wkr…MxpkWH4JPd4S", "0x00…00"]
  }
  ```
- Result: Operation ID <!-- .element: class="fragment" -->
  ```ts
  { jsonrpc: "2.0", id: "1", result: "GhwhKA4yL3…Roc29d8e" }
  ```
- Notification: <!-- .element: class="fragment" -->
  ```ts
  {
    event: "operationBodyDone",
    operationId: "GhwhKA4yL3…Roc29d8e",
    value: [
      "0x00…00",
      …
      "0x00…00",
    ]
  }
  ```

Notes:

The only exception, as the header is guaranteed to be already in the node.

---

### JSON-RPC Spec Playground

Notes:

Show basic correlating messages, pinning / unpinning

```ts
import { chainSpec } from "polkadot-api/chains/westend2"
import { getSmProvider } from "polkadot-api/sm-provider"
import { start } from "polkadot-api/smoldot"

const smoldot = start({
  maxLogLevel: 0,
})

const provider = getSmProvider(smoldot.addChain({ chainSpec }))

let id = 0
const connection = provider((message) => {
  const msg = JSON.parse(message)
  ellipsisBody(msg)
  console.log(msg)
  if (
    msg.method === "chainHead_v1_followEvent" &&
    msg.params.result.event === "newBlock"
  ) {
    const reqId = id++
    console.log("Request body", reqId)
    connection.send(
      JSON.stringify({
        jsonrpc: "2.0",
        id: reqId,
        method: "chainHead_v1_body",
        params: [msg.params.subscription, msg.params.result.blockHash],
      }),
    )
  }
})

connection.send(
  JSON.stringify({
    jsonrpc: "2.0",
    id: id++,
    method: "chainHead_v1_follow",
    params: [true],
  }),
)

function ellipsisBody(res: any) {
  if (
    res.method === "chainHead_v1_followEvent" &&
    res.params.result.event === "operationBodyDone"
  ) {
    res.params.result.value = res.params.result.value.map((v: string) =>
      v.length > 32 ? v.slice(0, 32) + "…" : v,
    )
  }
}
```

---

## Runtime Call

- Parameters
  ```ts
  [subscription: string, hash: string, fnName: string, params: string[]]

  { jsonrpc: "2.0", id: "1",
    method: "chainHead_v1_body",
    params: [
      "B4GEo…JPd4S",
      "0x00…00",
      "AccountNonceApi_account_nonce",
      "0x00…00"
    ]
  }
  ```
- Notification: <!-- .element: class="fragment" -->
  ```ts
  {
    event: "operationCallDone",
    operationId: "GhwhKA4yL3…Roc29d8e",
    output: "0x00…00"
  }
  ```

---

### Fetch metadata playground

Notes:

```ts
import { chainSpec } from "polkadot-api/chains/westend2"
import { getSmProvider } from "polkadot-api/sm-provider"
import { start } from "polkadot-api/smoldot"
import { decAnyMetadata, u32 } from "@polkadot-api/substrate-bindings"
import { toHex } from "polkadot-api/utils"

const smoldot = start({
  maxLogLevel: 0,
})

const provider = getSmProvider(smoldot.addChain({ chainSpec }))

let id = 0
const connection = provider((message) => {
  const msg = JSON.parse(message)

  if (msg.method === "chainHead_v1_followEvent") {
    if (msg.params.result.event === "initialized") {
      const lastFinalized = msg.params.result.finalizedBlockHashes.at(-1)
      connection.send(
        JSON.stringify({
          jsonrpc: "2.0",
          id: `${id++}-call`,
          method: "chainHead_v1_call",
          params: [
            msg.params.subscription,
            lastFinalized,
            "Metadata_metadata_at_version",
            toHex(u32.enc(15)),
          ],
        }),
      )
      msg.params.result.finalizedBlockHashes.forEach((hash: string) =>
        connection.send(
          JSON.stringify({
            jsonrpc: "2.0",
            id: `${id++}-unpin`,
            method: "chainHead_v1_unpin",
            params: [msg.params.subscription, hash],
          }),
        ),
      )
    }
    if (msg.params.result.event === "newBlock") {
      connection.send(
        JSON.stringify({
          jsonrpc: "2.0",
          id: `${id++}-unpin`,
          method: "chainHead_v1_unpin",
          params: [msg.params.subscription, msg.params.result.blockHash],
        }),
      )
    }

    if (msg.params.result.event === "operationCallDone") {
      const metadata = decAnyMetadata(msg.params.result.output)
      console.log("received metadata")
      console.log(metadata)
    } else {
      console.log(msg)
    }
  }
})

connection.send(
  JSON.stringify({
    jsonrpc: "2.0",
    id: id++,
    method: "chainHead_v1_follow",
    params: [true],
  }),
)
```

---

## Storage

- followSubscription: string
- hash: string
- items: Array
  - key: string
  - type
    - value
    - hash
    - closestDescendantMerkleValue
    - descendantsValues
    - descendantsHashes
- childTrie

---

### Chain storage recap

---

### RFC-9

---

# Challenges

- Correlating messages
- Which state is the one we want?
- Block pinning

---

## Best block state: Handling reorgs
