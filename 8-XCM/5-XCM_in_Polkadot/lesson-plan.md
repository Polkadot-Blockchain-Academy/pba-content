## Lesson 5: XCM in Polkadot Context

> Instructors: Rob?, RT Eng., Guest from Active Parachain?

### Why is this topic Important?

### Prerequisite Knowledge or Skills

- Use `polkadot-launch` to setup a relay chain an a few parachains with channels opened between them.
- SCALE serde, for troubleshooting activity

### Learning Outcome

### Learning Objectives

- Connect channels between chains
    - negotiation with other parachain (propose/accept cycle)
    - Shared 
- Send XCM for Polkadot and it's parachains

### Core Ideas to Convey

- Understand what XCM to compose based on the relative Multilocations and filters & barriers (Context or perspective)
- Understand the xcm-pallet and xcm-executor and how they integrate into a runtime
    - xcm-pallet is the “convenience” set of tools and extrinsics to interact with the xcm-executor
- Inspect a MultiLocation’s XCM config (runtime source, and to query RPC without it {not yet implemented?})
- Understand the consequences of runtime upgrades with respect to XCM:
  - Changing the XCM config
  - Pallet index needs to be immutable or you need to have migrations-like custom XCM executor to handle → VMP handler config {versioned by runtime}
  - Relative path migrations & conversions (integration testing needed)

### Activities

- Setup channel game?
    - need to lock a deposit, this is defined by relay chain.
    - Each student is a parachain, mock a system of setting up channels IRL. 