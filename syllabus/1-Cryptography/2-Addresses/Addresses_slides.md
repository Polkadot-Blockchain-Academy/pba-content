---
title: Addresses
description: Addresses in cryptography
duration: 30 min
---

# Addresses

---

## HDKD

Hierarchical Deterministic Key Derivation

<img style="width: 1100px;" src="../../../assets/img/1-Cryptography/HD-Deterministic-Wallet.png" />

---

## Hard vs. Soft

Key derivation allows one to derive (virtually limitless)<br/>child keys from one "parent".

Derivations can either be "hard" or "soft".

---

## Hard vs. Soft

<img style="width: 1200px;" src="../../../assets/img/1-Cryptography/soft-vs-hard-derivation.png">

---

## Hard Derivation

Hard derivation requires the secret key and derives new child secret keys.

Typical "operational security" usages should favor hard derivation over soft derivation because hard derivations avoid leaking the sibling keys, unless the original secret is compromised.

Always do hard paths first, then conclude in soft paths.

---

## Hard Derivation in Wallets

Wallets can derive keys for use in different consensus systems while only needing to back up one secret plus a pattern for child derivation.

<img style="width: 1000px;" src="../../../assets/img/1-Cryptography/Hard-Derivation-in-Wallets.png" />

---

## Hard Derivation in Wallets

Let's imagine we want to use this key on multiple networks, but we don't want the public keys to be connected to each other.

<img style="width: 1000px;" src="../../../assets/img/1-Cryptography/Hard-Derivation-in-Wallets.png" />

---

<!-- .slide: data-background-color="#4A2439" -->

# Rust Demo

## Hard Derivation

Notes:

Hard keys: Take a _path_ (data like a name/index), concatenate with the original key, and hash it for a new key.
They reveal nothing about keys above them, and only with the _path_ between it and children could they be recovered.

---

## Soft Derivation

Soft derivation allows one to create derived addresses from only the public key.
Contrary to hard derivation, _all_ keys are related.

Notes:

- With any key and the paths to children and. or parents, the public _and_ private keys can be recovered.
- Soft derivations can break some niche advanced protocols, but our sr25519 crate avoids supporting protocols that conflict with soft derivations.

---

## Soft Derivation

- Note that these generate new addresses, but use the same secret seed.
- We can also use the same paths, but only using the Account ID from `//polkadot`. It generates the same addresses!

---

## Soft Derivation in Wallets

Wallets can use soft derivation to link all payments controlled by a single private key, without the need to expose the private key for the address derivation.

**Use case:** _A business wants to generate a new address for each payment, but should be able to automatically give customers an address without the secret key owner deriving a new child._

Notes:

On the use case, taking each payment at a different address could help make the association between payment and customer.

See: <https://wiki.polkadot.network/docs/learn-accounts#soft-vs-hard-derivation>

---

<!-- .slide: data-background-color="#4A2439" -->

# Rust Demo

## Soft Derivation

Notes:

See the Jupyter notebook and/or HackMD cheat sheet for this lesson.

Mention that these derivations create entirely new secret seeds.

---

# Mnemonics and Seed Creation

Notes:

These are all different _representation_ of a secret. Fundamentally doesn't really change anything.

---

## Mnemonics

Many wallets use a dictionary of words and give people phrases,<br/>often 12 or 24 words, as these are easier to back up/recover than byte arrays.

Notes:

High entropy needed.
People are _bad_ at being random.
Some people create their own phrases... this is usually stupid.

---

## Dictionaries

<pba-cols>
<pba-col>

There are some standard dictionaries to define which words (and character sets) are included in the generation of a phrase. Substrate uses the dictionary from BIP39.

</pba-col>
<pba-col>

| No. | word    |
| --- | ------- |
| 1   | abandon |
| 2   | ability |
| 3   | able    |
| 4   | about   |
| 5   | above   |

<pba-flex style="font-size: .6em;" center>

_The first 5 words of the [BIP39 English dictionary](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt)_

</pba-col>
</pba-cols>

---

## Mnemonic to Secret Key

Of course, the secret key is a point on an elliptic curve, not a phrase.

BIP39 applies 2,048 rounds of the SHA-512 hash function<br/>to the mnemonic to derive a 64 byte key.

Substrate uses the entropy byte array from the mnemonic.

---

## Portability

Different key derivation functions affect the ability to use the same mnemonic in multiple wallets as different wallets may use different functions to derive the secret from the mnemonic.

---

## Address Types

Generally, you will encounter 3 different modern types of cryptography across most systems you use.

- Ed25519
- Sr25519
- ECDSA

We will go more in depth in future lectures!

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
