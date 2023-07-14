---
title: Designing DAG-based consensus
description: A formal, yet friendly consensus framework
---

# Designing DAG-based consensus

---

## Goals of this lecture
<br/>

1. $~$
2.
---

## Goals of this lecture
<br/>

1. formalize the consensus problem and related concepts
2.
---

## Goals of this lecture
<br/>

1. formalize the consensus problem and related concepts
2. provide a framework for designing DAG-based consensus protocols
---

## What is consensus?
<br/>

- a **process** of agreeing on the same result among a group of participants
- a fundamental **problem** in distributed computing
- a key **component** of blockchain technology stack
---

## Consensus features
<br/>

<pba-flex center>
liveness, safety, integrity
</pba-flex>
---

## We have already seen some
<br/>

Nakamoto

Babe

Grandpa

Sassafras

Tendermint

...
---

## Who is running the protocol?
<br/>

Participants, called **nodes**
---

## Nodes
<br/>

- nodes can be either honest or malicious
- honest nodes follow the protocol
- malicious nodes can deviate from the protocol in any way they want
- malicious nodes can collude with each other
- malicious nodes can be controlled by an adversary
---

## Public key infrastructure
<br/>

- every node has its own <font color="#c2ff33">private</font> and <font color="#e6007a">public</font> key
- every node <font color="#c2ff33">signs</font> messages with its <font color="#c2ff33">private</font> key
- every node <font color="#e6007a">verifies</font> messages with other nodes' <font color="#e6007a">public</font> keys
---

## Public key infrastructure
<br/>

authenticated point-to-point communication
---

## Adversary
<br/>

Adversary **can** control the network delays, but is _computationally bounded_, i.e. it **cannot** break the cryptography (like forging the signatures).
---

## Network
<br/>

Communication via network... but what kind of network?
---

## Network models
<br/>

synchronous

partially synchronous

asynchronous
---

synchronous network
- there exists a known upper bound on message delivery time
- intuition: there's a well-defined notion of a protocol round

partially synchronous network
 - there exists a known bound Δ, and an unknown point in time GST after which 
the communication becomes synchronous with delay Δ.
 - intuition: protocol will eventually work synchronously, but it needs to be safe before
 - sounds weird, but actually captures the reality of the internet pretty well

asynchronous network
 - there is no upper bound on message delay 
 - we assume that the adversary have full control over the message delays
 - intuition: you can never tell whether a node have crushed or has a long delay
 - the concept of a timeout is basically useless.

crucial results:
 - FLP theorem
   - It is impossible to have a deterministic protocol that solves consensus in an asynchronous system in which at least one process may fail by crashing.
 - Castro-Liskov theorem:
  - It is impossible to have a protocol that solves consensus in a partially synchronous system with 3f+1 nodes in which more than f processes are byzantine.

what does it mean?
Best one can hope for in asynchronous scenario is probabilistic protocol tolerating up to f faults for N=3f+1.

and this can be achieved (multiple protocols exist)
and we will target this scenario

note: dummy randomness

one more notion, once we are in the synchronicity:

responsiveness
 - tendermint or grandpa have to wait some delta time to proceed to next round
 - delta must be long enough to allow all honest nodes to send their messages
 - delta must be short enough to allow the protocol to make progress
 - in case of failure, we have to perform pretty expensive recovery procedure (like leader change)

in responsive protocols it is enough to wait for a 2f+1 messages to proceed to next round

asynchronous protocols must be responsive
in good network conditions, they are much faster

first exercise: let's try to design a protocol that works in a asynchronous network
basic piece: how to perform a reliable broadcast?

requirements and protocol: slide from lecture

in practice: heuristics

it's high time for blockchain protocol
in academia, called atomic broadcast

... <- Adam continuous / single-time

but we remember, we have to include randomness:

randomness beacon: slide

protocol timeline: slide
BFT protocols

DAG-based protocols

DAG - directed acyclic graph

how does it relate to consensus?
recall responsiveness: we need to wait for 2f+1 messages to proceed to next round
so we represent the messages as vertices in a graph
usually, we have a single broadcast/vertex per round
so we have a DAG of vertices, where each vertex represents a message
and we have edges between vertices that represent the dependencies between messages

sending a message in round n means that we received 2f+1 messages in round n-1

the core:
 - we maintain a local DAG representing our knowledge of the messages (units)
 - and we perform a local, offchain consensus on the DAG

two parts:
 - online: sending and receiving messages that contribute to the DAG
 - offline: everybody performs a local consensus on the DAG, just by looking at it

notes:
 - local DAGs might differ...
 - but they are guaranteed to converge to the same DAG
 - the offline consensus is guaranteed to produce the same result

adversary:
 - controls edges and its own messages

what about the randomness? it is put into the local consensus protocol

how does it relate to atomic consensus?
 - nodes receive transactions and put them into their units
 - we come up with a linear ordering of the units

digression: block production, dissemination and finalization
 - usually prod + dissemination is on one layer, and then, independently we can finalize blocks
 - in DAG, the border is naturally in another place: dissemination is separate from block production and finalization (which actually is instant)

differences: signatures, faster

what in more detail is this local consensus about?
recall, local copies might differ by a lot, blocks might have not come to all nodes yet, etc

the notion of availability:
 - intuitively: a block is available if most of the nodes have it and they know that most of the nodes know about it

if a unit is available it is a good candidate for being chosen as an anchor to extend current ordering

implementation: AlephZero

how do we solve atomic consensus: head

available unit is a good candidate for being chosen as a head
however, to fight with adversary, we need to employ randomness in choosing which of the available units to choose as a head

moze opzniac w nieskonczonosc
coin availablility


bonus: common randomness
