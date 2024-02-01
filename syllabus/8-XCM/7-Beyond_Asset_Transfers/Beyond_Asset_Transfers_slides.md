---
title: XCM Beyond Asset Transfers
description: Deep dive on advanced XCM use cases beyond asset transfers and bridging
duration: 1 hour
---

# XCM Beyond Asset Transfers

---

## Outline

<pba-flex center>

1. Pre-requisites
1. XCMultisig
1. XCM Universal Interface
1. XCM NFT Origins
1. Conclusion
1. Next Steps
1. References

</pba-flex>

---

## Pre-requisites

The following are expected:

<pba-flex center>

- Knowledge of core XCM concepts
- Knowledge of XCM chain configuration

</pba-flex>

---

## XCMultisig

[InvArch Network](https://invarch.network/) has the concept of XCMultisigs, these are entities that exist on the InvArch Network runtime and provide advanced multisig capabilities to users across many other blockchains.

Let's go over how that works!

Notes:

The reason for the name comes from their XCM functionality, more specifically from the idea that these entities have their logic defined in the InvArch Network runtime, but exist in all other connected chains with the same exact account address, thus allowing them to transact in these chains through XCM.

---v

### Overview

<diagram class="mermaid">
stateDiagram-v2
state Polkadot {
    direction LR

    state InvArch {
        direction LR
        v0: Multisig ID 0
        sxc: Send XCM Call
        vacc: 0x123...
        ixc: **XCM Converters**

        state if_state <<choice>>
        v0 --> if_state

        if_state --> sxc
        if_state --> ixc

        ixc --> vacc
        vacc --> [*]: Transact


        sxc --> h0
        sxc --> i0
        sxc --> b0
    }

    state HydraDX {
        direction LR

        h0: Multisig ID 0
        hmxs: **XCM Converters**
        hacc: 0x123...

        h0 --> hmxs
        hmxs --> hacc
        hacc --> [*]: Transact
    }

    state Interlay {
        direction LR

        i0: Multisig ID 0
        imxs: **XCM Converters**
        iacc: 0x123...

        i0 --> imxs
        imxs --> iacc
        iacc --> [*]: Transact
    }

    state Bifrost {
        direction LR

        b0: Multisig ID 0
        bmxs: **XCM Converters**
        bacc: 0x123...

        b0 --> bmxs
        bmxs --> bacc
        bacc --> [*]: Transact
    }

}
</diagram>

---v

### Message details

To better understand how this all works, let's go over the messages being sent and their origins.

```rust [1-17|19-37|39|41-57|59|61-79]
let multisig: MultiLocation = MultiLocation {
  parents: 1,
  interior: Junctions::X3(
    Junction::Parachain(<T as pallet::Config>::ParaId::get()),
    Junction::Plurality {
      id: BodyId::Index(0),
      part: BodyPart::Voice,
    },
  ),
};

let multisig_interior = Junctions::X1(
  Junction::Plurality {
    id: BodyId::Index(0),
    part: BodyPart::Voice,
  }
);

let destination = MultiLocation {
  parents: 1,
  interior: Junctions::X1(
    Junction::Parachain(1234)
  ),
};

let fee_asset_location = MultiLocation {
  parents: 1,
  interior: Junctions::X2(
    Junction::Parachain(1234),
    Junction::GeneralIndex(0),
  ),
};

let fee_multiasset = MultiAsset {
  id: AssetId::Concrete(fee_asset_location),
  fun: Fungibility::Fungible(1000000000000),
};

let call = vec![...];

let message = Xcm(vec![
  Instruction::WithdrawAsset(fee_multiasset.clone().into()),
  Instruction::BuyExecution {
    fees: fee_multiasset,
    weight_limit: WeightLimit::Unlimited,
  },
  Instruction::Transact {
    origin_kind: OriginKind::Native,
    require_weight_at_most: 5000000000,
    call: <DoubleEncoded<_> as From<Vec<u8>>>::from(call),
  },
  Instruction::RefundSurplus,
  Instruction::DepositAsset {
    assets: MultiAssetFilter::Wild(WildMultiAsset::All),
    beneficiary: multisig,
  },
]);

pallet_xcm::Pallet::<T>::send_xcm(multisig_interior, destination, message)?;

// Pallet XCM will then add a DescendOrigin instruction to index 0 of the message.
Instruction::DescendOrigin(multisig_interior)

// Which mutates the initial Origin
MultiLocation {
  parents: 1,
  interior: Junctions::X1(
    Junction::Parachain(<T as pallet::Config>::ParaId::get()),
  ),
}
// Becomes
MultiLocation {
  parents: 1,
  interior: Junctions::X3(
    Junction::Parachain(<T as pallet::Config>::ParaId::get()),
    Junction::PalletInstance(<T as pallet::Config>::INV4PalletIndex::get()),
    Junction::GeneralIndex(0u128),
  ),
}

```

---v

### XCM Converters

Now that we understand the origin and message structure, let's take a look at those _XCM Converters_!

```rust [1-8|10-25|27-35|37-42]
pub struct HashedDescription<AccountId, Describe>(PhantomData<(AccountId, Describe)>);
impl<AccountId: From<[u8; 32]> + Clone, Describe: DescribeLocation> ConvertLocation<AccountId>
	for HashedDescription<AccountId, Describe>
{
	fn convert_location(value: &Location) -> Option<AccountId> {
		Some(blake2_256(&Describe::describe_location(value)?).into())
	}
}

pub struct DescribeFamily<DescribeInterior>(PhantomData<DescribeInterior>);
impl<Suffix: DescribeLocation> DescribeLocation for DescribeFamily<Suffix> {
	fn describe_location(l: &Location) -> Option<Vec<u8>> {
		match (l.parent_count(), l.first_interior()) {
      [...]
			(1, Some(Parachain(index))) => {
				let tail_junctions = l.interior().clone().split_first().0;
				let tail = Location::new(0, tail_junctions);
				let interior = Suffix::describe_location(&tail)?;
				Some((b"SiblingChain", Compact::<u32>::from(*index), interior).encode())
			},
      [...]
			_ => return None,
		}
	}
}

pub struct DescribeBodyTerminal;
impl DescribeLocation for DescribeBodyTerminal {
	fn describe_location(l: &Location) -> Option<Vec<u8>> {
		match l.unpack() {
			(0, [Plurality { id, part }]) => Some((b"Body", id, part).encode()),
			_ => return None,
		}
	}
}

// Our resulting location description before hashing
(
  b"SiblingChain",
  Compact::<u32>::from(para_id),
  (b"Body", BodyId::Index(multisig_id), BodyPart::Voice).encode()
).encode()
```

---v

### What happens if we map AccountId origins to the exact accounts within?

## Account Impersonation!

<!-- .element: class="fragment" data-fragment-index="0" -->

Hey Chain B, I'm sending you a balance transfer request from one of my users, their address is "Chain B's treasury" ;)

<!-- .element: class="fragment" data-fragment-index="1" -->

# TRUST!

<!-- .element: class="fragment" data-fragment-index="2" -->

---

## XCM Universal Interface

XCM can be used as a general API abstraction on top of multiple blockchains.
With some clever usage, we can build chains that can be integrated by dApps in a generic manner, and also dApps that easily integrate multiple chains without any custom logic.

---v

## Concept

###### XCM Powered Multichain NFT Marketplace

Imagine an NFT marketplace where not only multiple chains are supported, but also any standards these chains choose to implement!

---v

### How?

<diagram class="mermaid">
stateDiagram-v2
  direction TB

    ui: UI
    xcm: XCM API
    indexer: Indexer

    ui --> xcm

    indexer --> ui

    xcm --> axti
    xcm --> mxti
    xcm --> cxti

    state Asset_Hub {
      axti: XCM AssetExchanger
      apu: Pallet Uniques
      apn: Pallet NFTs

      axti --> apu
      axti --> apn
    }

    state Moonbeam {
      mxti: XCM AssetExchanger
      mpe: Pallet EVM

      mxti --> mpe
    }

    state Chain_C {
      cxti: XCM AssetExchanger
      cpu: Pallet Uniques
      cpc: Pallet Contracts

      cxti --> cpu
      cxti --> cpc
    }

</diagram>

---v

### Matching NFTs

```rust [0|4-15|18-21]

MultiAsset {
  // Where to find the NFT (contract or collection in an NFT pallet)
  id: AssetId::Concrete (
    Multilocation {
      parents: 0,
      interior: Junctions::X3(
        // Parachain ID just so we can pre-check if this message was intended for this chain
        Junction::Parachain (para_id),
        // Pallet ID so we know which pallet we should be using to look up the NFT
        Junction::PalletInstance(pallet_id),
        // General Index to select a specific collection by integer id
        // Or GeneralKey to select a specific collection bv it's contract id
        Junction::GeneralIndex(collection_id) or Junction::GeneralKey(contract_address),
      )
    }
  ),
  // The NFT itself
  fun: Fungibility::NonFungible(
    // Specific NFT instance inside the collection selected by it's id
    AssetInstance::Instance(nft_id)
  )
}

```

---v

### Implementing AssetExchanger

```rust [1-20|22-38|40-48]
pub trait AssetExchange {
	/// Handler for exchanging an asset.
	///
	/// - `origin`: The location attempting the exchange; this should generally not matter.
	/// - `give`: The assets which have been removed from the caller.
	/// - `want`: The minimum amount of assets which should be given to the caller in case any
	///   exchange happens. If more assets are provided, then they should generally be of the
	///   same asset class if at all possible.
	/// - `maximal`: If `true`, then as much as possible should be exchanged.
	///
	/// `Ok` is returned along with the new set of assets which have been exchanged for `give`. At
	/// least want must be in the set. Some assets originally in `give` may also be in this set. In
	/// the case of returning an `Err`, then `give` is returned.
	fn exchange_asset(
		origin: Option<&MultiLocation>,
		give: Assets,
		want: &MultiAssets,
		maximal: bool,
	) -> Result<Assets, Assets>;
}

struct MyNftStandardExchanger;

impl AssetExchange for MyNftStandardExchanger {
	fn exchange_asset(
		origin: Option<&MultiLocation>,
		give: Assets,
		want: &MultiAssets,
		maximal: bool,
	) -> Result<Assets, Assets> {
		match (give, want) {
			(FUNGIBLE, NONFUNGIBLE) => MyNftPallet::buy(...),
			(NONFUNGIBLE, FUNGIBLE) => MyNftPallet::sell(...),
			(NONFUNGIBLE, NONFUNGIBLE) => MyNftPallet::swap(...),
			(FUNGIBLE, FUNGIBLE) => Err(give),
		}
	}
}

impl xcm_executor::Config for XcmConfig {
  ...
  type AssetExchanger = (
    MyNftStandardExchanger,
    EvmNftExchanger,
    PalletUniquesExchanger
  );
  type AssetTransactor = AssetTransactors;
}
```

---

## XCM NFT Origins

It is possible to use XCM to bridge non-fungibles, but if all you need is to authenticate ownership of an NFT from a smart contract chain to operate in another chain, you can skip bridging altogether.

---v

### How?

<diagram class="mermaid">
stateDiagram-v2
state Polkadot {
    alice: Alice

    alice --> nvc: Transact on InvArch as NFT 0 from collection 0x234

    state Astar {
        nvc: NFT Verifier Contract 0x123
        ncc: NFT Collection Contract 0x234

        nvc --> ncc: Does Alice own NFT 0?
        ncc --> nvc: Yes

        nvc --> nxc: **XCM Location**
    }

    state InvArch {
        nxc: NFT Origin XCM Converter
        vcr: Verifier Contract Registry
        pallets: Pallets that accept NFT Origin

        nxc --> vcr: Is 0x123 the registered verifier for Astar?
        vcr --> nxc: Yes
        nxc --> pallets: **NFT Origin**
    }

}
</diagram>

---v

### Mapping the location to an origin

```rust
// **XCM Location**
MultiLocation {
  parents: 1,
  interior: Junctions::X4(
    Junction::Parachain(Astar),
    Junction::GeneralKey(0x123),
    Junction::GeneralKey(0x234),
    Junction::GeneralIndex(0)
  )
}

// **NFT Origin**
Origin::Nft {
  parachain: Astar,
  collection: 0x1234,
  nft: 0
}
```

---

## Conclusion

During this presentation we went through a couple real world XCM use cases and some general tips for working with the message standard, the goal here is to leave you with some inspiration and some ideas, so that you can start tinkering with XCM to power your own ideas and supercharge blockchain applications!

---

## References

- [XCM source code](https://github.com/paritytech/polkadot/blob/master/xcm) - The source code for the main XCM implementation in the paritytech/polkadot repository.
- [XCM Documentation](https://paritytech.github.io/xcm-docs) - The official documentation for XCM: Cross-Consensus Messaging.
- [InvArch's Pallet Rings](https://github.com/InvArch/InvArch-Frames/tree/main/pallet-rings) - Reference implementation of an XCM abstraction pallet for XCMultisigs.
