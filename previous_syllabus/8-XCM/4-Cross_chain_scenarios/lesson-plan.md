## Lesson 3: Parachain Configuration for XCM

> {Likely} Instructor: RT Eng.? Shawn?

### Why is this topic Important?

### Prerequisite Knowledge or Skills

### Learning Outcome

- Understand and correctly configure XCM (pallet, executor, ...) for a parachain (executor and pallet) based on business needs
- Barriers & filters
- Asset transactors

### Learning Objectives

### Core Ideas to Convey

- XCM builder module (makes configuration easier)
- Barriers & filters
- Asset transactors
- Weight Traders
- wildcards? {maybe out of scope}
- pallet-xcm

### Activities and Exercises

- Versioning exercise to understand how versioning works
- Modify different parts of the configuration to understand their implications (in groups).
  Each group picks one topic
  - Remove free execution from relay and add a trader to charge for fee
  - Add pallet-assets to the parachain and add an asset transactor for it.
  - Modify your chain to be a 20 byte account instead of a 32 byte account.
  - Change the configuration to accept teleporting instead of reserve transfer assets.
