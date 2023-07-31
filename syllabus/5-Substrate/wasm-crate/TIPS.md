rustup target add wasm32-wasi
rustup target add wasm32-unknown-unknown

cargo init --lib
apply stuff (`cdylib`, `#[no_mangle]`)

cargo build --target wasm32-unknown-unknown --release

Install `wasmtime-cli`, maybe `cargo install wasmtime-cli`? Figure it out based on the `wasmtime-cli`
docs!

wasmtime ./target/wasm32-unknown-unknown/release/wasm-crate.wasm --invoke <func_name> <arg1> <arg2> ...

DON'T USE `bindgen`! That's for compiling wasm to browser!

A rust `lib` crate can be executed via `wasmtime`, a rust `bin` crate can be executed itself with
`cargo run`, but there are caveats. USE LIB CRATE for now.

If you want to play with the de-compiled WAM file:

wasm-gc target/wasm32-unknown-unknown/release/wasm_crate.wasm out.wasm
wasm2wat out.wasm | grep export

Even with `wasi`, in order to create a file you need permission.

Note that functions like `std::time::now()` and such are available in `wam32-unknown-unknown`, but
simply panic. The point is that you probably won't get a compile error, but it won't work either.

the point of `f32` is to show that you we can use float, but float are un-deterministic, so YOU
SHOULD NOT USE THEM IN YOUR RUNTIME.

Try and use `Vec<_>` or other types that are dynamically allocated. Recall that `wasm32-unknown-unknown` does not have an allocator!

Further ideas:

- make print work in wasm32-unknown-unknown using "host functions" (https://docs.wasmtime.dev/wasm-rust.html#importing-host-functionality).
- try and bring an external dependency in your crate. You will probably run into `feature = std`.
