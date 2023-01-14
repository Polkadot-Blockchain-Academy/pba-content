---
title: Introduction to FRAME
description: An introduction into FRAME, a framework for building Substrate runtimes.
duration: 1 hour
instructors: ["Shawn Tabrizi"]
---

# Introduction to FRAME

---

## What is FRAME?

FRAME is a Rust framework for more easily building Substrate runtimes.

---

## Explaining FRAME Concisely

<pba-flex center>

Writing the Sudo Pallet:

Without FRAME: 310 lines of code.

With FRAME: 2210 lines of code.

7x Smaller.

</pba-flex>


---

## Goals of FRAME

- Make it easy and concise for developers to develop.
- Provide maximum flexibility and compatibility for pallet developers.
- Provide maximum modularity for runtime developers.
- Be as similar to vanilla Rust as possible.

---

## Building Blocks of FRAME

- FRAME Development
	- Pallets
	- Macros
- FRAME Coordination
	- FRAME System
	- FRAME Executive
	- Construct Runtime

---

## Pallets

FRAME takes the opinion that the blockchain runtime should be composed of individual modules. We call these Pallets.

<image src="../../../assets/img/6-FRAME/frame1.svg" style="height: 600px">

---

## Building Blocks of Pallets

Pallets are composed of multiple parts common for runtime development:

- Dispatchable extrinsics
- Storage items
- Hooks for:
  - Block initialization,
  - Finalizing block (_!= block finality i.e. GRANDPA_)

---

## More Building Blocks of Pallets

And some less important ones:

- Events
- Errors
- Custom validation/communication with tx-pool
- Offchain workers
- A lot more! but you will learn about them later.

---

### "Shell" Pallet

```rust
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]  // snip
  #[pallet::event]   // snip
  #[pallet::error]   // snip
  #[pallet::storage] // snip
  #[pallet::call]    // snip
}
```

---

## FRAME Macros

Rust allows you to write Macros, which is code that generates code.

FRAME uses Macros to simplify the development of Pallets, while keeping all of the benefits of using Rust.

---

## See For Yourself

- `wc -l` will show the number of lines of a file.
- `cargo expand` will expand the macros to "pure" Rust.

```bash
➜  substrate3 git:(master) ✗ wc -l frame/sudo/src/lib.rs
     310 frame/sudo/src/lib.rs

➜  substrate3 git:(master) ✗ cargo expand -p pallet-sudo | wc -l
    Checking pallet-sudo v4.0.0-dev (/Users/shawntabrizi/Documents/GitHub/substrate3/frame/sudo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s

    2210
```

---

## FRAME System

The FRAME System is a Pallet which is assumed to always exist when using FRAME.
