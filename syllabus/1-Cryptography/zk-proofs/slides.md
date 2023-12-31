---
title: ZK proofs
description: Introduction to zero-knowledge proofs and zk-SNARKS
duration: 3.5 hour
---

# ZK Proofs

---

# Outline

<pba-flex center>

1. [ZK Proofs overview](#zk-proofs)
2. [Examples](#simple-zk-example)
3. [zk-SNARKs](#zk-snarks)
4. [Applications of zk-SNARKs](#zk-application)
5. [Under the hood of zk-SNARKs](#zk-practice)
6. [Generating ZK-proofs using Circom and snarkjs](#circom-snarkjs)

</pba-flex>

---

## ZK Proofs

How do we do private operations on a public blockchain<br />and have everyone know that they were done correctly?

Notes:

(we are working on substrate support for these and will use them for protocols)

---

## What is a ZK Proof?

- A prover wants to convince a verifier that something is true without revealing why it is true.

- They are mostly interactive protocols, but mostly we'll turn them non-interactive and deal with their non-interactive variety. 

Notes:
 - Interactive means multiple back and forthes between parties. It requires that the parties to be online at the same time.
 - Non-interactive means one party post their contribution and the other party could use the data at the time of their convinience. 
   This does not require the praties to be online at the same time.
---

## What can we show?

- NP relation: `function(statement, witness) -> bool`

- Prover knows a witness for a statement:

  - They want to show that they know it (_a proof of knowledge_)

  - ... Without revealing anything about the witness (_ZK_)

Notes:
    NP problems are problems that might be hard to solve but are relatively  easy to verify. Easy as in "polynomial time".

---

## ZK Proof Interface

- NP relation: `function(statement, witness) -> bool`

- `prove(statement, witness) -> proof`

- `verify(statement, proof) -> bool`

---

## Examples
---

## ZK Proof Example

_Example:_ Schnorr signatures are ZK Proofs
- $(s, e)$ such that $s = (\textrm{Random blinding factor }r) - \\ \textrm{Private Key} \times e$
- They show that the prover knows the private key such that $\textrm{private key} \times G = \textrm{Public Key}$ without revealing anything about it.
- The statement is the public key and the witness the private key.

---

## The ZK Proof Example we study in this module

_Example:_ Prover knows a non-trivial factorization of N.

- Prover wants to prove that they know $N = r \times s$  without revealing $r$ or $s$.

- The Prover should convince us they know the two non-trivial integer $r$ and $s$ that is $r,s \neq 1$ such that:

 $r\times s = N$ 

- Without revealing any other information about $r$ or $s$.

- The statement is $N$ and the witnesses are $r$ and $s$.


---
## ZK Proof properties.
 - Completeness: If the claim is true, then it *must* pass `verify(statement, proof) == true`
 - Statistical Knowledge Soundness: If the prover does not know the witness then `verify(statement, proof) == false` with high probability.
 - Zero Knowledge: the proof reveals nothing about the witness that was not revealed by the statement itself.
 - The common way of implementing zero knowledge protocol is by means of zk-SNARK.

Notes:
 
 - We prove Zero knowldege by showing a party without any knowledge of the witness, could fool an external party who was not been part of the interaction that they interacted with the prover and got a valid proof form the prover. 
---

## zk-SNARKs

---

## zk-SNARK

**Z**ero-**K**nowledge **S**uccinct **N**on-interactive **Ar**gument of **K**nowledge

- **Zero knowledge** - the proof reveals nothing about the witness that was not revealed by the statement itself.
- **Succinct** - the proof is small
- **Non-interactive** - Does not require live interaction between the prover and verifier.
- **Proof of knowledge** - if you can compute correct proofs of a statement, you should be able to compute a witness for it.

---

## What can we show?

- NP relation: `function(statement, witness) -> bool`

  - They want to show that they know it (_a proof of knowledge_)

  - ... Without revealing anything about the witness (_ZK_)

- With a small proof even if the witness is large (_succinctness_)

---

## What can we show?

- There are many SNARK schemes to produce succinct ZK proofs of knowledge (_ZK-SNARKs_) for every NP relation. We concentrate on PLONK in this course.

---

## Applications of zk-SNARKs

---

## ZK Proof Scaling

- A small amount of data, a ZK proof, and execution time can be used to show properties of a much larger dataset which the verifier doesn't need to know. 

- "doesn't want to" know as opposed to "is not supposed to" know.

---

## Scaling via ZK Proofs in Blockchain

- Large amount of data - a blockchain
- Verifier is e.g. an app on a mobile phone

Notes:

e.g. Mina do a blockchain with a constant size proof (of correctness of execution and consensus) using recursive SNARKs.

---

## Scaling via ZK Proofs in Blockchain

- The verifier is a blockchain: very expensive data and computation costs.

- Layer 2s using ZK rollups

Notes:

Of which Ethereum has many, ZKsync, ZKEVM etc.
Polkadot already scales better!

---

## Privacy

<pba-flex center>

A user has private data, but we can show<br />publicly that this private data is correctly used.<br />
An example would a private cryptocurrency:

- Keep who pays who secret
- Keep amounts secret, <br /> _But show they are positive!_

</pba-flex>

Notes:

You can do some of keeping amounts secret without ZK-SNARKs, but the positive part is difficult.
To do everything well, ZK-SNARKs are needed in e.g. ZCash and its many derivatives e.g. Manta.

---

## Under the hood of zk-SNARKs

Notes:

 - We are going to talk about the elementary math behind zk-SNARKs.
 - The goal is to familiarize you with the magic behind the zk-SNARKs.
 - Don't worry if you are not able to follow. Not essential.
---

## Making a SNARK out of the factorization problem
- The trick  is to transform our problem of proving the knowledge of factors (witnesses) into:
- A problem of knowledge of certain polynomials.
- Then verifier could ask me questions about those polynomials, and if the prover answers correctly,
- The verifier could be fairly confident that the prover knows that polynomial hence also the witness.

---

## Making an SNARK for knowledge of factors problem
- A routine way of to turning the problem into a polynomial is:
1. To represents our problem into an arithmetic circuit.
2. Then there are algorithms such as PLONK for representing the circuit as few univariate polynomials.

Notes: 
 - Mathematically circuit is a n-variate polynomials, with some of the variables are public and some are not.
---


## The conditions the prover's solution must satisfy
- $r \times s = N$
- We also need to make sure that prover doesn't fool us with trivial factors.
- To prevent them from using $r = 1$ or $s = 1$ we ask them to invert $r - 1$ and $s-1$:
  - $(r-1)*\frac{1}{r - 1} = 1$
  - $(s-1)*\frac{1}{s - 1} = 1$

---

## Overflow prevention
- Our polynomials are all defined modulo some prime $p$
- We need to prevent the prover from fooling us with a factorization like 

    $r \times s = N + q\times p$ where $q \neq 0$.

- This happens if $r$ and $s$ are too big and $r \times s$ overflows over $p$
- We should make sure that $r$ and $s$ are small.
- We use binary decomposition for that.
- $r = r_{0} + 2r_{1} + 4r_{2}$ where 
- We should only allow them to use 0 or 1 for $r_{i}$'s so they need to satisfy:
- $r_{i} \times (r_{i} - 1) = 0$

---

## Factorization Circuit

  $r \times s = N$

<img style="height: 500px; padding-left:100px" src="./img/factorization-circuit.png" />

---

## Prevent Factor 1 Circuit
 $(r-1)(\frac{1}{r - 1}) = 1$
 
<img style="height: 500px; padding-left:100px" src="./img/inhibit-1-circuit.png" />

---

## Prevent Big Factors: Binary decompostion circuit
$r = r_{0} + 2r_{1} + 4r_{2} \Rightarrow$

$r_{01} = r_{0} + 2r_{1}$

$r = r_{01} + 4r_{2}$

<img style="height: 500px; padding-left:100px" src="./img/binary-decomposition-circuit.png" />

---

## Prevent Big Factors: only allow 0 or 1 in binary decomposition
$r_{i} \times (r_{i} - 1) = 0$

<img style="height: 500px; padding-left:100px" src="./img/enforce-0-or-1-circuit.png" />

---

## Writing our circuit in Circom
 Circom demo.

---

## Circuit to SNARK Strategy
- To represent the circuit as a univariate polynomial called the "Trace Polynomial".
- The trace polynomial has a root for each gate of the circuit if the solution satisfies the gate relation.
- Then the verifier should be able to test if the polynomial actually has a root for every gate ...
- ... without knowing the polynomial: This is done using *polynomial commitment*.

---

## Universal PLONK Gate

- Supppose we have a left input $a$ and a right input $b$ and we are doing some addition and multiplication with them and the output is $c$. Then we could encode all of these operations as:

$Q_l\times a + Q_r \times b + Q_o \times c + Q_m \times a\times b + Q_c = 0$

- for some constant $Q_l$ $Q_r$ $Q_o$ $Q_m$ and $Q_c$
- in fact all the operation we discussed can be written using one of these gates.

---

## Gate table for factorization
 $r \times s = N$

 $Q_l\times a + Q_r\times b + Q_o\times c + Q_m\times a\times b + Q_c = 0$

 
 <img style="height: 200px; padding-left:100px" src="./img/gate-table-factorization.png" />

---

## Gate table for left input to be small and not 1
  
 $r_{01} = r_{0} + 2r_{1}$
 
 $r = r_{01} + 4r_{2}$ 

 $r_{i} \times (r_{i} - 1) = 0 \Rightarrow r_{i}^2 - r_{i} = 0$
 
 $(r-1)*\frac{1}{r - 1} = 1 \Rightarrow r\frac{1}{r - 1} - \frac{1}{r - 1} = 1$
 
 $Q_l\times a + Q_r\times b + Q_o\times c + Q_m\times a\times b + Q_c = 0$ 

<img style="height: 500px; padding-left:100px" src="./img/gate-table-left-input-less-than-8-and-not-1.png" />

---

## Gate table for the right input to be an integer and not 1
 $Q_l\times a + Q_r\times b + Q_o\times c + Q_m\times a\times b + Q_c = 0$
<img style="height: 500px; padding-left:100px" src="./img/gate-table-right-input-less-than-8-and-not-1.png" />

---

## Encode the trace as a polynomial T
- You can always encode a column of a table into a polynomial.
- $Q_l(x)$ such that $Q_l(1) = 0, Q_l(2) = 1, Q_l(3) = 1, Q_l(4) = -1 ,...$
- When you have one polynomial for each column then you can turn the whole table into a polynomial:

$Q_l(x)\times a(x) + Q_r(x)\times b(x) + Q_o(x)\times  c(x) + Q_m(x)\times a(x)\times b(x) + Q_c(x)$

$= 0$

---

## Compute the trace polynomial from the gate table
 SAGE demo

---

## Prove that Validity of T
- T encode every gate is evaluated correctly: Zero test.
- The wiring is correct: Permutation test (we are not discussing it in this course). 

---

# Zero test
- if f(x) = 0 for x = 1,..,13 then
- $f(x) = q(x) \times  (x-1)\times ...\times (x-13)$
- $f(x)/q(x) = (x-1)...(x-13)$
- How to verifier this.

---

## Zero test on the resulting polynomial.
 SAGE demo

---

# Zero test without knowing the polynomial: Polynomial commitment
- Polynomial commitment is a tool that let the prover announce the value of a polynomial $f(x)$ at some point $u$.
- convince the verifier which it has done so honestly.
- The prover first commit to the polynomial $f(x)$ so later on, they can't back off and cheat (and use another polynomial).
- Then the verifier is going to ask the prover to evaluate the polynomials in random point $u$.
- The verifier is able to be confident that $f(u) = v$.

---

# Zero test using polynomial commitment.
- The prover claims it has $f(x)$ satisfying the circuit.
- The prover is also able to compute $q(x)$ such that 
- $f(x) = q(x) \times  \prod(x-1)..(x-13)$
- The prover commit to $f$ and $q$.
- The verifier ask the prover to provide them with $f(u)$ and $q(u)$ for some random point $u$
- The verifier computes $\prod(u-1)...(u-13)$
- The verifier verifies that $f(u) = q(u)\times \prod(u-1)...(u-13)$ and if so believes that the prover has a solution.

---

# KZG Polynomial-commitment
- Is the most space efficient polynomial commitment.
- Uses elliptic curve cryptography.
- It requires trusted setup: a pre-computation with toxic waste which needs to be discarded to keep the scheme secure.

---

## Making ZK non-interactive
- The only interactive step is when verifier is quizzing prover with a random value $r$.
- We replace that with asking the prover to apply a secure hash function to his commitment to generate $r$.
- That way if the prover changes his commitment his point also changes without his control. 

---

## Use Circom to generate trace polynomials.
 Circom demo

---

## Use snarkjs to generate proofs
 Generate proof demo with snarkjs

---

## Use snarkjs to verify the proofs
Verify the proof snarkjs

---

## Practical Considerations

- Very powerful primitive

- Useful for both scaling and privacy

- One can design many protocols with ZK Proofs that wouldn't otherwise be possible

---

## Downside

- Slow prover time for general computation
- To be fast, need to hand optimize
- Very weird computation model:<br />
  Non-deterministic arithmetic circuits

 Notes: Weird as in binary arithmetic is hard, condition is hard. 
 taking square-root of an element mod p is easy
---

## Summary
- We discussed the general idea of ZK-Proofs.
- Their application for scalability and privacy.
- We looked how a sample ZK Proof is generated.
- We generate and verify the proof using modern ZK tools.
---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
