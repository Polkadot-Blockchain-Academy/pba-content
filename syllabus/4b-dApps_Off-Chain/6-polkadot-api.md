---
title: Polkadot-API
description: Polkadot-API in depth
---

# Polkadot-API

<span style="color: lightgray; font-size: 0.8em;">(aka PAPI)</span>

Notes:

So far we've seen the low-level details for interacting with a chain. Now we'll go a higher level of abstraction and see how Polkadot-API, in specific, solves those challenges, and what other features it adds.

---

## PAPI principles

- Light-client first
- Modular
- Light weight
- Composable
- Simplify dApp development

---v

### Packages

<img rounded src="./img/papi-packages.png" />

Notes:

https://excalidraw.com/#json=MlLv0U2YI3nfTZwEkRTf1,iaAmM8qLSSYKNFkoJtlNsA

Warn that observable-client is opinionated and unstable, it's an implementational detail of papi. The other packages are unopinionated or at least stable.

---v

### Polkadot-API

- Top-level package
- Main use case: dApps
- Chain-agnostic
- Query or subscribe to chain state
- Send and track transactions

---v

### CLI

- Codegen
- Generate TS types for chains
- Part of Compatibility API

---v

### Observable Client

- Internal detail
- Block pinning
- Subscriptions

---v

### Metadata Builders

- Codecs from metadata
  - Lookup
  - Storage
  - Runtime APIs
  - Transactions
  - Events

---v

### Substrate Bindings

- Known codecs for Substrate
  - SS58 Format
  - Block header
  - Metadata v14
  - Metadata v15
- Other known utilities
  - Storage hashers
  - Multisig account

---v

### Substrate Client

- JSON-RPC Spec
- TS Interface
- Correlations
  - Message IDs
  - Subscription IDs
  - Operation IDs

---v

### JSON-RPC Providers

```ts
type JsonRpcProvider = (onMessage: MessageCallback) => JsonRpcConnection

type MessageCallback = (message: string) => void

interface JsonRpcConnection {
  send: (message: string) => void
  disconnect: () => void
}
```

---

## Compatibility API

- Chain interactions are defined in the metadata.
- The metadata shows the "API" of the runtime.
- But the runtime is upgradeable!
- How to approach this?

---v

### Solution #1

- #YOLO #BreakEverything
- Offer types for interactions
- If the chain doesn't support it when running, too bad!
- Polkadot-API: `client.getUnsafeApi()`
- Polkadot-JS + Type Registry

---v

### Solution #2

- Get a unique value for each interaction
- Checksum based on the type for the interaction
- If the checksum changes, then the interaction is not compatible

---v

### Solution #2

<pba-cols style="font-size: 0.8em">
<pba-col>

```ts
Referenda.submit({
  proposal_origin: Enum {
    BigSpender,
    MediumSpender,
    SmallSpender,
    WishForChange
  },
  proposal: Enum {
    Inline(Binary),
    Lookup(Binary, number)
  },
  enactment_moment: Enum {
    After(number),
    At(number)
  }
})
```

</pba-col>
<pba-col>

```ts
hash(str) := str // For demo purposes
Checksum(void) := hash("0")
Checksum(number) := hash("1")
Checksum(Binary) := hash("2")
Checksum((a,b)) := hash("3(" +
  ChkS(a) + ChkS(b) + ")")

Checksum(Enum) := hash("4(" +
  forEach variant {
    name + Checksum(type)
  } + ")"
)

Checksum(Struct) := hash("5(" +
  forEach property {
    name + Checksum(type)
  } + ")"
)
```

</pba-col>
</pba-cols>

```ts
Checksum(proposal_origin) = "4(BigSpender0MediumSpender0Sma…ender0WishForChange0)"
Checksum((Binary, number)) = "3(21)"
Checksum(proposal) = "4(Inline2Lookup3(21))"
Checksum(enactment_moment) = "4(After1At1)"
Checksum(Referenda.submit) =
 "5(proposal_origin4(BigSpend…Change0)proposal4(Inline2Lookup3(21))enactm…r1At1))"
```

---v

### Solution #2

- Checksum changes if anything changes at all
- `polkadot-api@0.10.0`
- Problem: Cases where changes should not affect it

---v

### Solution #3

- Bundle type definitions
- Compare metadata types on runtime

```ts
Referenda.submit({
  proposal_origin: Enum {
    BigSpender,
    MediumSpender,
    SmallSpender,
    WishForChange,
    CouncilElection
  },
  proposal: Enum {
    Inline(Binary),
    Lookup(Binary, number)
  },
  enactment_moment: Enum {
    After(number),
    At(number)
  }
})
```

---v

### Solution #3

- There's more!
- We can compare based on the actual value
- Compatibility levels
  - Identical
  - Backwards compatible
  - Partial
  - Incompatible
- Different between "in" and "out"

---v

### Solution #3

<table>
  <thead>
    <tr>
      <th>Change</th>
      <th>Sending</th>
      <th>Receiving</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Add a property to a struct</td>
      <td><span style="color: #fc8d62">Incompatible</span></td>
      <td><span style="color: #80dbde">Backwards compatible</span></td>
    </tr>
    <tr>
      <td>Add an <b>optional</b> property to a struct</td>
      <td><span style="color: #80dbde">Backwards compatible</span></td>
      <td><span style="color: #80dbde">Backwards compatible</span></td>
    </tr>
    <tr>
      <td>Remove a property from a struct</td>
      <td><span style="color: #80dbde">Backwards compatible</span></td>
      <td><span style="color: #fc8d62">Incompatible</span></td>
    </tr>
    <tr>
      <td>Make an optional property mandatory</td>
      <td><span style="color: #8da0cb">Partial</span></td>
      <td><span style="color: #80dbde">Backwards compatible</span></td>
    </tr>
    <tr>
      <td>Add a variant to an Enum</td>
      <td><span style="color: #80dbde">Backwards compatible</span></td>
      <td><span style="color: #8da0cb">Partial</span></td>
    </tr>
    <tr>
      <td>Remove a variant from an Enum</td>
      <td><span style="color: #8da0cb">Partial</span></td>
      <td><span style="color: #80dbde">Backwards compatible</span></td>
    </tr>
    <tr>
      <td>Change the type of an optional property</td>
      <td><span style="color: #8da0cb">Partial</span></td>
      <td><span style="color: #8da0cb">Partial</span></td>
    </tr>
    <tr>
      <td>Change a u8 to a u128</td>
      <td><span style="color: #fc8d62">Incompatible</span></td>
      <td><span style="color: #fc8d62">Incompatible</span></td>
    </tr>
  </tbody>
</table>

---

## Descriptors

```ts
import { dot } from '@polkadot-api/descriptors'

// ...

const dotApi = client.getTypedApi(dot);

const account = await dotApi.query.System.Account.getValue(ALICE)
```

---v

## Descriptors

- Typescript definitions
```ts
  function api.query.System.Account(id: AccountID): Promise<{
    nonce: number,
    data: {
      free: bigint,
      reserved: bigint,
      frozen: bigint
    }
  }>
```

- Metadata types
  - ~500 KB SCALE per each chain
  - ~1.5 MB JSON
  - React/Vue: less than 140 KB
  - How to bundle efficiently?

---v

### Opportunities

1. Lazy-loading
2. Reduce information
3. Whitelisting
4. Common types

---v

### Lazy Loading

- Let the dApp render first
- Load descriptors in the background

```ts
// Static import
import metadata from './metadata';

// Dynamic import
const metadataPromise = import('./metadata');
```

Notes:

Challenges when dealing with promises.

---v

### Lazy Loading

```ts
import { dot } from '@polkadot-api/descriptors'

// ...

const dotApi = client.getTypedApi(dot);

const account = await dotApi.query.System.Account.getValue(ALICE)
```

Notes:

How can we let dev start interacting without having to first "await" the descriptors?

---v

### Reduce Information

- Metadata is massive
- We're only interested in data (lookup)
- Some types can also be merged (u8, u16, u32, …)

<img rounded src="./img/descriptors.png" />

Notes:

Checksum can be used to deduplicate types

https://excalidraw.com/#json=6r98ZeK6wQ0dsmSon8Lon,ybCarw4wIswxi3Hin5ArlQ

---v

### Whitelisting

- dApps don't use the full list of interactions
- We can offer API to reduce bundle sizes

---v

### Common types

- Multichain dApps
- Runtimes are very similar
- Combine all into one

Notes:

Tradeoff for dApps that are multichain but one at a time.
