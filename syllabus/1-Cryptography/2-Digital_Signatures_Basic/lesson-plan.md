# Lesson 2: Digital Signatures (Part 1)

## Core Ideas to Convey

The introduction was very conceptual.
This lecture will be hands on and actually use some of the primitives.

- Signature API
- Using hash functions in Substrate
- Signature payloads
- HDKD
- Key generation and backup

_Note: The instructor should have a Rust console open and should be showing how a signature is made during this section._

## Outline

- Getting a bit more into the primitives
  - Signature API, e.g. https://github.com/paritytech/polkadot-blockchain-academy/pull/127/files#diff-7635ccd71356c216312500ec4fdf67647047fd0024ffe7ea4c642b5adc4ee93fR57-R65 <!-- TODO Remove this check disable once repo is public ? likely not... --> <!-- markdown-link-check-disable-line -->
- Sign a message
  - Easy to do from console
  - Import `sp_core::sr25519` and sign a message
  - Verify a signature
  - Show message authenticity/integrity by altering the message in verification
- Blake2-256
  - Can do demo from console -> import `sp_core::Hasher`
  - Part of all signing processes
- Designing the signing payload
  - Important as part of system design to provide desired guarantees
  - Replay attacks: using the same signature in unintended contexts
    - Example: Authorizing a transfer one time should not imply authorizing it repeatedly
  - We use things like account nonces and checkpoint blocks to limit validity to some context
- Signature schemes: What are Sr25519 and Ed25519?
  - ECDSA (used initially in Bitcoin/Ethereum) was developed to work around the patent on Schnorr signatures, but today new applications are discuraged from deploying ECDSA.
  - ECDSA complicates more advanced cryptographic techniques, like threshold signatures.
    ECDSA's usage by blockchains has resulted in a minefield of problematic advanced ECDSA protocols, including broken or insanely complex threshold signatures schemes.
  - Dan Bernstein's Ed25519 is Schnorr signature designed to reduce mistakes in implementation and usage in classical applications, like TLS certificates.
  - Sr25519 addresses several small risk factors that emerged from Ed25519 usage by blockchains
- HDKD
  - Possible to derive several keys from a "parent"
  - Hard vs. soft
  - Hard derivation requires the secret key and derives new secret keys
    - All typical "operational security" usages should favor hard derivation over soft derivation because hard derivations avoid leaking the sibling keys, unless the original secret is compromized.
    - Use example: Wallets can derive keys for use in different consensus systems while only needing to back up one secret plus a pattern for child derivation.
  - Soft derivation allows one to create derived addresses from only the public key, so payers can create addresses for payees.
    - However, contrary to hard derivation, they all have related or effectively the same private key.
    - They typically leak information about related addresses.
    - Soft derivations are used by UTXO-like chains so that wallet front ends, or others services without the secret keys, can scan the chain for incoming transactions on the derived addresses.
    - Soft derivations can break some niche advanced protocols, but our sr25519 crate avoids supporting protocols that conflict with soft derivations.
- Mnemonics and secret seed creation
  - Many wallets use a dictionary of words and give people phrases, often 12 or 24 words, as these are easier to back up/recover than byte arrays.
    - Some people create their own phrases.
      This is usually stupid.
  - There are many ways to get from this phrase to a secret key (256 bits for our purposes).
    - BIP39 uses 2048 rounds of a hash function.
    - [Substrate uses the entropy byte array from the mnemonic](https://github.com/paritytech/substrate-bip39).
    - Both are valid ways to generate keys, but differences do affect mnemonic portability between wallets, because they will arrive at different keys from the same mnemonic.
    - [Some more info here](https://wiki.polkadot.network/docs/learn-accounts#address-generation-derivation-and-portability)

## Activities and Exercises

- **[EVCXR](https://github.com/google/evcxr)** can be installed
  - Google Repl for Rust
  - Console where you can type Rust declarations and snippets
- VSCode or another standardized editor
