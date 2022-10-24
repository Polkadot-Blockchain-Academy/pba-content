# Lesson 1: Introduction to Cryptography

## Core Ideas to Convey

### Contextual Assumtions

- People, systems, etc. communicate in the open. That is, we should assume that anyone, not only our intended recipient, may:
  - Read our messages
  - Try to tamper with them
  - Try to impersonate us
  - & c.
- _Why_ do people communicate in the open?
  - Snowden: Really no choice, all communication channels should be assumed to be open.
  - Kerckhoffs's Principle: If security depends on secret _methods_, they can easily be reverse engineered (e.g. CSS for DVDs).
  - Security should depend on knowledge of secret _information_, with open methods.
- Therefore, we'd like some guarantees:
  - Message authenticity (origin, provinence): If you receive a message from me, you know it's from me.
  - Data integrity: You know that message contents have not been tampered with.
  - (Optional) Data accessibility: We share an expectation that only those who know some secret information can access the data in a meaningful way. (Let's not get too carried away here, e.g. analyzing what one can glean from ciphertext exchange (e.g. "these two parties exchange frequent long messages").)
  - Nonrepudiation: (Asymmetric only), the signer of a message cannot deny signing it. E.g. "I didn't authorize that transfer" is not a legitimate claim in a cryptographic system.

### Working with constrained resources

- Several resources are constrained in practice:
  - Network, storage, etc. are limited
  - It's wasteful to send the same data again and again
  - But we want to know that we're talking about the same data
  - If we have a "fingerprint" of the data, it'd be useful as a reference to it as well
- Privacy is a constrained resource
  - "We once believed that packet switched networks were more or less private. Snowden showed that they were not."
- Hash functions
  - Succinct representation of some pre-image with the expectation that we are referring to the same pre-image (larger data set, history, commitment).
  - Map an infinite domain to a finite domain, but we assume collission resistance
  - Cryptographic vs. Non-cryptographic:
    - Cryptographic is slower, but provides stronger guarantees on collission resistance

### Practice

- Symmetric vs. Asymmetric crypto
  - Examples of each
  - Tradeoffs: key exchange, speed
  - Gav: "There are two separate pieces of data in asymmetric crypto, but one is strictly implied and derivable from the other so the use of the word "needs" is a bit weird. It needs one: the secret. From this secret you can get the other. Asymmetric encryption makes use of a derivable counterpart datum (aka the public key) to the secret, knowledge of which does not imply knowledge of the secret, but may be used to transform data in such a way that it cannot be recovered without the secret."
  - Asymmetric:
    - RSA, Elgamal, ECC (slowest to fastest for given security level)
    - We're mostly concerned with secp256k1/ecdsa, ed25519, sr25519 (specifics later)
    - EC requires 2x bits to AES (symmetric) for same bit security. E.g. 128 bit security requires 256 bit EC key. Of course this makes it a bit slower to operate on than symmetric systems, but comes with the asymmetric advantage of keeping the private key private.
- Encrypting Data
  - Stream vs. block ciphers
  - Encryption is only one use of cryptography.
  - Also Gav: "The ability to decrypt some data and reveal its underlying information directly implies knowledge of some secret, potentially unknown to the originator of the information. Supplying the original information (aka plain text message) can be used in a "challenge game" mechanism as one means of proving knowledge of the secret without compromising it. Typically though, this is done through digital signatures (coming next)."
- Digital signatures
  - Function on some data, using some secret information, that provides a "signature" such that a third party can reasonably verify knowledge of some secret information by the signer, without the signer revealing it, i.e. using only the data, the signature, and some public information.
  - This provides provenance guarantees: nobody is impersonating the signer.
  - Further, this provides integrity guarantees: even if the signed data is in plaintext, the verifier knows that the information has not been tampered with by any intermediate party.
  - Symmetric vs. Asymmetric
    - Problem with symmetric (shared key): If two parties don't trust each other, they can blame the other for signing a message for them.
    - Asymmetric cryptography provides nonrepudiation: In asymmetric crypto, only one party knows the secret. Unless this secret has been compromised, they cannot claim that they did not produce a valid signature.
  - Types used in Substrate: sr25519, ed25519, ecdsa
- Hash functions
  - Used for key derivation, digital signatures, PRNGs
    - Typically only sign 256 B of data. If a message is longer than that, we sign the hash.
    - "PRNG" => In Substrate, we use for things like generating keyless accounts
  - Secure vs. insecure => when an attacker can mine hashes to lopside a data structure
  - Substrate uses Blake2b (cryptographic) and TwoX64Concat (non-cryptographic)
    - TwoX is about 20x faster (source: https://github.com/Cyan4973/xxHash#benchmarks)
    - TwoX is safe when a user (read: attacker) cannot control keys, e.g. a system-assigned index.
    - Blake2b should be used for everything else.
  - Serve as keys in a database
  - Many data structures based on hashing (state trie, blockchain itself, some accounts)
- Erasure Coding
  - Need to introduce in this lecture? Or too early?
- Don't roll your own => it can probably be broken
- Attackers will always go for the weakest link => large key space does not equal security

### Ideas for exercises:

- List modern applications of cryptography (https, bank cards, medical implants, etc.)
- List historical cryptography (substitution cipher) and weaknesses
- List attacks (brute force, probabilistic, social, implementation, etc.)
- Exercises with subkey or Polkadot JS
  - Generate keypairs on different curves
  - Sign/verify some messages
  - No need to get into addresses/extrinsics at this point
  - Just to get familiarity with Substrate tooling

## Prerequisites

None, it should be understandable by someone with no maths knowedge.

## Learning Outcomes

The learner should understand what the most basic primitives are used for.

## Learning Objectives
