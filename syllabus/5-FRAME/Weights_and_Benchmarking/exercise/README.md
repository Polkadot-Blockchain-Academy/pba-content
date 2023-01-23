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
--output=./pallets/template/src/weights.rs \
--template=frame-weight-template.hbs
```

## Steps

- Review the Pallet Code.
- Write the benchmarks expected in the `benchmarks.rs` file.
- Test your benchmarks execute successfully.
- Run your benchmarks on your substrate node.
- Integrate the generated `weights.rs` file into your pallet:
  - Import the module.
  - "use" the `WeightInfo` trait.
  - Add a new Config type for the `WeightInfo`
  - Update all extrinsics so weight uses `WeightInfo`
  - Extrinsics with a `DispatchResultWithPostInfo` could also refund weight.
  - Update your `mock.rs` file for the new `WeightInfo` type.
  - Check that your pallet tests complete again, and there are no errors.
- Integrate the `weights.rs` file in your final runtime.
