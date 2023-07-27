RUST_LOG=grading=info WASM_FILE=$1 cargo nextest run -p runtime
cp /target/nextest/default/junit.xml ./result.xml
