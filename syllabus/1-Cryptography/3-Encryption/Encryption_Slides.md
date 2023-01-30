---
title: Encryption
description: A lesson on symmetric and asymmetric encryption.
duration: 1 hour
---

# Encryption

---

## Goals for this lesson

<pba-flex center>

- Learn about the differences between symmetric and asymmetric encryption.

</pba-flex>

---

## Symmetric Cryptography

Symmetric encryption assumes all parties begin with some shared secret information, a potentially very difficult requirement.<br>The shared secret can then be used to protect further communications from others who do not know this secret.

In essence, it gives a way of _extending_ a shared secret over time.

---

## Symmetric Encryption

<img style="width: 1100px" src="../../../assets/img/1-Cryptography/Symmetric-Cryptography.png"/>

Examples: ChaCha20, Twofish, Serpent, Blowfish, XOR, DES, AES

---

## Symmetric Encryption

#### _Example: XOR Cipher_

<pba-cols>
<pba-col>

The encryption and decryption functions are identical: applying a bitwise XOR operation with a key.

</pba-col>
<widget-column style="padding-right: 100px">

```text
Plain: 1010  -->Cipher: 0110
Key:   1100  |          1100
       ----  |          ----
       0110--^          1010
```

Notes:

A plaintext can be converted to ciphertext, and vice versa, by applying a bitwise XOR operation with a key known to both parties.

</pba-col>
</pba-cols>

---

## Symmetric Encryption

#### ⚠ Warning ⚠

We typically expect symmetric encryption to preserve little about the original plaintext.
We caution however that constructing these protocols remains delicate, even given secure primitives, with two classical examples being unsalted passwords and the [ECB penguin](https://tonybox.net/posts/ecb-penguin/).

---

### ECB penguin

<pba-cols>
<pba-col>

<img style="width: 300px" src="../../../assets/img/1-Cryptography/ECG-Penguin.png"/>

_Original image_

</pba-col>
<pba-col>

<img style="width: 300px" src="../../../assets/img/1-Cryptography/ECG-Penguin-Encrypted.png"/>

_Encrypted image_

</pba-col>
</pba-cols>

Notes:

Image sources: https://github.com/robertdavidgraham/ecb-penguin/blob/master/Tux.png and https://github.com/robertdavidgraham/ecb-penguin/blob/master/Tux.ecb.png

---

## Asymmetric Encryption

- Assumes the sender does not know the recipient's secret "key" 🎉😎
- Sender only knows a special identifier of this secret
- Messages encrypted with the special identifier can only be decrypted with knowledge of the secret.
- Knowledge of this identifier does not imply knowledge of the secret, and thus cannot be used to decrypt messages encrypted with it.
- For this reason, the identifier may be shared publicly and is known as the _public key_.

---

## Asymmetric Encryption

<img style="height: 600px" src="../../../assets/img/1-Cryptography/asymmetric-crypto-flow.svg"/>

---

## Why "Asymmetric"?

_Using only the public key_, information can be transformed ("encrypted") such that only those with knowledge of the secret are able to inverse and regain the original information.

i.e. Public key is used to encrypt but a different, _secret_, key must be used to decrypt.

---

## Commutative En-/Decryption

In a commutative structure, a message may be encrypted/decrypted<br>multiple times with potentially multiple keys.

The output does not depend on the order of operations.

---

## Diffie-Hellman Key Exchange

<img style="height: 500px" src="../../../assets/img/1-Cryptography/Diffie-Hellman_Key_Exchange_horizontal.svg"/>

Mixing Paint Visualization

Notes:

Mixing paint example.
Image Source: https://upload.wikimedia.org/wikipedia/commons/4/46/Diffie-Hellman_Key_Exchange.svg

---

## Commutative En-/Decryption

Encrypting a message with key $A$, and then encrypting the ciphertext with key $B$, would result in the same ciphertext had one encrypted with $B$ and then $A$.

\begin{align}
M &=> E_A(E_B(M)) == E_B(E_A(M)) => C \\\\
C &=> D_A(D_B(C)) == D_B(D_A(C)) \ => M
\end{align}

Elliptic curve cryptography is based on _commutative_ algebraic structures.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
