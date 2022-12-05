---
title: Exotic Primitives
description: More cutting edge cryptography used in blockchain
duration: 1 hour
instructors: ["Alistair Stewart"]
teaching-assistants: ["Dan Shields, Sacha Lansky"]
---

# Exotic Primitives

---

# Outline

1. [Verifiable Random Functions (VRFs)](#verifiable-random-functions-vrfs)
1. [Erasure coding](#erasure-coding)
1. [ZK Proofs](#zk-proofs)

---

# Verifiable Random Functions<br>(VRFs)

---

## Verifiable Random Functions<br>(VRFs)

<widget-center>

Deterministic commitment to an exponentially large pseudorandom sequence that seems random.

- Used to obtain <ins>private pseudorandomness</ins>, that is <ins>publicly verifiable</ins>

- A variation on a signature scheme:
  - still have private/public key pairs, input as message
  - in addition to signature, we get an output

---

## VRF Interface

- `sign(sk, input) -> signature`

- `verify(pk, signature) -> option output`

- `eval(sk,input) -> output`

---

## VRF Output properties

- Output is a deterministic function of _key_ and _input_
  - i.e. eval should be deterministic
- It should be pseudo-random
- But until the VRF is revealed, only the holder of the secret key knows the output

---

## VRF Usage

- Choose input after key, then the key holder cannot influence the output
- The output then is effectively a random number known only to the key holder
- But they can later reveal it, by publishing the VRF proof (signature)

Notes:

The signature proves that this is the output associated to their input and public key.

---

## VRF Example

- Playing a card game in a distributed and trustless way
- For player A to draw a card, the players agree on a new random number x
- A's card is determined by
  `eval(sk_A,x) mod 52`
- To play the card, A publishes the signature

---

## VRF Extensions

- Threshold VRFs / Common coin

  - generate the same random number if $t$ out of $n$ people participate

- RingVRFs
  - the VRF output could be from any one of a group of public keys.

Notes:

Common coins were used in consensus before blockchains were a thing.
Dfinity based their consensus on this.
But this needs a DKG, and it's unclear if a decentralized protocol can do those easily.

---

# Erasure Coding

---

## Erasure Coding

- Turn data into pieces (with some redundancy) so it can be reconstructed even if some pieces are missing.

- A message of $k$ symbols is turned into a coded message of $n$ symbols and can be recovered from any $k$ of these $n$ symbols

---

## Erasure Coding Classical use

Used for noisy channels:
- If a few bits of the coded data are randomly flipped, we can still recover the original data
- Typically $n$ is not much bigger than $k$

---

## Use in Decentralized Systems

- We have data we want to keep publicly available

  - but not have everyone store
  - but we don't trust everyone who is storing pieces

- Typically we use $n$ much bigger than $k$

---

# 

How do we do private operations on a public network and have everyone know that they were done correctly?

---

## Zero-Knowledge Proofs

---

## What is a ZK Proof?

- A prover wants to convince a verifier that something is true without revealing why it is true.

- They can be interactive protocols, but mostly we'll be dealing with the non-interactive variety.

---

## What can we show?

- NP relation: `function(statement, witness) -> bool`

- Prover knows a witness for a statement:

  - they want to show that they know it (_a proof of knowledge_)

  - ... without revealing anything about the witness (_ZK_)

---

## ZK Proof Interface

- NP relation: `function(statement, witness) -> bool`

* `prove(statement, witness) -> proof`

* `verify(statement, proof) -> bool`

---

_Example:_ Schnorr signatures are ZK Proofs

- They show that the prover knows the private key (the discrete log of the public key) without revealing anything about it.
- The statement is the public key and the witness the private key.

---

- **Proof of knowledge** - if you can compute correct proofs of a statement, you should be able to compute a witness for it.

- **Zero knowledge** - the proof reveals nothing about the witness that was not revealed by the statement itself.

---

## What can we show?

- NP relation: `function(statement, witness) -> bool`

  - they want to show that they know it (_a proof of knowledge_)

  - ... without revealing anything about the witness (_ZK_)

- with a small proof even if the witness is large (_succinctness_)

---

- There are many schemes to produce succinct ZK proofs of knowledge (_ZK-SNARKs_) for every NP relation.

---

## ZK Proof Scaling

A small amount of data, a ZK proof, and execution time can be used to show properties of a much larger dataset which the verifier doesn't need to know.

---

## Scaling via ZK Proofs in Blockchain

- Large amount of data - a blockchain
- Verifier is e.g. an app on a mobile phone

Notes:

e.g. Mina do a blockchain with a constant size proof (of correctness of execution and consensus) using recursive SNARKs.

---

## Scaling via ZK Proofs in Blockchain

- The verifier is a blockchain, with very expensive data and computation costs

- Layer 2s using ZK rollups

Notes:

Of which Ethereum has many, ZKsync, ZKEVM etc.
Polkadot already scales better!

---

## Privacy

A user has private data, but we can show publicly that this private data is correctly used.

Example: private cryptocurrency

- Keep amounts secret
  - but show they are positive!
- Keep who pays who secret

Notes:

You can do some of keeping amounts secret without ZK-SNARKs, but the positive part is difficult.
To do everything well, ZK-SNARKs are needed in e.g. ZCash and its many derivatives e.g. Manta.

---

## Practical Considerations

- Very powerful primitive

- Useful for both scaling and privacy

- One can design many protocols with ZK Proofs that wouldn't otherwise be possible

---

## Downside

- Slow prover time for general computation
- To be fast, need to hand optimize
- Very weird computation model: Non-deterministic arithmetic circuits

---

## Downsides Conclusion?

- So if you want to use this for a component, expect a team of skilled people to work for at least a year on it...
- But if you are watching this 5 years later, people have built tools to make it less painful.

---

## Summary

- VRF: Private randomness that is later publicly verifiable
- Erasure Coding: Making data robust against losses with redundancy
- ZK Proofs: Just magic, but expensive magic
