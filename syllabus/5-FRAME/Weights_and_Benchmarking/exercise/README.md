# FRAME Benchmarking Exercise

This is just a clone of the `substrate-node-template`.

## Testing your benchmarks

```bash
cargo test -p pallet-template --features runtime-benchmarks
```

## Compiling your node

```bash
cargo build --release --features runtime-benchmarks
```

> Note: In production, you would want to use `--profile=production` rather than `--release`, but it is way slower to compile, and `--release` already takes a while. Good enough for our exercises.

## Running Your Benchmarks

```bash
cargo run --release --features=runtime-benchmarks \
benchmark \
pallet \
--chain=dev \
--steps=50 \
--repeat=20 \
--pallet=pallet-template \
--extrinsic="*" \
--execution=wasm \
--wasm-execution=compiled \
--heap-pages=4096 \
--output=weights.rs
```
