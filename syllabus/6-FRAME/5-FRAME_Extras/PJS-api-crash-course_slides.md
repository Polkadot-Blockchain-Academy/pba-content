---
title: Polkadot JS API Crash Course
description: Polkadot JS API Crash Course for Web3 engineers
duration: 1 hour
---

# Polkadot JS API Crash Course

---v

## Context

- De facto library to work with all FRAME-based substrate runtimes.

<img src="../../../assets/img/6-FRAME/pjs.png" style="width: 500px" />

---v

## Overview

> https://github.com/kianenigma/polkadot-js-api-ts-template/

---v

### Overview

- `api.registry`
- `api.rpc`
- `@polkadot/keyring`.
- `@polkadot/utils`.

---v

### Overview

Almost everything else basically builds on top of `api.rpc`.

- `api.tx`
- `api.query`
- `api.consts`
- `api.derive`

---

## `api`

Notes:

```ts
import { ApiPromise, WsProvider } from "@polkadot/api";
const provider = new WsProvider("wss://rpc.polkadot.io");
const api = await ApiPromise.create({ provider });

api.stats;
api.runtimeVersion;
api.isConnected;
```

---

## `api.registry`

Notes:

```ts
api.registry.chainDecimals;
api.registry.chainTokens;
api.registry.chainSS58;

api.registry.metadata;
api.registry.metadata.pallets.map(p => p.toHuman());

api.registry.createType();
```

---

## `api.rpc`

- Can call into all RPC endpoints of the chain.
  All endpoints are divided based on on scope and method, such as `scope_method`, e.g. `system_chain`.

Notes:

```ts
api.rpc.chain.getBlock()

api.rpc.system.health()
await api.rpc.system.version()

await api.rpc.state.getRuntimeVersion()
await api.rpc.state.getPairs("0x")

await api.rpc.state.getKeysPaged("0x", 100)
await api.rpc.state.getStorage()
https://polkadot.js.org/docs/substrate/rpc#getstoragekey-storagekey-at-blockhash-storagedata
await api.rpc.state.getStorageSize("0x3A636F6465"),
```

---

## `Keyring`

Notes:

```ts
import KeyringPair from "@polkadot/keyring";
import { Keyring } from "@polkadot/api";
const kr = new Keyring({ type: "sr25519", ss58Format: 42 });
const account = kr.addFromUri("evil danger film ship lamp gorilla wear job despair garbage dial repair");

let sig = account.sign(new Uint8Array([1, 2, 3]));
account.verify(new Uint8Array([1, 2, 3]), sig, account.publicKey);
```

---

## `api.tx`

Notes:

```ts
let call = api.tx.balances.transfer("5FUDdxaaZfye6ogJgqHh3Usqd6WN6q8aApFH4XNjU9iDvC49", 1000000000000);
call.toHuman();
const signed = call.sign(account);
call.toHuman();

signed.send();
signed.send();

await call1.signAndSend(account, ({ events = [], status, dispatchError }) => {
  console.log(status.toHuman());
});

call
  .paymentInfo(account)(await api.rpc.payment.queryInfo(call.toHex()))
  .toHuman();
```

<!-- TODO: dry-run -->

---

## Types

- `createType`A powerful tool to keep in mind.
- everything is `Codec` by default, and has `toHuman`, `toU8a`, `toHex`, `toString` and **`.eq`**.

Notes:

```ts
import * as BN from "bn.js";
api.createType("Balance", new Uint8Array([1, 2, 3, 4]));
```

---

## Crypto Utils

Notes:

```ts
import { blake2AsHex, xxHashAsHex, checkAddress } from "@polkadot/util-crypto";
blake2AsHex("Foo");
xxHashAsHex("Foo");
```

---

## `api.query`

- `api.query.[module_name].[storage_name]()`
- `api.query.[module_name].[map_name](key)`
- `api.query.[module_name].[map_name].entries()`
- `api.query.[module_name].[double_map_name](key1, key2)`

Notes:

```ts
await api.query.system
  .number()(await api.query.system.lastRuntimeUpgrade())
  .toHuman()(await api.query.council.members())
  .toHuman();
```

```ts
// let's see what else a storage item has
api.query.council.proposalOf;
api.query.council.proposalOf
  .keyPrefix()(await api.query.council.proposalOf.entries())
  .forEach(([key, value]) => console.log(`${key} // ${value}`))(await api.query.council.proposalOf.entries())
  .forEach(([key, value]) => console.log(`${key} // ${key.args[0]} // ${key.slice(-32)} // ${value}`));
await api.query.council
  .proposalOf("0x678debcc07e2300db98fa74979c8e75ebd0075bee8f58bf7c1ca4bda724449f8")(
    await api.query.council.proposalOf.keys()
  )
  .map(x => x.toString());
```

---

## `api.consts`

- `api.consts.[config_name]()`

Notes:

```ts
api.consts.system.version.toHuman();
api.consts.system.blockWeights.toHuman();
api.consts.system.blockLength.toHuman();
```

---

## `api.derive`

Helpers that combine multiple calls and queries into one.
Entirely optional to use.

Notes:

```ts
await api.derive.accounts.hasIdentity("15S7YtETM31QxYYqubAwRJKRSM4v4Ua6WGFYnx1VuFBnWqdG");
const unsub = await api.derive.chain
  .subscribeNewHeads(h => {
    console.log(h.author);
    unsub();
  })(await api.rpc.chain.getBlockHash(100))
  .toHuman()(await api.rpc.chain.getBlock("0xedf9246b378fe4aa1c29d21c64b0bf9880553690ce6cd956c18c03310e49fa5f"))
  .toHuman();
api.derive.chain.getBlockByNumber(1000);

api.derive.chain.bestNumberFinalized();
```

---

## Exercise: Using Polkadot JS API

- Find the some interesting blocks in Polkadot/Kusama:
  - The largest block
  - The block that consumed the most weight.
  - Last remark that Shawn made onchain??? 🤔
- Bonus: use [subxt](https://github.com/emostov/substrate-subxt/).
