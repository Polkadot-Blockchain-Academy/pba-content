# PBP Lisbon 2026 - Project Guidelines

Welcome to the project phase of the Polkadot **Protocol Builders Program**. This document tells you what to build, how you will be evaluated, and how to make the most of two weeks on a stack that is still being shaped.

Read it end to end before you start. If something here is unclear or contradicts what a faculty member told you, ask - this is a living document.

---

## 1. You Are Building on an Alpha Stack (Read This First)

The Polkadot stack you will build on - Pallets, PVM smart contracts, Bulletin Chain, Statement Store, DotNS, Polkadot Desktop - is **effectively in alpha**. It is real, it works, and it is also still very much under active development. Components ship weekly. Interfaces shift. Some things you expect to work will not work, and the answer will sometimes be "nobody has tried that yet."

This is not a warning so you can lower your ambition. It is a warning so you can **reframe what success looks like**.

- **Hitting sharp edges is expected.** It is not a failure mode, it is a data point.
- **Filing a precise, reproducible bug report against the stack is valued.** A good bug report can be worth as much as a working feature.
- **Submitting a PR that fixes a stack issue you hit is valued highest of all.** It is the clearest possible signal you can give us.
- **Honest "here is what I tried, here is where I got stuck, here is what I learned" beats a polished demo that hides what broke.**

You are helping us discover the shape of the stack at the same time as you are building on it. That is the deal. Take it seriously and it will be one of the best things on your CV.

### The vision you are building toward

The Polkadot stack is reaching for a **fully decentralized application stack**: on-chain logic, decentralized storage (Bulletin Chain), verifiable name resolution (DotNS), local-first tooling (Polkadot Desktop), and frictionless deployment. The long-term goal is that an indie developer can ship a real product with no centralized dependencies and no gatekeepers. Your project is a concrete test of how close we are.

---

## 2. Hiring Reality Check

There are **no grades** in PBP. Instead, **your project will be reviewed directly by Parity for hiring consideration**.

This matters more than a grade would. A grade is a line in a transcript. A project is a thing a hiring manager can read, run, and argue about. Some of you will be talking to Parity about roles because of what you ship here.

What reviewers will look for:

- **Taste.** Did you choose a problem worth solving? Is your scope sane?
- **Shipping discipline.** Does the repo run? Is the README honest about what works and what doesn't?
- **Judgment under uncertainty.** When you got stuck, how did you reason about it? Did you pick a good next move?
- **Honesty.** Hiding what broke is worse than reporting it clearly.
- **Cleanliness.** Readable code. Meaningful commits. Not a dump.
- **Voice.** We can tell when a README was written by a human who understood the project. We can also tell when it wasn't.

---

## 3. What You Will Build (Hard Requirements)

Every student ships, **solo**, a project with exactly these components:

### 3.1 One backend component (pick one)

- **Substrate Pallet**
- **Solidity smart contract on EVM**
- **Solidity smart contract on PVM**
- **Rust smart contract on PVM**

### 3.2 One frontend component (pick one)

- **Web app** (React, Svelte, Vue, whatever).
- **CLI++** - a command-line frontend, but with ambition. A bare `clap`-style CLI on its own is **not enough** for this rubric. To count as CLI++, you must elevate it into one of:
  - a **TUI** (ratatui, textual, ink, etc.), or
  - an **MCP server** (yes, this counts as a frontend - it is how an AI agent "drives" your backend; this category is hot right now and reviewers are paying attention to it).

### 3.3 Mandatory integration

- **Web deploy on Bulletin Chain + DotNS.** Every project ships a web presence hosted on Bulletin Chain and reachable via a DotNS name. For web-app frontends, this *is* your app. For CLI++ frontends (TUI or MCP server), you still ship a web presence - at minimum a product / landing page describing what the project does, how to install it, and how to use it. No project is exempt. The point is that every student touches the decentralized deploy path end-to-end.

### 3.4 Scope rule

The backend + frontend combo is a **strict requirement**, not a default. Pure-infra projects (an indexer, a bridge, a standalone library) do not satisfy the rubric on their own. If you think you have a good reason to deviate, talk to faculty *before* you start building.

---

## 4. Path Certainty Matrix (Advisory)

The stack is not equally solid in every corner. You get to pick any valid combination of backend and frontend - this matrix tells you how likely it is to actually work end-to-end in two weeks.

| Backend | Frontend | Confidence it ships | Notes |
|---|---|---|---|
| Pallet | Web / CLI++ | **~100%** | Well-trodden path, full curriculum support, plenty of prior work to crib from. |
| Solidity on EVM | Web / CLI++ | **~90%** | Mature tooling, standard ecosystem, mostly-solved problems. Only issues will be if your contracts use really advanced gas optimisation/manipulation. |
| Solidity on PVM | Web / CLI++ | **~70%** | Expect toolchain quirks. It is technically more optimized than EVM but the compiler struggles with big/complex contracts. |
| Rust smart contract on PVM | Web / CLI++ | **~50%** | Bleeding edge. Real chance you hit walls nobody has hit yet. Its still early in its development. |

**Read this carefully:**

- These numbers are **advisory**, not prescriptive. You may pick any row. No faculty sign-off is required for the riskier paths - but we *strongly* recommend talking to us if you go for one.
- Riskier paths are **higher ceiling**. If you ship a Rust-contract-on-PVM project that actually works, reviewers will notice. Hard.
- Riskier paths are **higher floor risk**. If you choose the bottom row and get stuck for five days, that is a real outcome - you need to decide whether that trade is worth it for *you*.
- **Getting stuck is not failure as long as you document it well.** A Rust-on-PVM project that couldn't fully ship but contains two solid bug reports and a PR against the stack is a legitimate deliverable. Plan for that possibility if you pick a risky path.

---

## 5. Bonus Dimensions

These are the things that raise your ceiling beyond "built the required thing." None are mandatory. All are noticed.

- **Sensible use of Bulletin Chain / Statement Store in the project itself** - not just as website hosting. If storage or statements are part of your product's logic, that counts.
- **High-quality bug reports** filed against the stack while you build. Precise, reproducible, with minimal repro code.
- **PRs to fix stack issues** you hit. Even small ones. This is the strongest possible signal to reviewers.
- **Protocol ports** - take something interesting from another chain (x402 payments, encrypted/anonymous chat, decentralized file hosting, something else) and bring it to Polkadot.
- **Projects that gesture at Parity's broader vision** - games, decentralized collaboration tools, things that feel like the stack growing into its promise.

---

## 6. AI Usage Policy ("No Slop")

**TL;DR: AI is allowed and encouraged. You own every line you ship. "The AI wrote it" is not a defense at demo.**

We expect most of you to use AI tools heavily. Claude, Cursor, Copilot, Codex, whatever. That is fine - great, even. AI is part of how software is built in 2026, and using it well is a real skill. This section is about using it well.

### 6.1 Core stance

You are the engineer. The AI is a tool. The fact that a tool produced the code does not transfer responsibility for it - you are still on the hook for:

- Does it work?
- Do you understand it?
- Is it correct *in your project's context*?
- Can you explain it to a reviewer at demo?

If the answer to any of those is "no," do not commit it.

### 6.2 Responsibilities

- **Read every diff before you commit it.** Every line.
- **Test what you generate.** If you can't explain it, you can't commit it.
- **Prefer small verified steps to large unreviewed dumps.** A 50-line AI-generated patch you understand is worth more than a 500-line one you don't.

### 6.3 Good vs. bad examples

**Good:**
- "I used Claude to scaffold the XCM message format, then manually verified the encoding against the spec and added a round-trip test."
- "My MCP server handler was AI-drafted. I rewrote the auth path after reading how the session store actually works."
- "I let Cursor generate boilerplate for the React forms, then hand-wrote the signing flow because the generated version called an API that doesn't exist."
- "Couldn't get the contract compiling. Had Claude explain three possible causes, tested each, found the real one was a linker flag."

**Bad:**
- Copy-pasting 400 lines of generated pallet code, not running it, and watching it break at demo.
- Commit message: `fix stuff` on a 2000-line AI diff.
- Accepting an AI fix "because it compiled" without understanding *why* it compiled.
- Generated tests that assert `true == true` and exercise nothing.

### 6.4 Red flags reviewers will catch

These are the tells. Do not let them show up in your repo.

- **Hallucinated API calls** - functions, flags, or crate methods that do not exist.
- **Inconsistent naming across files** - `userId` in one file, `user_id` in the next, `uid` in a third, because different prompts produced different conventions.
- **Code that doesn't compile** but was committed anyway.
- **Tests that don't test** - fixtures with no assertions, or assertions that tautologically hold.
- **Stale comments** describing code that was later rewritten.
- **Commits with no narrative** - ten commits in a row titled `update`, `fix`, `more`, `wip`.

### 6.5 What "ownership" means in practice

At your 5-minute demo, we may ask you: *"walk me through this file - why is it structured this way?"* You need to be able to answer that question about any file in your repo. If you cannot, you did not ship it - the AI did, and you just pushed it.

---

## 7. Deliverables

At the end of the program you hand in:

1. **A GitHub repo** with:
   - Working code (backend + frontend).
   - A README covering: what the project does, which path you picked, how to run it, what works, what doesn't, known limitations.
   - Meaningful commit history.
2. **A working demo.** Live is preferred. A recorded fallback is always nice in case live is unsafe (e.g. testnet flakiness).
3. **A 5-minute pitch / demo presentation** on pitch day. See section 8 for what this should look like.
4. **A retrospective write-up** using [`retrospective-template.md`](./retrospective-template.md). This is the short post-mortem where you tell us what worked, what broke, and - most importantly - **what you'd tell the Parity team about the stack based on what you hit**. Reviewers read this carefully. It is not busywork.

---

## 8. Pitch Day (The 5-Minute Presentation)

On the final day of the program, every student gives a short presentation. This is not a formality - it is how reviewers (and your peers) first encounter your work. Treat it as the front door to your project.

### 8.1 Format

- **5 minutes** for your pitch. Strict. Practice with a timer.
- **Up to 2 extra minutes** afterward for questions from faculty and fellow students.
- **Slides are expected.** A simple deck is fine - readable, focused, not overstuffed. No template is mandated; use whatever you like.
- **A demo is expected.** Live is preferred. A recorded fallback is fine (and smart) if going live would be fragile - testnet flakiness, network issues, hardware quirks. Many strong pitches mix both.

### 8.2 What to cover (roughly in this order)

1. **The product.** What is it? In one sentence, then show us.
2. **Why it's interesting / relevant.** Why did you pick this problem? Who is it for? Why does it belong on the Polkadot stack specifically?
3. **The demo.** Walk through the thing working end-to-end - backend, frontend, decentralized deploy. This is the heart of the pitch. Spend time here.
4. **Learnings, problems, and what broke.** The honest part. Sharp edges you hit on the stack, bug reports you filed, things that surprised you, things you'd do differently. Reviewers value this section more than you'd guess.
5. **Next steps, future ideas, and feedback to the stack.** Where would you take this next? What's still rough? What should Parity know about the stack based on what you went through?

### 8.3 Tone

Short, sharp, honest. The worst pitches hide what broke and pad the demo with filler. The best ones leave a reviewer thinking *"I want to clone that repo and run it."* Aim for that.

---

## 9. Timeline

Project time runs during the final portion of the 2-week PBP program. Exact dates and daily schedule are in the cohort's [`syllabus/`](../../syllabus/) schedule.

Rough shape:

- **Week 1**: lectures, exercises, pick your idea, talk to faculty, start scaffolding.
- **End of week 1**: project kickoff - you should be building in earnest.
- **Week 2**: build, break, report, fix.
- **Final day**: pitch day.

**Plan your scope for two weeks, not four.** It is far better to ship a small thing that works than a large thing that half-works.

---

## 10. Process & Support

- **Office hours with faculty** are available throughout project time. Use them.
- **If you are on a risky path and stuck, come talk to us early.** "I am 48 hours into this and I don't know if it's possible" is a normal conversation to have. Waiting until day 10 to raise it is not.
- **Project ideas**: see [`../ideas.md`](../ideas.md) for a living idea bank. You are free to invent your own, or pick from the bank and make it yours.
- **Bug reports against the stack**: faculty will point you at the right repos (Substrate, PVM, Bulletin Chain, Polkadot Desktop) during week 1. When in doubt, ask before filing.

---

## 11. A Final Note

You are among the first cohorts building on the decentralized Polkadot app stack as a unified thing. A lot of what you find - good and bad - will directly shape how Parity builds this stack for the next few years. Take that seriously. Build something you would actually want to exist. Be honest about what happened. Have fun.

Good luck.
