# Lesson 6: Consensus (Babe, AURA, GRANDPA) - Block authoring and finality, along with how Substrate uses Keys and certificates

## Why is this topic Important?

- Knowing how to overcome consensus based issues that may come as you work with Substrate development.

## Core Ideas to Convey

- Block authorship
    - Aura
        - Round robin
        - Validator set
    - Babe
        - 2 layers
            - Random selection
            - Round robin
        - VRF / randomness
    - Proof of Work (contrasting)
    - InstantSeal as a testing tool
    - manualSeal as a testing tool
- Block finality 
    - Probabilistic
        - What you see already in PoW w/ ETH and BTC
    - "Deterministic"
        - Grandpa
            - Assumes 2/3s honest
        - There is no such thing as true determinism, just that people will get really fucked if they go back.
            - Over 66% of all tokens getting slashed
- Conflict resolution / Fork Rules
- Split between runtime and client of consensus
- Common Developer Interactions
    - Key Management
    - Session activity
        - Not producing any blocks in an session leads to unexpected epoch error
    - Finality Stalls
        - More than 1/3rd offline
    - Minimum validator requirements
    - Node Template vs Node vs Parachain Template

## Prerequisite Knowledge or Skills

- High Level Consensus Knowledge
    - Understand PoW and PoS at a high level
    - Understand longest chain rules
- Key Signing
- VRF

## Learning Outcome

Understand how block production and finalization works in Substrate as well as how to overcome/debug common issues.

## Learning Objectives

- Describe the role of Aura, Grandpa and Babe in reaching consensus, and how they work.
- Explain the key differences between BABE and AURA.
- Explain why block production and block finalization are split
- Describe a potential attacks on the different components above.
- Describe a potential developer errors on the different components above.
- Explain the interaction between the runtime and client-side code for consensus (for example how things are communicated from runtime to client through digests).
- Explain how BABE generates secure randomness (and what it is useful for)

## Activities and Exercises

- Have developers run into each error that was explained above, and resolve it.
- Given a chain of blocks and votes, draw a diagram to illustrate what the final canonical chain will be.
- Have users do the key management steps to setup basic block authoring
- After class: complete the set up a alice bob network tutorial