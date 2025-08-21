---
title: JAM services
description: An introduction to building on top of JAM
duration: 30 mins
---

# JAM services

---

# What we'll learn

<pba-flex center>

- ❌ How to build a JAM client
- ✅ How to build JAM services

---v

<pba-flex center>

Not about **building JAM** but about building **ON TOP** of JAM.

---v

<pba-flex center>

"To understand how Jam breaks up its service code is to understand Jam’s fundamental proposition of generality and scalability."

---

# Services

What can we build on JAM?

---v

## The service account

![Service accounts according to the GP](../../assets/img/7-Polkadot/gp-0.7.1-service-account.png)

Notes:

Explain each symbol:
- `s` is the storage, from bytes to bytes.
- `p` are the preimages, from hash to bytes.
- `l` are the preimage requests, from hash and length to times when they have been requested/provided.
- `f` is the gratis storage offset.
- `c` is the code hash.
- `b` is the balance.
- `g` and `m` are gas limits.
- `r` is the time slot at creation of the service.
- `a` is the time slot of the most recent accumulation.
- `p` is the parent service.

---v

## Similarities with smart contracts

<pba-flex center>

- Anyone can create one
- They have their own **code**, **storage** and **balance**

---v

## Differences with smart contracts

<pba-flex center>

- They have only two entrypoints: `refine` and `accumulate`
- `refine` is stateless and runs in-core
- `accumulate` is stateful and runs on-chain
- They have to procure `coretime`

Notes:

In the latest version of the GP to date, 0.7.1, `on_transfer` was removed in favor of a `transfer` host function in the `accumulate` entrypoint.

The `accumulate` code might more closely resemble a smart contract since it's executed by everyone in the network and has access to all the state.

---

# Authorizers

---v

They're the code that authorizes a particular **work** to be processed.

---v

![Services and authorizers](../../assets/img/7-Polkadot/jam-services-and-authorizers.png)

---

# Soo, how do I make something?

---v

Good news: you just need to implement `refine` and `accumulate`!

Bad news: you need to implement `refine` and `accumulate`.
<!-- .element: class="fragment" -->

---v

## PVM assembly

<pba-flex center>

```assembly
%stack_size = 4096
%rw_data_size = 12
%rw_data = 00 00 00 00 00 00 00 00 00 00 00 00

// ===== JAM entrypoints =====
jump @refine // Index 0
fallthrough
fallthrough
fallthrough
jump @accumulate // Index 5

pub @accumulate:
  // Counter
  a1 = 0
  // Accumulator
  a2 = 0
@target:
  // Fetch
  a3 = u8 [a1 + 0xfeff0003]
  // Accumulate
  a2 = a2 + a3
  // Increase counter
  a1 = a1 + 1
  // Condition
  jump @target if a1 <u 4
  // Change state
  u8 [0xfeff0010] = 3
  a1 = 1224
  a2 = 2335
  a3 = 3446
  a4 = 4557
  // 'write' host function
  ecalli 3
  // Return
  a0 = a2
  trap
```

Notes:

This is basically like writing evm opcodes or wat.

---v

## PolkaVM Examples

<pba-flex center>

https://hackmd.io/@polkadot/jamsdk

Notes:

We created a repository with everything needed to compile C and C++ code to PVM blobs.

---v

<pba-flex center>

```c++
#include "host.hpp"
#include <stdint.h> // Use the C header as we don't have a C++ STL.

POLKAVM_IMPORT(uint32_t, get_third_number);

// Demonstrate that this is C++ by using templates:
template<typename To, typename From>
To convert(From v) {
	return static_cast<To>(v);
}

uint32_t entry(uint32_t a, uint32_t b) {
	auto v = a * b + get_third_number();

	return convert<uint32_t>(v);
}

POLKAVM_EXPORT(uint32_t, entry, uint32_t, uint32_t);
```

---v

There must be a better way...

---

## JAM SDK

https://hackmd.io/@polkadot/jamsdk

---v

## Null authorizer

```rust
#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]

extern crate alloc;
use alloc::string::String;
use jam_pvm_common::{is_authorized::*, *};
use jam_types::*;

#[allow(dead_code)]
struct Authorizer;
jam_pvm_common::declare_authorizer!(Authorizer);

impl jam_pvm_common::Authorizer for Authorizer {
	fn is_authorized(param: AuthConfig, package: WorkPackage, core_index: CoreIndex) -> AuthTrace {
		info!(
			"Null Authorizer, [{core_index}], {} gas, {param} param, {} token",
			gas(),
			package.authorization
		);
		if package.authorization.0 != param.0 {
			panic!("Authorization failed")
		}
		let m = String::from_utf8_lossy(&package.authorization);
		alloc::format!("Auth=<{m}>").as_bytes().to_vec().into()
	}
}
```

---v

## Bootstrap service

For creating new services!

Notes:

Let's take a look at this one.

---

# CoreVM

For building programs!

---

# CoreChains

For building blockchains!

---

# CorePlay

For building actors!

---

# Ideas

Notes:

- Simple currency
- Bridge

---

<pba-flex center>

What will **YOU** build on JAM?
