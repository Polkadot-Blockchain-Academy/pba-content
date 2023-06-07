---
title: Introduction to Cryptography
description: Cryptographic primitives for Web3 engineers
duration: 1 hour
---

# Introduction to Cryptography

---

## Goals for this lesson

<pba-flex center>

- Understand the goals of cryptography
- Understand some network and contextual assumptions
- Learn what expectations cryptography upholds
- Learn the primitives

</pba-flex>

---

## Cryptography Landscape

<img style="height: 700px; padding-left:100px" src="../../../assets/img/1-Cryptography/crypto-mind-map.svg" />

Notes:

What is covered in this course is all connected subjects.
We will not cover any details for hybrid or interactive protocols in the course.

---

## Operating Context

##### _The internet is a public space._

We communicate over public channels.
Adversaries may want to:

<pba-flex center>

- Read messages not intended for them
- Impersonate others
- Tamper with messages

</pba-flex>

Notes:

Use e-mail as an example of an flawed system.

Some examples include:

- An attacker may impersonate your boss, trying to get you to send them money
- An attacker may change a message sent over a network, e.g. an instruction to transfer 100 EUR to 10000 EUR

Probably best for the teacher to ask students to participate with examples of application messages,
not just person-to-person messages.

---

## Operating Context

##### _Resources are constrained._

- **Network, storage, computation, etc.**: We don't want to send, store, or operate on the same data, but we want guarantees about it, e.g. that we agree on a message's contents.
- **Privacy**: We must assume that all channels can be monitored, and thus closed channels are heavily constrained (i.e. assumed to not exist).

---

## Open vs. Closed Channels

_Cryptography based on public systems is more sound._

**Kerckhoff's Principle:** Security should not rely on secret _methods_,<br/>but rather on secret _information_.

Notes:

There is no such thing as a "closed channel" :)

- Methods can be reverse engineered.
  After that, the communication channel is completely insecure.
  For example, CSS protection for DVDs.
- We always work with public, open protocols.

---

## Cryptographic Guarantees\*

<pba-flex center>

- Secure communication
- Data accessibility
- Message authenticity
- Data integrity
- Non-repudiation (later)

</pba-flex>

Notes:

Cryptography is one of the (most important) tools we have to build tools that are _guaranteed_ to work correctly.
This is regardless of who (human, machine, or otherwise) is using them and their intentions (good or bad).

Why an asterisk?
There generally are no perfect & absolute guarantees here, but for most practical purposes the bounds on where these fail are good enough to serve our needs as engineers and users.
Do note the assumptions and monitor their validity over time (like quantum tech).

---

## Data Accessibility

A party may gain access to information<br/>if and only if they know some secret (a key).

Notes:

The ability to decrypt some data and reveal its underlying information directly implies knowledge of some secret, potentially unknown to the originator of the information.
Supplying the original information (aka plain text message) can be used in a "challenge game" mechanism as one means of proving knowledge of the secret without compromising it.

Mention use of the term "plaintext".

Allegory: A private document stored on server where sysadmin has _access_ can be subpoenaed, violating assumed Attorney-Client Privilege on the document.

---

## Message Authenticity

Like physical signatures, cryptography may be used to give a reasonable expectation of a message's provenance (origin), in order to give the users the **credible** expectation that the stated origin is authentic.

Notes:

- Digital signatures should be difficult (practically speaking: impossible) to forge.
- Digital signatures should verify that the signer knows some secret, without revealing the secret itself.

---

## Data Integrity

Physical signatures provide weak authenticity guarantees<br/>(i.e. they are quite easy to forge), and no integrity guarantees.

---

## Data Integrity

<img style="width: 900px;" src="../../../assets/img/1-Cryptography/Data-Integrity.png" />

Notes:

For example, if you change the year on your university diploma, the dean's signature is still valid.
Digital signatures provide a guarantee that the signed information has not been tampered with.

---

## One-Way Functions

One-way functions form the basis of both<br/>**(cryptographic) hashing** and **asymmetric cryptography**.

- Functions for which we know fast algorithms to compute
- But for which we believe to be hard to invert
- And for which there may be some secret which makes it easy

Notes:

There are a lot of assumptions about why these functions are hard to invert, but we cannot rigorously prove it.
We often express inversion problems in terms of mathematical games or oracles.

---

## Hash Functions

**Motivation:** We often want a succinct representation of some data with the expectation that we are referring to the same data.
A "fingerprint".

Notes:

The following slides serve as an intro.
Many terms may be glossed over, and covered in detail later.
There are lessons later in this module dedicated to hashes and hash-based data structures.\_

---

## Hash Function Applications

Hashes can be useful for many applications:

<pba-flex center>

- Representation of larger data object<br/>(history, commitment, file)
- Keys in a database
- Digital signatures
- Key derivation
- Pseudorandom functions

</pba-flex>

---

## Cryptographic vs. Non-Cryptographic

Cryptographic hash functions provide stronger guarantees<br/>on the last three properties.

But non-cryptographic hash functions are much faster.

Notes:

Substrate uses both (more on that later).

---

## Symmetric Cryptography

Symmetric encryption assumes all parties begin with some shared secret information, a potentially very difficult requirement.<br/>The shared secret can then be used to protect further communications from others who do not know this secret.

In essence, it gives a way of _extending_ a shared secret over time.

---

## Symmetric Encryption

For example, the Enigma cipher in WW2. A _channel_ was initiated by sharing a secret ("key") between two participants. Using the cipher, those participants could then exchange information securely.

However, since the key contained only limited _entropy_ ("information"), enough usage of it eventually compromised the secret and allowed the allies to decode messages. Even altering it once per day was not enough.

---

## Asymmetric Cryptography

- In asymmetric cryptography, we devise a means to transform one value (the "secret") into some corresponding counterpart (the "public" key), preserving certain properties.

- We believe that this is a one-way function (that there is no easy/fast inverse of this function).

- Aside from preserving certain properties, we believe this counterpart (the "public key") reveals no information about the secret.

---

## Asymmetric Encryption

- _Using only the public key_, information can be transformed ("encrypted") such that only those with knowledge of the secret are able to inverse and regain the original information.

---

## Digital Signatures

- _Using the secret key_, information can be transformed ("signed") such that anyone with knowledge of the information and the counterpart public key is able to affirm the operation.

- Digital signatures provide message authenticity and integrity guarantees.

- _The next two lessons are dedicated to digital signatures,<br/>this is strictly an intro._

---

## Digital Signatures

**signing function**: a pure function which operates on some<br/>_message data_ and some _secret_ to yield a _signature_.

A **signature** _proves_ that the signer had knowledge of the secret,<br/>without revealing the secret itself.

The signature cannot be used to create other signatures.

Notes:

A **signing function** is a pure function which operates on some _message data_ (which may or may not be small, depending on the function) and some _secret_ (a small piece of information known only to the operator).
The result of this function is a small piece of data called a _signature_.

It has a special property: it proves (beyond reasonable doubt) that the signer (i.e. operator of the signing function) had knowledge of the secret and utilized this knowledge with the specific _message_ data, yet it does not reveal the secret itself, nor can knowledge of the signature be used to create other signatures (e.g. for alternative message data).

---

## Non-repudiation

Only those with knowledge of some secret information<br/>could have produced a valid signature.

The signer cannot claim that the signature was forged, unless they can defend a claim that the secret was compromised prior to signing.<br/>
Symmetric cryptography does not provide this guarantee: someone else knows the secret.

---

## Practical Considerations

**Symmetric cryptography** is much faster, but requires more setup (key establishment) and trust (someone else knows the secret).

**Asymmetric cryptography** is slow, but typically preserves specific algebraic relationships, which then permit more diverse if fragile protocols.

---

## Hybrid Cryptography

Hybrid cryptography composes new mechanisms from different cryptographic primitives.

For example:

- Symmetric encryption can provide speed, and often confidentiality,
- Hash functions can reduce the size of data while preserving identity,
- And asymmetric can dictate relations among the participants.

---

## Certifications

Certifications are used to make attestations about public key relationships.

Typically in the form of a _signature_ on:

- One or more cryptographically strong identifiers (e.g. public keys, hashes).
- Information about its ownership, its use and any other properties that the signer is capable of attesting/authorizing/witnessing.
- _(Meta-)information_ about this information itself, such as how long it is valid for and external considerations which would invalidate it.

Notes:

- Real application is the hierarchy of SSL certs.
  - Root keys -> State level entities -> Smaller entities.
- Web of Trust & GPG cross-signing

---

## Entropy, Randomness, and Key Size

- Entropy: Amount of non-redundant information contained within some data.
- Randomness: Unpredictability of some information. Less random implies lower entropy.
- Key size: Upper limit of possible entropy contained in a key. Keys with less random (more predictable) data have less entropy than this upper bound.
- One-time pad: A key of effectively infinite size. If it is perfectly random (i.e. has maximal entropy), then the cipher is theoretically unbreakable.

Notes:

Mention the upcoming "many time pad" activity, that exploits using a one time pad multiple times.

---

## Randomness Generation

```rust
fn roll_die() -> u32 {
  // Guaranteed random: it was achieved through a real-life die-roll.
  4u32
}
```

- Pseudo-random sequences
- Physical data collection (e.g. cursor movement, LSB of microphone)
- Specialised hardware (e.g. low-level noise on silicon gates, quantum-amplifiers)

---

## Summary

Cryptography is much more than encryption.

<pba-flex center>

- Communicate on public networks, in the open
- Access information
- Have expectations about a message's authenticity and integrity
- Prove knowledge of some secret information
- Represent large amounts of data succinctly

</pba-flex>

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions

<img style="height: 600px" src="../../../assets/img/1-Cryptography/crypto-mind-map.svg" />

##### _What insights did you gain?_

Notes:

Class discussion.
Last slide.
