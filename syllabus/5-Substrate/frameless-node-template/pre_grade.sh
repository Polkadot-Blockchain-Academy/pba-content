RUST_LOG=grading=info WASM_FILE=$1 cargo \
  nextest \
  run \
  --release \
  -p runtime \
  --failure-output immediate \
  --success-output immediate

cp ./target/nextest/default/junit.xml ./result.xml

# RUST_LOG=grading=info,frameless=debug WASM_FILE=/Users/kianenigma/Desktop/Parity/pba/assignment-3-frame-less-submissions/assignment-3-frame-less-snowmead/runtime.wasm cargo test -p runtime
