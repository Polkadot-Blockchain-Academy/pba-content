---
title: Smart Contracts Overview
description: Principles of coordination, contracts, smart contracts, and comparison with traditional legal infrastructure.
---

# Smart Contracts

## Overview

---

## Coordination

Coordination makes the boat go faster and the houses better

Examples of coordinated activities:
dancing, music band
money, business, trade
rowing team
building homes, buildings, roads
Doing the above safely through beaurocracies like inspection and certificaition proceedures, automobile registration

---v

## Why Should We Coordinate?

Coordination can happen by force / slavery
Could also be voluntary.

TODO figure of slavery. Or maybe two images juxtaposing voluntary and forced coordination. Like neighbors building a community center beside slaves building some building. Or indentured servants carrying heavy loads vs power lifters doing their sport.

Notes:

We could simply avoid considering slavery because it is immoral.
And this is a really good heuristic btw; I recommend you all avoid slavery because it is immoral in your daily lives.

We can also think of it as:
We want to create conditions (incentives) where voluntary coordination can happen freely and easily.
When we don't have these conditions, you do end up with slavery.
Because eventually someone sees the potential of coordination and forces it to happen.

---

primitive trade, dead weight loss, coordinating over time, this is another example of needing trust

---

So you want to make some kind of agreement to coordinate with another person.
But you don't want to get ripped off. (we'll build my house this week, then we'll build your house next month.)

Trust

- genuine trust - you can build it over time through a slow give and take like you do with your family or long term business partners for example
- you can believe that a person will act faithfully because they are incentivized to do so.

The first kind of trust is sometimes a "dirty word" in web3 spaces.

The truth is that this kind of trust can make things really efficient, and it is reasonable to leverage this trust _when it is warranted_

how to bootstrap genuine trust: start small and accept a small counterparty risk. Gradually increase over time.

It takes a long time and a lot of effort and cumulative risk to bootstrap this trust, so if you have it, you should use it.

When you shouldn't rely on genuine trust? when you don't have it. When the banker at the big bank tells you you can't have your money back unless you verify your identity by telling them where you live. (figure thinking do I really want this guy knowing where I live?)

---

So let's look at some existing technologies that allow us to scaffold the second kind of trust.
Like all technologies, the older more original versions are not always as sexy as the newer versions.
(show slides of bicycles, flying machines, electric lights evolving over time)

Promise

- how enforceable is it?
- does it work in small setting?
- does it work in a large setting?

We are at a point where we need to have some coordination between 8 billion people.
It doesn't have to all be fancy stuff like social media and metaverse and stuff.
But we at least need to coordinate on how much pollution we are gonna put into the air.
It's the modern equivalent of a primative village deciding "okay everyone, let's all agree we're gonna shit in the same corner of the village, not just wherever we feel like it."

Promise can make a village of 100 shit in the same corner, but not 8 billion.

---

Let's focus in on the idea of trade for a bit and explore some of the tools we've built there.

When you need to coordinate among strangers, you need promises to be backed by something.
One example is money, it allows strangers to coordinate over time because they don't have to hunt down the same person later to get the second half of the trade. They know that everyone else around them will accept the money for whatever goods and services they choose. In some ways the thing that is used for money is just a matter of convention. One thing we're going to try to do is make some improvements on money itself.

         promises - trade over time
        /                            \

simple swaps credit - trading an abstract notion of value today for
\ / a debt to be repaid over time
money - trade for abstract notion of value

trading over time requires trust for sure
money only requires trust to get started. After it has caught on, it is sticky. Everyone wants their money to still be valuable.

---

So how do we actually get people to keep their promises? Well, maybe social norm of keeping promises is kind of sticky too. If people tend to keep promises, the society prospers. But eventualy it will be sufficiently profitable to defect.

---

Contracts :tada:

Contracts are promises. They can involve money, but they can be so much more general than just money.
traditionally, contracts are special promises that the government will enforce.

The key idea is that the government is big enough to have power over individuals interacting, and if anyone tries to rip someone else off, the government will step in and make sure the contract is upheld.
This allows us to interact as if we trusted each other. Then the stickiness sets in and people actually do start to trust each other.

This is the power of contracts!
As long as the government fairly and reliably enforces them.

Of course sometimes the government can be a lot like the slave owner.
We need to watch out for that...

What is a Contract?
(to summarize)
A way for strangers to trust each other within some limited scope to accomplish something together.

---

What is a smart contract?

This CS guy Nick Szabo and some other cipherpunks realized in the 90s. That contract law and computer sciene actually had a lot in common.

Look at contracts in two parts

1. expressing them
2. executing/enforcing them

Programming languages are good at part 1 and computers are good at part 2.

So Szabo has this idea of the smart contract

---

Szabo definition
https://nakamotoinstitute.org/the-god-protocols/
https://nakamotoinstitute.org/formalizing-securing-relationships/
A machine program with rules that we could have defined in a contract, but instead a machine performs or verifies performance.

---

expression and enforcement...

Part 1 - Expression

Domain Specific Languages
are important
oldest writing samples are clerical
Humans have been inventing dsls forever.
It's one of our main things.

Contract law
is a DSL
is an important part of society
is hard to read (screenshot of legaleze)

Computer Programming
also a DSL
independant evolution from contract law
Can be quite hard to read (asm or solidity), good ones can also be easier to read. - example about napoleon wanting the average educated citizen to be able to understand the napoleonic code

They are not so different
They are both DSL meant to precisely express the nuances of detailed human agreements
They are both hard to read for non experts
someone in the early 90s had the realization that they were similar and had the idea that programming may be the future of law.
Nick Szabo

smart contracts broad definition
Insert definition

Cultures are different

- legal culture always adds, rarely removes
- legal culture is all about human judgement
- programmers love negative diffs
- programmers prefer to express things elegantly and minimally
- programmers like testing
- programming in about repeatability and objectivity

One goal of mine is to make smart contracts more accessible to everyday people so that you can read your own contracts

---

Part 2 - Execution / Enforcement

Once a legal agreement is made, it must be executed / enforced.

Traditional system
There is rarely an official submission moment.
Parties just sign.
Sometimes there is a witness
Sometimes even a notary
Rarely a submission to a judge.

Parties self execute.
On the happy path, no further action is necessary - this is actually a decided strength of the traditional system - it is maximally lazy
When there is a dispute, parties can go to court for a human judge to rule.
Judge enforces through powerful agencies like police and jails, and through social consensus.
In practice there is little consideration for scaling, queue length, ruling priority, resource exhaustion. But in principle there could be. The existing systems of the nation states could be optimized considerably without having to even adopt any real smart contracts.

Computer system
There is a deployment moment
You start some process on some computer and that computer faithfully executes the program for users to interact with.
At first you can think of it like a computer in the basement and we walk up to it when we want to interact. This is how nintendo works. Multiple controllers and multiple physically present users.
The internet makes it bettter in some ways. We can interact with our smart contract remotely instead of being physically present.
But now there is the question of the server operator. Are they really executing the correct contract?
The corrupt server opporator is analogous to a corrupt judge.

---

Code is law meme

---

web2 style contracts in actix_web or whatever

- nth caller
- blind bargaining
- simple money

---

slides about state machines

---

slides about web1,2,3, digital services where do we run these things?

The search for a neutral platform
Now that we have this great idea of smart contracts, we begin our search for a place to run them.
One common solution is a platform like amazon. Their business is to be reliable, they want a reputation for being reliable. And they mostly don't care about your shit. Until uncle sam comes knocking.

Another option is dedicated hardware.
vending machine
laundry machine
voting machine

---

P2P networking

PAB

Part 1 - Expression

Domain Specific Languages
are important
oldest writing samples are clerical
Humans have been inventing dsls forever.
It's one of our main things.

Contract law
is a DSL
is an important part of society
is hard to read (screenshot of legaleze)

Computer Programming
also a DSL
independant evolution from contract law
Can be quite hard to read (asm or solidity), good ones can also be easier to read. - example about napoleon wanting the average educated citizen to be able to understand the napoleonic code

They are not so different
They are both DSL meant to precisely express the nuances of detailed human agreements
They are both hard to read for non experts
someone in the early 90s had the realization that they were similar and had the idea that programming may be the future of law.
Nick Szabo

smart contracts broad definition
Insert definition

Cultures are different

- legal culture always adds, rarely removes
- legal culture is all about human judgement
- programmers love negative diffs
- programmers prefer to express things elegantly and minimally
- programmers like testing
- programming in about repeatability and objectivity

One goal of mine is to make smart contracts more accessible to everyday people so that you can read your own contracts

Part 2 - Execution / Enforcement

Once a legal agreement is made, it must be executed / enforced.

Traditional system
There is rarely an official submission moment.
Parties just sign.
Sometimes there is a witness
Sometimes even a notary
Rarely a submission to a judge.

Parties self execute.
On the happy path, no further action is necessary - this is actually a decided strength of the traditional system - it is maximally lazy
When there is a dispute, parties can go to court for a human judge to rule.
Judge enforces through powerful agencies like police and jails, and through social consensus.
In practice there is little consideration for scaling, queue length, ruling priority, resource exhaustion. But in principle there could be. The existing systems of the nation states could be optimized considerably without having to even adopt any real smart contracts.

Computer system
There is a deployment moment
You start some process on some computer and that computer faithfully executes the program for users to interact with.
At first you can think of it like a computer in the basement and we walk up to it when we want to interact. This is how nintendo works. Multiple controllers and multiple physically present users.
The internet makes it bettter in some ways. We can interact with our smart contract remotely instead of being physically present.
But now there is the question of the server operator. Are they really executing the correct contract?
The corrupt server opporator is analogous to a corrupt judge.
