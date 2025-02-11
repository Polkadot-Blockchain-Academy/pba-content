---
title: Runtime / Metadata
description: TODO
duration: 1.5 hours
owner: Carlo Sala
---

# Runtime / Metadata

---

## Runtime / Metadata

#### Content

- Mod. 2 recap: What is the runtime (and its metadata)?
- The metadata sets us apart of other blockchains.
- Metadata parts.
- Metadata issues.

---

## Runtime

- The runtime is the business logic of Polkadot-based networks.
- It defines the algorithm needed for determining the state of the next block.

---

## Metadata

- The Runtime's Metadata is a bunch of information needed to interact with the Runtime. It defines the shape of the
  interactions, instead of "what" do they do.
- It includes _enough_ information to perform any kind of interaction with the blockchain, as well as decoding its state.

---v

## Metadata shape

The metadata is versioned, and its current stable version is `15`.

```rust
struct RuntimeMetadataV15 {
  types: Vec<LookupEntry>,
  pallets: Vec<PalletMetadata>,
  extrinsic: ExtrinsicMetadata,
  ty: Type,
  apis: Vec<RuntimeApiMetadata>,
  outer_enums: OuterEnums,
  custom: CustomMetadata,
}
```

---

## Lookup

The lookup of the metadata defines the shape of all types used in the runtime.

```rust
struct LookupEntry {
  id: u32, // this one is unique, and sequential
  ty: Type,
}
```

Let's dive in the `Type`!

---v

## Type
