# Build the tool

cargo build

# Test examples:

Why did these messages fail?

## Example 1: Barrier error:

Block failture: 10946380
./target/release/debug-failed-dmp -p 2012 -b 10946379

## Example 2: Transact error:

Block failure: 10557896
./target/release/debug-failed-dmp -p 1000 -b 10557895

## Example 2: Overweight enqueued:

Block failure: 11884750
./target/release/debug-failed-dmp -p 1000 -b 10557895
