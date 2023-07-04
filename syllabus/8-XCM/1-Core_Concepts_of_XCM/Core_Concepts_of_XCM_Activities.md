## Exercises

Assume a setup where we have the relay chain, and two parachains with paraIds 1000 and 1001 respectively:

- Using XCM locations, how would you represent each of the following from the perspective of all three chains (relay, para1000, para1001)?
  - The relay chain
  - Parachain 1000
  - A 32 byte account in the relay
  - A 20 byte smart contract address (0x1111111111111111111111111111111111111111) in Parachain 1000
  - An asset whose Id is 1 from pallet instance 50 in Parachain 1001
  - Pallet instance 10 in the relay chain
  - An 80% backed governance origin in Parachain 1000

Hint: Use https://github.com/paritytech/polkadot/blob/master/xcm/src/v3/junction.rs to inspect for existing Junctions
