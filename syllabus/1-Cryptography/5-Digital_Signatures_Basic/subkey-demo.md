# Subkey Signature and HDKD (Hierarchical Deterministic Key Derivation) Demo

All the subkey examples also exist in [a jupyter notebook](./signature-demo.ipynb) for reference.
As an alternative, here are subkey examples to compliment/replace using the REPL.

## Key Generation

```sh
subkey generate

Secret phrase: s      desert piano add owner tuition tail melt rally height faint thunder immune
  Network ID:        substrate
  Secret seed:       0x6a0ea68072cfd0ffbabb40801570fa5e9f3a88966eaed9dedaeb0cf140b9cd8d
  Public key (hex):  0x7acdc47530002fbc50f413859093b7df90c27874aee732dca940ea4842751d58
  Account ID:        0x7acdc47530002fbc50f413859093b7df90c27874aee732dca940ea4842751d58
  Public key (SS58): 5Eqipnpt5asTm7sCFWQeJjsNJX5cYVJMid3zjKHjDUGKBJTo
  SS58 Address:      5Eqipnpt5asTm7sCFWQeJjsNJX5cYVJMid3zjKHjDUGKBJTo
```

## Sign

```sh
echo -n 'Hello Polkadot Blockchain Academy' | subkey sign --suri 'desert piano add owner tuition tail melt rally height faint thunder immune'
```

> Note, this changes each execution, this is one viable signature: `f261d56b80e4b53c70dd2ba1de6b9384d85a8f4c6d912fd86acab3439a47992aa85ded04ac55c7525082dcbc815001cd5cc94ec1a907bbd8e3138cfc8a382683`

## Verify

```sh
echo -n 'Hello Polkadot Blockchain Academy' | subkey verify  '0xf261d56b80e4b53c70dd2ba1de6b9384d85a8f4c6d912fd86acab3439a47992aa85ded04ac55c7525082dcbc815001cd5cc94ec1a907bbd8e3138cfc8a382683' \
    '0x7acdc47530002fbc50f413859093b7df90c27874aee732dca940ea4842751d58'
```

> Expect `Signature verifies correctly.`

## Tamper with the Message

> Last char in `Public key (hex)` - AKA URI - is changed:

```sh
echo -n 'Hello Polkadot Blockchain Academy' | subkey verify \
 '0xf261d56b80e4b53c70dd2ba1de6b9384d85a8f4c6d912fd86acab3439a47992aa85ded04ac55c7525082dcbc815001cd5cc94ec1a907bbd8e3138cfc8a382683' \
    '0x7acdc47530002fbc50f413859093b7df90c27874aee732dca940ea4842751d59'
Error: SignatureInvalid
```
