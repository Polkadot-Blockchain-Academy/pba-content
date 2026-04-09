---
title: FRAME with AI
description: Using AI tools to accelerate FRAME development
duration: 30 minutes
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# FRAME with AI

---

## Why AI for FRAME?

FRAME development involves a lot of boilerplate, macros, and patterns. AI tools can help you:

- Scaffold new pallets quickly
- Understand macro expansions
- Debug compilation errors
- Explore the polkadot-sdk codebase

---

## Tools

- Claude Code / Claude with polkadot-sdk context
- GitHub Copilot / Cursor
- Context7 MCP for up-to-date documentation

---

## What AI is Good At

- Generating pallet boilerplate (`#[pallet::config]`, `#[pallet::call]`, storage, events, errors)
- Explaining FRAME macro expansions
- Writing tests and benchmarks
- Suggesting correct trait bounds and type configurations

---

## What AI is Not Good At

- Knowing the latest API changes (always verify against the actual codebase)
- Security-critical logic (review all generated code carefully)
- Complex runtime interactions that require deep domain knowledge
- Replacing your understanding of the fundamentals

---

## Best Practices

- Always compile and test AI-generated code
- Use AI to learn patterns, then write your own code
- Ask AI to explain *why*, not just *what*
- Cross-reference suggestions with the polkadot-sdk source

---

## Live Demo

Let's build a simple pallet together using AI assistance.

---

<!-- .slide: data-background-color="#000000" -->

# Questions
