---
title: Open Gov
description: The Polkadot ecosystem on-chain governance solution
duration: 2 hours
instructors: ["Bradley Olson"]
---

# Open Gov

---

## Overview

- Why on-chain?
- Goals of on-chain governance
- Initial Solution, Governance V1
- Improvement, Open Gov 
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

- Lifecycle of a Referendum
- Origins and Tracks
- Support and approval threshold curves
- The Polkadot Fellowship
- Vote Delegation and conviction voting
- OpenGov and governance goals 

---

# Origins and Tracks

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
- Emergency tracks: EmergencyCanceler, EmergencyKiller

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

These curves are monotonically decreasing, meaning that the level of support and approval necessary to pass a proposal decrease as the proposal spends longer in the decision period.

---

## Example Support Curve

Moonbeam article image

---

## Example Approval Curve

Moonbeam article 2 images, approval curve and going in and out of confirmation

---

## The Polkadot Fellowship

---

## OpenGov and Governance Goals

- Open source + single process + track abstraction -> Transparency
- Cowboy proposal creation + greater throughput + per-track delegation -> Accessibility
- Accessibility + No special bodies -> Decentralization
- Limited origins + emergency tracks + white list -> Security
- Multiple tracks + low risk tracks -> Concurrency
- Low risk tracks + early confirmation -> Speed 
- Support and approval threshold curves -> Agility

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

# Questions?

---

## Farther Learning Resources 
OpenGov Article from Moonbeam Team - https://moonbeam.network/blog/opengov/
Gavin’s Polkadot Decoded 2022 talk - https://www.youtube.com/watch?v=EF93ZM_P_Oc

---