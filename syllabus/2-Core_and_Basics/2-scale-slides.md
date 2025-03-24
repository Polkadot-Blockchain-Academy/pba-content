---
title: SCALE
description: SCALE Codec
---

# SCALE

---

# SCALE: Introduction

## Serialization

- The process of translating a data structure or object state into a format that can be stored.
- Types of Serialization: Self-Describing vs Non Self-Describing
- Codecs: Encoders/Decoders

---

## SCALE: Simple Concatenated Aggregate Little-Endian

Designed for high-performance, copy-free data encoding and decoding in resource-constrained environments.

- Non Self describing: the context must be known to decode.
- Little-Endian: has some performance benefits, like free casting.

---

### Little-Endian

<img src="./endian.png" />

---

## Little-Endian vs Big-Endian Example:

- How to encode the `15` decimal value as a `u32`:

  - Big-Endian: &nbsp;&nbsp; `0x 00 00 00 ff`
  - Little-Endian: `0x ff 00 00 00`

---

## SCALE Basic Primitives

- u8, u16, u32, u64, u128, u256
- i8, i16, i32, i64, i128, i256
- boolean (specialized u8)

Notes:

Nothing fancy here

---

## SCALE: Compact

- It can optimally store any integer from `0` to `2^536` without wasting memory.

- The least significant 2 **bits** of the **first byte** indicate the number of bytes that are used.
  - `0` => 1 byte, from 0 to 64 (2^6 - 1)
  - `1` => 2 bytes, from 64 to 16383 (2^14 - 1)
  - `2` => 4 bytes, from 16384 to 1073741823 (2^30 - 1)
  - `4` => The remaing bits of the first byte indicate the length (+4)

---

## SCALE: Compact

- Examples:

  - `1` &nbsp; -> `0b_000001_00` &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; -> `0x04`<!-- .element: class="fragment" -->
  - `2` &nbsp; -> `0b_000010_00` &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; -> `0x08`<!-- .element: class="fragment" -->
  - `3` &nbsp; -> `0b_000011_00` &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; -> `0x09`<!-- .element: class="fragment" -->
  - `63` -> `0b_111111_00` &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-> `0xfc`<!-- .element: class="fragment" -->
  - `64` -> `0b_000000_01_00000001` -> `0x0101`<!-- .element: class="fragment" -->
  - `65` -> `0b_000001_01_00000001` -> `0x0501`<!-- .element: class="fragment" -->

---

## SCALE: Compact

- Exercise:
  - Manually decode `0x01ff` and `0x05`

---

## SCALE Complex types

A "complex" type is a codec that references other types.

- Enum (AKA variants, tagged unions, discriminated union, etc).<!-- .element: class="fragment" -->
- Tuples: Simply the concatenation of different types of codecs.<!-- .element: class="fragment" -->
- Structs: Same as tuples, but the values are named (only relevant on the context).<!-- .element: class="fragment" -->
- Vectors: A collection of a dynamic size of any other type.<!-- .element: class="fragment" -->
<li class="fragment">- Specialized Enums:
<ul>
  <li>- Option: The first byte indicates whether there is a value or not.</li>
  <li>- Result: An Enum which always has 2 different tags, one for success and one for error.</li>
</ul></li>
- Specialized Vector: String -> Vector(u8)<!-- .element: class="fragment" -->
- Opaque: A "meta-type" which is a "de-facto" standard, an specialized Vector of bytes.<!-- .element: class="fragment" -->

---

# SCALE Complex types: Enum

- The first byte indicates the index of the "tag".
- Example:

```js
Enum({
  foo: u16,
  bar: boolean,
  baz: _void,
});
```

- `0x000100` => `foo(1)`
- `0x0100` => `bar(false)`
- `0x02` => `baz`

- Observations:
  - Enums make it possible to have circular Codec definitions.
  - `Result` and `Option` are specialized Enums.
  - There can't be more than 256 options.

---

# SCALE Complex types: Structs and Tuples

- Structs and Tuples are conceptually the same thing, but Structs have named values in its definitions.
- Example:

```js
Tuple(Tuple(u8, u8, u8), boolean, Option(u32));
```

```js
Struct({
  color: Struct({ red: u8, green: u8, blue: u8 }),
  isReady: boolean,
  price: Option(u32),
});
```

---

# SCALE: Complex types: Vectors

- A compact encoded number indicates the number of instances that follow.

- Example:

```js
Vector(compact);
```

- `0x00` => `[]`
- `0x0404` => `[1]`
- `0x080400` => `[1, 0]`
- `0x0904000101` `[1, 0, 64]`

- Common Vectors:
  - `String` => `Vector(u8)`
  - `Bytes` => `Vector(u8)`

Notes:

Mention Fixed sized Arrays and explain why conceptually they are the same as a Tuple.

---

# SCALE: "Opaque"

- Sometimes its convenient to use a Vector of bytes, so that we don't have to decode everything.
- Example: The body of a block is: `Vector(Opaque(Extrinsic))` => `Vector(Vector(u8))`
