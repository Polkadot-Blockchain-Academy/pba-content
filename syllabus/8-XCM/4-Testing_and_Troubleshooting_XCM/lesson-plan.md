## Lesson 4: Testing & Troubleshooting XCM for your Parachain

> {Likely} Instructor: RT Eng.? Shawn?

### Why is this topic Important?

### Prerequisite Knowledge or Skills

- configured parachain ready for testing, not yet connected to a relay chain / simulator.

### Learning Outcome

- Launch testnets and send XCM on them in, using `polkadot-launch` {or other tooling}
-

### Learning Objectives

### Core Ideas to Convey

- Testnet configuration (mostly pre-req, refreshed)

### Activities and Exercises

- Test XCM using:
  - XCM simulator in Polkadot (no channels)
  - `polkadot-launch` and Apps UI
- Troubleshoot XCM
  - SCALE knowledge needed to inspect raw XCM bytes
  - Identify versioning issues
    - No default XCM version, so no fall-back to SEND anything from your network
- Compose XCM based on the relative Multilocations and filters & barriers (context/perspective) fro what you are trying to do.
  - Multiple routes possible, must select the best and explain
  - Give a broken XCM, and explain why it will fail (by inspection only, without executing)
