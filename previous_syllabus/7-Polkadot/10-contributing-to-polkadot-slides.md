---
title: Contributing to the Polkadot SDK
description: PBA, Fellowship, Contributing guide
duration: 60 min
---

# Contributing to the Polkadot SDK

---

<img style="width: 1000px" src="assets/Contributing/intro.png" />

---

## Prerequisite Knowledge

- Mid-Level Rust Programming Abilities
  - Fluency with the first 11 chapters of The Rust Book
- Mid-Level Understanding of Blockchains
  - Basics of Cryptography, Game Theory, Economics
  - Fundamentals of Bitcoin and Ethereum
  - Smart Contracts / State Machines
- Basic Understanding of Polkadot

---

## The Polkadot Blockchain Academy

<pba-cols>

<pba-col left>

<img style="width: 600px" src="assets/Contributing/PBA.png"/>

</pba-col>

<pba-col right>

- 5 Global Cohorts
- 5 Weeks, In-Person
- 8 Modules
- Per Cohort
  - ~1,000 Applicants
  - ~90 Students
  - ~50 Graduates
  - ~20 Hires

</pba-col>

</pba-cols>

https://polkadot.network/academy/

---

"I just graduated from the Polkadot Blockchain Academy…"

## _... now what?_

---

# Polkadot SDK

---

## The Mono Repo

<pba-cols>
<pba-col left>

<diagram class="mermaid">
%%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
flowchart LR
    A{Substrate} ---> D
    B{Polkadot} ---> D
    C{Cumulus} ---> D
    D{Polkadot SDK}
</diagram>

</pba-col>
<pba-col right>

- Merging into a single repository simplifies the development process
- Eliminates the need for “companion” PRs across multiple repositories.
- Improves collaboration among team members.
- Makes it easier to manage issues, pull requests, and documentation.

</pba-col>
</pba-cols>

---

## Runtime Extraction

Extraction of the various Polkadot Runtimes established decentralized ownership of this code and the network itself.

<diagram class="mermaid">
%%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
flowchart LR
    A{Polkadot-SDK} ---> B
    B{Polkadot Fellows} ---> C
    C{runtimes}
</diagram>

<pba-cols>
<pba-col left>

<pba-flex center>

Relay Chain Runtimes

- Polkadot
- Kusama

</pba-flex>

</pba-col>
<pba-col right>

<pba-flex center>

System Parachains

- Asset Hub
- Bridges Hub
- Collectives
- etc...

</pba-flex>

</pba-col>
</pba-cols>

---

## Repository Ownership

<pba-cols>
<pba-col left>

<diagram class="mermaid">
%%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
flowchart LR
    A{paritytech/polkadot-sdk}
</diagram>

<pba-flex center>

Parity Technologies

</pba-flex>

</pba-col>
<pba-col right>

<diagram class="mermaid">
%%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
flowchart LR
    A{polkadot-fellows/runtimes}
</diagram>

<pba-flex center>

The Polkadot Network via the Fellowship

</pba-flex>

</pba-col>
</pba-cols>

---

## Polkadot RFCs

- Requests for Comment (RFCs) are proposed changes to the technical implementation of the Polkadot network.
- The Polkadot Fellowship reviews and provides feedback to the RFCs.
- RFC approval is done on-chain either by the fellowship or through public referendum.
- The Polkadot Fellowship also stewards forward approved RFCs.

https://github.com/polkadot-fellows/RFCs

---

# Fellowship

---

## The Fellowship

- A technical organization that stewards the development of the Polkadot Network.
- Composed of core developers and researchers.
- Servants to the DOT holders via referendum signaling.

https://github.com/polkadot-fellows

---

### Fellowship Manifesto

<img style="width: 800px" src="assets/Contributing/Manifesto.png" />

https://github.com/polkadot-fellows/manifesto

---

### Polkadot Fellows

<img style="width: 800px" src="assets/Contributing/fellows.png" />

The current Polkadot Fellows.

---

## Ranks

<div class="grid grid-cols-3 text-small">

<div class="col-span-2">

| Dan  | Title           | Time from Dan I | Salary Factor (OECD) |
| ---- | --------------- | --------------- | -------------------- |
| n/a  | Candidate       | n/a             | 0.0                  |
| I    | Humble          | n/a             | 0.125                |
| II   | Proficient      | ~1 year         | 0.25                 |
| III  | Fellow          | ~2 years        | 1.0                  |
| IV   | Architect       | &gt; 3 years    | 1.5                  |
| V    | Architect Adept | &gt; 4 years    | 2.0                  |
| VI   | Grant Architect | &gt; 5 years    | 2.5                  |
| VII  | Free Master     | &gt; 6 years!   | 2.5                  |
| VIII | Master Constant | &gt; 11 years!  | 2.5                  |
| IX   | Grand Master    | &gt; 19 years!  | 2.5                  |

</br>

Salary RFC: https://github.com/polkadot-fellows/RFCs/pull/50/

OECD: Organisation for Economic Co-operation and Development

</div>
<div>

- The fellowship has different ranks based on their contributions to Polkadot.
- The requirements to be promoted are defined in the manifesto.
- Fellowship members can choose to accept a salary for their rank.
- Salary is based on the OECD average salary for engineers.

</div>
</div>

---

## Dan I

The requirements (condensed) to become a Dan I member of the Polkadot Fellowship are:

<pba-cols class="text-small">
<pba-col left>

- Three clear examples of a modest but substantial contribution to protocol development.
- Actively been involved in the design of a component deployed to the network.
- Substantially assisted in the analysis, or authoring of formalisation or implementation of a protocol component.
- Should be able to list all key goals, principles and tenets of Polkadot’s overall philosophy.

</pba-col>
<pba-col right>

<pba-flex left>

Possible examples of a “modest but substantial contribution” may be:

- identifying and correcting a non-trivial issue in protocol code or formalisation;
- being available and playing a crucial operational role for a network fix;
- proposing a reasonable and non-trivial protocol innovation; or
- doing a valuable, innovative and insightful refactoring or simplification.

</pba-flex>

</pba-col>
</pba-cols>

---

<img style="width: 1000px" src="assets/Contributing/omg.png" />

---

## Fellowship Clarity

- The Fellowship is a very new, and still developing organization.
- The Fellowship offers technical influence over the Polkadot Network and a regular income.
- It is attempting to provide human needs of consistent / regular income from a resilient decentralized source.
- **The barrier to entry is intentionally very high.**
- It is NOT a requirement to contribute to the Polkadot SDK or broader ecosystem.
- It is NOT the path for everyone, even not necessarily for all core developers.

---

## My recommended path to becoming a fellow…

- Join a team in the Polkadot (or broader) ecosystem aligned with the values of the fellowship.
- Work with them to split your time between their work and core contributions to the Polkadot SDK.
  - A team aligned with the values of the fellowship should be elated to have a fellow on their team.
- Navigate your way to Dan III, a clear milestone to receive a full-time living salary for your work.
  - ~3 years to learn, grow, and develop.
- If you want, transition to becoming a full time decentralized contributor to Polkadot.

---

## Issues / Ideas

---

#### **Before you can contribute**</br>you need to find an issue.

---

## A common mistake…

- Don’t come to the Polkadot SDK with brand new features that you want to implement.
- Polkadot is already a huge codebase, and does not want to maintain more code.
- As a new contributor in the ecosystem, you don’t necessarily have the context and knowledge to best understand what needs to be done, and how to integrate new features into the repo.
- The best way to start becoming an expert at Polkadot SDK (or any open source repo) is to pick up existing issues created by experts who can also mentor you.

---

## Find the right issues

_Not all issues are created equal:_

- _Difficulty:_ What level of Rust and/or Substrate expertise do you need to have to complete the issue?
- _Sensitivity:_ What parts of the codebase does the issue touch, and how could that potentially affect the network?
- _Clarity:_ How well understood is the solution to the issue?
- _Urgency:_ How quickly does this PR need to be completed?

---

#### Issue Filters: C1-Mentor, C2-Good-First-Issue, D0-Easy

<img style="width: 1000px" src="assets/Contributing/issues.png" />

---

## A Nonsensitive Entry Point to the Runtime

<img style="width: 1000px" src="assets/Contributing/example1.png" />

---

### An Issue that Rust Compiler Can Entirely Check

<img style="width: 1000px" src="assets/Contributing/example2.png" />

---

## Become a subject expert and code owner

- Training new open source contributors is very much a long term investment mindset.
- You are more likely to find issues to work on and people to mentor you if you create a focus area for your contributions.
- If you can become an expert in a topic area, you will be able to develop deeper and more relevant skills, and be able to transfer those skills to other topic areas.

---

## Keep conversations in public.

- The urge is to always try to open a direct message to quickly discuss and resolve questions.
- Culture for Polkadot is: global and async.
- Long form messages force writers to think more deeply about what is being asked and answered.
- Use DMs to ping for attention, but bring important conversations and information to the Forum, GitHub, and other public spaces.
  - You will be surprised how many lurkers use these conversations to educate themselves.

---

### Pull Requests & Reviews

---

## Breakdown Large PRs

A large PR is the easiest way to get repository owners to place your code at the bottom of their “TODO” stack. Split up large PRs into multiple parts!

- Take advantage of the Rust compiler.
  - Create a PR with just low sensitivity type changes
  - Sensitive logical changes in another PR which should be more compact to review.
  - Chain their dependencies on each other.
- Where you are refactoring many components (like pallets), do them one at a time where possible.

---

## Write Tests!

<img style="float:right;  padding-left:20px" src="assets/Contributing/boyscoutrule.png" />

- The best way for a reviewer to sanity check your work is to read and understand your tests, and verify it passes.
  - You will be surprised how even the most trivial changes can (and should) include a test!
- Try to get additional test scenarios from the original issue creator, to ensure your solution aligns with their vision of the problem.
- Write additional tests to cement your understanding of existing code, and add additional code coverage.

---

## Write Docs!

- Code always needs to be contextualized into what problem it is trying to solve and how it is choosing to solve that problem.
- Documentation in the code should allow a reader to answer what the intention of the code is.
- Documentation in the PR should guide the reviewer through the PR, highlighting the key areas of importance and the crux of the changes.

---

## Review Your Own PR

- Once you have a final PR open, and ready for review, go through and review your own PR.
- Leave comments on code that you might want to highlight for other reviewers.
  - For example sensitive changes that need extra scrutiny.
- Catch any dead code, leftover comments, todos, formatting problems, and other low hanging fruit.
  - As a reviewer, it is pretty discouraging to see the PR is not in great shape to be merged when reviewing it.

---

## My First PR 😬

<img style="width: 1000px" src="assets/Contributing/firstpr.png" />

https://github.com/paritytech/substrate/pull/868

---

## Get Paid!

<img style="width: 1000px" src="assets/Contributing/tip.png" />

---

## Expect Chaos

<img style="width: 1000px" src="assets/Contributing/visualizer.png" />

https://www.youtube.com/watch?v=aHcoFKhaUzA

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions?
