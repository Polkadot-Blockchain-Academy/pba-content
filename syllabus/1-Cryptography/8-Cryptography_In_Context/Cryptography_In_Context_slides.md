---
title: Cryptography In Context
description: Real-world considerations around cryptography
duration: 1 hour
---

# Cryptography in Context

---

# Outline

<pba-flex center>

1. Keeping Secrets Secret
1. Security vs Usability

</pba-flex>

---

## Secrets

What _is_ a secret in cryptography?

Data that nobody else knows, that you can use.

---

## How Secrets Stay Secret

In order for a cryptographic secret to stay secret, the _only_ thing about it that can be revealed is the output of known, secured cryptographic operations.

- A (sufficiently random) secret can be hashed, and the hash revealed.
- A private key can be used to generate a public key, and the public key revealed.
- A private key can be used to generate a signature, and the signature revealed.

---

## How Secrets Get Leaked

1. Inadvertently leaking information about the secret during normal operation.
1. Compromised digital or physical security leading to private key loss.

Notes:

Let's go over each of these in order.

---

## Side Channel Attacks

A side channel attack is when a cryptographic system is attacked, and the attacker has another source of information outputted by the system.

---

## Types of Side Channel Attacks

- Timing
  - Different instructions
  - CPU Branch prediction
  - Memory access
- RF emissions
- Power consumption
- Sound of a computer running

Notes:

There are many crazy forms of side channel attack, but the primary one is timing. Timing is also the only one that gets reliably sent back over a long distance.

Sources for exotic attacks:

- [general survey of EM side channel attacks](https://arxiv.org/abs/1903.07703)
- [sound-based attack](https://www.iacr.org/archive/crypto2014/86160149/86160149.pdf)
- [EM side channel attack from 15m with 500 traces only](https://www.diva-portal.org/smash/record.jsf?pid=diva2%3A1648290&dswid=6646)

---

## An Example

Imagine this is the source code for a password checker:

```rust
fn verify_password(actual: &[u8], entered: &[u8]) -> bool {
 for i in 0..actual.len() {
  if entered.get(i) != actual.get(i) {
   return false;
  }
 }
 true
}
```

What's the problem?

Notes:

Imagine you compile this into a little binary, and you are able to hit it repeatedly. When sending a guess into this, what information do you get back?

A boolean, and the amount of time from sending the password to getting back a response.

The problem is that the amount of time for a response reveals information about the password. An attacker can send in guesses repeatedly, and if it takes a longer amount of time to respond, that means more of the guess is correct.

---

## Example (Cont)

What if we changed the code to look like this?

```rust
fn verify_password(actual: &[u8], entered: &[u8]) -> bool {
 actual == entered
}
```

Is this safe?

Notes:
Now, we don't see any difference in the amount of lines of code or loops, right?

---

## Example (Cont)

What does the source code look like?

```rust[0|1|7-9|15]
// Use memcmp for bytewise equality when the types allow
impl<A, B> SlicePartialEq<B> for [A]
where
    A: BytewiseEq<B>,
{
    fn equal(&self, other: &[B]) -> bool {
        if self.len() != other.len() {
            return false;
        }

        // SAFETY: `self` and `other` are references and are thus guaranteed to be valid.
        // The two slices have been checked to have the same size above.
        unsafe {
            let size = mem::size_of_val(self);
            memcmp(self.as_ptr() as *const u8, other.as_ptr() as *const u8, size) == 0
        }
    }
}
```

Is this safe?

Notes:

Ok, still no. It looks like now the attacker can still figure out if the length of the password based on an early return. But what if we make sure all passwords are 16 bytes long. Now we are just using a single syscall. Is is safe then?

---

## Example (Cont)

Let's check on `memcmp`.

```text
memcmp(3) — Linux manual page

/* snip */

NOTES
       Do not use memcmp() to compare security critical data, such as
       cryptographic secrets, because the required CPU time depends on
       the number of equal bytes.  Instead, a function that performs
       comparisons in constant time is required.  Some operating systems
       provide such a function (e.g., NetBSD's consttime_memequal()),
       but no such function is specified in POSIX.  On Linux, it may be
       necessary to implement such a function oneself.

```

---

## So how could we do it?

This is from the `subtle` crate, which provides constant time equality.

```rust[0|14-15|20-28]
impl<T: ConstantTimeEq> ConstantTimeEq for [T] {
    /// Check whether two slices of `ConstantTimeEq` types are equal.
    ///
    /// # Note
    ///
    /// This function short-circuits if the lengths of the input slices
    /// are different.  Otherwise, it should execute in time independent
    /// of the slice contents.
    /* snip */
    #[inline]
    fn ct_eq(&self, _rhs: &[T]) -> Choice {
        let len = self.len();

        // Short-circuit on the *lengths* of the slices, not their
        // contents.
        if len != _rhs.len() {
            return Choice::from(0);
        }

        // This loop shouldn't be shortcircuitable, since the compiler
        // shouldn't be able to reason about the value of the `u8`
        // unwrapped from the `ct_eq` result.
        let mut x = 1u8;
        for (ai, bi) in self.iter().zip(_rhs.iter()) {
            x &= ai.ct_eq(bi).unwrap_u8();
        }

        x.into()
    }
}
```

Notes:

Now we've seen how hard it can be just to stop a very simple leak of timing information. Let's see what an actual cryptographic library concerns itself with.

---

## Ed25519's Guarantees

This is an excerpt from the [ed25519](https://ed25519.cr.yp.to/) description.

- **Foolproof session keys**. Signatures are generated deterministically; key generation consumes new randomness but new signatures do not. This is not only a speed feature but also a security feature.
- **Collision resilience**. Hash-function collisions do not break this system. This adds a layer of defense against the possibility of weakness in the selected hash function.

---

## Ed25519's Guarantees (Cont.)

- **No secret array indices**. The software never reads or writes data from secret addresses in RAM; the pattern of addresses is completely predictable. The software is therefore immune to cache-timing attacks, hyperthreading attacks, and other side-channel attacks that rely on leakage of addresses through the CPU cache.
- **No secret branch conditions**. The software never performs conditional branches based on secret data; the pattern of jumps is completely predictable. The software is therefore immune to side-channel attacks that rely on leakage of information through the branch-prediction unit.

---

## Takeway

Preventing side channel attacks is _hard_! Noticing sidechannel attacks is even harder!

### DO NOT ROLL YOUR OWN CRYPTO

---

## Using Cryptographic Libraries Safely

- Stay _above_ the abstraction barrier
- Validate each primitive's assumptions when combining primitives
- Use the most reputable library you can
- Realize when things need serious consideration
  - Some potentially scary terms: Curve point, padding schemes, IV, twisted curve, pairings, ElGamal

<!-- Maybe add something about this as an example:
https://github.com/MystenLabs/ed25519-unsafe-libs. To put it in few words, the interface for signing
data on some ed25519 libs was secret.sign(message, my_pubkey) instead of just secret.sign(message),
and because of that if you let an attacker control the my_pubkey input, you could result in unsafe
cryptography. -->

---

## Physical Security

Physical access to a running computer can usually let an attacker have full access to your secrets with enough effort.

Some possible means:

- Scanning all disk storage
- Take out the RAM and swap it into a different computer to read (cold boot attack)

---

## HSMs

An HSM is a **h**ardware **s**ecurity **m**odule. HSMs can make it much harder to impossible to steal cryptographic keys. An HSM will hold cryptographic keys, and perform operations on them.

Notes:

We don't go into this much, as there are many available resources around physical security and HSMs. This is just bringing up the ideas, in the context of what makes a cryptographic secret actually _secret_.

---

## Security vs Usability

The general usability of a system is typically inversely prportional to the security.

---

## Thought Experiment

Suppose I give you a secret that's too long to memorize.

At the end of a year, if nobody else knows the secret, I'll give you a million dollars.

What do you do?

---

## Thought Experiment

Suppose I give you a secret that's too long to memorize.

At the end of a year, if nobody else knows the secret, I'll give you a million dollars.

What do you do?

### Destroy it

---

## Thought Experiment

Suppose I give you a secret that's too long to memorize.

At the end of a year, if nobody else knows the secret **and you present me the secret**, I'll give you a million dollars.

What do you do?

---

## Thought Experiment

Suppose I give you a secret that's too long to memorize.

At the end of a year, if nobody else knows the secret **and you present me the secret**, I'll give you a million dollars.

What do you do?

### Hide it somewhere secure

Notes: Like a bank vault, box buried in the woods, etc

---

## Thought Experiment

Suppose I give you a secret that's too long to memorize.

At the end of a year, if nobody else knows the secret **and you present me the secret once per month**, I'll give you a million dollars.

What do you do?

---

## Thought Experiment

Suppose I give you a secret that's too long to memorize.

At the end of a year, if nobody else knows the secret **and you present me the secret every day**, I'll give you a million dollars.

What do you do?

---

## Application to Cryptographic Secrets

Cryptographic secrets are easy to have multiple of.

So don't make users use the same one for everything!

As much as possible, one root secret shouldn't be _both_ used regularly, and extremely valuable.
