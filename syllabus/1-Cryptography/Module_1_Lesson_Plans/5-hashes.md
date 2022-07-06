# Lesson 5: Hashes

## Core Ideas to Convey

1. Hashes in system usage.
2. What makes a good hash function?
   - Collision probability (estimate of how easy it is to crack)
   - crypto vs. not
   - When to use what impl of a hash and why
   - Birthday paradox (128 vs. 256)

## Outline

- Cryptographic vs. Non-Cryptographic Functions
	- Tradeoffs, when to use each one
- Speed
	- Some hash functions are designed to be slow
	- For our purposes, we generally want them to be fast
- Account creation
	- The property that hashes can be restricted to a fixed output size (e.g. 32 bytes) makes them useful for creating other things, like Account IDs
	- Multisig account generation, module account generation, etc.
- Motivation for choosing Blake2 in Substrate
- Hash linked structures
	- comparison to pointer based data structures
- Hash chains 
- Merkle trees
- proofs
- security via collision resistance explanation
- Use in blockchains
	- I think we definitely want a diagram of block headers in a hash chain containing the roots of Merkle trees here.
