---
title: Wasm Smart Contracts in Ink!
description: A working programmer’s guide to the crypto industry
---

<img rounded style="width: 600px;" src="./img/ink/ink-logo-with-squid-white.svg" />

# Smart Contracts in Ink!

Notes:

- we will pause for questions

---

## Introduction: Smart Contracts

> Collection of code and data (sometimes referred to as functions and state) that is deployed using cryptographically signed transactions on the blockchain network.

**-- US** **N**ational **I**nstitute of **S**tandards and **T**echnology

---

## Introduction: Smart Contracts

> A contract is a legally binding agreement between two or more parties that creates an obligation to do or not do a particular thing.

**-- wiki**

Legal contracts need two things:

- **Intention**: The parties must **intend** to enter the contractual agreement.
- **Consideration**: Consideration emphasizes the reciprocal nature of a contract (_Quid Pro Quo_).

---

## Introduction: Smart Contracts

- Alice agrees to give Bob a monetary gift for his birthday:
  - Alice and Bob decide to use an escrow service to facilitate the transfer.
  - Alice sets up an account with the chosen escrow agent and transfers the money.
  - The escrow holds the funds until the condition is met.
  - On Bob's birthday the escrow releases the funds.

---

## Introduction: Smart Contracts

- Can we use a Smart Contract for this purpose?
- Is this still a legal contract if we do?

---

<!-- TODOs-->
<!-- gas metering -->
<!-- Turing completness -->

## Introduction: Smart Contracts

- Smart contracts run **deterministic** code.
- They execute it in the context of a blockchain protocol.
- They execute publicly: anyone can instantly track asset transfers and other related information.
  - They are **auditable** by anyone.
- They allow for transacting anonymously.

---

## Introduction: ink! vs. Solidity

- Two most active and maintained languages are **Solidity** and Vyper, with ink! being a recent runner-up.

|                 | ink!                        | Solidity      |
| --------------- | --------------------------- | ------------- |
| Virtual Machine | Any Wasm VM                 | EVM           |
| Encoding        | Wasm                        | EVM Byte Code |
| Language        | Rust                        | Standalone    |
| Constructors    | Multiple                    | Single        |
| Tooling         | Anything that supports Rust | Custom        |
| Storage         | Variable                    | 256 bits      |
| Interfaces?     | Yes: Rust traits            | Yes           |

Notes:

- why another SC language

---

## Intro: ink! & Substrate

<img rounded style="width: 900px;" src="./img/ink/lego0.png" />

Notes:

- Substrate is a framework for developing customized blockchain runtimes from composable pallets.

---

## Intro: ink! & Substrate

<img rounded style="width: 900px;" src="./img/ink/lego1.png" />

Notes:

- pallet contracts provides
  - instrumentation
  - execution engine
  - gas metering

---

<img rounded style="width: 800px;" src="./img/ink/schema1.png" />

Notes:

- pallet contracts is oblivious to the programming language
- it accepts Wasm bytecode and executes it's instructions

---

<img rounded style="width: 800px;" src="./img/ink/schema2.png" />

Notes:

- contracts itself can be written in ink!

---

<img rounded style="width: 800px;" src="./img/ink/schema3.png" />

Notes:

- any other language that compilers to Wasm
- Solang
- or ask!

---

<font color="#8d3aed">**Questions?**</font>

---

## Hands-on: my first ink! contract

- Visit: https://github.com/Polkadot-Blockchain-Academy/ink-hands-on
- Navigate to `escrow` and read the instructions.

Notes:

- Piotr takes over
- we will be writing an escrow contract
- probaly a lunch break after

---

## Developing Smart Contracts

Notes:

---

## Contracts: Defining shared behaviour

- `ink::trait_definition` describes the contract behaviour.

```rust
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

```

Notes:

- PSP22 (ERC20-like) contract definition

---

## Contracts: Defining shared behaviour

- contracts that respect the definition need to implement it.

```rust
impl PSP22 for Token {
    #[ink(message)]
    fn total_supply(&self) -> u128 {
        self.data.total_supply()
    }

    ...

```

Notes:

- utilizes "normal" rust traits with some macro magic

---

## Cross-contract calls

- Sharing the trait definition allows for e.g a cross-contract call.

```
impl SimpleDex {
    use psp22_trait::{PSP22Error, PSP22};

    /// Returns balance of a PSP22 token for an account
    fn balance_of(&self, token: AccountId, account: AccountId) -> Balance {
        let psp22: ink::contract_ref!(PSP22) = token.into();
        psp22.balance_of(account)
    }
    ...
```

Notes:

---

## Contracts: Error handling

<div style="font-size: 0.72em;">

```rust [9|3-6|15]
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

- ink! uses idiomatic Rust for error handling: `Result<T,E>` type
- Use the Err variant to pass your own semantics
- Type aliases reduce boilerplate & enhance readability

Notes:

- panic! = same as returning Err variant (`Result` is just being nice)

---

## Error handling: call stack

```rust
#[ink(constructor)]
pub fn new() -> Self {
    Self { value: false }
}

#[ink(message)]
pub fn flip(&mut self) {
    self.value = !self.value;

    if self.env().block_number() % 2 != 0 {
      panic!("Oh no!")
    }

}

```

- What is the state of this contract when tx is called in an odd block number?

Notes:

---

## Error handling: cross-contract calls

<div style="font-size: 0.7em;">

```rust [13-19]
#[ink(storage)]
pub struct TransferCount {
    counter: u32,
}

#[ink(message)]
pub fn transfer(
    &mut self,
    token: AccountId,
    to: AccountId,
    amount: u128,
) -> Result<(), MostError> {
    let mut psp22: ink::contract_ref!(PSP22) = token.into();
    if psp22.transfer(to, amount, vec![]).is_ok() {
        self.counter += 1;
    }

    Ok(())
}
```

</div>

- What is the state of this contract when the transfer fails?

Notes:

---

## Events

```rust [1-9|3-6]
#[ink(event)]
pub struct BetPlaced {
    #[ink(topic)]
    player: AccountId,
    #[ink(topic)]
    bet_type: BetType,
    amount: Balance,
}
```

- Events are a way of letting the outside world know about what's happening inside the contracts.

Notes:

---

## Events and errors

```rust
#[ink(message)]
pub fn flip(&mut self) {

    Self::emit_event(
        self.env(),
        Event::Flipped(Flipped { }),
    );

    self.value = !self.value;

    if self.env().block_number() % 2 == 0 {
      panic!("Oh no!")
    }

}

```

- Will this event be emitted in an odd block?

Notes:

---

## Importance of events

<img rounded style="width: 250px;" src="./img/ink/transacting.svg" />

- Imagine a handfull of addresses transacting in PSP22 tokens among themselves.
- Contract state stores just their cumulative balance

Notes:

---

## Event indexers

- blockchain = append-only database
- events = logs
- indexers = log processors

An **indexer** is an off-chain process that listens to the events, processes them and stores normalized data in a persistent, queryable manner.

---

## Event indexers

- [Subsquid: https://subsquid.io/](https://subsquid.io/)
- [TheGraph: https://thegraph.com/](https://thegraph.com/)

---

## Events as storage

- Events can be used as a cheaper form of storage.
- EVM logs cost 8 gas per byte, whereas contract storage costs 20,000 gas per 32 bytes.

---

<font color="#8d3aed">**Questions?**</font>

---

# Storage

---

## Storage: deeper dive

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

Notes:

---

<img rounded style="width: 1000px;" src="./img/ink/storage.svg" />

<font color="#8d3aed">SCALE</font> (_<font color="#8d3aed">S</font>imple <font color="#8d3aed">C</font>oncatenated <font color="#8d3aed">A</font>ggregate <font color="#8d3aed">L</font>ittle <font color="#8d3aed">E</font>ndian_)

Notes:

---

## SCALE: examples of different types

<div style="font-size: 0.72em;">

| Type           | Decoding                              | Encoding example          | Remark                                                                         |
| -------------- | ------------------------------------- | ------------------------- | ------------------------------------------------------------------------------ |
| Boolean        | false / true                          | 0x0 / 0x1                 | encoded using least significant (rightmost) bit of a single byte               |
| Int (unsigned) | 42u32                                 | 0x2a000000                |                                                                                |
| Tuple          | (42u16, true)                         | 0x2a0001                  | concatenation of each encoded value                                            |
| Enum           | enum IntOrBool { Int(u8), Bool(bool)} | 0x002a or 0x0101          | first byte encodes the variant index, remaining bytes encode the data          |
| Vector         | [42u8, 0, 1]                          | 0x0c2a0001                | encoding of the vector length followed by conatenation of each item's encoding |
| Struct         | {x:42u32, y:true}                     | [0x2a,0x00,0x00,0x00,0x1] | names are ignored, Vec<u8> structure, only order matters                       |

</div>

Notes:

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

- By default ink! stores all storage struct fields under a single storage cell (called the **Packed** layout)

Notes:

- two types of layout

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

- The storage key of the contracts root storage struct defaults to `0x00000000`
- However you may store it under any arbitrary 4 bytes key instead

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

Notes:

- demonstration of the packed layout - value is stored under the root key

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

- Mapping consists of a key-value pairs stored directly in the contract storage cells.
- Each Mapping value lives under it's own storage key.
- Mapping values do not have a contiguous storage layout: **it is not possible to iterate over the contents of a map!**

Notes:

---

## Storage: working with `Mapping`

```rust
pub fn transfer(&mut self) {
    let caller = self.env().caller();

    let mut balance = self.balances.get(caller).unwrap_or(0);
    let endowment = self.env().transferred_value();

    balance += endowment;
}

```

- what is wrong here?

Notes:

- Mapping::get() has copy semantics

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

- `Mapping::get()` returns a local copy, not a mutable reference to the storage!

Notes:

---

## Lazy Storage

<img rounded style="width: 1000px;" src="./img/ink/storage-layout.svg" />

Notes:

---

## Storage: Lazy

```rust [1,5]
use ink::storage::{traits::ManualKey, Lazy, Mapping};

#[ink(storage)]
pub struct Roulette {
    pub data: Lazy<Data, ManualKey<0x44415441>>,
    pub bets: Mapping<u32, Bet, ManualKey<0x42455453>>,
}
```

- Every type wrapped in `Lazy` has a separate storage cell.
- `ManualKey` assignes explicit storage key to it.

Notes:

---

## Storage: Lazy

- Why would you want to use a `ManualKey` instead of a generated one?

---

# Upgradeable contracts

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

Notes:

---

## Upgradeability: storage

<div style="font-size: 0.72em;">

```rust [1-4,6-10|1-4,12-16]
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
```

</div>

- Make sure your updated code is compatible with the existing contracts state.
- Will the getter work with the new definition and the old storage ?

Notes:

- Various potential changes that can result in backwards incompatibility:
  - Changing the order of variables
  - Introducing new variable(s) before any of the existing ones
  - Changing variable type(s)
  - Removing variables

---

## Hands-on: storage migrations

<!-- <div style="font-size: 0.70em;"> -->

<!-- ```rust [] -->
<!-- pub struct OldState { -->
<!--     pub field_1: u32, -->
<!--     pub field_2: bool, -->
<!-- } -->

<!-- #[ink(storage)] -->
<!-- pub struct OldA { -->
<!--     old_state: Lazy<OldState, ManualKey<123>>, -->
<!-- } -->

<!-- #[ink(message)] -->
<!-- pub fn set_code(&mut self, code_hash: [u8; 32], callback: Option<Selector>) { -->
<!--   set_code_hash(&code_hash)?; -->
<!--   call_migrate()?; // delegates a `migrate` call to the new contract code, but using callers context -->
<!-- } -->
<!-- ``` -->

<!-- ```rust [] -->
<!-- pub struct UpdatedOldState { -->
<!--     pub field_1: bool, -->
<!--     pub field_2: u32, -->
<!-- } -->

<!-- #[ink(storage)] -->
<!-- pub struct NewA { -->
<!--     updated_old_state: Lazy<UpdatedOldState, ManualKey<123>>, -->
<!-- } -->

<!-- #[ink(message, selector = 0x4D475254)] -->
<!-- pub fn migrate(&mut self) -> Result<()> { -->
<!--     if let Some(OldContractState { field_1, field_2 }) = get_contract_storage(&123)? { -->
<!--         self.updated_old_state.set(&UpdatedOldState { -->
<!--             field_1: field_2, -->
<!--             field_2: field_1, -->
<!--         }); -->
<!--         return Ok(()); -->
<!--     } -->
<!--     return Err(Error::MigrationFailed); -->
<!-- } -->
<!-- ``` -->

<!-- </div> -->

<!-- Notes: -->

<!-- - if the new contract code does not match the stored state you can perform a storage migration -->
<!-- - think of regular relational DB and schema migrations -->
<!-- - a good pattern to follow is to perform the update and the migration in one atomic transaction: -->
<!--   - if anything fails whole tx is reverted -->
<!--   - won't end up in a broken state -->
<!--   - make sure it can fit into one block! -->

---

<!-- ## Contracts upgradeability: access control -->

<!-- ```rust [3] -->
<!-- #[ink(message)] -->
<!-- pub fn set_code(&mut self, code_hash: [u8; 32]) -> Result<()> { -->
<!--     ensure_owner(self.env().caller())?; -->
<!--     ink::env::set_code_hash(&code_hash)?; -->
<!--     Ok(()) -->
<!-- } -->

<!-- ``` -->

<!-- Notes: -->

<!-- - you DO NOT want to leave this message un-guarded -->
<!-- - solutions to `ensure_owner` can range from a very simple ones address checks -->
<!-- - to a multiple-role database of access controlled accounts stored and maintained in a separate contract -->

<!-- --- -->

# Common Vulnerabilities

---

## Past exploits: The DAO hack (2016)

- The DAO (Decentralized Autonomous Organization) was a crowdfunding project on the Ethereum network.
- In June 2016 an attacker drained funds (worth ~$50 million at that time) and was about to drain even more.
- The funds were moved into an account subject to a 28-day holding period under the terms of the DAO smart contract.

---

## Past exploits: The DAO hack (2016)

- The core Ethereum faced a difficult decision:
  - On one hand the blockchain promised to be decentralized and tamper-resistant.
  - On the other hand the public's confidence and optimism about the then young blockchain technology demanded an intervention.

---

## Past exploits: The DAO hack (2016)

- Eventually a vote in favour of **forking** the Ethereum history won out with 85% of the votes.
- Some miners decided to keep mining on the old history, and this sesulted in the creation of _Ethereum_ (ETH) and _Ethereum Classic_ (ETC) which operates to this day.

---

## Reentrancy attacks

<img style="margin-top: 50px;margin-bottom: 50px" width="600" src="./img/ink/reentrancy.png" />

- The DAO hacker used what became known as a _reentrancy_ attack.
- Attacker exploited a _fallback_ function in Solidity to create a loop that slowly syphoned funds out of the DAO contract.
- Fallback functions are special constructs in Solidity that are triggered in specific situations.

---

## Reentrancy: the fallback function

```solidity []
pragma solidity ^0.8.0;

contract FallbackExample {
    event Received(address sender, uint value);

    // Payable fallback function
    receive() external payable {
        emit Received(msg.sender, msg.value);
    }
}

```

- The fallback function does not take any arguments and has no return values.
- It can be marked as **payable** to allow the contract to receive funds.
- It is triggered if ETH is sent to the contract and there is no accompanying _calldata_ (a data location like memory or storage).

---

## Reentrancy: an example

<div style="font-size: 0.70em;">

```rust [1-4|6-15|17-33]
#[ink(storage)]
pub struct Dao {
    balances: Mapping<AccountId, Balance>,
}

#[ink(message, payable, selector = 0x4445504F)]
pub fn deposit(&mut self) -> Result<(), DaoError> {
    let caller = self.env().caller();

    let current_balance = self.balances.get(caller).unwrap_or_default();
    let updated_balance = current_balance + self.env().transferred_value();

    self.balances.insert(caller, &updated_balance);
    Ok(())
}

#[ink(message)]
pub fn withdraw(&mut self) -> Result<(), DaoError> {
    let caller = self.env().caller();
    let balance = self.balances.get(caller).ok_or(DaoError::NoDeposit)?;

    build_call::<DefaultEnvironment>()
        .call(caller)
        .call_flags(CallFlags::default())
        .exec_input(ExecutionInput::new(Selector::new([0x52, 0x45, 0x43, 0x56])))
        .transferred_value(balance)
        .returns::<()>()
        .try_invoke()??;

    self.balances.remove(caller);

    Ok(())
}
```

</div>

---

## Reentrancy: attacker's code example

<div style="font-size: 0.70em;">

```rust[3-9|11-12|17-22]
#[ink(message)]
pub fn attack(&mut self) -> Result<(), AttackerError> {
    build_call::<DefaultEnvironment>()
        .call(self.dao_address)
        .call_flags(CallFlags::default().set_allow_reentry(true))
        .exec_input(ExecutionInput::new(Selector::new([0x44, 0x45, 0x50, 0x4F])))
        .transferred_value(ONE_AZERO)
        .returns::<Result<(), DaoError>>()
        .invoke()?;

    let mut dao = DaoRef::from_account_id(self.dao_address);
    dao.withdraw()?;

    Ok(())
}

#[ink(message, payable, selector = 0x52454356)]
pub fn receive(&mut self) -> Result<(), DaoError> {
    let mut dao = DaoRef::from_account_id(self.dao_address);
    dao.withdraw()?;
    Ok(())
}
```

</div>

---

## Reentrancy: fixing the vulnerabilities

Checks-Effects-Interactions pattern

<div style="font-size: 0.70em;">

```rust[6,16]
#[ink(message)]
pub fn withdraw(&mut self) -> Result<(), DaoError> {
    let caller = self.env().caller();
    let balance = self.balances.get(caller).ok_or(DaoError::NoDeposit)?;

    self.balances.remove(caller);

    build_call::<DefaultEnvironment>()
        .call(caller)
        .call_flags(CallFlags::default())
        .exec_input(ExecutionInput::new(Selector::new([0x52, 0x45, 0x43, 0x56])))
        .transferred_value(balance)
        .returns::<()>()
        .invoke();

    // self.balances.remove(caller);

    Ok(())
}
```

</div>

---

<!-- ## Reentrancy: fixing the vulnerabilities -->

<!-- <div style="font-size: 0.70em;"> -->

<!-- ```rust [8] -->
<!-- #[ink(message)] -->
<!-- pub fn withdraw(&mut self) -> Result<(), DaoError> { -->
<!--     let caller = self.env().caller(); -->
<!--     let balance = self.balances.get(caller).ok_or(DaoError::NoDeposit)?; -->

<!--     build_call::<DefaultEnvironment>() -->
<!--         .call(caller) -->
<!--         .call_flags(CallFlags::default().set_allow_reentry(false)) -->
<!--         .exec_input(ExecutionInput::new(Selector::new([0x52, 0x45, 0x43, 0x56]))) -->
<!--         .transferred_value(balance) -->
<!--         .returns::<()>() -->
<!--         .invoke(); -->

<!--     self.balances.remove(caller); -->

<!--     Ok(()) -->
<!-- } -->
<!-- ``` -->

<!-- </div> -->

<!-- Note: -->

<!-- - Another fix id to contract to explicitely dissallow re-entering the `receive` function while the `withdraw` it is still executing. -->
<!-- - this way any re-entry is blocked -->
<!-- - ink! has built-in syntax for this, that works on the calls stack level -->
<!-- - mutex (mutually exclusive) pattern -->

<!-- --- -->

<!-- ## The Parity Wallet Hack (July 2017) -->

<!-- <img style="margin-top: 10px;margin-bottom: 10px" width="900" src="./img/ink/multisig_exploit_1.png" /> -->

<!-- - Vulnerability on the Parity Multisig Wallet allowed an attacker to steal > 150,000 ETH. -->
<!-- - Three high-profile multisig wallet contracts used to store funds from token sales were affected. -->
<!-- - The attacker sent 2 transactions to each of the affected contracts: -->
<!--   - the first to obtain ownership of the MultiSig. -->
<!--   - the second to move all of its funds. -->

<!-- --- -->

<!-- ## The Parity Wallet Hack: proxy pattern -->

<!-- ```solidity [] -->
<!-- address constant _walletLibrary = 0xcafecafecafecafecafecafecafecafecafecafe; -->

<!-- ... -->

<!-- function isOwner(address _addr) constant returns (bool) { -->
<!--   return _walletLibrary.delegatecall(msg.data); -->
<!-- } -->
<!-- ``` -->

<!-- - **Proxy pattern** is a way to reduce costs by sharing code between contracts. -->
<!-- - All logic is stored in a stateless library contract deployed once, and a lightweight contract proper is deployed as many times as neccesary. -->
<!-- - `DELEGATECALL` EVM instruction does the following: -->
<!--   - for whatever method that calls it, it will delegate that call to another contract, but using the context of the current contract. -->

<!-- Notes: -->

<!-- - ethereum's EVM does not have the separation between code and the SC instance like substrate does. -->
<!-- - think `super` call in Java. -->
<!-- - context = state, state of a library is not altered when delgating a call. -->
<!-- - all perfectly innocent and used ot this day, problem lies elsewhere. -->

<!-- --- -->

<!-- ## The Parity Wallet Hack: `initWallet` library function -->

<!-- <div style="font-size: 0.70em;"> -->

<!-- ```[1-4|6-16] -->
<!-- function initWallet(address[] _owners, uint _required, uint _daylimit) { -->
<!--   initDaylimit(_daylimit); -->
<!--   initMultiowned(_owners, _required); -->
<!-- } -->

<!-- function initMultiowned(address[] _owners, uint _required) { -->
<!--   m_numOwners = _owners.length + 1; -->
<!--   m_owners[1] = uint(msg.sender); -->
<!--   m_ownerIndex[uint(msg.sender)] = 1; -->
<!--   for (uint i = 0; i < _owners.length; ++i) -->
<!--   { -->
<!--     m_owners[2 + i] = uint(_owners[i]); -->
<!--     m_ownerIndex[uint(_owners[i])] = 2 + i; -->
<!--   } -->
<!--   m_required = _required; -->
<!-- } -->
<!-- ``` -->

<!-- </div> -->

<!-- - wallet library contained `initWallet` function that was called from the wallets constructor. -->
<!-- - and _i.e._ it called this logic. -->

<!-- Notes: -->

<!-- - it writes the owners of this contract to the contracts storage (state) -->
<!-- - still nothing technically nothing wrong here -->

<!-- --- -->

<!-- ## The Parity Wallet Hack: the critical mistake -->

<!-- ```solidity [] -->
<!-- function() payable { -->
<!--   // just being sent some cash? -->
<!--   if (msg.value > 0) -->
<!--     Deposit(msg.sender, msg.value); -->
<!--   else if (msg.data.length > 0) -->
<!--     _walletLibrary.delegatecall(msg.data); -->
<!-- } -->
<!-- ``` -->

<!-- - This exact code was defined in the wallet itself. -->
<!-- - Do you see what happens here? -->
<!-- - How would you fix this? -->

<!-- Notes: -->

<!-- - reckognize this? yes, this is a fallback function -->
<!-- - who can tell me what happens here? -->
<!-- - If a method with this name is not defined in this contract -->
<!-- - and if no ETH is being sent in the transaction -->
<!-- - and if there is some data in the message payload -->
<!-- - Then call the exact same method as it is defined in \_walletLibrary (but using `delegatecall` that is in the context of this contract) -->
<!-- - the hacker effectively re-initialized the wallet, overwrote the owners making himself the sole owner and stole the funds. -->

<!-- --- -->

<!-- ## The Parity Wallet Hack: the aftermath -->

<!-- - The `initWallet` and `initMultiowned` should have been defined as `internal` (aka private). -->
<!-- - There was no check in place for whether the wallet was already initialized. -->
<!-- - The raw `DELEGATECALL` should haven been replaced with a whitelist of calls that can be delegated. -->

<!-- Notes: -->

<!-- - so what was at fault here? -->
<!-- - you could argue there were at least three vulnerabilities -->
<!-- - fixing either of them would have prevented the exploit -->
<!-- - akin to e.g. running unsanitized SQL (sql injections). -->

<!-- --- -->

<!-- ## The Parity Wallet Hack: reloaded (November 2017) -->

<!-- <img rounded style="width: 900px;" src="./img/ink/anyone_can_kill_it.jpg" /> -->

<!-- - Following the fix for the original multisig vulnerability exploited in July a new version of the library contract was deployed. -->
<!-- - Unfortunately it contained another vulnerability ... -->
<!-- - Estimated losses totalled over 500,000 ETH. -->

<!-- Notes: -->

<!-- - USD 150 million USD back then -->
<!-- - including over 300,000 ETH from the Web3 Foundation team. -->
<!-- - hacker did not walk with funds this itme, rather locked them forever -->

<!-- --- -->

<!-- ## The Parity Wallet Hack reloaded: the what? -->

<!-- ```solidity [] -->
<!-- function kill(address _to) onlymanyowners(sha3(msg.data)) external { -->
<!--   suicide(_to); -->
<!-- } -->
<!-- ``` -->

<!-- - library contract contained this function. -->
<!-- - `suicide` (now `selfdestruct`) opcode was added to the EVM after the DAO hack. -->
<!-- - It removes a contract from the blockchain and sends its ETH balance to a designated recipient. -->
<!-- - Hacker was able to call this function on the _library_ contract itself, remove it and rendered all of the proxy contracts broken. -->
<!-- - `kill` is protected with a `onlymanyowners` modifier so how was he able to call it? -->

<!-- Notes: -->

<!-- - The DAO attack continued for days due to the immutability of Solidity contracts -->
<!-- - remmeber how whitehat hackers tried to syphon the funds only faster than the blackhat? -->
<!-- - This is why it was introduced - as a safety feature in case of security threats -->

<!-- --- -->

<!-- ## The Parity Wallet Hack reloaded: the how? -->

<!-- <div style="font-size: 0.72em;"> -->

<!-- ```solidity [] -->
<!-- uint public m_numOwners; -->

<!-- modifier only_uninitialized { if (m_numOwners > 0) throw; _; } -->

<!-- function initWallet(address[] _owners, uint _required, uint _daylimit) only_uninitialized -->
<!--   initDaylimit(_daylimit); -->
<!--   initMultiowned(_owners, _required); -->
<!-- } -->

<!-- function initMultiowned(address[] _owners, uint _required) internal { -->
<!--   ... -->
<!-- } -->
<!-- ``` -->

<!-- </div> -->

<!-- - in the aftermath of the July attack the above changes were added to the library: -->
<!--   - `initMultiowned` was given _internal_ visibility. -->
<!--   - `only_uninitialized` check was added. -->
<!-- - So what happened? -->

<!-- Notes: -->

<!-- - that does seems to fix the problem: if the attacker attempts to invoke `initWallet` on an already deployed contract it is rejected -->
<!-- - can anyone say what has happened? -->

<!-- --- -->

<!-- ## The Parity Wallet Hack reloaded: the how? -->

<!-- <div style="font-size: 0.72em;"> -->

<!-- ```solidity [] -->
<!-- uint public m_numOwners; -->

<!-- modifier only_uninitialized { if (m_numOwners > 0) throw; _; } -->

<!-- function initWallet(address[] _owners, uint _required, uint _daylimit) only_uninitialized -->
<!--   initDaylimit(_daylimit); -->
<!--   initMultiowned(_owners, _required); -->
<!-- } -->

<!-- function initMultiowned(address[] _owners, uint _required) internal { -->
<!--   ... -->
<!-- } -->

<!-- function kill(address _to) onlymanyowners(sha3(msg.data)) external { -->
<!--   suicide(_to); -->
<!-- } -->
<!-- ``` -->

<!-- - by default, Solidity functions have _public_ visibility. -->
<!-- - `initWallet` was public in the library itsef. -->
<!-- - By calling it hacker has made himself the owner of the library (since `m_numOwners == 0` in an un-initialized contract) and than called `kill`, passing his own address. -->
<!-- - [Details](https://github.com/openethereum/parity-ethereum/issues/6995) of the exploit: https://etherscan.io/address/0x863df6bfa4469f3ead0be8f9f2aae51c91a907b4#code -->

<!-- </div> -->

<!-- Notes: -->

<!-- - it had to be, all the library functions have to be public to be callable from the outside -->
<!-- - this time around hacker walked with no funds (as the lib had none) -->

<!-- --- -->

<!-- ## The Parity Wallet Hack reloaded: lessons for ink! development -->

<!-- ```rust -->
<!-- impl MyContract { -->

<!--   #[ink(message)] -->
<!--   pub fn terminate(&mut self) -> Result<()> { -->
<!--       let caller = self.env().caller(); -->
<!--       self.env().terminate_contract(caller) -->
<!--   } -->

<!--   ... -->
<!-- } -->
<!-- ``` -->

<!-- Notes: -->

<!-- - substate /ink! usually has no need for a proxy pattern but similar concepts do apply -->
<!-- - what is wrong with this code? -->
<!-- - how would you fix it? -->

<!-- --- -->

<!-- ## The Parity Wallet Hack reloaded: lessons for ink! development -->

<!-- ```rust [4,8-14] -->
<!-- #[ink(message)] -->
<!-- pub fn terminate(&mut self) -> Result<()> { -->
<!--     let caller = self.env().caller(); -->
<!--     self.ensure_owner()?; -->
<!--     self.env().terminate_contract(caller) -->
<!-- } -->

<!-- fn ensure_owner(&self) -> Result<(), GovernanceError> { -->
<!--     let caller = self.env().caller(); -->
<!--     match caller.eq(&self.owner) { -->
<!--         true => Ok(()), -->
<!--         false => Err(Error::NotOwner), -->
<!--     } -->
<!-- } -->
<!-- ``` -->

<!-- Notes: -->

<!-- - making sure only designated account(s) can call `terminate` -->

<!-- --- -->

## The BatchOverflow exploit (April 2018)

<img style="margin-top: 10px;margin-bottom: 10px" width="800" src="./img/ink/batch_overflow.png" />

- In two transaction an attacker was withdrew **~115e57 (115 octodecillion)** BEC (Beauty Coin).
- BEC was trading at USD 0.32, which makes the total value of the exploit **USD 3.7e60 (3.7 novemdecillion)**.
- It was followed by a number of similar exploits targeting ERC20 tokens, prompting many exchanges to halt ERC20 trading altogether.

---

## The BatchOverflow exploit: integer overflow

```rust
        128u8 + 128u8 == 0;
```

- Integer _overflow_ and _underflow_ occur when user supplied data **controls the value of an unsigned integer.**
- The user supplied data either adds to or subtracts beyond the limits of what the variable type can hold, causing it to **wrap around**.
- u8 holds 8 bits and `(128+128) mod 2^8 = 0`

---

## The BatchOverflow exploit: example

<div style="font-size: 0.72em;">

```rust [1|3-32]
type MyBalance = u8;

#[ink(message)]
pub fn batch_transfer(
    &mut self,
    receivers: Vec<AccountId>,
    value: MyBalance,
) -> Result<(), Error> {
    let count = receivers.len();

    if (count == 0 || count > 20 || value == 0) {
        return Err(Error::CannotTransfer);
    }

    let caller = self.env().caller();
    let amount = count as MyBalance * value;
    let caller_balance = self.get_balance(caller);

    if caller_balance < amount {
        return Err(Error::SenderBalanceTooLow);
    }

    self.balances.insert(caller, &(caller_balance - amount));

    for i in 0..cnt {
        let receiver = receivers.get(i).unwrap();
        let receiver_balance = self.get_balance(*receiver);
        self.balances.insert(receiver, &(receiver_balance + value));
    }

    Ok(())
}
```

</div>

Notes:

- can you spot the problem? Which line exactly is problematic?
- actually Rust is pretty good at catching runtime overflows, so this will panic, unless

```toml
[profile.dev]
overflow-checks = false
[profile.release]
overflow-checks = false
```

---

## The BatchOverflow exploit: example

<div style="font-size: 0.70em;">

```rust []
let caller = default_accounts::<DefaultEnvironment>().alice;
set_caller::<DefaultEnvironment>(caller);

let mut token = Overflows::new();
let receiver_one = default_accounts::<DefaultEnvironment>().bob;
let receiver_two = default_accounts::<DefaultEnvironment>().charlie;

let sender_balance_before = token.get_balance(caller);
assert_eq!(0, sender_balance_before);

let receiver_balance_before = token.get_balance(receiver_one);
assert_eq!(0, receiver_balance_before);

token
    .batch_transfer(vec![receiver_one, receiver_two], 128)
    .expect("Can transfer");

let receiver_balance_after = token.get_balance(receiver_one);

assert_eq!(128, receiver_balance_after);
```

</div>

Notes:

- neither sender nor receivers had any funds
- yet 128 tokens were created out of thin air

---

## The BatchOverflow exploit: example

<img style="margin-top: 10px;margin-bottom: 10px" height="400" src="./img/ink/printer.gif" />

- money printer goes brrr...
- How would you fix it?

</div>

Notes:

- money out of thin air
- see a fix?

---

## The BatchOverflow exploit: fixit

```rust [1|3-14]
let amount = Self::safe_multiply(count as MyBalance, value)?;

fn safe_multiply(a: MyBalance, b: MyBalance) -> Result<MyBalance, MyError> {
    if a == 0 || b == 0 {
        return Ok(0);
    }

    let c = a * b;
    if c / a == b {
        return Ok(c);
    }

    Err(MyError::Arithmetic)
}

```

- We perform a reverse operation (division)
- Value _c_ is divided by _a_ and we check whether this is equal to the value of _b_
- If this was an overflow and wrapped around back to 0 then this check fails incorrect.

---

## The BatchOverflow exploit: fixit

- runtime checks in Rust are performed only in _debug_ mode, as they hinder performance

```toml
[profile.release]
overflow-checks = false
```

- This prints 0 in _release_ mode and panics in _debug_:

```rust
let x: u8 = "128".parse().unwrap();
let val: u8 = x + 128;
println!("{}", val);
```

- it's best to be explicit:

```rust
let amount = (count as MyBalance).checked_mul(value)
              .ok_or(MyError::Arithmetic)?;
```

Notes:

- in Rust there is no need to roll your own code
- in Solidity take a look at the SafeMath library
- explicit to the compiler about what you want to do

---

<!-- # Maximal Extractable Value (MEV) -->

<!-- Notes: -->

<!-- - we will now talk about MEV (Maximal extractable value) -->

<!-- --- -->

<!-- # MEV -->

<!-- - The concept of MEV was first floated as early as 2014 (Ethereum pre-genesis), in the context of Proof-of-work. -->
<!-- - It was referred to as the _invisible tax_, the maximum value a miner can extract from moving around transactions when producing a block on a blockchain network. -->
<!-- - After the Merge (Ethereum's move to POS consensus) **Miner Extractable Value** became **Maximal extractable value**. -->

<!-- Notes: -->

<!-- - MEV skyrocketed during the 2021 DeFi summer. -->
<!-- - MEV was first applied in the context of proof-of-work, and referred to as the Miner Extractable Value. -->
<!-- - This is because in POW miners hold most of the power, controlling the transaction inclusion / exclusion and ordering. -->
<!-- - However in the proof-of-stake the validators are been responsible for these roles. -->
<!-- - The value extraction methods still exist though, so the term "Maximal extractable value" is now used instead. -->

<!-- --- -->

<!-- ## MEV: The Dark Forest -->

<!-- - We have now seen a number of smart contract exploits. -->
<!-- - Blockchain is a highly **competetive and adversarial** environment. -->
<!-- - But the dangers pale in comparison to the **mempool**. -->

<!-- Notes: -->

<!-- - if a smart contract can be exploited for profit it's just a matter of time when it will be. -->
<!-- - there is a lot af smart ppl spending a lot of time examining contracts for vulnerabilities. -->
<!-- - mempool is the set of pending, unconfirmed transactions. -->

<!-- --- -->

<!-- ## MEV: The Dark Forest -->

<!-- <img style="margin-top: 10px;margin-bottom: 10px" height="400" src="./img/ink/dark_forest.jpg" /> -->

<!-- <div style="font-size: 0.72em;"> -->

<!-- - Novel by Cixin Liu describes the concept of a "dark forest", the ultimate adversarial environment, where detection means certain destuction from the hands of apex predators. -->
<!-- - **Generalized Frontrunners** are bots looking for _any_ profitable transactions submitted to the mempool. -->
<!-- - _Ethereum is a Dark Forest, Dan Robinson and Georgios Konstantopoulos_ **[August 2020]**. -->

<!-- Notes: -->

<!-- - Cixin Liu [Si-Szin Lju] -->
<!-- - who's familiar with these novels? -->
<!-- - The Dark Forest is a 2-nd book in the sci-fi series Remembrance of the Earth's Past -->
<!-- - Publicly identifying someone else’s location is as good as directly destroying them -->
<!-- - These Frontrunners work by copying it and replacing the address with their own -->
<!--   - They can even execute the transaction in a sandbox and submit just the profitable internal transactions as their own. -->
<!--   - One encounter with such a bot was described by these two researchers. -->

<!-- </div> -->

<!-- --- -->

<!-- ## The Dark Forest -->

<!-- <img style="margin-top: 10px;margin-bottom: 10px" width="800" src="./img/ink/kurzgesagt.jpg" /> -->

<!-- <div style="font-size: 0.70em;"> -->

<!-- credit: **Kurzgesagt** , _Why We Should NOT Look For Aliens - The Dark Forest_ -->

<!-- </div> -->

<!-- - The Dark Forest theory is (one of) a solution to the Fermi's paradox. -->
<!-- - Great explanation in this episode of the Kurzgesagt. -->

<!-- Notes: -->

<!-- - also Stanislaw Lem in (_The New Cosmogony_, A Perfect Vacuum, 1971) -->
<!-- - David Brin (astronomer and author) -->

<!-- --- -->

<!-- ## MEV: The Dark Forest -->

<!-- - Someone had asked on the _#support_ channel of Uniswap whether it was possible to recover liquidity tokens that were erronously sent to the <font color="#8d3aed">[liquidity token base contract](https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol#L140)</font>. -->
<!-- - This meant _anyone_ who calls _burn_ on the Uniswap contract, passing own address, would receive the extra tokens (worth _USD 12K_). -->
<!-- - **Dan Robinson**, a researcher and a white-hat hacker offered to help. -->

<!-- Notes: -->

<!-- - Not even a bug, jusy a by-product of how UniswapV2 is designed -->
<!-- - For details I refer you to the original article -->
<!-- - Only that he knew this wouldn't be as easy as calling burn and returning the tokens to the owner -->
<!-- - ... because of the monsters in the forest -->

<!-- --- -->

<!-- ## MEV: The Dark Forest -->

<!-- <img style="margin-top: 10px;margin-bottom: 10px" width="900" src="./img/ink/flashboys.png" /> -->

<!-- - In another article, _Phil Daian et al, 2019_ talked about how one particular species of frontrunning bots, called **Generalized Frontrunners** scans the mempool for profitable transactions. -->
<!-- - If someone just submitted a _burn_ transaction to the mempool the Dark Forest bots would be imediately alerted. -->

<!-- Notes: -->

<!-- - Dan was well aware this would not be easy -->
<!-- - it was a ticking time bomb -->
<!-- - They had to desing an obfuscated rescue method to try and not alert the bots about the free money. -->

<!-- --- -->

<!-- ## The Dark Forest: To The Rescue -->

<!-- - Dan Robinson recruited the help of fellow engineers (i.e. [Georgios Konstantopoulos]() and [Alberto Cuesta Canada]() and a few others) and they came up with the following solution: -->

<!-- <div style="font-size: 0.62em;"> -->

<!-- ```Solidity [1-7 | 9-21 | 23-47] -->
<!-- interface IGetter { -->
<!--   function set(bool) external; -->
<!-- } -->

<!-- interface IPool { -->
<!--   function burn(address to) external returns (uint amount0, uint amount1); -->
<!-- } -->

<!-- contract Setter { -->

<!--   address private owner; -->

<!--   constructor () public { -->
<!--     owner = msg.sender; -->
<!--   } -->

<!--   function set(address getter, bool on) public { -->
<!--     require(msg.sender == owner, "no-owner"); -->
<!--     IGetter(getter).set(on); -->
<!--   } -->
<!-- } -->

<!-- Contract Getter is IGetter { -->
<!--   IPool private pool; -->
<!--   address private setter; -->
<!--   address private getter; -->
<!--   address private dest; -->
<!--   bool private on; -->

<!--   constructor(address pool_, address setter_, address getter_, address dest_) public { -->
<!--     pool = IPool(pool_); -->
<!--     setter = setter_; -->
<!--     getter = getter_; -->
<!--     dest = dest_; -->
<!--   } -->

<!--   function set(bool on_) public override { -->
<!--     require(msg.sender == setter, "no-setter"); -->
<!--     on = on_; -->
<!--   } -->

<!--   function get() public { -->
<!--     require(msg.sender == getter "no-getter"); -->
<!--     require(on == true, "no-break"); -->
<!--     pool.burn(dest); -->
<!--   } -->
<!-- } -->
<!-- ``` -->

<!-- </div> -->

<!-- Notes: -->

<!-- - the call to _burn_ is hidden inside a larger _get_ tx -->
<!-- - the tx is split into two, among two contracts: -->
<!--   - _Setter_ contract which, when called by its owner, would activate the Getter contract. -->
<!--   - _Getter_ when called by its owner, would make the burn call ONLY if activated else it reverts. -->
<!-- - Can you see the problem (or the challenge here?) -->

<!-- --- -->

<!-- ## The Dark Forest: the Monsters are real -->

<!-- - During the rescue attempts the **get** transaction would get rejected by the Infura node. -->
<!-- - Due to the time pressure and late night time, the final **get** tx slipped into a later block. -->
<!-- - When the it was finally executed it reverted with **INSUFFICIENT_LIQUIDITY_BURNED**, meaning a bot had already executed the internal _burn_ call and took the funds. -->

<!-- Notes: -->

<!-- - If the attacker only tried executing the get transaction, it would revert without calling the burn function. -->
<!-- - The hope was that by the time the attacker executed both the _set_ and _get_ txs in a sequence, to spot the internal call to _pool.burn_ and frontrun the rescue attempt, the get transaction would already be included in a mined block -->
<!-- - BUT : this means the _set_ and _get_ txs **have** to be included in the same block. -->
<!-- - Do you see what could have been done better? -->

<!-- --- -->

<!-- ## The Dark Forest: the Monsters are real -->

<!-- - Avoid public infrastructure. -->
<!-- - Some examples of a private pool transactions offerings: -->
<!--   - <font color="#8d3aed">[blocxroute](https://bloxroute.com/products/)</font><\!-- markdown-link-check-disable-line -\-> -->
<!--   - <font color="#8d3aed">[taichi network](https://taichi.network/)</font> -->
<!--   - <font color="#8d3aed">[1inch network](https://1inch.io/)</font> -->

<!-- Notes: -->

<!-- - maybe a better obfuscation (e..g get could be a no-op instea dof reverting if called without set) -->
<!--   - causing a bot to miss the internal tx -->
<!-- - contact a miner to manually include your tx in a block, skipping the mempool -->
<!-- - or sync your own node -->

<!-- --- -->

## Frontrunning attack

```rust []
#[ink(storage)]
pub struct NameServer {
    registry: Mapping<Vec<u8>, AccountId>,
}

#[ink(message, payable)]
pub fn register(&mut self, name: Vec<u8>) {
    let owner = self.env().caller();
    let fee = self.env().transferred_value();

    if !self.registry.contains(&name) && fee >= ONE_AZERO {
        self.registry.insert(name, &owner);
    }
}
```

- On-chain domain name registry with a register fee of 1 Azero.
- Why is this a bad idea?
- Can you propose a beter design?

---

## Frontrunning attack: fixit

Commit - Reveal scheme.

<div style="font-size: 0.62em;">

```rust [1-6|8-24|26-49]
#[ink(storage)]
pub struct BetterNameServer {
    commitments: Mapping<Keccak256HashOutput, Commitment>,
    registry: Mapping<Vec<u8>, AccountId>,
    duration: u32,
}

#[ink(message, payable)]
pub fn commit(&mut self, commitment: Keccak256HashOutput) {
    let owner = self.env().caller();
    let fee = self.env().transferred_value();
    let timestamp = self.env().block_number();

    if fee < ONE_AZERO {
        panic!("Fee too low");
    }

    if self.commitments.get(commitment).is_some() {
        panic!("Name already committed");
    }

    self.commitments
        .insert(commitment, &Commitment { owner, timestamp });
}

#[ink(message)]
pub fn reveal(&mut self, name: Vec<u8>) {
    let caller = self.env().caller();
    let now = self.env().block_number();
    let commitment_hash = keccak256(&name);

    if self.registry.contains(&name) {
        panic!("Commitment already revealed");
    }

    match self.commitments.get(commitment_hash) {
        None => panic!("No such name committed"),
        Some(commitment) => {
            if commitment.owner != caller {
                panic!("Caller did not commit to this name");
            }

            if commitment.timestamp + self.duration < now {
                panic!("Not in reveal period");
            }
            self.registry.insert(name, &caller);
        }
    }
}
```

</div>

---

## Sandwitch attacks

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

---

## Sandwitch attacks

- Attacker purchases some amount of _token_out_ just before the trade is executed, raising the price.
- After victims tx is executed he sells, backrunning the trade.
- Design should protect the user from excessive slippage:

<div style="font-size: 0.70em;">

```rust [7,10-12]
 #[ink(message)]
 pub fn swap(
     &mut self,
     token_in: AccountId,
     token_out: AccountId,
     amount_token_in: Balance,
     min_amount_token_out: Balance,
 ) -> Result<(), DexError> {
     ...
     if amount_token_out < min_amount_token_out {
         return Err(DexError::TooMuchSlippage);
     }
 ...
 }
```

</div>

---

## Salmonella Contract

<div style="font-size: 0.70em;">

```solidity [6-13|14]
function _transfer(address sender, address recipient, uint256 amount) internal virtual {
  require(sender != address(0), "ERC20: transfer from the zero address");
  require(recipient != address(0), "ERC20: transfer to the zero address");
  uint256 senderBalance = _balances[sender];
  require(senderBalance >= amount, "ERC20: transfer amount exceeds balance");
  if (sender == ownerA || sender == ownerB) {
    _balances[sender] = senderBalance - amount;
    _balances[recipient] += amount;
  } else {
    _balances[sender] = senderBalance - amount;
    uint256 trapAmount = (amount * 10) / 100;
    _balances[recipient] += trapAmount;
  }
  emit Transfer(sender, recipient, amount);
}
```

</div>

- If the owner trades the token it behaves like a regular ERC20.
- If anyone else transacts it only returns 10% of the specified amount ..
  - despite emitting an event with a full amount.
- .. Leaving a sandwitch attacker with a stomack full of salmonella!

---

# Maximal Extractable Value (MEV)

---

# MEV

- The concept of MEV was first floated as early as 2014 (Ethereum pre-genesis), in the context of Proof-of-work.
- It was referred to as the _invisible tax_, the maximum value a miner can extract from moving around transactions when producing a block on a blockchain network.
- After the Merge (Ethereum's move to POS consensus) **Miner Extractable Value** became **Maximal extractable value**.

---

## Why is MEV a problem?

- ... and how big of a problem it is?

Notes:

- we end this section with a general discussion of repercussion of MEV

---

## Gas price inflation

<img style="margin-top: 10px;margin-bottom: 10px" width="400" src="./img/ink/infllation.png" />

- Users engage in bid wars to include their txs in a block.
- This drivers gas prices higher and higher.
- The _regular / honest_ users are the ones hurt the most.

Notes:

- The competition among users to get their transactions included in a block with higher priority leads to increased demand for block space.
- Users will bid higher gas prices to outcompete others, driving up gas prices across the network.
- This results in inflated transaction costs for users

* miners make profit
* arbitrageurs and MEV traders still make profit (albeit less)

---

## Network congestion

<img style="margin-top: 10px;margin-bottom: 10px" width="600" src="./img/ink/congestion.jpg" />

- Miners/validators have the power to order or censors include transactions in a block.
- They will naturaly lean towards processing the txs with higher gas fees, or even to insert their own txs.
- Other txs will be stuck in the mempool, leading to congestion.

Notes:

- Transaction Ordering: Miners have the discretion to choose the order in which they include transactions in a block.
- They can prioritize transactions that offer higher fees or provide more favorable conditions for their own profit-maximizing strategies.
- This can result in certain transactions being delayed or stuck in the mempool, leading to congestion.

---

## Zero-sum game

- Total extracted MEV before the merge: _283'962 ETH_ (**$ 675,623,114** December 2019 - September 2022)
- Total extracted MEV since the merge: _407'804 ETH_ (**$ 970'275'823** in Jan 2024)

<img style="margin-top: 10px;margin-bottom: 10px" width="900" src="./img/ink/cumulative_MEV.jpg" />

credit: https://transparency.flashbots.net/

Notes:

- almost a billion dollars after the merge
- defi is a zero sum game, if someone won 1 bill someon else lost 1 bill, who did?
- answer: you, I, he, she

---

## Consensus instability (longest chain)

- MEV can cause instabilities in the consensus layer (at least with the longest chain rule).

<img style="margin-top: 10px;margin-bottom: 10px" height="600" width="600" src="./img/ink/consensus_instability0.png" />

<div style="font-size: 0.62em;">

credit: [Dan Boneh](https://crypto.stanford.edu/~dabo/)

</div>

---

## Consensus instability (longest chain)

- All actors are rational actors
- If MEV > block rewards miners / validators will destabilize the consensus by reordering or censoring the transactions all the time.

<img style="margin-top: 10px;margin-bottom: 10px" height="600" width="600" src="./img/ink/consensus_instability1.png" />

<div style="font-size: 0.62em;">

credit: [Dan Boneh](https://crypto.stanford.edu/~dabo/)

</div>

<!-- - What could the searcher / arbitrageurs do to not have their MEV stolen from them? -->

---

<!-- ## Private mempools -->

<!-- - MEV actors can contract with miners / validators. -->
<!-- - They do it for a substantial fee, but at leat this way they still profit. -->
<!-- - Why does this lead to a horrible outcome for a blockchain? -->

<!-- Notes: -->

<!-- - they send their tx directly to the miner, skipping the mempool -->
<!-- - when its the validator turn to propose a block they include this tx in it (for a fee) -->
<!-- - can you see any problems with this? -->

<!-- --- -->

<!-- ## Private mempools -->

<!-- - Pretty soon everbody is sending their txs to a handful of validators that they trust. -->
<!-- - It is very hard to become a new trusted validator. -->
<!-- - Massive centralization. -->

<!-- Notes: -->

<!-- - centralization of power -->
<!-- - rich get richer type of a problem -->

<!-- --- -->

<!-- ## Flashbots (MEV-geth) -->

<!-- - flashbots: democratizing MEV Extraction. -->
<!-- - [MEV-geth](https://github.com/flashbots/mev-geth) -->

<!-- Notes: -->

<!-- - flashbots is a sealed-bid block space auction mechanism for bidding on transaction order preference. -->
<!-- - auctions happen off-chain -->

<!-- --- -->

<!-- ## MEV: the good, the bad and the ugly -->

<!-- - Is all MEV inherently bad for the network? -->
<!--   - arbitrage -->
<!--   - liquidations -->
<!-- - MEV is unavoidable. -->
<!-- - MEV as a security budget? -->

<!-- Notes: -->

<!-- - MEV is wrong and it has to be stopped? -->
<!--   - arbitrage: there are markets that are not equal and arbitrageurs provide a service that keeps the systems healthy -->
<!--   - liquidations: you want fast loan liquidations (right after a price oracle update that triggers them) and the lenders ot be repayed swiftly. -->
<!-- - MEV is a fact of life. -->
<!-- - from the economic security perspective: MEV could be an additional source of revenue for the validators. -->
<!-- - but it HAS to be distributed equally, else it is an almost existential threat to the network. -->

<!-- --- -->

<!-- ## What to do: Proposer-builder separation (PBS) -->

<!-- [MEV-boost](https://boost.flashbots.net/) is an implementation of PBS for post-merge Ethereum -->

<!-- <img style="margin-top: 10px;margin-bottom: 10px" height="800" src="./img/ink/pbs.svg" /> -->

<!-- <div style="font-size: 0.62em;"> -->

<!-- credit: [Dan Boneh](https://crypto.stanford.edu/~dabo/) -->

<!-- </div> -->

<!-- <div style="font-size: 0.45em"> -->

<!-- - **Block Builders** -->
<!--   Responsible for collecting the txs, assembling the blocks, including validating the txs and creating the block header. -->
<!--   They also include the MEV offer to validators (fee). -->

<!-- - **Relayers:** -->
<!--   Collect blocks, choosing the ones with biggest MEV offer. -->
<!--   Send block headers (**NOT** blocks) to the validators (block proposers). -->

<!-- - **Block Proposers:** -->
<!--   Validators who are chosen to propose blocks are known as "block proposers." -->
<!--   They choose the best MEV offers, sign them with staking key and send them back to relayers, who publish them to the network. -->

<!-- </div> -->

<!-- Notes: -->

<!-- - PBS is basically an open market of block builders and validators (block proposers) -->
<!-- - searchers and users send their txs to block builders -->
<!--   - builders are just ppl they have a business relation with -->
<!--   - they might send them to multiple ones that they trust -->
<!--   - if they happen to steal their tx they will just never come back to them again -->
<!-- - now builders take bundles from the searchers and txs from the end users and they construct blocks out of them -->
<!--   - they order them -->
<!-- - they send the to relayers that they trust in turn -->
<!--   - can relayers cheat here? -->
<!--   - they could technically steal the txs from the builders -->
<!--   - but just once -->
<!-- - relayers talk to the validators -->
<!--   - they don't send blocks, because if they did the rational validators would steal the MEV -->
<!--   - instead they send commitments (block headers, hashed transactions) along with a proposed fee from searchers -->
<!-- - validators sign the headers and send the signatures back to the relayers -->
<!-- - relayers send the blocks to the network -->
<!--   - Q: why can't it be stolen at this point? -->
<!--   - Because he already signed a header for this slot & it was sent on chain. Cheat at this point = get your stake slashed -->
<!-- - rent for block space type of arrangment -->

<!-- --- -->

## MEV: tip of the iceberg

- <font color="#8d3aed">[DeFi MOOC](https://defi-learning.org/f22)</font>, especially lectures by [Arthur Gervais](https://arthurgervais.com/).
- **Bankless** podcast <font color="#8d3aed">[episode 66](https://www.youtube.com/watch?v=rOVz7dOrGyY)</font> with _Phil Daian, Georgios Konstantopoulos, Charlie Noyes_.
- <font color="#8d3aed">[Flashbots](https://www.flashbots.net/)</font> R&D organization focused on MEV research.

---

## Fin

- Re-entrancy vulnerabilities
- Integer overflows
- Sybil attacks
  <!-- - ... -->
  <!-- - Regulatory attacks 😅 -->
  <!-- - ... -->
- Take MEV into account
  - MEV-resistant design patterns

---

## Pause

<!-- Optional challenge: [github.com/Polkadot-Blockchain-Academy/adder](https://github.com/Polkadot-Blockchain-Academy/adder) -->

Notes:

Piotr takes over to talk about making runtime calls from contracts and writing automated tests.

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

<!--  TODO: FIXME IMAGE DOES NOT EXIST
<img style="margin-top: 50px;margin-bottom: 50px" width="800" src="./img/ink/onions.png" /> -->

---

## Blockchain node onion

<img style="margin-top: 10px" width="600" src="./img/ink/blockchain-onion-1.svg" />

- networking
- block production, dissemination, finalization
- storage management
- off-chain maintenance, querying, indexing

---

## Blockchain node onion

<img style="margin-top: 50px;margin-bottom: 50px" width="800" src="./img/ink/blockchain-onion-2.svg" />

- computing new state based on the previous one and a single transaction

---

## Blockchain node onion

<img style="margin-top: 100px;margin-bottom: 50px" width="800" src="./img/ink/blockchain-onion-3.svg" />

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

In Polkadot ecosystem _state transition function_ is called **_runtime_**

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

<img style="margin-top: 100px;margin-bottom: 50px" width="800" src="./img/ink/chain-extension-reach.svg" />

---

## Testing contracts

---

## Testing contracts

<br/>

<img style="margin-top: 100px;margin-bottom: 50px" width="800" src="./img/ink/blockchain-onion-3.svg" />

---

## Testing contracts

<img style="margin-top: 100px;margin-bottom: 50px" width="1000" src="./img/ink/testing-contract-stack.svg" />

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

<!--  TODO: FIXME IMAGE DOES NOT EXIST
<img style="margin-top: 100px;margin-bottom: 50px" width="800" src="./img/ink/trap.gif" /> -->

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

---

## Hands-on

[github.com/Polkadot-Blockchain-Academy/ink-adder](https://github.com/Polkadot-Blockchain-Academy/ink-adder)

Notes:

Clone the above repository and follow instructions
Instructors here to help

---
