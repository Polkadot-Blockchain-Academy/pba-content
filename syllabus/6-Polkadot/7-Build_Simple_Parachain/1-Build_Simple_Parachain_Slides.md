---
title: Build Simple Parachain
description: We're going to build a simple parachain without Cumulus
duration: 1.5 hours
---

# Agenda for Today

- Build a simple collator without Cumulus
- Introduction to Cumulus and how to build a Parachain
- Workshop: Manually registering a parachain
- Workshop: How to acquire a parachain slot

## Before we begin:

```sh
git clone https://github.com/paritytech/polkadot/
cd polkadot
cargo build --release
cargo build -p test-parachain-adder-collator --release
```

# Build Simple Blobchain

We're going to build a simple blobchain without Cumulus!

Notes:

This will not a be a blockchain, but a blobchain.
Meaning it will have some state and the rules for validating
state transitions, but it will not have any transactions.

---

## Parachain requirements

A parachain needs two things:

- A WASM runtime with `validate_block` function exposed
- Node side that can sync relay chain blocks and talk to the relay chain

Notes:

Talking to the relay chain means speaking the networking protocol of
Polkadot to distribute the PoV.

---

## Minimal Parachain runtime

```rust
#![no_std]
#![cfg_attr(
	not(feature = "std"),
	feature(core_intrinsics, lang_items, core_panic_info, alloc_error_handler)
)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[cfg(feature = "std")]
/// Wasm binary unwrapped. If built with `BUILD_DUMMY_WASM_BINARY`, the function panics.
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect(
		"Development wasm binary is not available. Testing is only \
						supported with the flag disabled.",
	)
}

#[cfg(not(feature = "std"))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
	core::intrinsics::abort()
}

#[cfg(not(feature = "std"))]
#[alloc_error_handler]
#[no_mangle]
pub fn oom(_: core::alloc::Layout) -> ! {
	core::intrinsics::abort();
}

#[cfg(not(feature = "std"))]
#[no_mangle]
pub extern "C" fn validate_block(_params: *const u8, _len: usize) -> u64 {
	loop {}
}
```

Notes:

The panic and oom handlers are Rust-specific things you don't need to worry about.
If we actually include an infinite loop into the `validate_block` function,
a parablock will never be backed/included by the relay chain validators.

---

## Parachain node side

- Our node will sync relay chain blocks
- When importing the new best block, we'll connect to the backing group
- Then we'll advertise our block ("collation") to a validator in the group
- The validator will request the collation from us using `collator-protocol`
- Now it's in the hands of validators to include our block

Notes:

Validators are shuffled into small backing groups, which rotate 
regularly with `group_rotation_frequency`.
Currently, collators can only produce the next block after their previous
block has been included by the relay chain (remember `CandidateIncluded`).
Since inclusion happens in the next block after candidate being backed,
this means collators can only produce blocks every 12s. Async backing
will change that.

---

## Collator-protocol

Polkadot contains the implementation of the both collator and validator
side of the collator protocol.

```rust
/// What side of the collator protocol is being engaged
pub enum ProtocolSide {
	/// Validators operate on the relay chain.
	Validator {
		/// The keystore holding validator keys.
		keystore: SyncCryptoStorePtr,
		/// An eviction policy for inactive peers or validators.
		eviction_policy: CollatorEvictionPolicy,
	},
	/// Collators operate on a parachain.
	Collator(
		PeerId,
		CollatorPair,
		IncomingRequestReceiver<request_v1::CollationFetchingRequest>,
	),
}
```

Notes:

We're going to use Polkadot as a library configured for the collator side.

---

## Time to look into the code

> https://github.com/paritytech/polkadot/tree/master/parachain/test-parachains/adder

---

## Exercise

Make the state of the Parachain a fixed sized field that evolves at each block according to
Game of Life and print the state of each collation.

> https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

> https://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust

---

<!-- .slide: data-background-color="#4A2439" -->

## Questions?

---

## References

