---
title: Blockchain Overview
description: Initial look at web history and the problems blockchain and web3 are solving
duration: 30 - 45 minutes
---

# Overview of Blockchains

Notes:

Introduce instructors, tell about background, glad to be here.
Let's all learn together.
Shoot the shit until the initial nervousness starts to wear off.

---

## Goals

- Trustless provisioning of infrastructure. <!-- .element: class="fragment" -->
- Ways to coordinate with people we don't trust. <!-- .element: class="fragment" -->

Notes:

So. What are we doing here? What are our goals?

First:
Something kind of like a server, that doesn't rely on a server operator, and has strong guarantees like Cryptography has to offer.

Next: Coming to a shared understanding of a common history, and therefore a common state, of a system.

---v

## Comparison with Cryptography

Cryptography provides strong guarantees about _messages_ without a trusted party, regardless of the conduit over which a message was transported.

Notes:

Crypto guarantees:

- No tampering
- No eavesdropping
- Authenticity of the author

We want these same _kinds_ of guarantees, not just about messages, but about entire systems.

---

## Web 0

Telegraph, Telephone

Users transmit information peer-to-peer.

Crypto not typically used except by military, but upheld guarantees when used.

<img src="./img/overview/Web0.png" />

---

## Web 1

Introduction of always-on servers.

Still mostly peer-to-peer.

Cryptography more often, but still not ubiquitous.

<img style="width: 300px;" src="./img/overview/Web1.png" />

---

## Web 2

Introduction of **Digital Services** with **Rich State**.

Administered by service providers: "Send _us_ your information."

However, users must place faith in the service provider.

Cryptographic guarantees are about interactions with the service provider, not peers.

<img style="width: 300px;" src="./img/overview/Web2.png" />

---v

## Digital Services

People rely on digital services every day.
They are inescapable and valuable.

- Twitter, Instagram, Facebook, etc.
- Journalism and sources
- Banks
- Lawyers, notaries, regulators

Notes:

- Ask class for more examples
- Digital services are not bad in and of themselves. They are very valuable. We use all of these every day. We are even using some to administer this course. But they are also hard to escape.

---v

## Trust Example

Two users on Twitter:

- Trust that we are seeing information from the same database\*
- Trust that if a tweet is from X, then X wrote that tweet\*
- Trust that others see our messages as from us\*
- Trust that the messages we see are the messages the users wrote\*
- Trust that we're interacting with the application as equals

Notes:

- Cryptography actually provides a lot of these guarantees, but not when an intermediary has stepped in between users.
- This is one example, but class should discuss.

---v

## God Mode Enabled

In web 2, service providers can perform abuses:

- Censoring user interaction
- Restricting users interaction
- Failing to produce requested data
- Mutating state opaquely

---v

## Thought experiment: Digital Currency

Bitcoin's application was digital currency - a trivially simple application.

Could this be built with Web2 technology?

Notes:

Yep it could. This is the kind of simple app you might build in a Freshman year course
on modern web interfaces. It just needs to maintain a set of bank notes and their owners (or alternatively a set of accounts and their balances.) So why didn't this exist in web 2? Because the provider could print money. Or steal money. Or freeze funds.
Side thought. How different is this from fiat currencies?

---v

## Distributed Applications in Web 2

Providers run redundant data centers to prevents accidents.

But it still assumes benevolent participants and some trusted leader.

Notes:

Even in web2 we start to see the idea of redundancy to prevent accidents from natural disasters, sabotage, hardware failure etc.
But we do not yet see disintermediation. In web 2, the masses become beholden to the service providers who were free to extract value and manipulate the users.

In fact redundant systems were widely studied even before web 2. Consider a flight computer that has sensors for things like air speed and altitude. If one sensor fails we want the plane to keep flying.

---

## Web3

- <span>A provision of digital services without the need to trust a service _provider_.</span> <!-- .element: class="fragment" -->
- Providers do not need to be trusted; they are economically incentivized to behave honestly. <!-- .element: class="fragment" -->
- Allow users to interact with a common system without trusting any intermediaries. <!-- .element: class="fragment" -->

Notes:

We want to maintain the value, versatility, and richness of Web2, but remove the trust, and possibility of extractive behavior.

---

# Desired Properties

---v

## Permissionless access

Anyone should be able to access and interact with the system.

---v

## Privacy

Users should have credible expectations about what information they give up about themselves.

---v

## Authenticity

Users should have credible expectations about the messages they see, regardless of the platform the messages are on.

---v

## Finality

Users should be able to form credible expectations about when a state transition is final.

---v

## Behavior

The system should behave as expected, even if system operators do not.

---v

## Unstoppability

No individual actor, company, state, or coalition should be able to degrade any of these properties.

---

## A Shared History

<img style="height: 500px;" src="./img/overview/sapiens.jpg" />

Notes:

So now we understand the goals of web3. How do we achieve them? The key is allowing users to agree on a shared history. The simplest blockchains do nothing more than timestamp and attest to a stream of historical records. In Web 2, and indeed often in governmental bureaucracies, users have no visibility into the history of the app. They must trust the provider to accurately represent the current state. By giving the service provider the power to change the story, we give them the power to shape our understanding of reality and consequently our behavior.

---v

## A Shared History

> Any large-scale operation - whether a modern state, a medieval church, or an archaic tribe - is rooted in common stories that exist only in people's collective imaginations.

> Telling effective stories is not easy. The difficulty lies ... in convincing everyone else to believe it. Much of history revolves around this question: How does one convince millions of people to believe particular stories about gods, nations, or LLCs?

<!-- .element: class="fragment" -->

_-- Yuval Noah Harari, Sapiens --_

---

## State Machines

We can formalize this notion of shared story with state machine model.

<img width="800px" src="./img/overview/state-machine-general.svg" />

Notes:

Most systems that we care about can be modeled as state machines. A state machine is not a real machine that you can touch. It is a model comprised of a set of states and a set of rules about how to transition between the states.

---v

## Labelled Transition Systems

Sometimes you can map the entire state space as an LTS.

Other times it is too big.

<img src="./img/overview/state-machine-arbitrary-history.png" />

Notes:

Consider if we tried to map all possible states of a social media app or a digital currency. Sometimes an LTS drawing like this is useful, other times it would be too large or even infinite. Even still, sometimes drawing part of it can help you think about what the states and transitions might be.

Example: Audio Amplifier

There are three buttons (louder, quieter, mute). The amplifier's volume output can be one of four levels: Silent, Quiet, Normal, Loud. Volume up and down should be pretty clear. When you mute the output immediately goes silent, and when you unmute it goes back to whatever it was before.

Q: How should we model the states?
A: We could make the states directly be the volume levels.

---v

## Example: Light Switch

Simple Switch: 2 States, 1 Transition

**Labelled Transition System** <!-- .element: class="fragment" data-fragment-index="2" -->

<img src="./img/overview/light-switch-lts.svg" /> <!-- .element: class="fragment" data-fragment-index="2" -->

**History** <!-- .element: class="fragment" data-fragment-index="3" -->

<img src="./img/overview/light-switch-history.svg" /> <!-- .element: class="fragment" data-fragment-index="3" -->

---v

## State Machine Example: Digital Cash

Each state is a set of bank notes. Where a bank note has an amount and an owner.
A transition involves a user consuming (spending) some bank notes and creating new ones.

<img width="800px" src="./img/overview/state-machine-cash.svg" />

Notes:

Not all conceivable transitions are valid. Imagine a user consuming a bank note worth 5 coins, and creating two new ones each worth 3 coins.

---v

## Sate Machine Example: Social Media

Each state is a set of posts and their associated comments and emoji reaction counts.
A transition involves, making a new post, or reacting to someone elses, or commenting

<img src="./img/overview/state-machine-social.svg" />

Notes:

There is not a single model here. Some state machines will allow deleting or editing posts, while others will not. Some will allow disliking posts while others only allow liking.

---v

## More State Machine Examples:

<pba-flex center>

- Ride sharing platform
- Voting system
- Blackjack table

</pba-flex>

Notes:

Let's brainstorm what the state and the transitions might be for each of these.

---v

## Shared Story of a State Machine

<pba-cols>
<pba-col>
<pba-flex center>

If we agree on:

- The starting state<br/>(aka genesis state)
- The history of transitions

</pba-col>
<pba-col>
</pba-flex>

Then we _MUST_ agree on:

<pba-flex center>

- The current state

</pba-flex>

</pba-col>
</pba-cols>

Notes:

Now that we have a formal math-y model of systems that we care about, we can see that the notion of shared stories being powerful is more than slick language of philosophical mumbo jumbo. Even the term genesis state (or genesis block) is taken straight from mythology. We aren't newly discovering or inventing the idea that having a shared understanding of our past is important. It dates back to pre-history. We are just formalizing it and applying it to digital services.

I have a talk from 2020 that goes into more detail and philosophy about this shared history idea and its relation to blockchain:
https://www.youtube.com/watch?v=FZXk3_RTvfc

It is one of my favorite talks I've ever given.

---

## Blockchains (Finally)

A blockchain can be thought of in three parts

<pba-cols>
<pba-col>
<pba-flex center>

**State Machine**

What does the state hold?

What are the _rules_ to change it?

</pba-flex>
</pba-col>
<pba-col>
<pba-flex center>

**Shared History** (data structure)

Which potential histories exist?

</pba-flex>
</pba-col>
<pba-col>

<pba-flex center>

**Consensus**

Which history is the real one?

What part of history is final?

</pba-flex>
</pba-col>
</pba-cols>

Notes:

Let's end this lesson with a little history of what we have seen in the blockchain space so far.

First, each blockchain tracks some state machine. We've discussed several examples of what that might be already, we'll code some simple examples shortly, and we'll spend all of module 5 digging into how to create a blockchain-friendly production-ready state machine.

Next is the Blockchain Data structure. This data structure is basically a linked list of state transitions. But unlike the linked lists you studied in your data structures course, it isn't just linked by memory addresses or any other malleable thing. Instead it is cryptographically linked so that if anyone presents a different history, you can tell right away that you don't agree on a shared history. We'll dive into this data structure in the next lesson.

Finally, is a consensus mechanism. Defining a state machine alone does not uniquely define a history. There are many possible valid histories. Just like the many worlds interpretation of quantum mechanics. To really agree on the current state, we need to agree on which of the possible histories is the real one.

---v

## Short History of Blockchains

---v

## Bitcoin

<img rounded style="width: 500px;" src="./img/overview/bitcoin-transaction.png" />

Uses an unspent transaction output (UTXO) model & Proof of Work (PoW) consensus. <!-- .element: class="fragment" -->

Notes:

Who knows where the figure is from?

CLICK

Bitcoin was first.
It was a money app.
The first app chain.
It hosts a smart contract in the broad definition.

It was the first time most people considered a digital service that was not run by a particular person.

Figure source: [Bitcoin white paper](https://bitcoin.org/en/bitcoin-paper)

---v

## Litecoin, Monero, Dogecoin

<img src="./img/overview/altcoins.png" />

Notes:

Only a few year later people realized they could fork the code and make small changed and improvements.
Some changes were trivial.
Some were interesting: monero and privacy.

For me personally, this was a small crisis.
I thought bitcoin was the one global one.
Aren't these other coins undermining the narrative?
NO! The point is that anytime you don't like the system or have an idea for a better one, you can do it!
If you don't like bitcoin, build your own coin and make it better.
Let the market decide.

---v

## Ethereum

- Generalization of Bitcoin: provides a quasi-Turing-complete VM
- Uses an account-based system
- Accounts can store balances, but can also store executable code (smart contracts)
- Each contract can have its own internal state and API

---v

## Polkadot, Cosmos, Avalanche, Near, ...

Notes:

Iterations on Ethereum's general computing platform. Strive for scalability interoperability etc. Experiment with different consensus models.
