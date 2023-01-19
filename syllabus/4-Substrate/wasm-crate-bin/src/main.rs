#[no_mangle]
fn add_one(x: i32) -> i32 {
	x + 1
}

#[no_mangle]
fn time() {
	let now = std::time::Instant::now();
	println!("Greet, the time is {:?}!", now);
}

#[no_mangle]
fn add_one_file(x: i32) {
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("./output").unwrap();
    let output = x +1;
    file.write_all(&output.to_string().as_bytes()).unwrap();
}

#[no_mangle]
fn add_float(x: f32, y: f32) -> f32 {
    x + y
}

fn main() {
    time();
    println!("{:?}", add_float(10.11, 10.22));
}
