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
  <span style="font-size: 0.7em; opacity: 0.7">(Pro tip: Always use `true`)</span>
- Result: Subscription ID
  ```ts
  { jsonrpc: "2.0", id: "1", result: "B4GEopiw1w38Wkr…MxpkWH4JPd4S" }
  ```
- Notifications
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

### New Block

```ts
{
  event: "newBlock",
  blockHash: "0x00…000",
  parentBlockHash: "0x00…000",
  newRuntime: {…}
}
```

---

### Best Block Changed

```ts
{
  event: "bestBlockChanged",
  bestBlockHash: "0x00…000"
}
```

---

### Finalized

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

https://excalidraw.com/#json=rV-hfREHgg-bLmhQtMb0o,jc3goNcep8O8tg_HANnz5A

---

# Block Pinning

- Light-client friendly
- Initialized event `finalizedBlockHashes`
- New Block event `blockHash`
- `chainHead_v1_unpin`

---

### Stop

```ts
{
  event: "stop"
}
```
