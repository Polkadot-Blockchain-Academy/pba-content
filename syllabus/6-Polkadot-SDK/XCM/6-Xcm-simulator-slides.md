---
title: XCM Simulator
description: Introduction to the XCM simulator
duration: 1 hour
---

# XCM Simulator

Notes:

The simulator offers a playground for applying all the knowledge we've gained so far.
You can use the past slides as reference for working with it.

---v

## What you'll learn

- How to setup a mock network with the XCM simulator

---

# What is it?

A collection of macros that create a mock network we can use to play around with XCM.

---

# Creating a mock network

---v

## Runtimes

We first need to create our runtimes, a relay chain and a minimum of a parachain.

Notes:

To create those, we can look back at the FRAME lessons.

---v

## `decl_test_relay_chain`

```rust
decl_test_relay_chain! {
  pub struct Relay {
    Runtime = relay_chain::Runtime,
    RuntimeCall = relay_chain::RuntimeCall,
    RuntimeEvent = relay_chain::RuntimeEvent,
    XcmConfig = relay_chain::XcmConfig,
    MessageQueue = relay_chain::MessageQueue,
    System = relay_chain::System,
    new_ext = relay_ext(),
  }
}
```

Notes:

XCM simulator provides a macro for declaring a mock relay chain.

We need to specify a couple of things:

- Runtime: The actual runtime of our relay chain.
- RuntimeCall, RuntimeEvent: Runtime aggregated types
- XcmConfig: The struct that contains our XCM configuration.
- MessageQueue, System: Specific pallets on our runtime.
- new_ext: A function that returns an instance of `TestExternalities`.
  We can initialize storage for our tests with this.

---v

## `decl_test_parachain`

```rust
pub struct ParaA {
  Runtime = parachain::Runtime,
  XcmpMessageHandler = parachain::MessageQueue,
  DmpMessageHandler = parachain::MessageQueue,
  new_ext = para_ext(),
}
```

Notes:

XCM simulator provides a macro for declaring a mock parachain.

We need to specify less things:

- Runtime: The actual runtime of our parachain.
- XcmpMessageHandler, DmpMessageHandler: The message handlers, always the message queue pallet.
- new_ext: A function that returns an instance of `TestExternalities`.

---v

## `decl_test_network`

```rust
decl_test_network! {
  pub struct MockNet {
    relay_chain = Relay,
    parachains = vec![
      (1000, ParaA),
      (2000, ParaB),
    ],
  }
}
```

Notes:

We declare our network by specifying our relaychain and parachains.

---

# Simple Test

```rust
#[test]
fn test_xcm() {
  MockNet::reset();

  ParaA::execute_with(|| {
    let message: Xcm<()> = <some_xcm>;
    let destination = Location::new(1, Parachain(2000));
    parachain::XcmPallet::send(
      parachain::RuntimeOrigin::signed(<some_account>),
      Box::new(VersionedLocation::V4(destination)),
      Box::new(VersionedXcm::V4(message)),
    );
  });

  ParaB::execute_with(|| {
    // Message arrives, do some asserts.
  })
}
```

Notes:

First we reset the state of the network.
Then we create a message and call the XCM pallet's `send` extrinsic.
We must wrap the location and XCM in a `Versioned*` type.

---

# Next steps

- XCM emulator
- Zombienet
- Chopsticks

Notes:

The evolution of the simulator is the XCM emulator, it has more testing utilities.
The simulator should be used as a playground, and the emulator as a tool for end-to-end tests.

There's also zombienet for spawning a local network and testing against it with either typescript or a DSL.

There's also chopsticks for forking a live chain and testing against it.

---

# Workshop

We'll now jump on the simulator, configure XCM and execute and send messages.
