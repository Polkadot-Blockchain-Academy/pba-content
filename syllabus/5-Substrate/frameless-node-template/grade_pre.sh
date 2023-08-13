RUST_LOG=grading=debug WASM_FILE=$1 cargo \
  nextest \
  run \
  --release \
  -p runtime \
  --failure-output immediate \
  --success-output immediate

cp ./target/nextest/default/result.xml ./result.xml
cat result.xml | jtm  > result.json

# RUST_LOG=grading=info,frameless=debug WASM_FILE=/Users/kianenigma/Desktop/Parity/pba/assignment-3-frame-less-submissions/assignment-3-frame-less-snowmead/runtime.wasm cargo test -p runtime
