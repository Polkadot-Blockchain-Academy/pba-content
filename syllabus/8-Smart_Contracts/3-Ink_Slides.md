---
title: ink!
description: An introduction on what ink! is and how it ties into Substrate.
duration: 1 hour
---

---

<img rounded style="width: 800px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/ink-logo-with-squid-white.svg" alt="ink!" />

---

<pba-cols>
<pba-col>

### Agenda

</pba-col>
<pba-col>

|          |        |                    |
| -------- | ------ | :----------------- |
| Today    | 1pm    | ink! Intro         |
|          | 2pm    | Activity           |
|          | 3:30pm | Hints for tomorrow |
|          |        | &nbsp;             |
| Tomorrow | 1pm    | We help you        |
|          | 2pm    | Game               |
|          | 3:30pm | Perfect Solutions  |

---

## How does ink! tie into Substrate?

<img rounded style="width: 900px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/lego0.png" />

---

## How does ink! tie into Substrate?

<img rounded style="width: 900px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/lego1.png" />

---

<img rounded style="width: 900px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/schema0.png" />

---

<img rounded style="width: 900px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/schema1.png" />

---

<img rounded style="width: 900px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/schema2.png" />

---

<img rounded style="width: 900px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/schema3.png" />

---

<img rounded style="width: 1200px; " src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/contracts-vs-parachain1.png" />

---

<img rounded style="width: 1200px; " src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/contracts-vs-parachain2.png" />

---

<img rounded style="width: 1200px; " src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/contracts-vs-parachain3.png" />

Notes:

Smart Contract vs. Parachain
Parachain:

- Only requirement: Minimal Polkadot API
- Trusted Code

Smart Contracts:

- Untrusted Code
- Requires Metering
- Fixed payment paradigm (gas fees)

---

<img rounded style="width: 1200px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/use-case0.jpg" />

Notes:

- Motivation
- Use Case 1: Wrap Pallet
- Smart Contracts as “first class citizen”
  - ➜ Smart Contract Parachain + $UVP_for_Contracts

---

<img rounded style="width: 1200px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/use-case1.jpg" />

Notes:

- Use Case 2: Expose Business Logic
- Smart Contracts as “second class citizen”
  - ➜ Parachain adding customizability for its business logic

---

<img rounded style="width: 1200px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/use-case2.jpg" />

Notes:

- Use Case 3: Embrace Prototyping
- Prototyping before going for own parachain

---

# The ink! language

Notes:

Just Rust
Debugging, Testing, Tooling, clippy, cargo fmt, fuzzing

---

## `Cargo.toml`

```toml [1-9|2|4-5|7-8]
[dependencies]
ink = { version = "4.0.0", default-features = false }

# Encoding/Decoding
scale = { package = "parity-scale-codec", ... }

# Metadata
scale-info = { version = "2", ... }
```

---

```rust [1-48]
mod my_contract {

    struct MyContract {
        value: bool,
    }


    impl MyContract {

        fn new() ➜ Self {
            MyContract { value: true }
        }


        fn get(&self) ➜ bool {
            self.value
        }


        fn flip(&mut self) {
            self.value = !self.value;
        }
    }
}
```

Notes:

Hello ink!

---

```rust [1-24|1-2|3-6|9-12|14-17|19-22]
#[ink::contract]
mod my_contract {
    #[ink(storage)]
    struct MyContract {
        value: bool,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new() ➜ Self {
            MyContract { value: true }
        }

        #[ink(message)]
        pub fn get(&self) ➜ bool {
            self.value
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }
    }
}
```

---

## Idiomatic Rust

```rust
#[ink(message)]
pub fn do_it(&self) -> Result<(), Error> {
  Err(Error:OhNo)
}
```

---

## Trait Definitions

```rust
#[ink::trait_definition]
pub trait BaseErc20 {
  #[ink(message)]
  fn total_supply(&self) -> Balance;

  // -- snip --
}
```

Notes:

Can be implemented by multiple contracts.

---

## Environment

```rust
pub enum DefaultEnvironment {}

impl Environment for DefaultEnvironment {
    const MAX_EVENT_TOPICS: usize = 4;

    type Balance = u128;
    type Timestamp = u64;
    type BlockNumber = u32;
    // --snip--
}
```

---

<img rounded style="width: 1000px; " src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/testing.png" />

---

## Unit Tests

```rust
#[cfg(test)]
mod tests {

    #[test]
    fn default_works() {
        let flipper = Flipper::default();
        assert_eq!(flipper.get(), true);
    }

}
```

---

## Integration Tests

```rust [1-5,17,19|6-9|11-12|14-15]
#[cfg(test)]
mod tests {

    #[ink::test]
    fn default_works() {
        // given
        let my_contract = MyContract::default();
        let accounts =
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

        // when
        ink_env::test::set_value_transferred::<ink_env::DefaultEnvironment>(10);

        // then
        assert!(my_contract.received_ten());
    }

}
```

---

## E2E Tests

```rust [1-30|1-2|3-4|6-11|1-20]
#[ink_e2e::test]
async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    // given
    let constructor = FlipperRef::new_default();

    // when
    let contract = client
        .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    // --snip--

    Ok(())
}
```

Notes:
When the function is entered, the contract was already
built in the background via `cargo contract build`.
The `client` object exposes an interface to interact
with the Substrate node.

---

# `$ cargo contract`

[https://github.com/paritytech/cargo-contract](https://github.com/paritytech/cargo-contract)

<!-- .element: class="fragment" -->

---

## Metadata?

<img rounded style="width: 1250px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/metadata.png" />

---

<img rounded style="width: 1100px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/build-artifacts1.png" />

---

<img rounded style="width: 1100px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/build-artifacts2.png" />

---

<img rounded style="width: 1100px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/build-artifacts3.png" />

---

# ink!-ternals

<img rounded style="width: 1300px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/inkternals1.png" />

---

# ink!-ternals

<img rounded style="width: 1300px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/inkternals2.png" />

---

# ink!-ternals

<img rounded style="width: 1300px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/inkternals3.png" />

---

# ink!-ternals

<img rounded style="width: 1300px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/inkternals4.png" />

---

# ink!-ternals

<img rounded style="width: 1300px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/inkternals5.png" />

---


## Local Development

<img rounded style="width: 1200px" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/contracts-node.png" />

[`substrate-contracts-node`](https://github.com/paritytech/substrate-contracts-node)

---

## Testnet

<img rounded style="width: 1200px" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/contracts-on-polkadot-js.png" />

[Rococo Testnet](https://ink.substrate.io/testnet)

---

## Developer UIs

<div class="flex-container">
<div class="left"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

<img rounded style="width: 600px; border: 1px solid #e6007a;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/contracts-ui.png" />

[https://contracts-ui.substrate.io](https://contracts-ui.substrate.io)

</div>
<div class="right"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

<img rounded style="width: 600px; border: 1px solid #e6007a;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/polkadot-js-contracts.png" />

[https://polkadot.js.org/apps](https://polkadot.js.org/apps)

</div>
</div>

---

## Developer UIs

<img rounded style="width: 900px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/developer-uis-2.svg" />

---

## Documentation

<img rounded style="width: 1000px; border: 1px solid #e6007a" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/use-ink.png" />

[www.use.ink](https://use.ink)

---

<img rounded style="width: 1000px; border: 1px solid #e6007a" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/spanish.jpg" />

---

## Documentation

<img rounded style="width: 1000px; border: 1px solid #e6007a" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/ink_env.png" />

---

# Building a Dapp on ink!

---

## Reading Contract Values: RPC

<img rounded style="width: 900px; margin-top:25px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/rpc.png" />

---

## Reading Contract Values: Events

<img rounded style="width: 900px; margin-top:25px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/events.png" />

---

<img rounded style="width: 700px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/venn.png" />

---

## Security Comparison Solidity

---

<div class="flex-container">
<div class="left fragment" data-fragment-index="1">
<img rounded style="width: 150px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/solidity.png" />
<br/>
<img rounded style="width: 600px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/solidity-re-entrancy-attack.jpg" />
</div>
<div class="right fragment" data-fragment-index="2" style="margin-left: 50px;">
<img rounded style="width: 150px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/squid-round.png" />

- Built-in re-entrancy protection
- Fine-grained control

</div>
</div>

Notes:

Re-entrancy Protection

---

<div class="flex-container">
<div class="left fragment" data-fragment-index="1">
<img rounded style="width: 150px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/solidity.png" />

```solidity
pragma solidity 0.7.0;

contract Lottery {

  function withdrawWinnings() {
    require(msg.sender = …);
    _sendWinnings(msg.sender);
  }

  function _sendWinnings() {
    msg.sender.transfer(this.balance);
  }

}
```

</div>
<div class="right fragment" data-fragment-index="2" style="margin-left: 50px;">
<img rounded style="width: 150px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/squid-round.png" />

- Functions private by default
- Needs to be annotated explicitly
- Required: `pub` + `#[ink(message)]`

</div>
</div>

---

<img rounded style="width: 1000px" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/swc.png" />

---

<img rounded style="width: 1200px" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/swc0.png" />

<img rounded style="width: 1200px" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/swc1.png" />

<img rounded style="width: 1200px" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/swc2.png" />

<br/>
<img rounded style="width: 80px" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/dots-white.svg" />

Notes:

- Mutating values
- Ownership & Borrow checker

---

<img rounded style="width: 1100px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/Questions_2.svg" />
