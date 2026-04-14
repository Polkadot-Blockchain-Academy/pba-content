# PBP Lisbon 2026 - Project Guidelines

Welcome to the project phase of the Polkadot **Protocol Builders Program**. This document tells you what to build, how you will be evaluated, and how to make the most of two weeks on a stack that is still being shaped.

Read it end to end before you start. If something here is unclear or contradicts what a faculty member told you, ask - this is a living document.

---

## 1. You Are Building on an ever evolving Stack (Read This First)

The Polkadot stack you will build on is ever evolving. Some of it like the Pallets are pretty battle tested, but new tech like - PVM smart contracts, Bulletin Chain, Statement Store, DotNS - is **effectively in alpha**. It is real, it works, and it is also still very much under active development. Components ship weekly. Interfaces shift. Some things you expect to work will not work, and the answer will sometimes be "nobody has tried that yet."

This is not a warning so you can lower your ambition. It is a warning so you can **reframe what success looks like**.

- **Hitting sharp edges is expected.** Things are still a work in progress.
- **Filing a precise, reproducible bug report against the stack is valued.** A good bug report can be worth as much as a working feature.
- **Submitting a PR that fixes a stack issue you hit is valued highest of all.** It is the clearest possible signal you can give us.
- **A clear "here is what I tried, here is where I got stuck, here is what I learned" beats a polished demo that hides what broke.** Your struggles are valuable signal for us.

You are helping us discover the shape of the stack at the same time as you are building on it.

### The vision you are building toward

The Polkadot stack is reaching for a **fully decentralized application stack**: on-chain logic, decentralized storage (Bulletin Chain), verifiable name resolution (DotNS), frictionless deployment and more that we won't use yet because it's still early in the making. The long-term goal is that an indie developer can ship a real product with no centralized dependencies and no gatekeepers. Your project is a concrete test of how close we are.

---

## 2. Think of This as Onboarding

There are **no grades** in PBP. Think of this program as your **2-week onboarding at Parity**.

You have Parity engineers on staff the entire time as mentors. You are given real tools, real code, and the freedom to build anything. At the end, your project is reviewed directly by Parity for hiring consideration.

The question we are asking: _"Would I want this person on my team?"_

What reviewers will look for:

- **Self-starters.** You don't wait to be told what to do. You explore, you try things, you ask when stuck.
- **Builders with taste.** Did you choose a problem worth solving? Does the project take advantage of what Web3 actually provides?
- **Stack improvers.** You find a bug? You open an issue. You can fix it? You open a PR. This is the strongest signal you can give us.
- **Collaborators.** You help others, share what you learn, and make the people around you better.
- **Shipping discipline.** Does the repo run? Is the README clear about what works and what doesn't?
- **Code quality.** Readable code. Meaningful commits. No dead code or leftover scaffolding.
- **Active communicators.** You say what works, what doesn't, and what you'd do differently.

---

## 3. What You Will Build (Hard Requirements)

Every student ships, **solo**, a project with exactly these components:

### 3.1 One backend component (pick one)

- **Substrate Pallet**
- **Solidity smart contract on EVM**
- **Solidity smart contract on PVM**
- **Rust smart contract on PVM**

### 3.2 One frontend component (pick one)

- **Static web app**.
- **CLI++** - a command-line frontend, but with ambition. A bare `clap`-style CLI on its own is **not enough** for this rubric. To count as CLI++, you must elevate it into one of:
  - a **TUI** (ratatui, textual, ink, etc.), or
  - an **MCP server** (yes, this counts as a frontend - it is how an AI agent "drives" your backend; this category is hot right now and reviewers are paying attention to it).

### 3.3 Mandatory integration

- **Deploy to Paseo with Bulletin Chain + DotNS.** Every project ships a web presence hosted on Bulletin Chain and reachable via a DotNS name on the **Paseo** testnet. For web-app frontends, this _is_ your app. For CLI++ frontends (TUI or MCP server), you still ship a web presence - at minimum a product / landing page describing what the project does, how to install it, and how to use it. No project is exempt. The point is that every student touches the decentralized deploy path end-to-end.

### 3.4 Note on parachain projects

If you are building a **pallet** (custom parachain runtime), you do **not** need to launch a public parachain or acquire coretime. Your parachain should work locally with **zombienet** — that is sufficient for the demo. Your frontend still needs to be deployed on Paseo via Bulletin Chain + DotNS (section 3.3), and should connect to your local zombienet for the demo.

### 3.5 Scope rule

The backend + frontend combo is a **strict requirement**. Pure-infra projects (an indexer, a bridge, a standalone library) do not satisfy the rubric on their own. If you think you have a good reason to deviate, talk to faculty _before_ you start building.

---

## 4. Path Certainty Matrix (Advisory)

The stack is not equally solid in every corner. You get to pick any valid combination of backend and frontend - this matrix tells you how likely it is to actually work end-to-end in two weeks.

| Backend                    | Frontend    | Confidence it ships | Notes                                                                                                                                                |
| -------------------------- | ----------- | ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pallet                     | Web / CLI++ | **~100%**           | Well-trodden path, full curriculum support, plenty of prior work to crib from.                                                                       |
| Solidity on EVM            | Web / CLI++ | **~90%**            | Mature tooling, standard ecosystem, mostly-solved problems. Only issues will be if your contracts use really advanced gas optimisation/manipulation. |
| Solidity on PVM            | Web / CLI++ | **~70%**            | Expect toolchain quirks. It is technically more optimized than EVM but the compiler struggles with big/complex contracts.                            |
| Rust smart contract on PVM | Web / CLI++ | **~50%**            | Bleeding edge. Real chance you hit walls nobody has hit yet. Its still early in its development and not many people have built with it.              |

**Read this carefully:**

- These numbers are **advisory**, not prescriptive. You may pick any row. No faculty sign-off is required for the riskier paths - but we _strongly_ recommend talking to us if you go for one.
- Riskier paths are **higher ceiling**. If you ship a Rust-contract-on-PVM project that actually works, it will help us making sure that the stack works and is getting more mature.
- **Getting stuck is not failure as long as you document it well.** A Rust-on-PVM project that couldn't fully ship but contains two solid bug reports and a PR against the stack is a legitimate deliverable. Plan for that possibility if you pick a risky path. We truly value help in improving the stack.

---

## 5. Bonus Dimensions

These are the things that raise your ceiling beyond "built the required thing." None are mandatory. All are noticed.

- **Sensible use of Bulletin Chain / Statement Store in the project itself** - not just as website hosting. If storage or statements are part of your product's logic, that counts.
- **High-quality bug reports** filed against the stack while you build. Precise, reproducible, with minimal repro code.
- **PRs to fix stack issues** you hit. Even small ones. This is the strongest possible signal to reviewers.
- **Protocol ports** - take something interesting from another chain (x402 payments, encrypted/anonymous chat, decentralized file hosting, something else) and bring it to Polkadot.
- **Projects that gesture at Parity's broader vision** - games, decentralized collaboration tools, things that feel like the stack growing into its promise.
- **Any tooling to help others build projects** - skills.md files or any other tool reusable by other people to make them ship faster is always appreciated.

---

## 6. AI Usage Policy ("No Slop")

**TL;DR: AI is allowed and encouraged. You own every line you ship. "The AI wrote it" is not a defense at demo.**

We expect most of you to use AI tools heavily. Claude, Cursor, Copilot, Codex, whatever. That is fine - great, even. AI is part of how software is built in 2026, and using it well is a real skill. This section is about using it well.

### 6.1 Core stance

You are the engineer. The AI is a tool. The fact that a tool produced the code does not transfer responsibility for it - you are still on the hook for:

- Does it work?
- Do you understand it?
- Is it correct _in your project's context_?
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
- Accepting an AI fix "because it compiled" without understanding _why_ it compiled.
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

At your 5-minute demo, we may ask you: _"walk me through this file - why is it structured this way?"_ You need to be able to answer that question about any file in your repo. If you cannot, you did not ship it - the AI did, and you just pushed it.

### 6.6 AI in ReadMe

The "no slop" rule is not just about code. It applies even harder - to prose.
In particular to your README. AI-generated prose is faster to produce than code, harder to spot at a glance, and ten times more likely to make it through unread.

A few things to internalise:

- **Humans read every word you ship.** AI generates them in seconds; we read them in minutes. A 4000-word README that nobody asked for is not "thorough" - it is wasting reviewer time. Length is not a virtue. Density and clarity are.
- **A wall of confident-sounding text is slop, even if every sentence is technically correct.** Reviewers can feel the difference between a README written by someone who built the thing and one assembled out of prompts. So can your peers.
- **Rule of thumb for your README: 90% you, 10% AI.** Use AI for grammar, spelling, formatting cleanup, maybe rephrasing a clunky paragraph. Do **not** use it to draft the document. The structure, the framing, the choices about what to include and what to cut - those need to be yours. Same for the retrospective.
- **If you expect us to read it, you should have read it yourself. Twice.** End to end. Out loud, even. If you can't be bothered, neither can we, and we will notice.

---

## 7. Deliverables Checklist

At the end of the program you hand in:

- [ ] **GitHub repo** with working code (backend + frontend)
- [ ] **Deployed on Paseo** — live and reachable at your .dot name via Bulletin Chain
- [ ] **README** covering:
  - [ ] What the project does
  - [ ] Which path you picked (backend + frontend)
  - [ ] How to run it end-to-end
  - [ ] What works and what doesn't
  - [ ] Known limitations and design compromises
  - [ ] URL of your decentralized deployment (.dot name)
- [ ] **Meaningful commit history** (no `fix fix fix wip more`)
- [ ] **Working demo** — live preferred, recorded fallback if live is risky (see section 9)
- [ ] **5-minute pitch** with slides on pitch day (see section 9)
- [ ] **Retrospective** using [`retrospective-template.md`](./retrospective-template.md) — what worked, what broke, and what you'd tell Parity about the stack. Reviewers read this carefully.

---

## 8. Before You Submit

A final pass before you push your last commit. None of this is busywork - every item here is a thing reviewers will notice if you skip it. Run this list at end-of-day on day 12, not the morning of pitch day.

### 8.1 Code

- **It builds.** Clean build, no errors, no warnings. If you have warnings you can't kill, document why in the README.
- **Tests pass.** The full suite, not just the one you wrote last.
- **No dead code.** Remove unused functions, storage items, events, errors, scaffolding from earlier directions you abandoned.
- **No "example" cruft.** If you started from a template, strip the parts you didn't use.
- **Formatted.** Run your language's standard formatter (`cargo +nightly fmt`, `prettier`, `gofmt`, whatever applies).
- **No code that can panic on the happy path.** Tests and benchmarks are fine. Production paths are not.
- **Documented where it isn't obvious.** Comments explain _why_, not _what_. Note the compromises and TODOs you'd revisit.

### 8.2 Repo

- **Meaningful git history.** Reviewers will read your commits. `fix`, `wip`, `more` ten times in a row is a red flag (see section 6.4).
- **README at the repo root**, covering everything in the section 7 checklist. Include the Bulletin Chain / DotNS URL so reviewers can click straight through.
- **Retrospective filled in** using [`retrospective-template.md`](./retrospective-template.md). Don't leave this for the morning of pitch day.

### 8.3 Deployment

- **Decentralized deploy is live and reachable** on Paseo at your DotNS name (see section 3.3). If it isn't, reviewers can't see your work. Test it from a fresh browser session - not the one you've been developing in.

### 8.4 Demo

- **Live demo dry-run.** Run it end-to-end at least once on the network you'll use on pitch day. Have a recorded fallback ready in case the live version flakes (see section 9).

If any of these is "I'll get to it tomorrow" the night before pitch day, you've already lost time.

---

## 9. Pitch Day (The 5-Minute Presentation)

On the final day of the program, every student gives a short presentation. This is not a formality - it is how reviewers (and your peers) first encounter your work. Treat it as the front door to your project.

### 9.1 Format

- **5 minutes** for your pitch. Strict. Practice with a timer.
- **Up to 2 extra minutes** afterward for questions from faculty and fellow students.
- **Slides are expected.** A simple deck is fine - readable, focused, not overstuffed. No template is mandated; use whatever you like.
- **A demo is expected.** Live is preferred. A recorded fallback is fine (and smart) if going live would be fragile - testnet flakiness, network issues, hardware quirks. Many strong pitches mix both.

### 9.2 What to cover (roughly in this order)

1. **The product.** What is it? In one sentence, then show us.
2. **Why it's interesting / relevant.** Why did you pick this problem? Who is it for? Why does it belong on the Polkadot stack specifically?
3. **The demo.** Walk through the thing working end-to-end - backend, frontend, decentralized deploy. This is the heart of the pitch. Spend time here.
4. **Learnings, problems, and what broke.** This is the part Parity cares most about. Sharp edges you hit on the stack, bug reports you filed, things that surprised you, things you'd do differently. Reviewers value this section more than you'd guess.
5. **Next steps, future ideas, and feedback to the stack.** Where would you take this next? What's still rough? What should Parity know about the stack based on what you went through?

### 9.3 Tone

Short, sharp, direct. The worst pitches hide what broke and pad the demo with filler. The best ones leave a reviewer thinking _"I want to clone that repo and run it."_ Aim for that.

---

## 10. Timeline

Project time runs during the final portion of the 2-week PBP program. Exact dates and daily schedule are in the cohort's [`syllabus/`](../../syllabus/) schedule.

Rough shape:

- **Week 1**: lectures, exercises, pick your idea, talk to faculty, start scaffolding.
- **End of week 1**: project kickoff - you should be building in earnest.
- **Week 2**: build, break, report, fix.
- **Final day**: pitch day.

---

## 11. Process & Support

- **Parity engineers and faculty are here as mentors** throughout project time. Use them.
- **Ask early, not late.** If you're stuck on something small, ask for help after ~30 minutes. Time is limited and mentors know tricks that commonly unblock people in seconds. "I've been stuck for 2 days" is a missed opportunity — don't let that happen.
- **Project ideas**: see [`../ideas.md`](../ideas.md) for a living idea bank. You are free to invent your own, or pick from the bank and make it yours.
- **Bug reports against the stack**: faculty will point you at the right repos (Substrate, PVM, Bulletin Chain, Polkadot Desktop) during week 1. When in doubt, ask before filing.

---

## 12. A Final Note

You are among the first cohorts building on the decentralized Polkadot app stack as a unified thing. A lot of what you find - good and bad - will directly shape how Parity builds this stack for the next few years. Take that seriously. Build something you would actually want to exist. Document what you found — good and bad. Have fun.

Good luck.
