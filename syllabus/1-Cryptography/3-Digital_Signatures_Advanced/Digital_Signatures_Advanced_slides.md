---
title: Advanced Digital Signatures
description: More exotic digital signature methods
duration: 1 hour
---

# Advanced Digital Signatures

---

### Certificates

A certificate is one issuing key signing a message containing another certified key, which attests to some properties or relationship about the certified key.

Notes:

We must already trust the issuing key to give this attestation any significance, traditionally provided under "Certificate Authority" or "Web of Trust" schemes.

---

### Certificates

Issuing keys do not necessarily have unilateral certification capabilities, although some do.

For example, the certificate transparency protocol for TLS certificates helps protect against compromised Certificate Authorities.

---

### Certificates in Web3

We are building systems that don't have a "Certificate Authority".
But we can still use certificates in some niche instances.

Notes:

Potential example to give verbally:

- Session keys are a set of keys that generally run in online infrastructure.
  An account, whose keys are protected, can sign a transaction to certify all the keys in the set.
- Session keys are used to sign operational messages, but also in challenge-response type games to prove availability by signing a message.

---

### Multi-Signatures

We often want signatures that must be signed by multiple parties to become valid.

- Require some threshold of members to agree to a message
- Protect against key loss

---

### Types of Multi-Signature

<pba-flex center>

- Verifier enforced
- Cryptographic threshold
- Cryptographic non-threshold (a.k.a. signature aggregation)

---

### Verifier Enforced Multiple Signatures

We assume that there is some verifier, who can check that some threshold of individual keys have provided valid signatures.

This could be a trusted company or third party.
For our purposes, _it's a blockchain_.

---

### Verifier Enforced Multiple Signatures

Multiple signatures enforced by a verifier generally provide a good user experience, as no interaction is required from the participants.

Notes:

This good experience comes at the cost of using state and more user interactions with the system, but is generally low.

---

### Cryptographic Threshold Multi-Sigs

Cryptographic multi-signatures can be achieved purely on the signer side (without support from the verifier).
This makes more compact signatures compatible with legacy systems.

<pba-flex center>

_Example: "5 of 7 key holders have signed this message."_

Notes:

These require multi-party computation (MPC) protocols, which add some complexity for the signing users.

---

### Cryptographic Non-Threshold Multi-Sigs

Sometimes we do not need a threshold represented in a public key.
But we want a succinct way to demonstrate that multiple parties have signed a message.

<pba-flex center>

_Example: "5 key holders have signed this message."_

---

### Key Generation - Threshold

Threshold multi-signature schemes require that all signers run a _distributed key generation_ (DKG) protocol that constructs key _shares_.

The secret encodes the threshold behavior, and signing demands some threshold of signature _fragments_.

This DKG protocol breaks other useful things, like hard key derivation.

---

### Key Generation - Non-Threshold

In non-threshold multi-signatures, signatures from individual public keys are aggregated.

Each participant can choose their own key to use for the multi-signature.

---

### Schnorr Multi-Sigs

Schnorr signatures are primarily used for threshold multi-sig.

- Fit legacy systems nicely, and can reduce fees on blockchains.
- Reduce verifier costs in bandwidth & CPU time, so great for certificates.
- Could support soft key derivations.

---

### Schnorr Multi-Sigs

However, automation becomes tricky.

We need agreement upon the final signer list and two random nonce contributions from each prospective signer, before constructing the signature fragments.

---

### BLS Signatures

BLS signatures are especially useful for aggregated (non-threshold) multi-signatures (but can be used for threshold as well).

Signatures can be aggregated without advanced agreement upon the signer list, which simplifies automation and makes them useful in consensus.

Verifying individual signatures is _slow_, but verifying aggregated ones is relatively fast.

(Coming to Substrate soon.)

---

### BLS Signatures

Allows multiple signatures generated under multiple public keys for multiple messages to be aggregated into a single signature.

<pba-flex center>

- Uses heavier pairing friendly elliptic curves than ECDSA/Schnorr.
- Very popular for consensus.

---

### BLS Signatures

However...

- DKGs remain tricky (for threshold).
- Soft key derivations are typically insecure for BLS.
- Verifiers are hundreds of times slower than Schnorr, due to using pairings, for a single signature.
- But for hundreds or thousands of signatures on the same message, aggregated signature verification can be much faster than Schnorr.

---

### Schnorr and BLS Summary

Schnorr & BLS multi-signatures avoid complicating verifier logic, but introduce user experience costs such as:

- DKG protocols
- Reduced key derivation ability
- Verification speed

---

### Ring Signatures

- Ring signatures prove the signer lies within some "anonymity set" of signing keys, but hide which key actually signed.
- Ring signatures come in many sizes, with many ways of presenting their anonymity sets.
- Anonymous blockchain transactions typically employ ring signatures.

Notes:

- ZCash uses a ring signature based upon Groth16 zkSNARKs which makes the entire chain history be the anonymity set.
- Monero uses ring signatures with smaller signer sets.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
