---
title: Build Simple Parachain
description: We're going to build a simple parachain without Cumulus
duration: 1.5 hours
---

# Build Simple Parachain

We're going to build a simple parachain without Cumulus!

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

## Building a collator that adds number

---

## Exercise

TODO

---

<!-- .slide: data-background-color="#4A2439" -->

## Questions?

---

## References

