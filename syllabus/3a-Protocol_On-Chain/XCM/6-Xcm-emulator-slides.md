---
title: XCM Emulator
description: Introduction to the XCM emulator.
duration: 1 hour
---

# XCM Emulator

Notes:

The emulator will let us use all the knowledge we've gained so far.

---v

## What you'll learn

- How to use the XCM emulator for testing your cross-chain interactions.

---

# What is it?

A collection of macros that create a mock network we can use test our XCM configuration.

---

# Creating a mock network

---v

## Runtimes

We first need to create our runtimes, a relay chain and a minimum of a parachain.

---v

## Relay chains

```rust
decl_test_relay_chains! {
  pub struct Relay {
    runtime = relay_runtime,
    core = {
      SovereignAccountOf: location_converter,
    },
    pallets = {
      ...
    },
    genesis = genesis(),
    on_init = {
      ...
    },
  }
}
```

Notes:

XCM emulator provides a macro for declaring mock relay chains.

We need to specify a couple of things:

- runtime: The actual runtime of our relay chain.
- core: Some core functionality used by the emulator, things like handlers and converters.
- pallets: All pallets you want to use for tests.
- genesis: Genesis configuration for testing, you can register assets for example.
- on_init: You can call some `on_initialize`s here.

---v

## Parachains

```rust
decl_test_parachains! {
  pub struct ParaA {
    runtime = parachain_runtime,
    core = {
      XcmpMessageHandler: parachain_runtime::XcmpQueue,
			LocationToAccountId: parachain_runtime::configs::xcm_config::LocationToAccountId,
			ParachainInfo: parachain_runtime::ParachainInfo,
			MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
      ...
    },
    pallets = {
      System: parachain_runtime::System,
			Balances: parachain_runtime::Balances,
			ForeignAssets: parachain_runtime::ForeignAssets,
			PolkadotXcm: parachain_runtime::PolkadotXcm,
      ...
    },
    genesis = genesis(),
    on_init = {
      ...
    },
  }
}
```

Notes:

XCM emulator provides a macro for declaring mock parachains.

We need to specify the same things.

---v

## The entire network

```rust
decl_test_networks! {
  pub struct MockNet {
    relay_chain = Relay,
    parachains = vec![
      ParaA,
      ParaB,
    ],
    bridge = (),
  }
}
```

Notes:

We declare our network by specifying our relay chain and parachains.
We can also declare a bridge to test sending messages to different networks.
In that case we also use `decl_test_bridges!` to define the bridges.

---

# Using the mock network

---v

```rust
$crate::paste::paste! {
	pub trait [<$name ParaPallet>] {
		$(
			type $pallet_name;
		)*
	}

	impl<N: $crate::Network> [<$name ParaPallet>] for $name<N> {
		$(
			type $pallet_name = $pallet_path;
		)*
	}
}
```

Notes:

This one's a mouthfull because it's a procedural macro.
Given `ParaA`, it will create a struct `ParaAParaPallet`, which we usually alias to `ParaAPallet`,
that has all the pallets we specified when creating the mock network as associated types.

---v

```rust
pub trait Chain: TestExt {
	type Network: Network;
	type Runtime: SystemConfig;
	type RuntimeCall: Clone + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>;
	type RuntimeOrigin;
	type RuntimeEvent;
	type System;
	type OriginCaller;

	fn account_id_of(seed: &str) -> AccountId {
		get_public_from_string_or_panic::<sr25519::Public>(seed).into()
	}

	fn account_data_of(account: AccountIdOf<Self::Runtime>) -> AccountData<Balance>;

	fn events() -> Vec<<Self as Chain>::RuntimeEvent>;
}
```

Notes:

The `Relay`, `ParaA` and `ParaB` that we created in the previous step implement this trait
and we can use it to access all their aggregated enums and get events from them.

---

# Example test

```rust[|1-2|3|5|6-13|15-22|23-25|28-29|30-32|34-43|45-48]
#[test]
fn example_test() {
  MockNet::reset();

  ParaA::execute_with(|| {
    // Initialize state.
    let sender = /* ... */;
    let initial_balance = /* ... */;
    type Balances = <ParaA as ParaAPallet>::Balances;
    assert_ok!(<Balances as fungible::Mutate<_>>::mint_into(
      &sender,
      initial_balance
    ));

    let message: Xcm<()> = <some_xcm>;
    let destination = Location::new(1, [Parachain(2001)]);
    assert_ok!(<ParaA as ParaAPallet>::XcmPallet::execute(
      <ParaA as Chain>::RuntimeOrigin::signed(<some_account>),
      Box::new(VersionedXcm::V5(message)),
      Weight::from_parts(...), // Some max weight.
    ));

    let expected_balance = /* ... */;
    let final_balance = <Balances as fungible::Inspect<_>>::balance(&sender);
    assert_eq!(final_balance, expected_balance);
  });

  // Calling `execute_with` makes the chain process incoming messages.
  ParaB::execute_with(|| {
    // You can print all events!
    let events = <ParaB as Chain>::events();
    dbg!(&events);

    // And assert them.
    type RuntimeEvent = <ParaB as Chain>::RuntimeEvent;
    assert_expected_events!(
      ParaB,
      vec![
        RuntimeEvent::MessageQueue(
          pallet_message_queue::Event::Processed { success: true, .. }
        ) => {},
      ],
    );

    let receiver = /* ... */;
    let asset_id = /* ... */;
    type ForeignAssets = <ParaB as ParaBPallet>::ForeignAssets;
    let receiver_balance = <ForeignAssets as fungibles::Inspect<_>>::balance(asset_id, &receiver);
  })
}
```

Notes:

First we reset the state of the network.
Then we create a message and call the XCM pallet's `execute` extrinsic.
We must wrap the location and XCM in a `Versioned*` type.

---

# Real examples

Notes:

Go to the Polkadot SDK repo and look at the cumulus emulated tests: https://github.com/paritytech/polkadot-sdk/tree/master/cumulus/parachains/integration-tests/emulated/tests/assets/asset-hub-westend/src/tests.

---

# Next steps

Chopsticks

Notes:

Chopsticks is for forking a live chain and testing against it.
Lets you spawn a network via typescript code and write tests using it.
