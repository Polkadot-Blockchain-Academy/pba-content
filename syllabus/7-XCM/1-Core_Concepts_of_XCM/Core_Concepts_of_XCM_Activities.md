## Exercises

Assume a setup where we have the relay chain, and two parachains with paraIds 1000 and 1001 respectively:

- Using v2 multilocations, how would you represent each of the following locations?
  - The relay chain
  - Parachain 1000
  - A 32 byte account in the relay
  - A 20 byte smart contract address (0x1111111111111111111111111111111111111111) in Parachain 1000
  - An asset whose Id is 1 in Parachain 1001
  - A pallet whose index is 10 in the relay chain
  - An 80% backed governance origin in Parachain 1000
  - The technical committee origin in the relay chain

Make sure you answer with the multilocation as seen from all three relative locations (relay, para100, para1001)

Hint: Use https://github.com/paritytech/master/blob/master/xcm/src/v2/junction.rs to inspect for existing Junctions
