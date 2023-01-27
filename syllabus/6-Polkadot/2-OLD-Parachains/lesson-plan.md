# Parachains

Lesson 5, Module 4

Instructor: Bernhard Schuster, Robert Klotzner

<hr>

Goals:

- Understand demands on Blockchains and how Polkadot tackles them
- Understand how parachain consensus works on Polkadot
- Understand what a valid/invalid parachain block is

---

## Problems of pre-existing blockchains

- Slow
- Don't scale
- Smart Contract based

## Goals of Parachains

- Become scalable
- Stay secure
- Keep good liveness guarantees
- Performance by specialization

## Develop Parachain Consensus together with audience

- Start with existing blockchains and their problems: Ethereum/Bitcoin
- How are we going to achieve the scalability goal?
  -> Pick up answers that go into the right direction.
- Start sketching a solution and check whether goals are matched.
  -> Some are not, iterate further.

## First try

### Outline

- Split validators up into known groups -> sharding
- Each group validates a parachain

-> Nice picture of grouped valiators and collators of parachains connecting to
them.

### Problems

1. Liveness: If an assigned group is malicious they can completely stall a parachain by just not doing their job.
1. Groups must be relatively large to maintain security (~40 validators).
1. Not too bad, but we would need to escalate to more validators very quickly in case some validators don't provide results -> Again Liveness.

### Refinement:

1. Rotate validators, have them not validate the same parachain all the time.

What about 2 and 3?

How can we get better in terms of performance and scalability?
The problem of the above is, that validators know what other validators are validating - so an attacker can wait until only/mostly bad guys are validating and then get a bad block in, with no issues maybe DoSing a few good guys.
If DoSing does not work, there are no consequences, the attacker can simply try again.

## Second try

### Goals/Problems to solve:

- Validators should not know who else is checking
- But collators need to know which validators to send their collation (their
  PoV) to - so validators must be known.
- We can split into known and unknown validators, but how would unknown
  validators get the collation? (If they request it, they reveal themselves)
- Also if the known validators are all malicious (or most of them), they could
  simply not provide the collation to any good validator requesting them - hence
  avoiding any potential consequences of a failed attempt.

### Introducing availability

We are going to introduce stages:

1. Some validators are known in advance and accept collations and sign a
   statement saying it is valid.
   They now have skin in the game.
1. Only once they committed to collations (parachain blocks) being valid, a second set of validators (the approval checkers) reveal themselves.
   If they find the initial backers did something bad, they get punished (disputes).

Doesn't sound too bad.
But does this work already? Can someone spot the problem?

### Availability

We need to make sure data is available, so the bad guys can't get away by just not providing the data when getting caught.

Approval checkers will only reveal themselves once it is made sure that the data for checking their work (the PoV) will actually be available.
How do we do that?
... Let students guess a bit.
Then explain the solution (erasure encoded chunks) and its properties:

1. Minimal possible overhead (Performance).
1. Guaranteed availability under byzantine assumptions.

### Approval Checking

Approval checkers recover the data and check again.
They either confirm the candidate or they raise a dispute, at which point all validators will check and the validators on the losing side of the dispute get slashed.

## Alternative Solutions/Why Approval Checking and Disputes?

Scalability is reached, by not letting all the nodes do the same work, but have different nodes have different responsibilities in a rotating fashion.
(Backing & Approval checkers for each parachain)

An alternative solution would be to make the backing group relatively large (around 40 checkers for 1000 validators) and thus make it statistically so unlikely that it consists of only malicious nodes, that it does not matter in practice.
Like the likelihood for a successful attack would only be like 1 in a million years.

The problem with this is two fold though:

1. Liveness: To keep the system secure we would need to escalate to more checkers very quickly in case checkers don't show up.
1. We have a hard lower limit on the number of checkers, which limits
   scalability.

With approval checkers and disputes we have a system where we can safely increase the likelihood of a successful attack to for example 1 in 50 years, because each try will cost the attacker millions of Dollars worth of DOT.
So any realistic attacker would go bankrupt by the time it would succeed.
Thus we get scalability by making use of gamblers ruin.

## Summary:

- Make sure some checking validators (approval checkers) are not known in advance.
- Make sure data is available and backing checkers have committed to the candidate.
- Approval checkers check (assignments, reveal) - bad backers are caught, dispute is raised -> punishment -> gamblers ruin
