# Module 1: Cryptography

> “Cryptography rearranges power: it configures who can do what, from what”
>
> Phillip Rogaway, [The Moral Character of Cryptographic Work](https://eprint.iacr.org/2015/1162.pdf)

## Lessons Plan

This module consists of a combination of lectures, activities, and sections of the accompanying exercises to to students at what time.
As there is a large amount of variance in student experience around cryptography, we want to make sure that we have a variety of content for students of different experience levels.

### Graded Assignment

> **All graded assignments and solutions must remain private to the Academy staff, faculty, and enrolled students!**

There is a private to the Academy graded exercise, in a _template repository_ titled `pba-cryptography--master` that should be introduced after the first lectures are complete.
Instructors should \_create a per cohort, **private**, derived from the master copy, make it a template repo, and configure the cohort Github Classroom to use this new repo as an assignment.

The '`px...` assignment problem' sections indicates that students are now capable of completing them with the content so far covered.
They should be encouraged to start working on them as time allows _outside of class time!_
They should not zone-out of class to complete this during class time, they should prioritize in-class activities and exercises over working on the assignment!

### Day 1

#### Morning

1. [1-Intro](1-Intro-slides.md)
1. [2-Addresses](2-Addresses-slides.md)
   - 🪄 [`subkey` demo](./materials/subkey-demo.md)
1. `p1_hashing` assignment problem
1. `p9_attacks::low_entropy_hash` assignment problem

#### Afternoon

1. [3-Hashes](3-Hashes-slides.md)
   - 🪄 [hashing demo](./materials/hash-rust-repl-demo.md)
1. `p2_addresses` assignment problem
1. `p3_encryption` assignment problem
1. `p8_aes_modes` assignment problem for advanced students
   ☕ Break
   <!-- FIXME move to separate MONO repo for crypto or embed into book-->
1. Many time pad assignment problem

### Day 2

#### Morning

1. [4-Encryption](4-Encryption-slides.md)
1. [5-Basic_Signatures](5-Basic_Signatures-slides.md)
   - 🪄 [signature demo](./materials/signature-demo.ipynb)
1. `p4_signing` assignment problem
1. ☕ Break
1. [6-Advanced_Signatures](6-Advanced_Signatures-slides.md)
1. `p6_merkle` assignment problem

#### Afternoon

1. [7-Hash_Based_Data_Structures-slides](./7-Hash_Based_Data_Structures-slides.md)
1. `p5_data_integrity_and_recovery` assignment problem
1. `p7_exotics` assignment problem if it exists
1. ☕ Break
   <!-- FIXME move to separate MONO repo for crypto-->
1. [VRF poker](./materials/vrf_card_activity.rs)

### Day 3 (0.5 day)

#### Morning

1. [9-ZK_Proofs](9-ZK_Proofs-slides.md)

#### Afternoon

1. [8-Exotic_Primitives](8-Exotic_Primitives-slides.md)
1. [10-Cryptography_In_Context](10-Cryptography_In_Context-slides.md)
1. `p9_attacks::timing_attacks` assignment problem
