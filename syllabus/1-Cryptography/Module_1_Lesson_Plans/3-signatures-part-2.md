# Lesson 3: Digital Signatures (Part 2)

## Core Ideas to Convey

This section will be less hands on, but will cover some more advanced usage of signatures.

## Outline

- Certificates
  - Linking keys
  - When a key might be compromised
  - Revoking a key
- Multi-signatures
  - We often want multi-signatures in which signatures only become valid once multiple parties sign.
  - In these, we typically want that some threshold k-of-n signs, which protects against lost keys, signers maliciously not participating, etc.
    We therefore do not discuss non-threshold multi-signatures.
- Three flavors of threshold multi-signatures
  - Verifier enforced aka on-chain threshold multi-signatures make verifiers both check multiple seperate signatures and run threshold verification logic.
    Although flexible, these increase banwidth costs if verifiers are numerous, but should provide good users experence.
    We leave these to elsewhere in the course.
    - An on-chain multi-signature simply places all signatures on chain separately, and determines their combined effect via on-chain logic.
      We typically use these in Substrate for simplicity, and because parachains pay less for space than other blockchains.
  - Signer enforce aka off-chain cryptographic threshold multi-signatures require two multi-party computations (MPCs):
    - Initially all signers run distributed key generation (DKG) protocol that collaboratively constructs secret key shares whose structure encodes the desired threshold behavior.
      DKGs break hard key derivations.
    - After this, each signing demands some threshold of signers provide signature fragments, which necessarily cover previously agreed upon mesages.
    - Threshold multi-signatures require special keys built with a multi-party computation called a distributed key generation (DKG) protocol/ceremony, but the resulting key encodes the threshold verification logic.
  - Off-chain cryptographic non-threshold multi-signatures (signature aggregation)
    - Cryptographic multi-signatures with non-threshold cryptography.
      We do not discuss the non-threshold flavor because applying them remains subtle and out of scope.
- Schnorr (threshold cryptographic) multi-signatures
  - Improves operational security without altering or complicating verifier logic.
  - Also reduces verifier costs in bandwidth & CPU time, so great for certificates.
  - Automation becomes tricky: We need agreement upon the final signer list, and two random nonce contrbutions from each prospective signer, before constructing the signature fragments.
  - Schnorr supports soft key defivations.
- BLS (coming to a Substrate near you soon)
  - Another elliptic curve based signature
  - Uses heavier pairing friendly elliptic curves than ECDSA/Schnorr.
  - Also verifiers are hundreds of times slower than Schnorr, due to using pairings, so they typically only appear in exotic situations, e.g. light clients.
  - Automates easier: Aggregation without advance agreement upon the signer list.
    Very popular for concensus.
    DKGs remain tricky however.
  - Soft key derivations are typically insecure for BLS.
- Summary: Schnorr & BLS multi-signatures avoid complicating verifier logic, but both bring costs like slightly worse user experence.
- Ring signatures
  - Ring signatures prove the signer lies within some "anonymity set" of signing keys, but hide which key actually signed.
  - Ring signatures come in many sizes, with many ways of presenting their anonymity sets.
  - Anonymous blockchain transactions typically employ ring signatures.
  - ZCash uses a ring signature based upon Groth16 zkSNARKs which makes the entire chain history be the anonymity set.

<!-- Anonymous credentials is a huge field of cryptographic research, but anonymous credentials built with ring signatures tend to be more compatible with blockchains. -->
