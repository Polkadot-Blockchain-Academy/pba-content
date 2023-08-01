---
title: Cumulus Deep Dive
description: Cumulus, Transforming Solo to Parachain
duration: 1.5 hours
---

# Cumulus Deep Dive

Notes:

Hi again everyone. Hope the coffee break was sufficient to clear your heads of availability cores

---

### Outline

<pba-flex center>

1. [What is Cumulus?]()
1. [Cumulus and Para-Relay Communication]()
1. [How Cumulus Enables Runtime Validation]()
1. [Cumulus in the Parachain Runtime]()
1. [Cumulus in the Parachain Client]()
1. [Collation Generation Subsystem]()
1. [Runtime Upgrades]()

</pba-flex> 

---

## What is Cumulus

---

## Cumulus' Key Processes

- Collation generation and announcement
- Follow relay "new best head" to update para "new best head"
- Follow relay finalized block to update para finalized block
- Request parablocks not shared by peers from relay (data recovery)

Notes:

New best head means a new block that is at the head of the fork most preferred by BABE

---

## Cumulus and Para-Relay Communication

---

## Collations

---

<!-- .slide: data-background-color="#4A2439" -->

## Questions

---

## References

1. https://github.com/paritytech/cumulus/blob/master/docs/overview.md