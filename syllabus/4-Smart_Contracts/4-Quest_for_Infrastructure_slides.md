---
title: The Quest for Infrastructure
description: Summary and review Smart Contract principles, and observation that we have not yet thwarted the tyranical server operator
---

# The Quest for Infrastructure

OR

Where can I actually run these things!?

---

Quick Review of Smart Contracts

---v

Expression: Write code instead of long confusing legalese

---v

Execution: Let the computer run it instead of the judge interpreting it.

---

Quick Review of Service Providers

---v

They are the technicians that keeo the infra running.

They don't ahve any particular morals, certainly not the same ones you have.

They can be evil

---v

They can back door

Insert demo of hacking the actix-web contract from the backend

---

# Where to Run the Contracts?

Notes:

We know we need somewhere to run these things with no back door or trusted party.

We have seen some hints so far. Let's check them out.

---v

## Diversity and Redundancy

* Geographical - for natural disasters and terrorist attacks
* Jurisdictional - to resist the incumbant governments - they are the ultimate server operators
* Moral - so all compases are represented in the network, and no grou can impose hegemony
* Of compute hardware - incase some is bugged or backdoored or prohibitively expensive

Notes:

Web2 gets a lot of this right. At least they are good at the first two and preventing accidental data loss etc. There is a lot to be kept from web 2.

But indeed there is also some to be thrwon out or improved.

---v

## P2P Networking

Replace the operator with a system where peers all have power.

Notes:

We saw well how this worked out well in the file sharing and anonymous browsing domains (bit torrent,)

---v

## Reproduceable Execution

* Computers are better than courts
* PABs make distributed code execution practical

Notes:

We saw even back in the early 2000s with java web applets that allowing more people to run the same program is hugely useful.

A few decades later we have much better tech for this, and it is even more valuable.

PABs make it practical for diverse parties all over the world to run the same software deterministically on their bespoke hardware in their unique environments.

---

# Blockchain

## Solves ALL Your Yroblems

Heavenly graphic. Maybe Moses coming down Mt Sainai with blockchain photoshopped where the stone tablets should be.

---v

## Solves Some Specific Problems

Allows us to replace the central server operator with a P2P network and Consensus System

Notes:

Actually it is a falacy that blockchain solves all our problems.
It actually solves some very specific coordination problems.
It also brings some efficiency improvements to modern beuarocracies.
It does not automatically make everyone elieve the same things or magically arrive in happy unicorn bunny land.
Tomorrow and Thursday will dig in on how the blockchain and its P2P network work together.

---v

## Blockchain Datastructure

todo figure

Notes:

We'll discuss two new blockchain related topics.
First is the blockchain data structure which you can see here.
This one is forked which is when things get really interesting, and when you need to invoke the second part

---v

## Blockchain Consensus

todo

Notes:

Consensus deals with how we agree which version of the data structure is real.
It is an interesting and complex topic, but we first need to learn a bit more about how the data structure allows us to track a shared story.

---v

## Reading Assignment

For tomorrow please read the bitcoin whitepaper