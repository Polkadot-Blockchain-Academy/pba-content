#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
	x + 1
}

#[no_mangle]
pub extern "C" fn greet() {
	let now = std::time::Instant::now();
	println!("Greet, the time is {:?}!", now);
}

/*
cargo build --target wasm32-unknown-unknown --release
cargo build --target wasm32-wasi --release
wasmtime run target/wasm32-unknown-unknown/release/wasm_crate.wasm --invoke add_one 10
wasmtime run target/wasm32-unknown-unknown/release/wasm_crate.wasm --invoke greet
wasmtime run target/wasm32-wasi/release/wasm_crate.wasm --invoke greet

use wasm-gc to clean them wasm files. Compare sizes, grep the wasm2wat
*/
