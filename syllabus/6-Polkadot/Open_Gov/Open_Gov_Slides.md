---
title: Open Gov
description: The Polkadot ecosystem on-chain governance solution
duration: 1 hour
instructors: ["Bradley Olson"]
---

# Open Gov

---

## Overview

- Why on-chain?
- Goals of on-chain governance
- Initial Solution, Governance V1
- Improvement, OpenGov 
- How is it going? By the numbers.

---

## Why On-chain?

Off-chain governance: 
- Formal proposal drafted by core dev team
- Discussions, debates, and media campaigns
- Hard fork

Issues:
- Centralization
- Low throughput
- Long decision period
- Little accessibility

---

## Goals of On-chain Governance

- Transparency: Decisions by who and by what rules?
- Decentralization: Distributed power, weighted only by commitment/conviction
- Security: Harmful proposals don't pass or have limited scope
- Accessibility: Easy to draft, to receive vote, and to vote yourself
- Concurrency: Maximize simultaneus referenda as security allows
- Speed: Each referendum completed as fast as security allows
- Agility: Speed responsive to support/controversy

---

## Governance V1 

- Single track
- Sudo origin
- Conservative security
- No concurrency
- 28 day referenda
- 1 month execution delay
- Emergency referenda by technical committee
- Most proposals by council
- Fully council controlled roles such as tipping

---

## Gov V1, Room for Improvement

The good:
- Security
- Transparency

The bad:
- Decentralization
- Concurrency
- Speed
- Agility

---

## OpenGov Overview

- Origins and Tracks
- Lifecycle of a Referendum
- Support and approval threshold curves
- The Polkadot Fellowship
- Vote Delegation by Track
- OpenGov and governance goals 

---

## Origins

- Level of privelege that code executes with
- Similar to user on Unix
- Proposal is two things
    - Operation: What should happen
    - Origin: Who authorizes it
- Many operations require a specific origin


---

## Origins and Tracks

- Each origin is served by a referendum track
- A track can serve more than one origin
- These tracks are totally independent from one another
- Track examples: Root, ParachainAdmin, BigSpender, Tipper
- Emergency tracks: ReferendumCanceler, ReferendumKiller

---

## Track Parameters

Parameters give us the ability to find an optimal balance between security and throughput. The security needs of the Tipper track are very different than those of the Root track.

- Lead-in period
- Decision period
- Confirmation period
- Minimum enactment period
- Concurrency, how many referenda can run in this track at a time
- Support and Approval threshold curves

---

## Handy Table

(Image of tracks table from polkaworld article)

---

## Lifecycle of A Referendum

Image from moonbeam article

---

## Criteria for Passing a Proposal

Conviction: Locking tokens for longer periods scales their voting impact up to a maximum of 6x with a lockup duration of 896 days
Approval: Approving votes/total votes cast, weighted by conviction
Support: Approving votes/total possible vote pool, disregarding conviction

---

## Deciding and Confirming Periods

- If Approval and Support thresholds met, confirmation period begins
- Approval and Support must remain above respective thresholds for entire period
- Confirmation period concludes -> proposal approved early
- Decision period expires -> proposal rejected

---

## Support and Approval Threshold Curves

We want the agility to quickly pass uncontrovercial proposals while deliberating for longer on proposals which draw lower turnout or approval.

This need is addressed through the use of curves defining thresholds that must be met to pass a proposal. 

These curves are monotonically decreasing, meaning that the level of support and approval necessary to pass a proposal decrease as a proposal spends longer in its decision period.

---

## Example Support and Approval Curves

(Image from)
https://polkaworld.medium.com/a-hands-on-guide-for-kusamas-open-gov-governance-system-98277629b0c5

---

## Polkadot Fellowship: Motivation

Typical path to safety: Lower throughput and restricted origins

But in emergencies we may need to pass proposals that both require root origin and are time critical!

Solution: Some sort of oracle capable of providing expert information

---

## Oraclizing Expert Information

1. Track everyone's level of expertise
2. Allow experts to register sentiment
3. Aggregate opinions by level of expertise

How are these accomplished?

---

## Enter The Polkadot Fellowship

(Some Polkadot Fellowship Image)

---

Purely on-chain membership body to recognize and compensate all individuals who hold and use expert knowledge of Polkadot in line with its broad interest and philosophy

Members hold rank denoting proven level of expertise and commitment as recognized by their peers and, for higher ranks, through general referendum.

---

## Who Make up the Fellowship?

- Experts in the Polkadot core protocol who maintain a consistant level of active contribution
- Notably this does not include core developers of independent parachain protocols, which should develop their own protocol specific fellowships as needed.
- Currently: < 100 core developers, mostly from Parity or the Web3 Foundation
- Next year or two: Hundreds
- Ideal far future: Tens of thousands, independent of any centralized entity
- Only one fellowship for Polkadot and Kusama

---

## Function of the Fellowship

- WhiteListedCaller track
    - Root priveleges
    - More agile
    - Maintains reasonable safety via Fellowship
- White list proposals must pass two votes
    - Expertise weighted Fellowship vote via second referendum pallet instantiation
    - Same general referendum as other tracks

Just an oracle!

My suspician: Secondarily intended to cultivate a long term base of Polkadot core developers outside of Parity

---

## Vote Delegation

- Traditional delegation: You entrust one third party with your vote on all matters
- Delegation by track: You may choose to delegate your vote to one or more third parties on a per track basis
- EX: Tipper vote delegated to local ambassador, WhiteListedCaller vote delegated to Parity Technologies, vote retained for all other tracks
- This is new and exciting!

---

## OpenGov and Governance Goals

- Open source + single process + track abstraction -> Transparency
- Cowboy proposal creation + greater throughput + per-track delegation -> Accessibility
- Accessibility + No special bodies -> Decentralization
- Limited origins + emergency tracks + white list -> Security
- Multiple tracks + low risk tracks -> Concurrency
- Low risk tracks + early confirmation -> Speed 
- Support and approval threshold curves + white list -> Agility

---

# OpenGov, by The Numbers

---

## Governance Activity

(bar chart 1) Referenda concluded/day (1.222), 5.5x more than DOT in same time period (0.222). 3.44x more than KSM in final month before OpenGov launch (0.355)

---

## Proposal Origins

(bar chart 2) Origins of proposals, Dot since OpenGov release (2/12) 16% Democracy. OpenGov (66/66) 100% democracy 

---

## Treasury Usage

(bar chart 3) Based on data from the latest spend periods on Polkadot and Kusama respectively, Polkadot is burning 2.2x more treasury funds as % of market cap per adjusted spending period (0.0378% vs 0.017%)

---

## OpenGov and You

- Participate in OpenGov and Polkadot Fellowship on Polkadot and Kusama
- Customized OpenGov instances per-parachain
- Custom fellowships per parachain
- Potentially non-technical fellowships, such as a fellowship for brand ambassadors

---

## Farther Learning Resources 
PolkaWorld Hands on OpenGov Article - https://polkaworld.medium.com/a-hands-on-guide-for-kusamas-open-gov-governance-system-98277629b0c5
OpenGov Article from Moonbeam Team - https://moonbeam.network/blog/opengov/
Gavin’s Polkadot Decoded 2022 talk - https://www.youtube.com/watch?v=EF93ZM_P_Oc

---

# Questions?