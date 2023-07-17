cargo build --target wasm32-unknown-unknown --release
cargo build --target wasm32-wasi --release
cp target/wasm32-unknown-unknown/release/wasm_crate.wasm unknown.wasm
cp target/wasm32-wasi/release/wasm_crate.wasm wasi.wasm

cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-wasi
cp target/wasm32-unknown-unknown/debug/wasm_crate.wasm debug.unknown.wasm
cp target/wasm32-wasi/debug/wasm_crate.wasm debug.wasi.wasm
