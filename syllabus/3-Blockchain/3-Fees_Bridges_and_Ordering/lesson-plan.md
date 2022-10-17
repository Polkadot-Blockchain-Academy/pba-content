# Lesson 3: Fees, Ordering, and Bridges

## Why is this topic Important?

Almost all blockchains have fees and are single-threaded (thus demanding a linear execution order). Likewise, multiple blockchains exist that may interact with each other over bridges. All of these come with security implications and tradeoffs.

## Core Ideas to Convey

- Fees & Ordering
  - What are our main constraints?
    - Execution time
    - Runtime is single threaded (for now)
    - Lots of people might want to use this state machine
  - We need to decide,
    - Which transactions to include
    - What order to arrange them
- Fees
  - What do you charge for?
    - In Bitcoin, block space only
    - In Ethereum, "gas", i.e. per instruction
    - In Substrate, time and space
  - Smart Contracts & Gas
    - Just set a fee for each instruction in the VM
    - Contracts can compose the lower level VM into applications
    - Users call into these contracts, and pay for each instruction that actually gets executed
    - Specify a max gas and a max cost per gas unit
    - If they run out of gas, then execution halts and any state changes are reverted (atomic)
    - Discussion with students about the pros and cons of this
  - Weight
    - More general metering
    - SC aside, used when the STF does not allow permissionless deployment of instructions
    - The actual metering can happen pre-runtime
    - Some primitive calculations can happen at runtime (e.g. max length of some array)
    - Should be easy to compute in order to decide whether or not to dispatch
    - Fee is taken upfront, once a call is dispatched there is no safety belt that halts execution
  - Fee strategies
    - Greedy (just offer block producers some amount)
    - Substrate/EIP1559: fixed fee + tip
- Ordering
  - A block has limited space
    - In Substrate, 1/3 the block _time_
      - 1/3 for author execution
      - 1/3 for network propagation
      - 1/3 for validation
    - In Ethereum, some gas limit
  - What should an author do when they have too many transactions?
    - Just take the highest fee
    - Prioritize by highest fee / weight ratio
  - Attacks
    - Transactions are open instructions, anyone can decode them
    - Being permissionless and all, block authors may have accounts on chain too
    - When an author sees others' transactions, they may act too
    - Frontrunning, sandwitching, etc.
- Transactional Systems
  - Most blockchains are transactional, they need to be "woken up"
  - Substrate provides certain hooks
    - On Initialize, On Finalize, etc.
    - These are part of the STF, so authors have no choice but to include them
    - Very powerful primitive that developers can harness to remove authority power
- Bridges
  - Transport layer between independent consensus systems
  - Trust implications
    - What if one chain is attacked?
    - How to handle messages?
  - Best case: Don't use bridges, they are very attackable and have inverse security properties
    - That is, the more value on them, the more at risk they are
    - This is in contrast to, e.g. PoS, where more value at stake makes the system more secure
  - OK case: On-chain light client
    - Quick discussion on what a light client is
    - Should be able to "know" when changes on another chain are final
  - Worst case: Notary/collatoral based system
    - Sometimes the only way to do it.

## Learning Outcome

After this lecture, students should be able to evaluate extrinsic ordering strategies and how to interpret messages from other consensus systems.

## Learning Objectives

- Explain how extrinsics are ordered within a block
- Describe fee-based and fee-less prioritization methods
- Explain the risks and assumptions when interacting with external consensus systems

## Prerequisite Knowledge or Skills

- Knowledge of block authorship and structure (what goes into a block)
- Concept of separate consensus systems

## Activities and Exercises

- Joshy TODO
