---
title: ZK proofs (Deep dive)
description: Introduction to zero-knowledge proofs and zk-SNARKS
duration: 2 hour
---

# Advanced Lecture: ZK Proofs

---

# Outline

<pba-flex center>

1. [ZK Proofs overview](#zk-proofs)<!-- .element: class="fragment" data-fragment-index="0" -->
2. [Examples](#simple-zk-example)<!-- .element: class="fragment" data-fragment-index="1" -->
3. [zk-SNARKs](#zk-snarks)<!-- .element: class="fragment" data-fragment-index="2" -->
4. [Applications of zk-SNARKs](#zk-application)<!-- .element: class="fragment" data-fragment-index="3" -->
5. [Advanced ZK Proofs](#advanced-zk)<!-- .element: class="fragment" data-fragment-index="4" -->

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
NP problems are problems that might be hard to solve but are relatively easy to verify. Easy as in "polynomial time".

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

- Prover sends $R=r \times G$ for random $r$
- Verifier sends random $c$ 
- Prover replies with $s = (\textrm{Random blinding factor }r) -  \textrm{Private Key} \times c$
- Verifier checks that $s \times G =R - c \times \textrm{Public Key}$ <!-- .element: class="fragment" data-fragment-index="0" -->
- Non-interactive signature $(R,s)$ with $c=\textrm{Hash}(\textrm{Public Key} || R || \textrm{message})$ <!-- .element: class="fragment" data-fragment-index="1" -->
- They show that the prover knows the private key such that $\textrm{private key} \times G = \textrm{Public Key}$ without revealing anything about it.<!-- .element: class="fragment" data-fragment-index="2" -->
- The statement is the public key and the witness the private key. <!-- .element: class="fragment" data-fragment-index="2" -->

---

## The ZK Proof Example we study in this module

_Example:_ Prover knows a non-trivial factorization of N

- Prover wants to prove that they know $N = r \times s$ without revealing $r$ or $s$.<!-- .element: class="fragment" data-fragment-index="0" -->
- The Prover should convince us they know the two non-trivial integer $r$ and $s$ that is $r,s \neq 1$ such that:<!-- .element: class="fragment" data-fragment-index="1" -->

$r\times s = N$ <!-- .element: class="fragment" data-fragment-index="1" -->

- Without revealing any other information about $r$ or $s$.<!-- .element: class="fragment" data-fragment-index="2" -->
- The statement is $N$ and the witnesses are $r$ and $s$.<!-- .element: class="fragment" data-fragment-index="3" -->

---

## ZK Proof properties

- Completeness: If the claim is true, then it _must_ pass `verify(statement, proof) == true`
- Knowledge Soundness: If the prover does not know the witness then `verify(statement, proof) == false` with high probability
- Zero Knowledge: the proof reveals nothing about the witness that was not revealed by the statement itself<!-- .element: class="fragment" data-fragment-index="0" -->
- The common way of implementing zero knowledge protocol is by means of zk-SNARK<!-- .element: class="fragment" data-fragment-index="1" -->

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

## How do (polynomial-based) SNARKs prove of knowledge

- Translate our problem into an arithmetic circuit which should output zero if we know the solution.<!-- .element: class="fragment" data-fragment-index="0" -->
- Make a polynomials which have roots in specific values using the solutions.<!-- .element: class="fragment" data-fragment-index="1" -->
- Prove that we know the polynomial by evaluating it at random values and provig that we have evaluated it correctly.<!-- .element: class="fragment" data-fragment-index="2" -->
- Most popular SNARKs uses polynomials, other SNARKs could use other mathematical structures such as vectors, etc<!-- .element: class="fragment" data-fragment-index="3" -->

- Use polynomial commitments instead of sending polynomials<!-- .element: class="fragment" data-fragment-index="4" -->

Notes:

- We can not send the polynomial as a proof: 1. it is too big (so not succinct) 2. reveals the secret solution. Instead we use polynomial commitment schemes.

---

# SNARK = PIOP + commitment + Fiat-Shamir
- PIOP (Polynomial interactive oracle proof)
- PIOP => ARK
- PIOP + Poly Commitment => SARK
- PIOP + Poly Commitment + Fiat-Shamir => SNARK

---

# SARK → SNARK

- Everytime the prover need the verifier to provide them with a random value... <!-- .element: class="fragment" data-fragment-index="1" -->
- The prover apply a hash function to all it has already provided to the verifier. <!-- .element: class="fragment" data-fragment-index="2" -->
- This way the prover is unable to cheat and control the value and break the systemc. <!-- .element: class="fragment" data-fragment-index="3" -->
- This is know Fiat-Shamir Transform <!-- .element: class="fragment" data-fragment-index="4" -->

---

## Applications of zk-SNARKs

---

## ZK Proof Scaling

- A small amount of data, a ZK proof, and execution time can be used to show properties of a much larger dataset which the verifier doesn't need to know.
- "doesn't need/want to" know as opposed to "is not supposed to" know.<!-- .element: class="fragment" data-fragment-index="1" -->

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

## Practical Considerations

- Very powerful primitive<!-- .element: class="fragment" data-fragment-index="0" -->
- Useful for both scaling and privacy<!-- .element: class="fragment" data-fragment-index="1" -->
- One can design many protocols with ZK Proofs that wouldn't otherwise be possible<!-- .element: class="fragment" data-fragment-index="2" -->

---

## Downside

- Slow prover time for general computation<!-- .element: class="fragment" data-fragment-index="1" -->
- To be fast, need to hand optimize<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:
Very weird computation model?
Weird as in binary arithmetic is hard, condition is hard.
taking square-root of an element mod p is easy.
Non-deterministic arithmetic circuits

---

## Under the hood of zk-SNARKs

- We are going to talk about the elementary math behind zk-SNARKs.<!-- .element: class="fragment" data-fragment-index="0" -->
- The goal is to familiarize you with the magic behind the zk-SNARKs.<!-- .element: class="fragment" data-fragment-index="1" -->
- But we are not aming at making you zk-Proof expert in one lecture.<!-- .element: class="fragment" data-fragment-index="2" -->

---

## Recall steps to SNARKify your problem

- Turn the problem into an arithmetic circuit (describe it only with +, -, x). <!-- .element: class="fragment" data-fragment-index="0" -->
- Make polynomials from the circuit and your secret solution.<!-- .element: class="fragment" data-fragment-index="1" -->
- But we are not aming at making you zk-Proof expert in one lecture.<!-- .element: class="fragment" data-fragment-index="2" -->

---

## Making a SNARK out of the factorization problem

- The trick is to transform our problem of proving the knowledge of factors (witnesses) into:<!-- .element: class="fragment" data-fragment-index="0" -->
- A problem of knowledge of certain polynomials.<!-- .element: class="fragment" data-fragment-index="1" -->
- $x^n + a_{n-1} x^{n-1} + \dots + a_1x + a_0$ <!-- .element: class="fragment" data-fragment-index="2" -->
- Then verifier could ask me questions about those polynomials, and if the prover answers correctly,<!-- .element: class="fragment" data-fragment-index="3" -->
- The verifier could be fairly confident that the prover knows that polynomial hence also the witness.<!-- .element: class="fragment" data-fragment-index="4" -->

---

## Making an SNARK for knowledge of factors problem

- A routine way of to turning the problem into a polynomial is:

1. To represents our problem into an arithmetic circuit.<!-- .element: class="fragment" data-fragment-index="1" -->
2. Then there are algorithms such as PLONK for representing the circuit as few univariate polynomials.<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

- Mathematically circuit is a n-variate polynomials, with some of the variables are public and some are not.

---

## The conditions the prover's solution must satisfy

- $r \times s = N$<!-- .element: class="fragment" data-fragment-index="0" -->
- We also need to make sure that prover doesn't fool us with trivial factors.<!-- .element: class="fragment" data-fragment-index="1" -->
- prevent them from using $r = 1$ or $s = 1$ only using polynomial equations.<!-- .element: class="fragment" data-fragment-index="2" -->
- $r \neq 1$ is not a polynomial equation.<!-- .element: class="fragment" data-fragment-index="3" -->
- So we ask them to invert $r - 1$ and $s-1$:<!-- .element: class="fragment" data-fragment-index="4" -->
  - $(r-1)\times \frac{1}{r - 1} = 1$<!-- .element: class="fragment" data-fragment-index="5" -->
  - $(s-1)\times \frac{1}{s - 1} = 1$<!-- .element: class="fragment" data-fragment-index="6" -->

---

## Overflow prevention

- Our polynomials are all defined modulo some prime $p$<!-- .element: class="fragment" data-fragment-index="0" -->
- We need to prevent the prover from fooling us with a factorization like<!-- .element: class="fragment" data-fragment-index="1" -->
  $r \times s = N' = N + q\times p$ where $q \neq 0$.<!-- .element: class="fragment" data-fragment-index="2" -->
- This happens if $r$ and $s$ are too big and $r \times s$ overflows over $p$<!-- .element: class="fragment" data-fragment-index="3" -->
- We should make sure that $r < \sqrt{p}$ and $s < \sqrt{p}$ are small.<!-- .element: class="fragment" data-fragment-index="4" -->
- We use binary decomposition instead.<!-- .element: class="fragment" data-fragment-index="5" -->
- $r = r_{0} + 2r_{1} + 4r_{2}$ where<!-- .element: class="fragment" data-fragment-index="6" -->
- We should only allow them to use 0 or 1 for $r_{i}$'s so they need to satisfy:<!-- .element: class="fragment" data-fragment-index="7" -->
- $r_{i} \times (r_{i} - 1) = 0$<!-- .element: class="fragment" data-fragment-index="8" -->

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

<!--## Writing our circuit in Circom

Circom demo.

--- -->

# Advanced ZK Proofs

---

# Outline

<pba-flex center>

1. [Under the hood of zk-SNARKs](#zk-practice) <!-- .element: class="fragment" data-fragment-index="1" -->
2. [Introduction to Plonk](#plonk) <!-- .element: class="fragment" data-fragment-index="2" -->
3. [Polynomial commitment](#polynomial-commitment) <!-- .element: class="fragment" data-fragment-index="3" -->

</pba-flex>

---

## Circuit to SNARK Strategy

- To represent the circuit as a univariate polynomial called the "Trace Polynomial".<!-- .element: class="fragment" data-fragment-index="1" -->
- The trace polynomial is equal to zero at each "gate" of the circuit if the solution satisfies the gate relation.<!-- .element: class="fragment" data-fragment-index="2" -->
- Then the verifier should be able to test if the polynomial actually has a root for every gate.<!-- .element: class="fragment" data-fragment-index="3" -->
- ... without knowing the polynomial: This is done using "polynomial commitment".<!-- .element: class="fragment" data-fragment-index="4" -->

---

## Universal PLONK Gate

<img style="height: 200px; padding-left:100px" src="./img/factorization-circuit.png" />

- Supppose we have a left input $a$ and a right input $b$ and we are doing some addition and multiplication with them and the output is $c$.<!-- .element: class="fragment" data-fragment-index="1" -->
- Then we could encode all of these operations as:<!-- .element: class="fragment" data-fragment-index="2" -->
  $Q_l\times a + Q_r \times b + Q_o \times c + Q_m \times a\times b + Q_c = 0$<!-- .element: class="fragment" data-fragment-index="3" -->
- for some constant $Q_l$ $Q_r$ $Q_o$ $Q_m$ and $Q_c$<!-- .element: class="fragment" data-fragment-index="4" -->
- in fact all the operation we discussed can be written using one of these gates.<!-- .element: class="fragment" data-fragment-index="5" -->

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

$(r-1)\times\frac{1}{r - 1} = 1 \Rightarrow r\frac{1}{r - 1} - \frac{1}{r - 1} = 1$

$Q_l\times a + Q_r\times b + Q_o\times c + Q_m\times a\times b + Q_c = 0$

<img style="height: 500px; padding-left:100px" src="./img/gate-table-left-input-less-than-8-and-not-1.png" />

---

## Gate table for the right input to be an integer and not 1

$Q_l\times a + Q_r\times b + Q_o\times c + Q_m\times a\times b + Q_c = 0$
<img style="height: 500px; padding-left:100px" src="./img/gate-table-right-input-less-than-8-and-not-1.png" />

---

## Encode the trace as a polynomial T

- You can always encode a column of a table into a polynomial.<!-- .element: class="fragment" data-fragment-index="1" -->
- $Q_l(x)$ such that $Q_l(1) = 0, Q_l(2) = 1, Q_l(3) = 1, Q_l(4) = -1 ,...$<!-- .element: class="fragment" data-fragment-index="2" -->
  <img style="height: 300px; padding-left:100px" src="./img/gate-table-left-input-less-than-8-and-not-1.png" /><!-- .element: class="fragment" data-fragment-index="2" -->
- When you have one polynomial for each column then you can turn the whole table into a polynomial:<!-- .element: class="fragment" data-fragment-index="3" -->
  $Q_l(x)\times a(x) + Q_r(x)\times b(x) + Q_o(x)\times  c(x) + Q_m(x)\times a(x)\times b(x) + Q_c(x)$<!-- .element: class="fragment" data-fragment-index="4" -->
  $= 0$<!-- .element: class="fragment" data-fragment-index="4" -->

---

<!--
## Compute the trace polynomial from the gate table

SAGE demo

--- -->

## Prove that Validity of T

- T encode every gate is evaluated correctly: Zero test.
- The wiring is correct: Permutation test.

---

# Zero test

- if f(x) = 0 for x = 1,..,13 then
- $f(x) = q(x) \times  (x-1)\times ...\times (x-13)$
- $f(x)/q(x) = (x-1)...(x-13)$
- How to verify this.

---

<!--

# Zero test on the resulting polynomial.

SAGE demo

--- -->

# Proving the correctness of the wiring

- So far we have proven that the we have a solution which satisfies each round of PLONK gate.<!-- .element: class="fragment" data-fragment-index="1" -->
- However we have not proven that we are using the outputs from previous rounds in correct places in each round.<!-- .element: class="fragment" data-fragment-index="2" -->
- So the prover could cheat and use the values.<!-- .element: class="fragment" data-fragment-index="3" -->
- The verifier need enforce equality of reused values in different rounds.<!-- .element: class="fragment" data-fragment-index="4" -->
- This is actually the hard creative bit in PLONK and that is what P stands for in PLONK.<!-- .element: class="fragment" data-fragment-index="5" -->

---

<!--
# The wiring in a glance

- TODO: wired table

--- 

# The Trace polynomial

---

# The wiring permutation we want to prove

-- Define the permutation.

---

# Naive Permutation check wtih zero test

--- -->

# Developing a wiring enforcement gadget/polynomial

- We take a step back and develop some tool to tackle this. <!-- .element: class="fragment" data-fragment-index="1" -->
- They sound random and irrelevant at first but it all make sense at the end. <!-- .element: class="fragment" data-fragment-index="2" -->

---

# Product check

- We have a polynomial $f(x)$ and we want to prove that:<!-- .element: class="fragment" data-fragment-index="1" -->
- $\prod_{i \in \{1..39\}}f(i) = 1$.<!-- .element: class="fragment" data-fragment-index="2" -->
- We could perform a a zero test $\prod_{i \in \{1..39\}}f(i)$ but the degree is huge. <!-- .element: class="fragment" data-fragment-index="3" -->
- Instead we introduce a new polynomial: <!-- .element: class="fragment" data-fragment-index="4" -->
- $t(x) = \prod_{i \in \{1..x+1}}f(i)$. <!-- .element: class="fragment" data-fragment-index="5" -->
- We have a nice recursion: $t(x + 1) = t(x)f(x+1)$ for $x \in \{1..39}$ <!-- .element: class="fragment" data-fragment-index="6" -->

---

# Product check

- The observeration is that if you have the recursion:
  $t(x + 1) = t(x)f(x+1)$ for $x \in \{1..39}$
- And you know $ t(39) = 1 $ then you know that:  <!-- .element: class="fragment" data-fragment-index="1" -->
- $\prod\_{i \in \{1..39}}f(i)$.  <!-- .element: class="fragment" data-fragment-index="1" -->
- We intepolate $t$ and it will have order 38 (vs 38 \* 13)$ <!-- .element: class="fragment" data-fragment-index="2" -->
- We run a zero test on $t(x + 1) - t(x)f(x+1) = 0$ for $\{1,...,39\}$ <!-- .element: class="fragment" data-fragment-index="3" -->

---

# Ratio check

- We can run the product check to prove $\prod_{i \in \{1..39\}}f(i)/g(i) = 1$.<!-- .element: class="fragment" data-fragment-index="2" -->
- $t(x + 1) = t(x)f(x+1)/g(x + 1)$ <!-- .element: class="fragment" data-fragment-index="2" -->
- We can only run a zero test polynomials. <!-- .element: class="fragment" data-fragment-index="3" -->
- Run zero test on $t(x + 1)g(x + 1) - t(x)f(x+1)$. <!-- .element: class="fragment" data-fragment-index="3" -->

---

# Permutation check

- Now we want to use the ratio check to enforce the wiring we have.
- note that let $\psi$ be a permutation which preserve $T$ i.e. we have $T(a) = T(\psi(a))$ then
- ${(a, T(a))| \forall a \in \{1,..,39}} == {(\psi(a), T(a))| \forall a \in \{1,...,39\}\}$
- Then for any random $u_1, u_2$
  $\prod\_{a\in\{1,\dots,39\}}\frac{u_1 - u_2 * a - T(a)}{u_1 - u_2 * \psi(a) - T(\psi(a))} = 1$.

---

# Proof of wiring being correct

- The verifier runs a zero test on $T(x) - N$.
- The verifier runs a zero test on $T(3x) - a(x), $T(3x+1) - b(x), $T(3x+2) - c(x)$
- A Permutation check on $T(x)$ and $T(\psi(x))$ Which is a zero test on
- $t(x + 1)(u_1 - u_2* (x+1) - T(x + 1)) -$
  $t(x)(u_1 - u_2 * (psi(x)+1) - T(psi(x)+1)) = 0$

---

<!--

# Proof of correctness of the wiring on the trace polynomial.

SAGE demo

--- -->

# Zero test without knowing the polynomial: Polynomial commitment

- When the prover tells the $f(x)$ at some point $u$ ($f(u)$) without revealing $f(x)$.<!-- .element: class="fragment" data-fragment-index="1" -->
- It is a tool to convince the verifier which it has done so honestly.<!-- .element: class="fragment" data-fragment-index="2" -->
- The prover first commit to the polynomial $f(x)$ so later on, they can't back off and cheat (and use another polynomial).<!-- .element: class="fragment" data-fragment-index="3" -->
- Then the verifier is going to ask the prover to evaluate the polynomials in random point $u$.<!-- .element: class="fragment" data-fragment-index="4" -->
- The verifier is able to be confident that $f(u) = v$.<!-- .element: class="fragment" data-fragment-index="5" -->

---

# Zero test using polynomial commitment

- The prover claims it has $f(x)$ passing the zero test.<!-- .element: class="fragment" data-fragment-index="1" -->
- The prover is also able to compute $q(x)$ such that <!-- .element: class="fragment" data-fragment-index="2" -->
- $f(x) = q(x) \times  \prod(x-1)..(x-13)$<!-- .element: class="fragment" data-fragment-index="2" -->
- The prover commit to $f$ and $q$.<!-- .element: class="fragment" data-fragment-index="3" -->
- The verifier ask the prover to provide them with $f(u)$ and $q(u)$ for some random point $u$<!-- .element: class="fragment" data-fragment-index="4" -->
- It is very unlikely that the prover is able to lie about $f(u)$ and $q(u)$ given he has commited to $f$ and $q$.<!-- .element: class="fragment" data-fragment-index="5" -->
- The verifier computes $\prod(u-1)...(u-13)$<!-- .element: class="fragment" data-fragment-index="6" -->
- The verifier verifies that $f(u) = q(u)\times \prod(u-1)...(u-13)$ and if so believes that the prover has a solution.<!-- .element: class="fragment" data-fragment-index="7" -->

---

# KZG Polynomial-commitment

- Is the most space efficient polynomial commitment.<!-- .element: class="fragment" data-fragment-index="1" -->
- Uses elliptic curve cryptography.<!-- .element: class="fragment" data-fragment-index="2" -->
- It requires trusted setup: a pre-computation with toxic waste which needs to be discarded to keep the scheme secure.<!-- .element: class="fragment" data-fragment-index="2" -->

---

## Making ZK non-interactive

- The only interactive step is when verifier is quizzing prover with a random value $r$.<!-- .element: class="fragment" data-fragment-index="1" -->
- We replace that with asking the prover to apply a secure hash function to his commitment to generate $r$.<!-- .element: class="fragment" data-fragment-index="2" -->
- That way if the prover changes his commitment his point also changes without his control. <!-- .element: class="fragment" data-fragment-index="3" -->

---

<!--

## Use Circom to generate trace polynomials.

Circom demo

--- -->

<!--
## Use snarkjs to generate proofs

Generate proof demo with snarkjs

--- -->
<!--
## Use snarkjs to verify the proofs

Verify the proof snarkjs

--- -->




## Summary

- We discussed the general idea of ZK-Proofs.
- Their application for scalability and privacy.
- We looked how a sample ZK Proof is generated.
- We generate and verify the proof using modern ZK tools.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
