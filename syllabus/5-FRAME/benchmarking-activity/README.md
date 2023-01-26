# FRAME Benchmarking Exercise

This is a clone of the `substrate-node-template` that includes a `Template` pallet with some arbitrary logic, specifically designed to help you understand how to define the benchmarking for the pallet calls. In addition to these calls, the pallet contains specific ones for a voting mechanism, which is closer to a more daily use case. The goal of this exercise is to define the benchmarking functions for these calls and configure each call to use the weights that result from the benchmarking process.

## Step-by-step Guide for Benchmarking.

1. Review the pallet code located in `substrate-node-template/pallets/template/src/lib.rs` and locate all the calls defined in this pallet. It is important to understand the logic of these calls best as we can since benchmarking is mostly about defining "the most expensive" scenario for each particular case.

2. Locate the benchmarking file `substrate-node-template/pallets/template/src/benchmarking.rs`. In the `benchmarks!` macro, you will define the specific benchmarks functions:

```rust
benchmarks! {
   extrinsic_name {
       /* setup initial state */
   }: _{ /* execute extrinsic or function */ }
   verify {
       /* verify final state */
   }
}
```

3. Once you have defined your benchmarks, it is important to test that all the functions execute successfully (check [Testing your benchmarks](#testing)).

4. Compile your node including the benchmarking feature and specifically run the benchmarking function tested in the previous step (check [Compiling your node with benchmarks](#compile) & [Running Your Benchmarks](#running)).

5. If the benchmarks run correctly, a new module with the weight values will be created under the `weights.rs` file. The next step is to integrate this file into your pallet:

   - Import the module.
   - "use" the `WeightInfo` trait.
   - Add a new Config type for the `WeightInfo`
   - Update all extrinsics so the weight used is the one coming from `WeightInfo` (the functions from the WeightInfo trait should match each call).
   - Integrate the `weights.rs` file in your runtime.

6. As we added the new WeightInfo type to the pallet Config, update your mock.rs file as well.

7. Up to this point, we have a "correct" benchmark implementation for each call, but it may not be very optimized in terms of fees. Remember that benchmarks should calculate the most expensive scenario. What happens if our call only uses 10% of the calculated max capacity? We would be overcharging the user for the unused 90% (which is something we don't want to do of course). To optimize the fees and avoid overcharging, the next step is to refund the unused weight:

   - Make use of the return type `DispatchResultWithPostInfo` for eacg extrinsic and within the returned `Ok()` result, provide the distinct value that shows how much reads/writes were actually used as part of the execution of this extrinsic. Note that the arguments for the benchmarking function should also change slightly.

8. Check that your pallet tests complete again with no errors and run the benchmarks again. At this point it would be interesting to compare the weight values calculated from the two processes.

9. Finally, compile your node again.

> In case you have any question about the benchmarking process (or actually any substrate question), checking the codebase of any FRAME pallet is always a good practice. For example, you can use the assets pallet as an example.

## <a name="testing">Testing your benchmarks</a>

```bash
cargo test -p pallet-template --features runtime-benchmarks
```

## <a name="compiling">Compiling your node wtih benchmarks</a>

```bash
cargo build --release --features runtime-benchmarks
```

> Note: In production, you would want to use `--profile=production` rather than `--release`, but it is way slower to compile, and `--release` already takes a while. Good enough for our exercises.

## <a name="running">Running Your Benchmarks.</a>

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
