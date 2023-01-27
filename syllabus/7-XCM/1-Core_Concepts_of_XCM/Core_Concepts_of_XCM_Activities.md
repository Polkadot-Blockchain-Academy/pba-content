## Exercises

Assume a setup where we have the relay chain, and two parachains with paraIds 1000 and 1001 respectively:

- Using v2 multilocations, how would you represent each of the following locations?
  - The relay chain
    - **From Parachain 1000: MultiLocation { parents: 1, interior: Here }**
    - **From Parachain 1001: MultiLocation { parents: 1, interior: Here }**
    - **From Relay Chain: MultiLocation { parents: 0, interior: Here }**
  - Parachain 1000
    - **From Parachain 1000: MultiLocation { parents: 0, interior: Here }**
    - **From Parachain 1001: MultiLocation { parents: 1, interior: X1(Parachain(1001)) }**
    - **From Relay Chain: MultiLocation { parents: 0, interior: X1(Parachain(1001)) }**
  - A 32 byte account in the relay
    - **From Parachain 1000: MultiLocation { parents: 1, interior: X1(AccountId32 { network: Any, id: ALICE }) }**
    - **From Parachain 1001: MultiLocation { parents: 1, interior: X1(AccountId32 { network: Any, id: ALICE }) }**
    - **From Relay Chain: MultiLocation { parents: 0, interior: X1(AccountId32 { network: Any, id: ALICE }) }**
  - A 20 byte smart contract address (0x1111111111111111111111111111111111111111) in Parachain 1000
    - **From Parachain 1000: MultiLocation { parents: 0, interior: X1(AccountKey20 { network: Any, id: sc_address }) }**
    - **From Parachain 1001: MultiLocation { parents: 1, interior: X2( Parachain(1000), AccountKey20 { network: Any, id: sc_address }) }**
    - **From Relay Chain: MultiLocation { parents: 0, interior: X2( Parachain(1000), AccountKey20 { network: Any, id: sc_address }) }**
  - An asset whose Id is 1 in a pallet instance 50 in Parachain 1001
    - **From Parachain 1000: MultiLocation { parents: 1, interior:  X3(Parachain(1001), PalleInstance(50), GeneralIndex (1)) }**
    - **From Parachain 1001: MultiLocation { parents: 0, interior:  X2(PalleInstance(50), GeneralIndex(1)) }**
    - **From Relay Chain: MultiLocation { parents: 0, interior:  X3(Parachain(1001), PalleInstance(50), GeneralIndex (1)) }**
  - Pallet instance 10 in the relay chain
    - **From Parachain 1000: MultiLocation { parents: 1, interior:  X1(PalleInstance(10)) }**
    - **From Parachain 1001: MultiLocation { parents: 1, interior:  X1(PalleInstance(10)) }**
    - **From Relay Chain: MultiLocation { parents: 0, interior:  X1(PalleInstance(10)) }**
  - An 80% backed governance origin in Parachain 1000
    - **From Parachain 1000: MultiLocation { parents: 0, interior:  X1(Plurality { id: Legislative, part: Fraction { nom: 80, denom: 100 }}) }**
    - **From Parachain 1001: MultiLocation { parents: 1, interior: X2(Parachain(1000), Plurality { id: Legislative, part: Fraction { nom: 80, denom: 100 }}) }**
    - **From Relay Chain: MultiLocation { parents: 0, interior: X2(Parachain(1000), Plurality { id: Legislative, part: Fraction { nom: 80, denom: 100 }}) }**
  - The technical committee origin in the relay chain
    - **From Parachain 1000: MultiLocation { parents: 1, interior:  X1(Plurality { id: Technical, part: Voice }) }**
    - **From Parachain 1001: MultiLocation { parents: 1, interior:  X1(Plurality { id: Technical, part: Voice }) }**
    - **From Rela y Chain: MultiLocation { parents: 0, interior:  X1(Plurality { id: Technical, part: Voice }) }**

Make sure you answer with the multilocation as seen from all three relative locations (relay, para100, para1001)

Hint: Use https://github.com/paritytech/polkadot/blob/master/xcm/src/v2/junction.rs to inspect for existing Junctions
