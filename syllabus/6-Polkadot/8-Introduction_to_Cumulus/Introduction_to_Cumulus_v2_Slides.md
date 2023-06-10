---
title: Cumulus Deep Dive
description: Cumulus, Transforming Solo to Parachain
duration: 1 hours
---

# Cumulus Deep Dive

---

<!--
### Outline

<pba-flex center>

1. [What is Cumulus?](#what-is-cumulus)
1. [Cumulus Runtime Validation](#cumulus-runtime-validation)
1. [Cumulus on the Node Side](#cumulus-on-the-node-side)
1. [Transform Solo to Parachain](#transform-solo-to-parachain)
1. [References](#references)

</pba-flex> 
-->

## What is Cumulus?

> Cumulus clouds are shaped sort of like dots; together they form a system that is intricate, beautiful and functional

SDK for building substrate/FRAME-based Parachains
<!-- .element: class="fragment" data-fragment-index="1" -->

---v

<div class="r-stack">
<img src="../assets/spc_1.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="0" -->

<img src="../assets/spc_2.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="1" -->

<img src="../assets/spc_3.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="2" -->
</div>

Notes:

1.
- Substrate is a framework for building blockchains
- But only "solo" chains
- Split into runtime/node side
- FRAME allows you to build modular components reused by runtimes

2.
- Polkadot makes uses of Substrate
- The concept of Parachains is introduced in Polkadot
- It implements Parachain Sharding and Validation as node and runtime-side logic
- Has its own networking protocols built with Substrate/libp2p

3.
- Cumulus uses the generic types of Substrate
- These generic types/interfaces are extended to make them work with/for Parachains
- Polkadot itself is providing APIs that are used by Cumulus to implement the Substrate interfaces/types

---

## What does Cumulus?

---v
<div class="r-stack">
<img src="../assets/cumulus_sketch_1.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="0" -->
<img src="../assets/cumulus_sketch_2.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="1" -->
<img src="../assets/cumulus_sketch_3.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="2" -->
<img src="../assets/cumulus_sketch_4.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="3" -->
</div>

Notes: 
- A substrate Based Chain is composed by:
  - Client
  - Runtime (= STF)

- Cumulus act as a wrapper around a those two parts to make them adapt to the polkadot protocol.

---v

- Enables runtime validation
- Makes the parachain  to communicate with the relay chain

Notes: 

In the next sections those parts will be deeply covered

---

## How Cumulus Enables Runtime Validation

---

### What's Runtime Validation?

- What the relay chain does is: validate state transition between blocks, through the process of **Runtime Validation**
- An important contraint: the relay chain must be able to execute the Runtime Validation without knowing the all previous state of the parachain
- What is needed by the relay chain?
  - The STF of the parachain (= Proof Validation Function)
  - The minimum amount of data to make possible the validation (= Proof of Validity)

---v

#### Proof Validation Function - PVF

- The STF of the Parachain must be store on the Relay Chain

``` rust [6]
/// A struct that carries code of a parachain validation function and its hash.
///
/// Should be cheap to clone.
#[derive(Clone)]
pub struct Pvf {
    pub(crate) code: Arc<Vec<u8>>,
    pub(crate) code_hash: ValidationCodeHash,
}
```

</br>

- New state transitions that occur on a parachain must be validated against the registered PVF 

---v

#### Proof Of Validity - POV

- Polkadot requires that a Parachain block is transmitted in a fixed format: **PoVBlock**
- PoVBlock contains:
  - Header of the new Block
  - Transactions of the Parachain as opaque blobs 
  - Witness data
  - Outgoing messages

---v
  
##### Witness Data

- Validator needs those to validate the new block (without requiring the full state of the Parachain)
- Composed by the data used in the state transition with the relative inclusion proof in the previous state
- The witness data make possible the in-memory reconstruction of the merkle trie needed for the Runtime Validation

</br>

- This makes the biggest limitation of the PoV (max 5MiB)

  
---v

###### Example of Witness Data

<div class="r-stack">
<img src="../assets/pov_inclusion_proof_1.svg" style="width: 1500px" />
<!-- .element: class="fragment fade-out" data-fragment-index="1" -->
<img src="../assets/pov_inclusion_proof_2.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="1" -->
</div>

---v

#### Little caveat - validate_block

<pba-cols>
<pba-col center>

<img src="../assets/cumulus_sketch_4.svg" width = "800vw"/>

</pba-col>
<pba-col center>

- The PVF is not only a copy paste of the parachain Runtime
<!-- .element: class="fragment" data-fragment-index="1" -->

</br>

- validate_block is a function required to be present in the PVF
<!-- .element: class="fragment" data-fragment-index="2" -->

</br>

**WHY!?**
<!-- .element: class="fragment" data-fragment-index="3" -->

</pba-col>
</pba-cols>

Notes: 

In the PVF in the image the Runtime is not the only thing present, there is also a function needed to interpret all the information that are coming from the Parachain but opaque to the relay chain.

---v

#### Before let's do step back

<div class="r-stack">
<img src="../assets/pov_path_1.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="0" -->
<img src="../assets/pov_path_2.svg" style="width: 1500px" />
<!-- .element: class="fragment" data-fragment-index="1" -->
</div>


---v

#### What the validate_block function actually does?

```rust [1|2-3|5|7-10|12-13]
fn validate_block(input: InputParams) -> Output {
    // First let's initialize the state
    let state = input.storage_proof.into_state().expect("Storage proof invalid");

	replace_host_functions();

    // Run `execute_block` on top of the state
    with_state(state, || {
        execute_block(input.block).expect("Block is invalid")
    })

    // Create the output of the result
    create_output()
}
```


Notes:

We construct the sparse in-memory database from the storage proof inside the block data and
then ensure that the storage root matches the storage root in the `parent_head`.

We replace all the storage related host functions with functions inside the wasm blob.
This means instead of calling into the host, we will stay inside the wasm execution.
This is very important as the relay chain validator hasn't the state required to verify the block.
But
we have the in-memory database that contains all the values from the state of the parachain
that we require to verify the block.

- On solo chains we also run the block import on some state
- This state belongs to the parent of the block that should be imported

- `create_output` includes for example:
  - the number of processed messages
  - The upward messages sent
  - Is there a runtime upgrade to schedule?


---v

<img src="../assets/validate_block.svg" style="width: 1500px" />


---

### Data Availability Protocol

- The Availability and Validity (AnV) protocol of Polkadot allows the network to be efficiently sharded among parachains while maintaining strong security guarantees

- The PoV is too big to be included on-chain, validators will then produce a constant size **Candidate Block Recepeit** to represent the just validate block

- PoVBlock, though, can't be lost because other checks on the parachain block needs to be done

---v

#### Pending Availability Phases

- Erasure coding is applied to the PoV
- At least 2/3 + 1 validators must report that they possess their piece of the code word. Once this threshold of validators has been reached, the network can consider the PoV block of the parachain available 

- NOT SURE HOW MUCH OF THIS IS IN CONFLICT WITH Async Backing

---

### What cumulus does for you

- Starting from a Substrate-based chain Cumulus:
  - from runtime creates the PVF, with an automatic implementation of the `validate_block` function
  - changes the block production behaviour to produce also the PoVBlock 

Notes:
Cumulus changes the compilation behavior to produce beside the normal state transition function used by the collator, also the PFV to a wasm bloc, stored in the target folder

---v

### Parachain System Pallet

```rust
//! `cumulus-pallet-parachain-system` handles low-level details
//! of being a parachain.
/// It's responsibilities include:
//!
//! - ingestion of the parachain validation data
//! - ingestion of incoming downward and horizontal
//!   messages and dispatching them
//! - coordinating upgrades with the relay-chain
//! - communication of parachain outputs, such as
//!   sent messages, signalling an upgrade, etc.
```

<!--
### Cumulus Validation Blob

```rust
fn create_output(block: Block) -> ValidationResult {
	let head_data = HeadData(block.header().encode());

	let new_validation_code = crate::NewValidationCode::<PSC>::get();
	let upward_messages = crate::UpwardMessages::<PSC>::get();
	let processed_downward_messages = crate::ProcessedDownwardMessages::<PSC>::get();
	let horizontal_messages = crate::HrmpOutboundMessages::<PSC>::get();
	let hrmp_watermark = crate::HrmpWatermark::<PSC>::get();

	ValidationResult {
		head_data,
		new_validation_code: new_validation_code.map(Into::into),
		upward_messages,
		processed_downward_messages,
		horizontal_messages,
		hrmp_watermark,
	}
}
```

Notes:

- `create_output` includes for example:
  - the number of processed messages
  - The upward messages sent
  - Is there a runtime upgrade to schedule?
-->

---

## Cumulus on the Node Side

- [Relay chain interface](#relay-chain-interface)
- [Finality](#finality)
- [Triggering Block Authoring](#triggering-block-authoring)
- [Ensuring Block Availability](#ensuring-block-availability)

---

## Relay chain interface

<img rounded src="../../../assets/img/5-Polkadot/cumulus/relay-chain-interface.png" style="width: 600px" />

Notes:

The relay chain interface is responsible for following the relay chain and providing block and finality notification stream along with some runtime api calls into the relay chain state for message processing.
It can be run as an in-process full-node or a separate RPC node.


---

## Finality

```rust
loop {
    let finalized = finalized_relay_chain_blocks_stream.next().await;

    let parachain_block = match get_parachain_block_for_relay_chain_block(finalized) {
        Some(b) => b,
        None => continue,
    };

    set_finalized_parachain_block(parachain_block);
}
```

---

## Triggering Block Authoring

```rust
loop {
    let imported = import_relay_chain_blocks_stream.next().await;

    if relay_chain_awaits_parachain_candidate(imported) {
        let pov = match parachain_trigger_block_authoring(imported) {
            Some(p) => p,
            None => continue,
        };

        relay_chain_distribute_pov(pov)
    }
}
```

Notes:

- `parachain_trigger_block_authoring` itself can decide if it wants to build a block.
- e.g. the parachain having a block time of 30 seconds

---

## Ensuring Block Availability

- On a solo chain a block gets part of the canonical chain by:
  - Being distributed to other nodes in the network
  - Being a valid block that can be imported by a majority of the validators
- On a Parachain a block only needs to be accepted by the relay chain validators to be part of the canonical chain
- The problem is that a collator can send a block to the relay chain without distributing it in the Parachain network
- So, the relay chain could expect some parent block for the next block that no one is aware of

Notes:

- Collators can be malicious and just do not propagate their block in the network
- Collators could crash after sending the block to the relay chain, but before propagating it in the Parachain network.

---

## Ensuring Block Availability

```rust
loop {
    let imported = import_relay_chain_blocks_stream.next().await;

    let candidate = match get_backed_parachain_candidate_for_relay_block(imported) {
        Some(c) => c,
        None => continue,
    };

    spawn(|| {
        wait(some_time).await;

        if !is_block_known(candidate.hash) {
            let pov = recover_candidate(candidate);

            let header = import_pov(pov);
            announce_block(header);
        }
    })
}
```

Notes:

- PoV recovery
- Relay chain stores the PoVs for 24 hours
- Every node relay chain/parachain can ask the relay chain validators for their piece to restore the PoV

---

## Runtime Upgrades

- Every Substrate blockchain supports runtime upgrades
- Updating a Parachain runtime is not as easy as updating a standalone blockchain runtime
- Runtime upgrade is delay by a parameter configured by the relay chain
- The first Parachain block that will be included after X relay chain blocks needs to apply the upgrade.
- Cumulus will make sure that the runtime update is applied at the correct block

Notes:

A Parachain will follow the same paradigm, but the relay chain needs to be informed before the update.
Cumulus will provide functionality to notify the relay chain about the runtime update.
The update will not be enacted directly; instead it takes X relay blocks (a value that is configured by the relay chain)
before the relay chain allows the update to be applied.
The first Parachain block that will be included after X relay chain blocks needs to apply the upgrade.
If the update is applied before the waiting period is finished, the relay chain will reject the Parachain block for inclusion.
The Cumulus runtime pallet will provide the functionality to register the runtime upgrade and will also make sure that the
update is applied at the correct block.
https://github.com/paritytech/cumulus/blob/master/docs/overview.md#runtime-upgrade

---

### Transform Solo to Parachain

To convert a Substrate runtime into a Parachain runtime, the following code needs to be added to the runtime:

```rust
cumulus_pallet_parachain_system::register_validate_block!(Block, Executive);
```

Notes:

When compiling a runtime that uses Cumulus, a Wasm binary is generated that contains the full code of the Parachain runtime plus the validate_block functionality.
This binary is required to register a Parachain on the relay chain.

---

### Transform Solo to Parachain

Time for an exercise!

- [Convert a solo chain How-to guide](https://docs.substrate.io/reference/how-to-guides/parachains/convert-a-solo-chain/)

---

### Migrating a parachain

In Cumulus, take a look at:

- [./pallets/solo-to-para/src/lib.rs](https://github.com/paritytech/cumulus/blob/master/pallets/solo-to-para/src/lib.rs)
- [./parachains/runtimes/starters/seedling/src/lib.rs](https://github.com/paritytech/cumulus/blob/master/parachains/runtimes/starters/seedling/src/lib.rs)

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions

---

## References

1. https://github.com/paritytech/cumulus/blob/master/docs/overview.md
