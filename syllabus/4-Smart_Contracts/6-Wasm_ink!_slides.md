---
title: WASM Smart Contracts in Ink!
description: A working programmer’s guide to the crypto industry
---

<img rounded style="width: 600px;" src="img/ink/ink-logo-with-squid-white.svg" alt="ink!" />

# WASM Smart Contracts in Ink!
A working programmer’s guide

NOTE:
- ask questions during the lecture, don't wait until the end
- practical, but we go deeper where needed
- some complexity is ommited in the examples (examples are not a production code)

---

## Intro: ink! vs. Solidity

|                 | ink!                        | Solidity      |
|-----------------|-----------------------------|---------------|
| Virtual Machine | Any Wasm VM                 | EVM           |
| Encoding        | Wasm                        | EVM Byte Code |
| Language        | Rust                        | Standalone    |
| Constructors    | Multiple                    | Single        |
| Tooling         | Anything that supports Rust | Custom        |
| Storage         | Variable                    | 256 bits      |
| Interfaces?     | Yes: Rust traits            | Yes           |

NOTE:
- students are freshly of an EVM lecture so might be wondering why another SC language
- Virtual Machine: any WASM VM: yes in theory, in practice bound pretty close to the Substarte & the contracts pallet
- Tooling: Solidity has been around for years, enjoys the first-to-market advantage (but ink! is a strong contender)
- The EVM operates on 256 bit words (meaning anything less than 32 bytes will be treated by the EVM as having leading zeros)

---

## Intro: ink! overview

- DSL in Rust
- Inherits all the benefits of Rust
  - Modern functional language
  - Type & Memory safety
- Compiled to WASM
  - Ubiquitous
  - Fast

NOTE:
- ink! is not a separate language
- enjoys access to a vast collection of libraries developed for other purposes
- WASM is targeting the browsers and quickly becoming the "assembly" od the web in lieu of JS

---

## Intro: ink! & Substrate

<img rounded style="width: 900px;" src="img/ink/lego0.png" />

NOTE:
- Technically you could take a SC written in ink! and deploy it to any WASM-powered blockchain.
  - in practice not that straight-forward.
- ink! is closely tied to the larger Substrate framework.
- Substarte is a framework for developing customized blockchain runtimes from composable pallets.

---

## Intro: ink! & Substrate

<img rounded style="width: 900px;" src="img/ink/lego1.png" />

NOTE:
- contracts written in ink! are compiled to WASM bytecode
- pallet contracts provides
  - instrumentation
  - excution engine
  - gas metering

---

<img rounded style="width: 800px;" src="img/ink/schema1.png" />

NOTE:
- pallet contracts is oblivious to the programming language
- it accepts WASM bytecode and executes it's instructions

---

<img rounded style="width: 800px;" src="img/ink/schema2.png" />

NOTE:
- contracts itself can be written in ink!

---

<img rounded style="width: 800px;" src="img/ink/schema3.png" />

NOTE:
- But also any other language that compilers to WASM
  - Solang
  - or ask!

---

## Development: Prerequisites

Install the required tooling

```bash
sudo apt install binaryen
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install dylint-link
cargo install cargo-contract --force
```

NOTE:
- Binaryen is a compiler and toolchain infrastructure library for WebAssembly
- at the moment ink! uses a few unstable Rust features, thus nightly is require
- rust source code is needed to compile it to wasm
- wasm target is added
- cargo-contract is a batteries included CLI tool for compiling, deploying and interacting with the contracts

---

## Development: cargo-contract

Create a contract

```bash
cargo contract new flipper
```

```
  /home/CloudStation/Blockchain-Academy/flipper:
  drwxrwxr-x 2 filip filip 4096 Jul  7 11:11 .
  drwxr-xr-x 5 filip filip 4096 Jul  7 11:11 ..
  -rwxr-xr-x 1 filip filip  573 Jul  7 11:11 Cargo.toml
  -rwxr-xr-x 1 filip filip  285 Jul  7 11:11 .gitignore
  -rwxr-xr-x 1 filip filip 5186 Jul  7 11:11 lib.rs
```

NOTE:
- ask how many student have written some code in Rust, this should feel familiar to them

---

## Development: Cargo.toml

<div style="font-size: 0.72em;">

```toml
[package]
version = "0.1.0"
authors = ["fbielejec"]
edition = "2021"

[dependencies]
ink = { version = "=4.2.1", default-features = false }
scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }
scale-info = { version = "2.6", default-features = false, features = ["derive"], optional = true }

[lib]
path = "lib.rs"

[features]
default = ["std"]
std = [
    "ink/std",
    "scale/std",
    "scale-info/std",
]
```

</div>

NOTE:
- who knows why is the std library not included by default?
- Answer: contracts are compiled to WASM (executed ib a sandboxed environment with no system interfaces, no IO, no networking)

---

## Developing contracts

contract code

<div style="font-size: 0.62em;">

```rust
#[ink::contract]
pub mod flipper {

    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
```

</div>

NOTE:
- basic contract that flips a bit in storage
- contract will have a storage definition, constructor(s), messages
- groupped in a module

---

## Developing contracts: Compilation & artifacts

Compile:

```bash
cargo +nightly contract build
```

Artifacts:

```
 [1/*] Building cargo project
    Finished release [optimized] target(s) in 0.09s

The contract was built in RELEASE mode.

Your contract artifacts are ready. You can find them in:
/home/CloudStation/Blockchain-Academy/flipper/target/ink

  - flipper.contract (code + metadata)
  - flipper.wasm (the contract's code)
  - flipper.json (the contract's metadata)
```

NOTE:
- produces WASM bytecode and some additional artifacts:
- .wasm is the contract compiled bytecode
- .json is contract ABI aka metadata (for use with e.g. dapps)
  - definitions of events, storage, transactions
- .contracts is both of these together

---

## Developing contracts: instantiate

Deploy:

```bash
cargo contract instantiate --constructor default --suri //Alice
  --skip-confirm --execute
```

Output:

<div style="font-size: 0.82em;">

```bash [13-14]
 Dry-running default (skip with --skip-dry-run)
    Success! Gas required estimated at Weight(ref_time: 138893374, proof_size: 16689)
...
  Event Contracts ➜ CodeStored
         code_hash: 0xbf18c768eddde46205f6420cd6098c0c6e8d75b8fb042d635b1ba3d38b3d30ad
       Event Contracts ➜ Instantiated
         deployer: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         contract: 5EXm8WLAGEXn6zy1ebHZ4MrLmjiNnHarZ1pBBjZ5fcnWF3G8
...
       Event System ➜ ExtrinsicSuccess
         dispatch_info: DispatchInfo { weight: Weight { ref_time: 2142580978, proof_size: 9009 }, class: Normal, pays_fee: Yes }

   Code hash 0xbf18c768eddde46205f6420cd6098c0c6e8d75b8fb042d635b1ba3d38b3d30ad
    Contract 5EXm8WLAGEXn6zy1ebHZ4MrLmjiNnHarZ1pBBjZ5fcnWF3G8
```

</div>

NOTE:
- we see a bunch of information on gas usage
- we see two events one for storing contract code another for instantiating the contract
  - why is that?
  - code & instance are separated, we will come back to that
- finally we see code hash and the newly created contracts address

---

## Interacting with the contracts: queries

```bash
cargo contract call --contract 5EXm8WLAGEXn6zy1ebHZ4MrLmjiNnHarZ1pBBjZ5fcnWF3G8
  --message get --suri //Alice --output-json
```

- contract state?
- tip: `default` constructor was called

NOTE:
- who can tell me what will be the contract state at this point?

---


## Interacting with the contracts: queries

<!-- Query the contract state: -->

<!-- ```bash -->
<!-- cargo contract call --contract 5EXm8WLAGEXn6zy1ebHZ4MrLmjiNnHarZ1pBBjZ5fcnWF3G8 -->
<!--   --message get --suri //Alice --output-json -->
<!-- ``` -->

<!-- Result: -->

```[6]
"data": {
  "Tuple": {
    "ident": "Ok",
    "values": [
      {
        "Bool": false
      }
    ]
  }
}
```
---

## Interacting: transactions

Sign and execute a transaction:

```bash
cargo contract call --contract 5EXm8WLAGEXn6zy1ebHZ4MrLmjiNnHarZ1pBBjZ5fcnWF3G8
  --message flip --suri //Alice --skip-confirm --execute
```

Query the state:

```bash
cargo contract call --contract 5EXm8WLAGEXn6zy1ebHZ4MrLmjiNnHarZ1pBBjZ5fcnWF3G8
  --message get --suri //Alice --output-json
```

Result:

<div style="font-size: 0.82em;">

``` [6]
"data": {
  "Tuple": {
    "ident": "Ok",
    "values": [
      {
        "Bool": true
      }
    ]
  }
}
```

</div>

NOTE:
- if I query it again the bit is flipped
- no surprises there

---

## Dev environment: Contracts UI

<img rounded style="width: 1400px;" src="img/ink/contracts_ui_1.jpg" />

NOTE:
- there is also a graphical env for deploying & interacting with contracts
- deploy & create an instance of flipper
---

## Dev environment: Contracts UI

<img rounded style="width: 1400px;" src="img/ink/contracts_ui_2.jpg" />

NOTE:
- call a transacttion

---

## Dev environment: Contracts UI

<img rounded style="width: 1400px;" src="img/ink/contracts_ui_3.jpg" />

NOTE:
- query state

---

## Deeper dive: Storage

```rust
use ink::storage::Mapping;

#[ink(storage)]
#[derive(Default)]
pub struct Token {
    total_supply: Balance,
    balances: Mapping<AccountId, Balance>,
    allowances: Mapping<(AccountId, AccountId), Balance>,
}
```

NOTE:
- now that we dipped our toes lets dissect more
- starting with the storage
- what does this code actually put into the chain storage?

---

<img rounded style="width: 1000px;" src="img/ink/storage.svg" />

<font color="#8d3aed">SCALE</font> (*<font color="#8d3aed">S</font>imple <font color="#8d3aed">C</font>oncatenated <font color="#8d3aed">A</font>ggregate <font color="#8d3aed">L</font>ittle <font color="#8d3aed">E</font>ndian*)

NOTE:
- Pallet contracts storage is organized like a key-value database
- SCALE codec is not self-describing (vide metadata)
- each storage cell has a unique storage key and points to a SCALE encoded value

---

## SCALE: examples of different types

<div style="font-size: 0.72em;">

| Type    | Decoding                              |                     Encoding | Remark                                                                         |
|---------|---------------------------------------|------------------------------|--------------------------------------------------------------------------------|
| Boolean | true                                  |                          0x0 | encoded using least significant bit of a single byte                           |
|         | false                                 |                          0x1 |                                                                                |
| Unsigned int | 42                                  |  0x2a00                         |                            |
| Enum    | enum IntOrBool { Int(u8), Bool(bool)} |            0x002a and 0x0101 | first byte encodes the variant index, remaining bytes encode the data          |
| Tuple   | (3, false)                            |                       0x0c00 | concatenation of each encoded value                                            |
| Vector  | [4, 8, 15, 16, 23, 42]                | 0x18040008000f00100017002a00 | encoding of the vector length followed by conatenation of each item's encoding |
| Struct  | {x:30, y:true}                        | [0x1e,0x0,0x0,0x0,0x1]       | names are ignored, Vec<u8> structure, only order matters                       |

</div>

---

## Storage: Packed Layout

```rust [6]
use ink::storage::Mapping;

#[ink(storage)]
#[derive(Default)]
pub struct Token {
    total_supply: Balance,
    balances: Mapping<AccountId, Balance>,
    allowances: Mapping<(AccountId, AccountId), Balance>,
}
```

* By default ink! stores all storage struct fields under a single storage cell (`Packed` layout)

NOTE:
- Types that can be stored entirely under a single storage cell are called Packed Layout
- by default ink! stores all storage struct fields under a single storage cell
- as a consequence message interacting with the contract storage will always need to read and decode the entire contract storage struct
- .. which may be what you want or not

---


## Storage: Packed Layout

```rust [1-4,7]
use ink::storage::traits::{
    StorageKey,
    ManualKey,
};

#[ink(storage)]
pub struct Flipper<KEY: StorageKey = ManualKey<0xcafebabe>> {
    value: bool,
}
```

* The storage key of the contracts root storage struct defaults to `0x00000000`
* However you may store it under any arbitrary 4 bytes key instead

---

## Storage: Packed Layout

<div style="font-size: 0.82em;">

```json
  "storage": {
    "root": {
      "layout": {
        "struct": {
          "fields": [
            {
              "layout": {
                "leaf": {
                  "key": "0xcafebabe",
                  "ty": 0
                }
              },
              "name": "value"
            }
          ],
          "name": "Flipper"
        }
      },
      "root_key": "0xcafebabe"
    }
  }
```

</div>

NOTE:
- here a demonstartion of packed layout - value is stored under the root key

---

## Storage: Un-packed Layout

```rust [1,7-8]
use ink::storage::Mapping;

#[ink(storage)]
#[derive(Default)]
pub struct Token {
    total_supply: Balance,
    balances: Mapping<AccountId, Balance>,
    allowances: Mapping<(AccountId, AccountId), Balance>,
}
```

* Mapping consists of a key-value pairs stored directly in the contract storage cells.
* Each Mapping value lives under it's own storage key.
* Mapping values do not have a contiguous storage layout: **it is not possible to iterate over the contents of a map!**

NOTE:
- Use Mapping when you need to store a lot of values of the same type.
- if your message only accesses a single key of a Mapping, it will not load the whole mapping but only the value being accessed.
- there are other collection types in ink!: HashMap or BTreeMap (to name a few).
  - these data structures are all Packed, unlike Mapping!

---

## Storage: working with `Mapping`

```rust
pub fn transfer(&mut self) {
    let caller = self.env().caller();

    let balance = self.balances.get(caller).unwrap_or(0);
    let endowment = self.env().transferred_value();

    balance += endowment;
}


```

* what is wrong here?

NOTE:
- working with mapping:
- Mapping::get() method will result in an owned value (a local copy), as opposed to a direct reference into the storage. Changes to this value won't be reflected in the contract's storage "automatically". To avoid this common pitfall, the value must be inserted again at the same key after it was modified. The transfer function from above example illustrates this:

---

## Storage: working with `Mapping`

```rust
pub fn transfer(&mut self) {
    let caller = self.env().caller();

    let balance = self.balances.get(caller).unwrap_or(0);
    let endowment = self.env().transferred_value();

    self.balances.insert(caller, &(balance + endowment));
}
```

* `Mapping::get()` returns a local copy, not a mutable reference to the storage!

NOTE:
- working with mapping:
- `Mapping::get()` method will result in an owned value (a local copy).
- Changes to this value won't be reflected in the contract's storage at all!
- you need to inserted it again at the same key.

---

## Storage: Lazy

```rust [1,5-6]
use ink::storage::{traits::ManualKey, Lazy, Mapping};

#[ink(storage)]
pub struct Roulette {
    pub data: Lazy<Data, ManualKey<0x44415441>>,
    pub bets: Mapping<u32, Bet, ManualKey<0x42455453>>,
}
```

* Every type wrapped in `Lazy` has a separate storage cell.
* `ManualKey` assignes explicit storage key to it.

NOTE:
- packed layout can get problematic if we're storing a large collection in the contracts storage that most of the transactions do not need access too
- there is a 16kb hard limit on a buffer used for decoding, contract trying to decode more will trap / revert
- mapping provides per-cell access
- Lazy storage cell can be auto-assigned or chosen manually
- Using ManualKey instead of AutoKey might be especially desirable for upgradable contracts, as using AutoKey might result in a different storage key for the same field in a newer version of the contract.
  - This may break your contract after an upgrade!

---

## Storage: Lazy

<img rounded style="width: 1000px;" src="img/ink/storage-layout.svg" />

NOTE:
- only the pointer (the key) to the lazy type is stored under the root key
- only when there is a read of `d` will the pointer be de-referenced
- lazy is a bit of a mis-nomer here, because storage is already initialized

---

## Constructors

```rust [1,6,11|2,7-8]
#[ink(constructor)]
pub fn new(init_value: bool) -> Self {
    Self { value: init_value }
}

#[ink(constructor)]
pub fn default() -> Self {
    Self::new(Default::default())
}

#[ink(constructor)]
pub fn non_default() -> Self {
    Self::new(false)
}
```

NOTE:
- no limit of the number of constructors
- constructors can call other constructors

---

## Queries

```rust
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

```

- `#[ink(message)]` is how we tell ink! this is a function that can be called on the contract
- `&self` is a reference to the object you’re calling this method on

NOTE:
- returns information about the contract state
- .. as stored on chain (agreed to by the nodes)

---

## Mutations

```rust [1-4|1]
#[ink(message, payable)]
pub fn place_bet(&mut self, bet_type: BetType) -> Result<()> {
    let player = self.env().caller();
    let amount = self.env().transferred_value();
    ...
```

- `&mut self` is a mutable reference to the object you’re calling this method on
- `payable` allows receiving value as part of the call to the ink! message

NOTE:
- constructors are inherently payable
- ink! message will reject calls with funds if it's not marked as such

---

## Error handling

<div style="font-size: 0.72em;">

```rust [1-4|8-11,16|14,20]
pub enum MyResult<T, E> {
    Ok(value: T),
    Err(msg: E),
}

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MyError {
    InkEnvError(String),
    BettingPeriodNotOver,
}

#[ink(message)]
pub fn spin(&mut self) -> Result<()> {
    if !self.is_betting_period_over() {
        return Err(MyError::BettingPeriodNotOver);
    ...
};

pub type Result<T> = core::result::Result<T, MyError>;

```

</div>

- ink! uses idiomatic Rust error handling: `Result<T,E>` type
- Use the Err variant to pass your own semantics
- Type aliases reduce boilerplate & enhance readability


NOTE:
- ink! uses idiomatic Rust error handling
- ~~messages are the `system boundary`~~
- returning error variant reverts the transaction
  - panicing is the same as returning Err variant (`Result` is just being nice)

---

## Error handling: call stack

```rust
#[ink(message)]
pub fn flip(&mut self) {
    self.value = !self.value;

    if self.env().block_number() % 2 != 0 {
      panic!("Oh no!")
    }

}

```

- what is the state of this contract if the tx is called in an odd block number?

NOTE:
- answer: whatever it was prior to the tx:
  - returning error variant reverts the entire tx on the call stack

---

## Events

```rust
#[ink(event)]
#[derive(Debug)]
pub struct BetPlaced {
    #[ink(topic)]
    player: AccountId,
    #[ink(topic)]
    bet_type: BetType,
    amount: Balance,
}
```

- Events are a way of letting the outside world know about what's happening inside the contract.
- `#[ink(event)]` is a macro that defines events.
- Topics mark fields for indexing.

<!-- NOTE: -->

---

## Events

```rust
#[ink(message)]
pub fn flip(&mut self) {

    if self.env().block_number() % 2 == 0 {
      panic!("Oh no!")
    }

    Self::emit_event(
        self.env(),
        Event::Flipped(Flipped { }),
    );

    self.value = !self.value;
}

```

- will this event be emitted in an odd block?

NOTE:
- answer: yes, but only because I reverted the condition :)

---

## Defining shared behaviour

<div style="font-size: 0.5em;">

```rust [1-14|16-25]
#[ink::trait_definition]
pub trait PSP22 {
    #[ink(message)]
    fn total_supply(&self) -> Balance;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance;

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error>;
    ...

impl SimpleDex {
    use psp22_trait::{PSP22Error, PSP22};

    /// Returns balance of a PSP22 token for an account
    fn balance_of(&self, token: AccountId, account: AccountId) -> Balance {
        let psp22: ink::contract_ref!(PSP22) = token.into();
        psp22.balance_of(account)
    }

    ...

```

</div>

* Trait Definition: `#[ink::trait_definition]`
* Wrapper for interacting with the contract: `ink::contract_ref!`

NOTE:
- (part of) PSP22 (ERC20 like) contract definition
- all contracts that respect this definition need to implement it
- you can now share the trait definition with other contracts
- while getting a typed reference to an instance

---

## Contracts upgradeability: `set_code_hash`

```rust [3]
#[ink(message)]
pub fn set_code(&mut self, code_hash: [u8; 32]) -> Result<()> {
    ink::env::set_code_hash(&code_hash)?;
    Ok(())
}

```

- Within SC's lifecycle it is often necessary to perform an upgrade or a bugfix.
- Contract's code and it's instance are separated.
- Contract's address can be updated to point to a different code stored on-chain.

NOTE:
- append only != immutable
- proxy pattern known from e.g. solidity is still possible
- within the Substrate framework contract's code is stored on-chain and it's instance is a pointer to that code

---

## Contracts upgradeability: access control

```rust [3]
#[ink(message)]
pub fn set_code(&mut self, code_hash: [u8; 32]) -> Result<()> {
    ensure_owner(self.env().caller())?;
    ink::env::set_code_hash(&code_hash)?;
    Ok(())
}

```

NOTE:
- you DO NOT want to leave this message un-guarded
- solutions to `ensure_owner` can range from a very simple ones address checks
- to a multiple-role database of access controled accounts stored and maintained in a separate cotnract

---

## Upgradeability: storage

<div style="font-size: 0.72em;">

```rust [1-4,6-10|1-4,12-16|18-21]
#[ink(message)]
pub fn get_values(&self) -> (u32, bool) {
    (self.x, self.y)
}

#[ink(storage)]
pub struct MyContractOld {
    x: u32,
    y: bool,
}

#[ink(storage)]
pub struct MyContractNew {
    y: bool,
    x: u32,
}

#[ink(message)]
pub fn get_values(&self) -> (u32, bool) {
    (self.y, self.x)
}
```

</div>

- Make sure your updated code is compatible with the existing contracts state
- Will this updated code work with the new definition and the old storage ?

NOTE:
- Various potential changes that can result in backwards incompatibility:
  - Changing the order of variables
  - Introducing new variable(s) before any of the existing ones
  - Changing variable type(s)
  - Removing variables
- Will this work? (no, SCALE encoding is oblivious to names, only order matters)

---

## Upgradeability: storage migrations

<div style="font-size: 0.82em;">

```rust [1-13|15-17]
// new contract code
#[ink(message)]
pub fn migrate(&mut self) -> Result<()> {
    if let Some(OldContractState { field_1, field_2 }) = get_contract_storage(&123)? {
        self.updated_old_state.set(&UpdatedOldState {
            field_1: field_2,
            field_2: field_1,
        });
        return Ok(());
    }

    return Err(Error::MigrationFailed);
}

// old contract code
#[ink(message)]
pub fn set_code(&mut self, code_hash: [u8; 32], callback: Option<Selector>)
```

</div>

NOTE:
- if the new contract code does not match the stored state you can perform a storage migration
- think of regular relational DB and schema migrations
- a good pattern to follow is to perform the update and the migration in one atomic transaction:
  - if anyting fails whole tx is reverted
  - won't end up in a broken state
  - make sure it can fit into one block!

---

## Common Vulnerabilities

```rust
impl MyContract {

  #[ink(message)]
  pub fn terminate(&mut self) -> Result<()> {
      let caller = self.env().caller();
      self.env().terminate_contract(caller)
  }

  ...
}
```

- What is wrong with this contract?
- How would you fix it?

NOTE:
- we start easy
- answer: no AC in place

---

## Common Vulnerabilities

```rust [3,8,12-14]
    #[ink(storage)]
    pub struct SubstrateNameSystem {
        registry: Mapping<AccountId, Vec<u8>>,
    }

    impl SubstrateNameSystem {
        #[ink(message, payable)]
        pub fn register(&mut self, name: Vec<u8>) {
            let owner = self.env().caller();
            let fee = self.env().transferred_value();

            if !self.registry.contains(owner) && fee >= 100 {
                self.registry.insert(owner, &name);
            }
        }
```

- On-chain domain name registry with a register fee of 100 pico.
- Why is this a bad idea?

NOTE:
everything on-chain is public
this will be front-run in no time
Can you propose a better design?
Answer: commit / reveal or an auction

---

## Common Vulnerabilities

<div style="font-size: 0.72em;">

```rust [3-7,12,18]

#[ink(message)]
pub fn swap(
    &mut self,
    token_in: AccountId,
    token_out: AccountId,
    amount_token_in: Balance,
) -> Result<(), DexError> {
    let this = self.env().account_id();
    let caller = self.env().caller();

    let amount_token_out = self.out_given_in(token_in, token_out, amount_token_in)?;

    // transfer token_in from user to the contract
    self.transfer_from_tx(token_in, caller, this, amount_token_in)?;

    // transfer token_out from contract to user
    self.transfer_tx(token_out, caller, amount_token_out)?;
    ...
}
```

</div>

- Contract is a <font color="#8d3aed">DEX</font> <font color="#8d3aed">D</font>ecentralized <font color="#8d3aed">EX</font>change, follows the popular <font color="#8d3aed">AMM</font> (<font color="#8d3aed">A</font>utomated <font color="#8d3aed">M</font>arket <font color="#8d3aed">M</font>aker) design.
- Tx swaps the specified amount of one of the pool's PSP22 tokens to another PSP22 token according to the current price.
- What can go wrong here?

NOTE:
Answer:
- no slippage protection in place.
- bot will frontrun the victim's tx by purchasing token_out before the trade is executed.
- this purchase will raise the price of the asset for the victim trader and increases his slippage
- if the bot sells right after the victims tx (back runs the victim) this is a sandwitch attack

---

## Common Vulnerabilities

- Integer overflows
- Re-entrancy vulnerabilities
- Sybil attacks
- ...
- Regulatory attacks :rofl:
- ...

NOTE:
- long list of possible attacks
- too long to fit into one lecture
- baseline: get an audit from a respectable firm
- publish your source code (security by obscurity is not securoty)

---


## Pause

NOTE:
Piotr takes over to talk about making runtime calls from contracts and writing automated tests

---

## Interacting with the execution environment

```rust [5-6]
impl MyContract {
  ...
  #[ink(message)]
  pub fn terminate(&mut self) -> Result<()> {
      let caller = self.env().caller();
      self.env().terminate_contract(caller)
  }
  ...
}
```

---

## Blockchain node onion

---

## Blockchain node onion
<br/>

<img style="margin-top: 50px;margin-bottom: 50px" width="800" src="img/ink/onions.png" />

---

## Blockchain node onion

<img style="margin-top: 10px" width="600" src="img/ink/blockchain-onion-1.svg" />

- networking
- block production, dissemination, finalization
- storage management
- off-chain maintenance, querying, indexing

---

## Blockchain node onion

<img style="margin-top: 50px;margin-bottom: 50px" width="800" src="img/ink/blockchain-onion-2.svg" />

- computing new state based on the previous one and a single transaction

---

## Blockchain node onion

<img style="margin-top: 100px;margin-bottom: 50px" width="800" src="img/ink/blockchain-onion-3.svg" />

- executing contract calls

---

## Standard API

- `caller()`
- `account_id()`
- `balance()`
- `block_number()`
- `emit_event(event: Event)`
- `transfer(dest: AccountId, value: Balance)`
- `hash_bytes(input: &[u8], output: &mut [u8])`
- `debug_message(msg: &str)`
- [_and many more_](https://docs.rs/ink_env/4.2.1/ink_env/index.html#functions)

---

## Standard API

```rust
impl MyContract {
  ...
  #[ink(message)]
  pub fn terminate(&mut self) -> Result<()> {
      let caller = self.env().caller();
      self.env().terminate_contract(caller)
  }
  ...
}
```
---

## Interacting with the state transition function

<br/>

<div class="flex-container fragment">
<div class="left">
<div style="text-align: center"> <center><h2><pre> User API </pre></h2></center> </div>

<ul>
<li>token transfer</li>
<li>staking</li>
<li>voting</li>
<li>contract call</li>
<li>...</li>
</ul>
</div>

<div class="left fragment">
<div style="text-align: center"> <center><h2><pre> Contract API </pre></h2></center> </div>

<ul>
<li>advanced cryptography</li>
<li>bypassing standard restrictions</li>
<li>outsourcing computation</li>
<li>...</li>
</ul>
</div>
</div>

---

## Interacting with the state transition function

<br/>

<div class="flex-container">
<div class="left">
<div style="text-align: center"> <center><h2><pre> User API </pre></h2></center> </div>
<div style="text-align: center"> <center><h2><pre> (usually for humans) </pre></h2></center> </div>

<ul>
<li>token transfer</li>
<li>staking</li>
<li>voting</li>
<li>contract call</li>
<li>...</li>

**_runtime call_**
</ul>
</div>

<div class="left">
<div style="text-align: center"> <center><h2><pre> Contract API </pre></h2></center> </div>
<div style="text-align: center"> <center><h2><pre> (only for contracts) </pre></h2></center> </div>

<ul>
<li>advanced cryptography</li>
<li>bypassing standard restrictions</li>
<li>outsourcing computation</li>
<li>...</li>

<br/>

**_chain extension_**
</ul>
</div>
</div>

---

## Runtime

<br/>

In Polkadot ecosystem _state transition function_ is called ***runtime***

---

## Calling runtime

<br/>

```rust [7-10]
#[ink(message)]
pub fn transfer_through_runtime(
    &mut self,
    receiver: AccountId,
    value: Balance,
) -> Result<(), RuntimeError> {
    let call_object = RuntimeCall::Balances(BalancesCall::Transfer {
        receiver,
        value,
    });
    
    self.env().call_runtime(&call_object)
}
```

---

## Calling runtime

<br/>

```rust [12]
#[ink(message)]
pub fn transfer_through_runtime(
    &mut self,
    receiver: AccountId,
    value: Balance,
) -> Result<(), RuntimeError> {
    let call_object = RuntimeCall::Balances(BalancesCall::Transfer {
        receiver,
        value,
    });
    
    self.env().call_runtime(&call_object)
}
```

---

## Chain extensions

<br/>

Chain extension is a way to extend the runtime with custom functionalities _dedicated to contracts_.

---

## Chain extensions

<br/>

**ink! side:**
 - provide `ChainExtension` trait
 - include extension in the `Environment` trait instantiation

<br/>

**runtime side:**
 - handling extension calls
 - extension logic itself

---

## Provide `ChainExtension` trait

```rust [1-7]
#[ink::chain_extension]
pub trait OutsourceHeavyCrypto {
  type ErrorCode = OutsourcingErr;

  #[ink(extension = 41)]
  fn outsource(input: Vec<u8>) -> [u8; 32];
}

pub enum OutsourcingErr {
  IncorrectData,
}

impl ink::env::chain_extension::FromStatusCode for OutsourcingErr {
  fn from_status_code(status_code: u32) -> Result<(), Self> {
    match status_code {
      0 => Ok(()),
      1 => Err(Self::IncorrectData),
      _ => panic!("encountered unknown status code"),
    }
  }
}
```

---

## Provide `ChainExtension` trait

```rust [9-21]
#[ink::chain_extension]
pub trait OutsourceHeavyCrypto {
  type ErrorCode = OutsourcingErr;

  #[ink(extension = 41)]
  fn outsource(input: Vec<u8>) -> [u8; 32];
}

pub enum OutsourcingErr {
  IncorrectData,
}

impl ink::env::chain_extension::FromStatusCode for OutsourcingErr {
  fn from_status_code(status_code: u32) -> Result<(), Self> {
    match status_code {
      0 => Ok(()),
      1 => Err(Self::IncorrectData),
      _ => panic!("encountered unknown status code"),
    }
  }
}
```

---

## Include extension in the `Environment` trait instantiation

<br/>

```rust
pub enum EnvironmentWithOutsourcing {}
impl Environment for EnvironmentWithOutsourcing {
    ... // use defaults from `DefaultEnvironment`
    type ChainExtension = OutsourceHeavyCrypto;
}

#[ink::contract(env = crate::EnvironmentWithOutsourcing)]
mod my_contract {
  ...
}
```

---

## Include extension in the `Environment` trait instantiation

<br/>

```rust
#[ink::contract(env = crate::EnvironmentWithOutsourcing)]
mod my_contract {
  fn process_data(&mut self, input: Vec<u8>) -> Result<(), OutsourcingErr> {
    self.env().extension().outsource(subject)
  }
}
```

---

## Handling extension calls

<br/>

```rust [5-11]
pub struct HeavyCryptoOutsourcingExtension;

impl ChainExtension<Runtime> for HeavyCryptoOutsourcingExtension {
  fn call<E: Ext>(&mut self, env: Env) -> Result<RetVal, DispatchError> {
    match env.func_id() {
      41 => internal_logic(),
      _ => {
        error!("Called an unregistered `func_id`: {func_id}");
        return Err(DispatchError::Other("Unimplemented func_id"))
      }
    }
    Ok(RetVal::Converging(0))
}
```

---

## Chain extension: reaching even further

<img style="margin-top: 100px;margin-bottom: 50px" width="800" src="img/ink/chain-extension-reach.svg" />

---

## Testing contracts

---

## Testing contracts

<br/>

<img style="margin-top: 100px;margin-bottom: 50px" width="800" src="img/ink/blockchain-onion-3.svg" />

---

## Testing contracts

<img style="margin-top: 100px;margin-bottom: 50px" width="1000" src="img/ink/testing-contract-stack.svg" />

---

## Unit tests

<br/>

```rust [1-3]
#[ink::test]
fn erc20_transfer_works() {
  let mut erc20 = Erc20::new(100);

  assert_eq!(erc20.balance_of(BOB), 0);
  // Alice transfers 10 tokens to Bob.
  assert_eq!(erc20.transfer(BOB, 10), Ok(()));
  // Bob owns 10 tokens.
  assert_eq!(erc20.balance_of(BOB), 10);

  let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
  assert_eq!(emitted_events.len(), 2);

  // Check first transfer event related to ERC-20 instantiation.
  assert_transfer_event(
    &emitted_events[0], None, Some(ALICE), 100,
  );
  // Check the second transfer event relating to the actual transfer.
  assert_transfer_event(
    &emitted_events[1], Some(ALICE), Some(BOB), 10,
  );
}
```

---

## Unit tests

<br/>

```rust [5-9]
#[ink::test]
fn erc20_transfer_works() {
  let mut erc20 = Erc20::new(100);

  assert_eq!(erc20.balance_of(BOB), 0);
  // Alice transfers 10 tokens to Bob.
  assert_eq!(erc20.transfer(BOB, 10), Ok(()));
  // Bob owns 10 tokens.
  assert_eq!(erc20.balance_of(BOB), 10);

  let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
  assert_eq!(emitted_events.len(), 2);

  // Check first transfer event related to ERC-20 instantiation.
  assert_transfer_event(
    &emitted_events[0], None, Some(ALICE), 100,
  );
  // Check the second transfer event relating to the actual transfer.
  assert_transfer_event(
    &emitted_events[1], Some(ALICE), Some(BOB), 10,
  );
}
```

---

## Unit tests

<br/>

```rust [11-22]
#[ink::test]
fn erc20_transfer_works() {
  let mut erc20 = Erc20::new(100);

  assert_eq!(erc20.balance_of(BOB), 0);
  // Alice transfers 10 tokens to Bob.
  assert_eq!(erc20.transfer(BOB, 10), Ok(()));
  // Bob owns 10 tokens.
  assert_eq!(erc20.balance_of(BOB), 10);

  let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
  assert_eq!(emitted_events.len(), 2);

  // Check first transfer event related to ERC-20 instantiation.
  assert_transfer_event(
    &emitted_events[0], None, Some(ALICE), 100,
  );
  // Check the second transfer event relating to the actual transfer.
  assert_transfer_event(
    &emitted_events[1], Some(ALICE), Some(BOB), 10,
  );
}
```

---

## E2E tests

<br/>

```rust [1-7]
#[ink_e2e::test]
async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
  let constructor = Erc20Ref::new(total_supply);
  let erc20 = client
          .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
          .await
          .expect("instantiate failed");
  
  let mut call = erc20.call::<Erc20>();
  let total_supply_msg = call.total_supply();
  let total_supply_res = client
          .call_dry_run(&ink_e2e::bob(), &total_supply_msg, 0, None)
          .await;
  ...
}
```

---

## E2E tests

<br/>

```rust [9-13]
#[ink_e2e::test]
async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
  let constructor = Erc20Ref::new(total_supply);
  let erc20 = client
          .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
          .await
          .expect("instantiate failed");
  
  let mut call = erc20.call::<Erc20>();
  let total_supply_msg = call.total_supply();
  let total_supply_res = client
          .call_dry_run(&ink_e2e::bob(), &total_supply_msg, 0, None)
          .await;
  ...
}
```

---

## E2E tests

<br/>

```rust [14]
#[ink_e2e::test]
async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
  let constructor = Erc20Ref::new(total_supply);
  let erc20 = client
          .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
          .await
          .expect("instantiate failed");
  
  let mut call = erc20.call::<Erc20>();
  let total_supply_msg = call.total_supply();
  let total_supply_res = client
          .call_dry_run(&ink_e2e::bob(), &total_supply_msg, 0, None)
          .await;
  ...
}
```

---

## E2E pipeline: traps, traps everywhere

<div style="font-size: 0.6em">

1. Preparing and encoding transaction data (_client side_)
1. Signing the transaction (_client side_)
1. Sending transaction to a node (_client side_)
1. Block and event subscribing (_client side_)
1. Transaction pool processing (_node side_)
1. Block building (_node side_)
1. Block dissemination (_node side_)
1. Import queue processing (_node side_)
1. Block finalizing (_node side_)
1. Block execution (_node side_)
1. Transaction execution (_runtime side_)
1. Event emitting (_node side_)
1. Event capturing (_client side_)
1. Event processing (_client side_)
1. State fetching via RPC calling (_client side_)
1. State report (_node side_)
1. State validation (_client side_)

</div>

---

## E2E pipeline: traps, traps everywhere

<img style="margin-top: 100px;margin-bottom: 50px" width="800" src="img/ink/trap.gif" />

---

## Test core

<br/>

1. Preparing and encoding transaction data (_given_)
1. Transaction execution (_when_)
1. State validation (_then_)

---

## quasi-E2E tests

<br/>

Interact directly with runtime, skipping node layer.


---

## quasi-E2E tests

<br/>

```rust
#[test]
fn flipping() -> Result<(), Box<dyn Error>> {
  let init_value = Session::<MinimalRuntime>::new(transcoder())?
      .deploy_and(bytes(), "new", &["true".to_string()], vec![])?
      .call_and("flip", &[])?
      .call_and("flip", &[])?
      .call_and("flip", &[])?
      .call_and("get", &[])?
      .last_call_return()
      .expect("Call was successful");

  assert_eq!(init_value, ok(Value::Bool(false)));

  Ok(())
}
```

---

## Local playing with contracts using `drink-cli`
