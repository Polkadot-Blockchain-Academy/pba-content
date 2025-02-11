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

- Mod. 2 recap:
  - What is the runtime (and its metadata)?
  - The metadata sets us apart of other blockchains.

---

## Runtime

- The runtime is the business logic of Polkadot-based networks.
- It defines the algorithm needed for determining the state of the next block.
- It is part of the state, not part of the host.

Notes:

Compared to other blockchains (such as Ethereum) the logic to determine the next block state is not part of the
state itself. Therefore, it can't be modified without a change in the host itself.

---

## Metadata

- The Runtime's Metadata is a bunch of information needed to interact with the Runtime. It defines the shape of the
  interactions, instead of "what" do they do.
- It includes _enough_ information to perform any kind of interaction with the blockchain, as well as decoding its state.

---

## Metadata

### Shape

The metadata is versioned, and its current stable version is `15`.

```typescript
interface RuntimeMetadataV15 {
  types: Vec<LookupEntry>;
  pallets: Vec<PalletMetadata>;
  extrinsic: ExtrinsicMetadata;
  ty: Type; // not useful, it is the type of the runtime itself
  apis: Vec<RuntimeApiMetadata>;
  outer_enums: OuterEnums;
  custom: CustomMetadata; // we won't enter this one, un-typed extra metadata
}
```

---

## Metadata

### Lookup

The lookup of the metadata defines the shape of all types used in the runtime.

<span style="font-size: 0.6em; opacity: 0.6">(This is already `scale-info`)</span>

```typescript
interface LookupEntry {
  id: u32; // this one is unique, and sequential
  path: Vec<String>; // where is it found in Rust code
  type_params: Vec<TypeParameter>; // Rust generics
  type_def: TypeDef; // THE JUICE!
  docs: Vec<String>; // Rust docs
}
```

Notes:

Path and docs are self-descriptive. Type params only give information about the generics used in Rust.

---

## Metadata

### Lookup `type_def`

```typescript
enum TypeDef {
  Composite, // struct
  Variant, // enum
  Sequence, // variable length
  Array, // fixed length
  Tuple,
  Primitive, // essentially integers and booleans
  Compact,
  BitSequence,
}
```

Notes:

Explain a bit every one. Don't focus particularly, it is just a quick summary.

---

## Metadata

### Runtime APIs

They allow to execute functions present in the runtime.

```typescript
interface RuntimeApiMetadata {
  name: String;
  methods: Vec<{
    name: String;
    inputs: Vec<{ name: String; ty: Type }>; // lookup index
    output: Type; // as well!
    docs: Vec<String>;
  }>;
  docs: Vec<String>;
}
```

---v

Example: `Metadata` APIs

```json
{
  "name": "Metadata",
  "methods": [
    {
      "name": "metadata",
      "inputs": [],
      "output": 862,
      "docs": [
        " Returns the metadata of a runtime."
      ]
    },
    {
      "name": "metadata_at_version",
      "inputs": [
        {
          "name": "version",
          "type": 4
        }
      ],
      "output": 863,
      "docs": [
        " Returns the metadata at a given version.",
        "",
        " If the given `version` isn't supported, this will return `None`.",
        " Use [`Self::metadata_versions`] to find out about supported metadata version of the runtime."
      ]
    },
    {
      "name": "metadata_versions",
      "inputs": [],
      "output": 121,
      "docs": [
        " Returns the supported metadata versions.",
        "",
        " This can be used to call `metadata_at_version`."
      ]
    }
  ],
  "docs": [
    " The `Metadata` api trait that returns metadata for the runtime."
  ]
},
```

---

## Metadata

### Outer Enums

They are sugar! One can build this type by himself by aggregating all pallets information.

These types are handy though to decode any `call_data`, `event`, or module `error`.

```typescript
interface OuterEnums {
  // lookup indices
  call_enum_ty: Type;
  event_enum_ty: Type;
  error_enum_ty: Type;
}
```

---

## Metadata

### Pallets

TODO

```typescript
interface PalletMetadata {
  name: String;
  storage: Option<PalletStorageMetadata<T>>;
  calls: Option<PalletCallMetadata<T>>;
  event: Option<PalletEventMetadata<T>>;
  constants: Vec<PalletConstantMetadata<T>>;
  error: Option<PalletErrorMetadata<T>>;
  index: u8;
  docs: Vec<String>;
}
```
