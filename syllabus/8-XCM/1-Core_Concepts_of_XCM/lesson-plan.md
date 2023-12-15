# Lesson 1: Core Concepts, Terms, and Logic of XCM

## Why is this topic Important?

- Fundamental to all aspects here, the syntax and concepts must be well understood.

## Prerequisite Knowledge or Skills

- FRAME (Storage Items, Dispatchables, Event, Errors, etc.)
- Parachains in general

## Learning Outcome

- Define the terms and concepts that XCM relies on, and know where the sources of truth are when in doubt to reference.

## Learning Objectives

## Core Ideas to Convey

- Cover all of https://github.com/paritytech/xcm-format
- Instancing
- Aggregation
  - What does this mean?
- Weight (in XCM)
- Exotic Stuff: e.g., Genesis, Logs, Signed Extensions

## Activities and Exercises

# Dan Notes

- [Moonbuilders Workshop: A Technical Introduction to XCM on Moonbeam](https://www.youtube.com/watch?v=5HD5rFBqvQ4)

  - [their docs on xcm](https://docs.moonbeam.network/builders/interoperability/xcm/overview/)
  - Great overall intro, [slides are great](https://docs.google.com/presentation/d/1dKZiP1LUltfjJ4cHiB1XtJAGla3sngXSc7sFj84zRKk/)...
    might want to ask to use?
    Copy content perhaps
  - 39:20 demo code w/ xTokens (XC-20) - It's possible to have different versions for specific fields/xcm programs{?} (destination, beneficiary) in a single XCM?
    Why?

- Activities
  - Make XCM that does some logic, given the account balance you have on a single parachain to execute a few increasingly more complex routes to things.
  - XCM message that is overly complex.
    Have student identify and rewrite with less program steps & routes
  - XCM that is HERE only, and thus should be a local extrinsic instead, have them discover this and outline the right calls.
  - Open channels manually on a testnet, configure correctly.
  - Issue XCM on a testnet

### Keith meeting 2022-05-05

- XCMP has two _meanings_
  - Generalized category of transp mechanisms (HRMP, DMP, UMP)
    - These are not all concretely defined, an incorrectly used interchangeably
  - Concretely the one transp protocol for parachains on the same relay chain.
- HRMP
  - Uses relay to store XCM between parachains, not optimal
- How can a sovereign account be verified exactly?
  TODO
- NOTE paraID CAN change over time!
  - Don't hard code these into a dapp!!
  - **SOVEREIGN ACCOUNT FOR PARACHAINS SWITCHING ID MIGHT BE TAKEN OVER!**
    - Reserve assets & Barriers would be lost / given to other chains
    - This would be most explicit for HRMP, XCMP might have less reliance to paraID.
    - MultiLocation is based on paraID...

### [moonbeam slides](https://docs.google.com/presentation/d/1dKZiP1LUltfjJ4cHiB1XtJAGla3sngXSc7sFj84zRKk/edit#slide=id.g112909de4e6_0_92)

- slide 10:
  - sov account vs. a reserve ~~account~~ chain: These are not the same.
    - Have a reserve chain that could have
- slide 12:
  - might have the over-constricted/missing nuance/detail model vs this answer: Stack Exchange example: https://substrate.stackexchange.com/questions/37/how-can-i-transfer-assets-using-xcm/38#38

## TODO

- Wiki updates, VMP could replace XCMP, this is false
- meeting with Rob and other impl people to about XCMP and actual things in the works now and planned
  - [ ] Start with a forum post on this instead of a call to start. - Clarify XCMP details about UMP/DMP use etc. - What interaction does XCMP have with relay chian now and in the future?
        In detail, stepwise.
- https://github.com/paritytech/xcm-format

  - updates to american english
  - ensure that terms used are uniform for ALL of us, internally and get this as the source of truth.
    (also update glossary for academy)

- we need support team to get up to date on XCM to answer basic questions!! AND alumni of academy!

  - seed the idea that they can make HUGE REP GAINS for these Q&A on SE!

- MORE CONTENT: XCM testing and troubleshooting with Nacho & DS team

- [ ] Dan to polish, Keith to review, then Gav to review
