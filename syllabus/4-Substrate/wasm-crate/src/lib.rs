#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
	x + 1
}

#[no_mangle]
pub extern "C" fn time() {
	let now = std::time::Instant::now();
	println!("Greet, the time is {:?}!", now);
}

#[no_mangle]
pub extern "C" fn add_one_file(x: i32) {
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("./output").unwrap();
    let output = x +1;
    file.write_all(&output.to_string().as_bytes()).unwrap();
}

#[no_mangle]
pub extern "C" fn add_float(x: f32, y: f32) -> f32 {
    println!("{}, {}", x, y);
    x + y
}

#[no_mangle]
pub extern "C" fn div(x: i32, y: i32) -> i32 {
    x / y
}

/*
cargo build --target wasm32-unknown-unknown --release
cargo build --target wasm32-wasi --release
wasmtime run target/wasm32-unknown-unknown/release/wasm_crate.wasm --invoke add_one 10
wasmtime run target/wasm32-unknown-unknown/release/wasm_crate.wasm --invoke greet
wasmtime run target/wasm32-wasi/release/wasm_crate.wasm --invoke greet

use wasm-gc to clean them wasm files. Compare sizes, grep the wasm2wat
wasm-gc target/wasm32-unknown-unknown/release/wasm_crate.wasm out.wasm

https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md#running-common-languages-with-wasi
*/
