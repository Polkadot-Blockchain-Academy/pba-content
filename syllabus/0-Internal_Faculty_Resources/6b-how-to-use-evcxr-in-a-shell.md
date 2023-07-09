# How to use [Evcxr](https://github.com/google/evcxr) in a Jupyter Notebook and/or REPL

## \[Note on use\] Reset, keeping build artifacts

Run the Rust commands in an Evcxr shell to "reset" the kernel, keeping build artifacts:

```rust
:clear // Clear all state, **keeping compilation cache**, use this over a kernel restart when possible. You will need to re-run the :deps to have them loaded into state.
:last_compile_dir // Show where the target is for cargo for this kernel, in case you want to recover these
```

Set the env var `EVCXR_TMPDIR=<the output of :last_compile_dir>` to get the REPL to use this dir.

```sh
# Launching Evcxr from a shell
EVCXR_TMPDIR=<dir from :last_compile_dir> evcxr
```

## Notebook Kernel & REPL Environment Setup

The below should be run at kernel startup before you start.
All dependencies that we need to build _before_ anything else in this notebook will work.

Instead of rebuilding see the `Reset, keeping build artifacts` section to clear REPL state only.

```rust
:dep sp-core = { version = "7.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.36" }

// Loading & building dependencies crates here takes *a while*!
// Run this while you move on to the readings below.
// Notice: A kernel restart removes all target artifacts except those in sccache!
// ONLY restart only if explicitly needed.
```

## Digital Signatures Example

Here we demonstrate a few parts of the [Substrate Primitives (`sp_core`)](https://paritytech.github.io/substrate/master/sp_core/index.html) library for interacting with keys and signatures.

```rust
use sp_core::{
	blake2_256,
	crypto::{Derive, Ss58Codec, Ss58AddressFormatRegistry},
	DeriveJunction,
	hexdisplay::HexDisplay,
	Pair as _,
	sr25519::{Pair, Public},
};
```

## Key Generation

```rust
// Generate a secret key.
let (pair, mnemonic, _) = Pair::generate_with_phrase(None);
```

```rust
// Reveal your Secret Seed Phrase
mnemonic
```

```rust
// Derive the public key.
let pk = pair.public();
```

```rust
// Print public key as raw bytes
pk.0
```

```rust
// Print public key hex encoded
<HexDisplay>::from(&pk.0)
```
