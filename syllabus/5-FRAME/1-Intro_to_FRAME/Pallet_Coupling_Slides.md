---
title: FRAME Pallet Coupling
description: A look into how multiple pallets interact.
duration: 1 hour
instructors: ["Shawn Tabrizi"]
---

# Pallet Coupling

---

## Overview

Substrate believes in building modular and composable blockchain runtimes.

The building blocks of a FRAME runtime are Pallets.

Pallet coupling will teach you how to configure multiple pallets to interact with each other.

---

## Types of Coupling

- Tightly Coupled Pallets

	- Pallets which are directly connected to one another.
	- You must construct a runtime using exactly the pallets which are tightly coupled.

- Loosely Coupled Pallets

	- Pallets which are connected "loosely" with a trait / interface.
	- You can construct a runtime using any pallets which satisfy the required interfaces.

---

## Tightly Coupled Pallets

Tightly coupling is often an easier, but less flexible way to have two pallets interact with each other.

It looks like this:

```rust [2]
#[pallet::config]
pub trait Config: frame_system::Config + pallet_treasury::Config {
	// -- snip --
}
```

Note that everything is tightly coupled to `frame_system`!

---

## What Does It Mean?

If Pallet A is tightly coupled to Pallet B, then it basically means:

> This Pallet A requires a runtime which is also configured with Pallet B.

You do not necessarily need Pallet A to use Pallet B, but you will always need Pallet B if you use Pallet A.

---

## Example: Treasury Pallet

The Treasury Pallet is a standalone pallet which controls a pot of funds that can be distributed by the governance of the chain.

There are two other pallets which are tightly coupled with the Treasury Pallet: Tips and Bounties.

You can think of these like "Pallet Extensions".

---

## Treasury, Tips, Bounties

`pallet_treasury`

```rust
#[pallet::config]
pub trait Config<I: 'static = ()>: frame_system::Config { ... }
```

---

## Cyclic Dependencies

Rust does not allow crates to have cyclic dependencies.

However, trait interfaces must be shared across the pallets which implement or depend on them.

The Solution: Move traits to a shared `primitives` crate.

---
