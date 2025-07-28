---
title: Ink!
description: Smart contracts in polkadot
---

<img style="width: 300px" src="../../assets/img/0-Shared/logo/ink-white.png" />

---

## ink!

- Smart contracts in Polkadot
- Implemented in Rust
- Exports PolkaVM + Metadata

---

## Smart Contracts

<img src="./img/smartcontracts-1.svg" />

Notes:

https://excalidraw.com/#json=yQC2mtGUS-ngozXnv3LWY,_QUAy86R85h-16dNNVbhEQ

---

## Smart Contracts

<img src="./img/smartcontracts-2.svg" />

---

## Smart Contracts

- Contract storage
- Constructor
- Transactions
  - Called "messages" instead
- Events

Notes:

There are deposits associated when instantiating a contract and adding more values to storage.

---

## Lifecycle

<img src="./img/sc-lifecycle.svg" />

Notes:

https://excalidraw.com/#json=zKnc7S6Mm8gMyYf8leLGB,mP3Q22jaxfWu-SNsb9EIXQ

---

## Endpoints

- Runtime API: Dry-run
- Transaction: Perform some change

---

## ink! + PAPI

```ts
// pnpm papi ink add metadata.json

import { contracts } from "@polkadot-api/descriptors";

const inkClient = getInkClient(contracts.psp22);
```

Notes: At this level, PAPI inkClient only gives TS definitions for encoding/decoding messages

---

## ink! + PAPI

```ts
// Takes in the message name
const increaseAllowance = inkClient.message("PSP22::increase_allowance");

// Encode the data for that message
const messageData = increaseAllowance.encode({
  delta_value: 100_000_000n,
  spender: ADDRESS.bob,
});

const response = await typedApi.apis.ReviveApi.call(
  ADDRESS.alice, // Origin
  ADDRESS.psp22, // Contract address
  0n, // Value
  undefined, // GasLimit
  undefined, // StorageDepositLimit
  messageData
);
```

---

# Let's create our own!

Notes:

Ideas:

- Canvas
- Auction
