### Why is this topic Important?

- Learners will need to understand Polkadot and the difference between a parachain and a stand-alone chain. How does Polkadot work on a conceptual level?

### Prerequisite Knowledge or Skills

- Blockain concepts
- Substrate conceptually, and some experience in codebase
  - FRAME understand what pallets are but not how they are constructed (yet) DAN TO CHECK ON STATUS WITH FRAME TEAM
- Substrate Forkless upgrades conceptually

### Learning Outcome

- Learners will be able to identify the different parts of the Polkadot architecture and how they work together to create a system with particular benefits and constraints.

### Learning Objectives

-

### Core Ideas to Convey

- ## How Polkadot works on a conceptual level
- What are parachains?

  - The difference between a solo chain and a parachain
  - Main difference is consensus for a parachain is coming from the relay chain
  - A parachain is more restricted than a general Substrate solo-chain
    - Block (size & time) are limited
    - Explicitly use Cumulus (next lesson) for finality
    - Block authoring logic is up to the parachain to define
    - Block validation must be expressible in self contained way
  - Collators vs. Validators
    - In parachain development, you're creating a collator.
    - Compare and contrast the benefits of being a collator vs. being a validator.
  - Shared Security (what's needed to be a parachain)

- Governance considerations

  - **{Raul should be invited to discuss this and write content, perhaps a 1 hour history lesson and overview of how things actually are done, with examples}** Governance: How does Polkadot think about governance? Polkadot is a substrate blockchain that has these particular modules:
    - About the governance of Polkadot; forkless upgrades.
    - Voting rights in Polkadot: What does it mean that each chain has governance? What is governance?
    - Forkless Upgradeability; what are the implications of this? Bad governance can get chains into a bad state.
    - How did Polkadot evolve into what we have today?
      - History of Polkadot (pseudo-chain that upgraded itself to be a governed chain with new parachain logic)

- The relationship between Polkadot and Substrate

### Activities and Exercises

- Label the Polkadot network diagram with all the parts. Describe how Polkadot works with this diagram. (To link here)

---

### Core Ideas to Convey

- How Polkadot works on a conceptual level
- The relationship between Polkadot, Cumulus, Substrate and other elements
- Layer 0
- What are parachains?
- Collators vs. Validators
  - In parachain development, you're creating a collator.
  - Compare and contrast the benefits of being a collator vs. being a validator.
- The difference between a solo chain and a parachain
  - Main difference is consensus for a parachain is coming from the relay chain
  - Constraints: For a parachain, there are a lot of things you can get from the relay chain. But there are also constraints that make it a different use case; size of the proof and being able to validate a proof without accessing anything from the **(didn't hear)** side (self-contained proof).
  - A parachain is tightly confined and will break if certain
    - Don't get to play with consensus as much.
- Broad relationship between relay chains and parachains

  - Shared Security (what's needed to be a parachain)
  - How do you ensure your validation is correct without Cumulus?
  - **{Raul should be invited to discuss this, perhaps a 1 hour history lesson and overview of how things actually are done, with examples}** Governance: How does Polkadot think about governance? Polkadot is a substrate blockchain that has these particular modules:
    - About the governance of Polkadot; forkless upgrades.
    - Voting rights in Polkadot: What does it mean that each chain has governance? What is governance?
    - Forkless Upgradability; what are the implications of this? Bad governance can get chains into a bad state.
    - How did Polkadot evolve into what we have today?
      - History of Polkadot (pseudochain that upgraded itself to be a governed chain with new parachain logic)

- What pallets are used in Polkadot? - ..
