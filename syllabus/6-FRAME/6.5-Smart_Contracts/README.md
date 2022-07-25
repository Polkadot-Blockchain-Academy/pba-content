# Required Software on Presentation Computer

This software must be installed on the presentation computer
for the demos:

- `cargo-contract`: Installation instructions can be found [here](https://github.com/paritytech/cargo-contract#installation).
- `binaryen`: The installation instructions above should have covered this if you have a mainstream os.
- `wabt`: The installation instructions above should have covered this if you have a mainstream os.
- `bat`: [Installation instructions](https://github.com/sharkdp/bat#installation).
- `substrate-contracts-node`: [Download here](https://github.com/paritytech/substrate-contracts-node/releases).
- `jq`: [Installation instructions](https://stedolan.github.io/jq/download).
- `cargo-expand`: [Installation instructions](https://github.com/dtolnay/cargo-expand#installation).
- Clone ink! repository : `git clone https://github.com/paritytech/ink.git`.

If everything is installed you can test that all necessary stuff is there by checking this:

```
cd ink/examples/flipper/ && cargo +nightly contract build && cat target/ink/flipper.contract | jq . | bat
```

You should see a JSON file displayed.

Please then check that this command also works in the `ink/flipper` folder:

```
wasm2wat target/ink/flipper.wasm | bat
```

This should display a file starting with `(module`.

Please then check that this command works too in the `ink/flipper` folder:

```
cargo expand
```

Please then check that this also works:

```
substrate-contracts-node --tmp
```

The node should be starting up, you won't see blocks being produced though
due to the node configuration (occurs only when transactions are issued and
then instantaneous).

_Please let us know:_
* _In which folder on the presentation computer the `ink` repository is cloned to._
* _In which folder the `substrate-contracts-node` binary resides._
