---
title: Smart Contracts
description: Substrate based smart contracts for Web3 Engineers
duration: 1 hour
---

# Smart Contracts

---

## Smart Contracts VS Pallets

<div class="flex-container">
<div class="left" style="font-size: smaller">

Additional layer of logic on top of a blockchain’s existing core logic.

- There are three safeguards in place to make Smart Contracts work properly and securely.
  - _Predefined Fees_: Ensure that contract developers are charged for computation and storage defined at runtime level.
  - _Sandbox_: Can't modify core blockchain storage or the storage of other contracts directly.
  - _Revert_: Reverting transactions when they fail so no state is updated when things go wrong.

In contrast, pallets are compiled together to create the base runtime of a parachain.
Runtime development has no safeguards against malicious actors and inexperienced developers.

</div>
</div>

Notes:

Smart contracts add an additional layer of logic on top of a blockchain’s existing core logic.
One benefit of this structure is that anyone can build onto and recreate an existing smart contract and its runtime logic to create a modified version of a given code or protocol.
A particularly important feature to smart contracts is the way they perform cross-contract calling to delegate and message other contracts.
Anyone is capable of publishing smart contracts onto existing blockchains, including malicious actors and inexperienced developers.
However, there are a number of safeguards for this.

In contrast, pallets are compiled together to create the base runtime of a parachain.
Runtime development has no safeguards against malicious actors and inexperienced developers.
You have full control of the underlying logic that each node on your network will run.
You have full access to each and every storage item across all of your modules, which you can modify and control.
You can even brick your chain with incorrect logic or poor error handling.

Pallets also provide granular control of transaction (TX) fees, allowing developers to be able to make certain, or all, TXs free.
This implementation can get as specific as only charging fees when certain TX conditions are met.

All of this control provided by Substrate pallets creates a double-edged sword.
More control in “bricking” or blocking your own chains means more room for errors, compared to smart contracts.

---

## Smart Contracts Pallets

Substrate provides different Smart Contracts' options.

- Pallet Contracts - Wasm execution environment
- Frontier - EVM execution environment

<img rounded style="width: 600px; margin-top:-7f0px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/frontier/contracts-runtime.png" alt="Contracts runtime" />

Notes:

Smart Contracts don't come by default in a Substrate chain.
We need to add an execution environment to enable this interactions.
Substrate chains need to have either the Smart Contracts pallet or Frontier.

---

## Ethereum Smart Contracts Compatibility

Smart contracts on [Frontier](https://github.com/paritytech/frontier) can be implemented using any language which can compile smart contracts to EVM-compatible bytecode.

<br/>

<div class="right">
<img rounded style="width: 200px; margin-right: 70px" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/frontier/solidity.png" alt="Solidity Logo" />
<img rounded style="width: 200px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/frontier/vyper.png" alt="Solidity Logo" />
</div>

<pba-flex center>

> Solidity $~~~~$ & $~~~~~~$ Vyper

</pba-flex>
</div>

Notes:

Smart contracts on Frontier can be implemented using Solidity, Vyper, and any other language which can compile smart contracts to EVM bytecode.
Frontier aims to provide a low-friction and secure environment for the development, testing, and execution of smart contracts that is compatible with the existing Ethereum developer toolchain.

</div>

---

## Frontier

Frontier provides an Ethereum compatibility layer for Substrate.<br/>
It has two components that can be activated separately.

<img rounded style="width: 600px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/frontier/Frontier.png" alt="Frontier Logo" />

Notes:

**Pallet EVM**: This is the pallet that enables functionality of running EVM contracts.
Existing EVM code can be used from there, using addresses and values mapped directly to Substrate.

**Pallet Ethereum**: The pallet, combined with the RPC module, enables Ethereum block emulation, validates Ethereum-encoded transactions, and allows existing dapps to be deployed on a Substrate blockchain with minimal modifications.

A Web3 JS RPC call from a DApp or existing Ethereum developer tool, such as Truffle, is received by a Frontier node.
The node will have both Web3 RPCs and Substrate RPCs available, which means that you can use Ethereum or Substrate tools when interacting with a Frontier node.
These RPC calls are handled by associated Substrate runtime functions.
The Substrate runtime checks signatures and handles any extrinsics.
Smart contract calls are ultimately passed to the EVM to execute the state transitions.

---

## Pallet EVM

Wraps the SputnikVM

```rust [1-2|1-3|1-4|1-5|1-6|1-7|1-8|1-9|1-10|1-11|1-12|1-13|1-14|1-15|1-16]
impl pallet_evm::Config for Runtime {
  type FeeCalculator = FixedGasPrice;
  type GasWeightMapping = FrontierGasWeightMapping;
  type BlockHashMapping = EthereumBlockHashMapping<Self>;
  type WeightPerGas = WeightPerGas;
  type CallOrigin = EnsureAddressTruncated;
  type WithdrawOrigin = EnsureAddressTruncated;
  type AddressMapping = HashedAddressMapping<BlakeTwo256>;
  type Currency = Balances;
  type Event = Event;
  type Runner = pallet_evm::runner::stack::Runner<Self>;
  type PrecompilesType = FrontierPrecompiles<Self>;
  type PrecompilesValue = PrecompilesValue;
  type ChainId = ChainId;
  type BlockGasLimit = BlockGasLimit;
  type OnChargeTransaction = pallet_evm::EVMCurrencyAdapter<Balances, DealWithFees<Runtime>>;
  type FindAuthor = FindAuthorTruncated<Aura>;
}
```

Notes:

It uses the SputnikVM written by Wei Tang from Parity and it wraps it in a substrate pallet so that we can put an existing EVM inside the substrate runtime.

---

## Pallet EVM - GasWeightMapping

```rust [3, 5, 15]
impl pallet_evm::Config for Runtime {
  type FeeCalculator = FixedGasPrice;
  type GasWeightMapping = FrontierGasWeightMapping;
  type BlockHashMapping = EthereumBlockHashMapping<Self>;
  type WeightPerGas = WeightPerGas;
  type CallOrigin = EnsureAddressTruncated;
  type WithdrawOrigin = EnsureAddressTruncated;
  type AddressMapping = HashedAddressMapping<BlakeTwo256>;
  type Currency = Balances;
  type Event = Event;
  type Runner = pallet_evm::runner::stack::Runner<Self>;
  type PrecompilesType = FrontierPrecompiles<Self>;
  type PrecompilesValue = PrecompilesValue;
  type ChainId = ChainId;
  type BlockGasLimit = BlockGasLimit;
  type OnChargeTransaction = pallet_evm::EVMCurrencyAdapter<Balances, DealWithFees<Runtime>>;
  type FindAuthor = FindAuthorTruncated<Aura>;
}
```

---

```rust
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND / GAS_PER_SECOND;

pub struct FrontierGasWeightMapping;

impl GasWeightMapping for FrontierGasWeightMapping {
  fn gas_to_weight(gas: u64) -> Weight {
    gas.saturating_mul(WEIGHT_PER_GAS)
  }
  fn weight_to_gas(weight: Weight) -> u64 {
    u64::try_from(weight.wrapping_div(WEIGHT_PER_GAS)).unwrap_or(u32::MAX as u64)
  }
}
```

---

## Pallet EVM [Internals]

```rust
#[pallet::weight(T::GasWeightMapping::gas_to_weight(*gas_limit))]
```

---

## Pallet EVM - BlockHashMapping

<div style="font-size: 0.7em;">

```rust [1,4]
impl pallet_evm::Config for Runtime {
  type FeeCalculator = FixedGasPrice;
  type GasWeightMapping = FrontierGasWeightMapping;
  type BlockHashMapping = EthereumBlockHashMapping<Self>;
  type WeightPerGas = WeightPerGas;
  type CallOrigin = EnsureAddressTruncated;
  type WithdrawOrigin = EnsureAddressTruncated;
  type AddressMapping = HashedAddressMapping<BlakeTwo256>;
  type Currency = Balances;
  type Event = Event;
  type Runner = pallet_evm::runner::stack::Runner<Self>;
  type PrecompilesType = FrontierPrecompiles<Self>;
  type PrecompilesValue = PrecompilesValue;
  type ChainId = ChainId;
  type BlockGasLimit = BlockGasLimit;
  type OnChargeTransaction = pallet_evm::EVMCurrencyAdapter<Balances, DealWithFees<Runtime>>;
  type FindAuthor = FindAuthorTruncated<Aura>;
}
```

---

```rust
/// Returns the Ethereum block hash by number.
pub struct EthereumBlockHashMapping<T>(PhantomData<T>);
impl<T: Config> BlockHashMapping for EthereumBlockHashMapping<T> {
  fn block_hash(number: u32) -> H256 {
    BlockHash::<T>::get(U256::from(number))
  }
}
```

</div>

Notes:

Does a conversion from a block number into an Ethereum block hash.

The hash mapping allows an EVM contract to ask for the parent block hash or grandparent block hash.
parent block hash or the grandparent block hash.
It is common to return the previous substrate blocks because
then you at least get some pseudoentropy that you could use.
In order to use this specific Block Hash Mapping,
we need to have the Ethereum Pallet which handles this type of data format.

---

---

## Pallet EVM - Currency

```rust [9]
impl pallet_evm::Config for Runtime {
  type FeeCalculator = FixedGasPrice;
  type GasWeightMapping = FrontierGasWeightMapping;
  type BlockHashMapping = EthereumBlockHashMapping<Self>;
  type WeightPerGas = WeightPerGas;
  type CallOrigin = EnsureAddressTruncated;
  type WithdrawOrigin = EnsureAddressTruncated;
  type AddressMapping = HashedAddressMapping<BlakeTwo256>;
  type Currency = Balances;
  type Event = Event;
  type Runner = pallet_evm::runner::stack::Runner<Self>;
  type PrecompilesType = FrontierPrecompiles<Self>;
  type PrecompilesValue = PrecompilesValue;
  type ChainId = ChainId;
  type BlockGasLimit = BlockGasLimit;
  type OnChargeTransaction = pallet_evm::EVMCurrencyAdapter<Balances, DealWithFees<Runtime>>;
  type FindAuthor = FindAuthorTruncated<Aura>;
}
```

Currency type will be treated as the main currency inside your EVM.
It becomes the
native token of the ethereum environment.

---

## Pallet EVM - AddressMapping

```rust [8]
impl pallet_evm::Config for Runtime {
  type FeeCalculator = FixedGasPrice;
  type GasWeightMapping = FrontierGasWeightMapping;
  type BlockHashMapping = EthereumBlockHashMapping<Self>;
  type WeightPerGas = WeightPerGas;
  type CallOrigin = EnsureAddressTruncated;
  type WithdrawOrigin = EnsureAddressTruncated;
  type AddressMapping = HashedAddressMapping<BlakeTwo256>;
  type Currency = Balances;
  type Event = Event;
  type Runner = pallet_evm::runner::stack::Runner<Self>;
  type PrecompilesType = FrontierPrecompiles<Self>;
  type PrecompilesValue = PrecompilesValue;
  type ChainId = ChainId;
  type BlockGasLimit = BlockGasLimit;
  type OnChargeTransaction = pallet_evm::EVMCurrencyAdapter<Balances, DealWithFees<Runtime>>;
  type FindAuthor = FindAuthorTruncated<Aura>;
}
```

---

## Pallet EVM - AddressMapping

<pba-cols>
<pba-col style="font-size: 0.8em;">

- Ethereum uses a 160-bit hex string as its public address.
- On Substrate, we call this format H160, for 160-bit hash.
- The EVM environment has its own balance called the EVM deposit, which can be withdrawn by Substrate native accounts.

</pba-col>
<pba-col>

<img rounded style="width: 800px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/frontier/EVM-accounts.png" alt="JS-Events" />

</pba-col>
</pba-cols>

Notes:

Ethereum uses a 160-bit hex string as its public address, which users can send and receive transactions.
On Substrate, we call this format H160, for 160-bit hash.
For a blockchain to be Ethereum-compatible, they must use H160 accounts, as that is the only account format that EVM and all tools within the Ethereum ecosystem will recognize.
This rule applies to Substrate chains with the EVM pallet as well.

The EVM environment has its own balance called the EVM deposit, which can be withdrawn by Substrate native accounts.
A native Substrate SS58 address can be converted to an H160 address that is mapped to an EVM deposit, and an EVM H160 address can be converted to an SS58 address.
This allows the accounts to transfer tokens from the native balance to an EVM balance, and vice versa.
However, this is a lossy conversion, so you cannot convert an SS58 to H160 and back to the same SS58 address.
Furthermore, you can only sign transactions within the environment that the account was created.
So for example, through normal ways, you cannot sign an EVM transaction with a native account or the other way around.

---

## Pallet EVM - AddressMapping

<img rounded style="width: 600px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/frontier/Polkadot-JS-EVM-Events.png" alt="JS-Events" />

Notes:

If we look at the Ethereum transaction event in detail, we can see the token minting event and the burning event with the correct amount.
However, one interesting detail is the account.
Although the EVM RPC returns an H160 address, the Substrate RPC will return an SS58 address for all accounts within the EVM sandbox.
This is because, in Substrate, the default address format is always SS58.

---

> So how does Substrate convert the address format,<br/>and how does it handle the balances?

---

## Ethereum address to SS58 ???

<div style="font-size: 0.85em;">

```[1|3-6|7-9|11-12|14-15]
0xe31b11a052afc923259949352b2f573a21301ba4 -> 5HAc4UzLYQuyjHbpEPicC7bAjnofHqRWYStRKqA5WfreMKWk

subkey inspect 5HAc4UzLYQuyjHbpEPicC7bAjnofHqRWYStRKqA5WfreMKWk
  Public key (hex):   0xe1ad20aae239ccbb609aa537d515dc9d53c5936ea67d8acc9fe0618925279f7d
  SS58 Address:       5HAc4UzLYQuyjHbpEPicC7bAjnofHqRWYStRKqA5WfreMKWk

subkey inspect --public 0xe1ad20aae239ccbb609aa537d515dc9d53c5936ea67d8acc9fe0618925279f7d
  Public key (hex):     0xe1ad20aae239ccbb609aa537d515dc9d53c5936ea67d8acc9fe0618925279f7d
  SS58 Address:         5HAc4UzLYQuyjHbpEPicC7bAjnofHqRWYStRKqA5WfreMKWk

blake2Hash(toByteArray(["evm:", "0xe31b11a052afc923259949352b2f573a21301ba4"]))
0xe1ad20aae239ccbb609aa537d515dc9d53c5936ea67d8acc9fe0618925279f7d

ss58Encode(0xe1ad20aae239ccbb609aa537d515dc9d53c5936ea67d8acc9fe0618925279f7d, 42)
5HAc4UzLYQuyjHbpEPicC7bAjnofHqRWYStRKqA5WfreMKWk
```

</div>

Notes:

there's this concept of account mappings and the idea is that inside of the evm you have standard ethereum style
h160 accounts and they will have some data associated with them
native accounts and the standard way to do it
is called the hash truncated account mapping that means you take the h160 address you hash it and then you take
the first 32 bytes and map that to a substrate style account 32 account id 32

---

## Pallet EVM [Internals]

```rust
type AddressMapping = HashedAddressMapping<BlakeTwo256>;

/// Hashed address mapping.
pub struct HashedAddressMapping<H>(sp_std::marker::PhantomData<H>);

impl<H: Hasher<Out = H256>> AddressMapping<AccountId32> for HashedAddressMapping<H> {
  fn into_account_id(address: H160) -> AccountId32 {
    let mut data = [0u8; 24];
    data[0..4].copy_from_slice(b"evm:");
    data[4..24].copy_from_slice(&address[..]);
    let hash = H::hash(&data);

    AccountId32::from(Into::<[u8; 32]>::into(hash))
  }
}
```

---

## AccountId20 (H160)

- Moonbeam has 100% compatibility with Ethereum.
- AccountId becomes H160 in all the runtime.

```rust
type AddressMapping = IntoAddressMapping;

pub struct IntoAddressMapping;

impl<T: From<H160>> pallet_evm::AddressMapping<T> for IntoAddressMapping {
  fn into_account_id(address: H160) -> T {
    address.into()
  }
}

```

Notes:

Another way that you can do it which is the way Moonbeam does.
They change the main account type in the runtime to be H160.
The substrate native account that backs your evm account is exactly the same
account account.
There's no hashing or truncation happening.

When configuring the Account ID lookup, we need to change `AccountIdLookup` to `IdentityLookup`
AccountIdLookup -> Is a lookup implementation returning the `AccountId` from a `MultiAddress`
IdentityLookup -> Is a lookup implementation returning the input value.

---

## Pallet Ethereum

- Ethereum formatted data.
- Ethereum style blocks in storage.

```rust
impl pallet_ethereum::Config for Runtime {
  type Event = Event;
  type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
}

```

Notes:

Pallet ethereum is going to allow us to send ethereum formatted data to our chain and
receive ethereum formatted data from our chain.
It keeps the Ethereum style block into storage.
This allows any block indexer, explorer or other clients to understand
the data that is stored and remain compatible.

---

## Precompiles

- Common functionality for smart contracts.
- Compiled and accessible to any Smart Contract.
- Simple as an opcode.
- Workaround to get out of the sandbox in a very limited way.

Notes:

A precompile means a common functionality for smart contracts which has been compiled, so that Ethereum nodes can run this efficiently.
From a contract's perspective, this is just a single command like an opcode.
These contracts are implemented as a native implementation.

The EVM environment works on top of Substrate.
This means that the block height of the EVM sandbox will depend on the host Substrate network and the host Substrate network will be able to access the EVM state, but the EVM sandbox will not be able to access or mutate the host Substrate network’s state through normal means.
Substrate state transition from EVM can only be achieved via custom EVM precompiled contracts.

---

## Assets Precompiles

_Solidity Interface_

<div style="font-size: 0.87em;">

```solidity
pragma solidity ^0.8.0;

/**
    * @title ERC20 interface
    * @dev see https://github.com/ethereum/EIPs/issues/20
    * @dev copied from https://github.com/OpenZeppelin/openzeppelin-contracts
    */
interface IERC20 {
    /**
    * @dev Returns the name of the token.
    * Selector: 06fdde03
    */
    function name() external view returns (string memory);

    /**
    * @dev Returns the symbol of the token.
    * Selector: 95d89b41
    */
    function symbol() external view returns (string memory);

    /**
    * @dev Returns the decimals places of the token.
    * Selector: 313ce567
    */
    function decimals() external view returns (uint8);

    /**
     * @dev Total number of tokens in existence
     * Selector: 18160ddd
     */
    function totalSupply() external view returns (uint256);

    /**
     * @dev Gets the balance of the specified address.
     * Selector: 70a08231
     * @param who The address to query the balance of.
     * @return An uint256 representing the amount owned by the passed address.
     */
    function balanceOf(address who) external view returns (uint256);

    /**
     * @dev Function to check the amount of tokens that an owner allowed to a spender.
     * Selector: dd62ed3e
     * @param owner address The address which owns the funds.
     * @param spender address The address which will spend the funds.
     * @return A uint256 specifying the amount of tokens still available for the spender.
     */
    function allowance(address owner, address spender)
        external view returns (uint256);

    /**
     * @dev Transfer token for a specified address
     * Selector: a9059cbb
     * @param to The address to transfer to.
     * @param value The amount to be transferred.
     */
    function transfer(address to, uint256 value) external returns (bool);

    /**
     * @dev Approve the passed address to spend the specified amount of tokens on behalf
     * of msg.sender.
     * Beware that changing an allowance with this method brings the risk that someone may
     * use both the old
     * and the new allowance by unfortunate transaction ordering. One possible solution to
     * mitigate this race condition is to first reduce the spender's allowance to 0 and set
     * the desired value afterwards:
     * https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
     * Selector: 095ea7b3
     * @param spender The address which will spend the funds.
     * @param value The amount of tokens to be spent.
     */
    function approve(address spender, uint256 value)
        external returns (bool);

    /**
     * @dev Transfer tokens from one address to another
     * Selector: 23b872dd
     * @param from address The address which you want to send tokens from
     * @param to address The address which you want to transfer to
     * @param value uint256 the amount of tokens to be transferred
     */
    function transferFrom(address from, address to, uint256 value)
        external returns (bool);

    /**
     * @dev Event emitted when a transfer has been performed.
     * Selector: ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
     * @param from address The address sending the tokens
     * @param to address The address receiving the tokens.
     * @param value uint256 The amount of tokens transfered.
     */
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 value
    );

    /**
     * @dev Event emitted when an approval has been registered.
     * Selector: 8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925
     * @param owner address Owner of the tokens.
     * @param spender address Allowed spender.
     * @param value uint256 Amount of tokens approved.
     */
    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 value
    );
}

```

</div>

---

## Assets Precompiles

_Substrate Implementation_

<div style="font-size: 0.87em;">

```rust
...
  fn total_supply(
    asset_id: AssetIdOf<Runtime, Instance>,
    handle: &mut impl PrecompileHandle,
  ) -> EvmResult<PrecompileOutput> {
    handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;

    // Parse input.
    let input = handle.read_input()?;
    input.expect_arguments(0)?;

    // Fetch info.
    let amount: U256 =
      pallet_assets::Pallet::<Runtime, Instance>::total_issuance(asset_id).into();

    // Build output.
    Ok(succeed(EvmDataWriter::new().write(amount).build()))
  }

  fn balance_of(
    asset_id: AssetIdOf<Runtime, Instance>,
    handle: &mut impl PrecompileHandle,
  ) -> EvmResult<PrecompileOutput> {
    handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;

    // Read input.
    let mut input = handle.read_input()?;
    input.expect_arguments(1)?;

    let owner: H160 = input.read::<Address>()?.into();

    // Fetch info.
    let amount: U256 = {
      let owner: Runtime::AccountId = Runtime::AddressMapping::into_account_id(owner);
      pallet_assets::Pallet::<Runtime, Instance>::balance(asset_id, &owner).into()
    };

    // Build output.
    Ok(succeed(EvmDataWriter::new().write(amount).build()))
  }

  fn transfer(
    asset_id: AssetIdOf<Runtime, Instance>,
    handle: &mut impl PrecompileHandle,
  ) -> EvmResult<PrecompileOutput> {
    handle.record_log_costs_manual(3, 32)?;

    // Parse input.
    let mut input = handle.read_input()?;
    input.expect_arguments(2)?;

    let to: H160 = input.read::<Address>()?.into();
    let amount = input.read::<BalanceOf<Runtime, Instance>>()?;

    // Build call with origin.
    {
      let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
      let to = Runtime::AddressMapping::into_account_id(to);

      // Dispatch call (if enough gas).
      RuntimeHelper::<Runtime>::try_dispatch(
        handle,
        Some(origin).into(),
        pallet_assets::Call::<Runtime, Instance>::transfer {
          id: asset_id,
          target: Runtime::Lookup::unlookup(to),
          amount,
        },
      )?;
    }
  ...
```

---

<img style="width: 400px;" src="../../assets/img/0-Shared/logo/webassembly-blue.png" />

- Stack-based virtual machine.
- Compilation of high-level languages like C/C++ and Rust.
- Fast, efficient, and safe, with near-native performance.
- WebAssembly has only four primitive types: i32, i64, f32 and f64.

Notes:

The WebAssembly (Wasm) architecture is a binary instruction format for a stack-based virtual machine.
It is designed as a portable target for compilation of high-level languages like C/C++ and Rust.
Wasm is designed to be fast, efficient, and safe, with near-native performance and no need for plugins or additional downloads.
Wasm code is written in a low-level assembly-like language, and is then compiled into a binary format that can be executed on the web.
The architecture is designed to be flexible and extensible, allowing developers to create custom tools and libraries to make web applications more powerful and interactive.

The WebAssembly binary is a sequence of instructions that are executed on a stack-based machine.
Simple instructions perform operations on data; they consume their operands from the stack and produce a result that is placed on the stack.
Control instructions alter the control flow of the code.
A program can call functions directly or indirectly through a function table.

---

## Stack-based Virtual Machine

<img rounded style="width: 1000px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/wasm/Stack_1.png" />

We want to add 2 numbers.

Notes:

Let’s start with a trivial example: suppose your program needs to add two numbers.
To do that in a stack VM, the program will push the first number to the stack, push the second and then execute some form of the special instruction add, that will pop the first two elements of the stack and replace them with their sum.
Let’s see it step by step:

---

## Stack-based Virtual Machine

<img rounded style="width: 1000px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/wasm/Stack_2.png" />

I) We start by pushing the first element to the stack.

Notes:

The SP is the stack pointer, which refers to the head of the stack.
The IP is the instruction pointer, which points to the address of the current instruction to execute.
Let’s now execute the first instruction:

---

## Stack-based Virtual Machine

<img rounded style="width: 1000px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/wasm/Stack_3.png" />

II) We push the second element to the stack.

Notes:

You can see that the stack now contains 1 and that the instruction pointer has been moved to the next instruction.
Let’s now simulate the second instruction:

---

## Stack-based Virtual Machine

<img rounded style="width: 1000px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/wasm/Stack_4.png" />

III) Finally we can execute the add instruction.

Notes:

Finally we can execute the add instruction.
The head of the stack and the previous element have been popped and replaced with their sum.

---

## How to Print "hello world!"?

```json
            h    e    l    l    o         w    o    r    l    d    !
            0x68 0x65 0x6C 0x6C 0x6F 0x20 0x77 0x6F 0x72 0x6C 0x64 0x21
```

Notes:

Unicode, specifically U8 = 2 byte characters.

---

```WebAssembly [1-50|17-20|21-22|23-26|27-32|33-41|41-50|54-55 ]
(module
;; Creates the "print" function and takes a single parameter of type i32.
 (func (export "print") (param $0 i32)
 ;; Calls the "puts" function with the parameter 4.
  (call $puts
   (i32.const (i32.const 4))
  )
 )

 ;; Creates a table with 0 entries and sets the type of each entry to anyfunc.
 (table 0 anyfunc)
 ;; Creates a memory page.
 (memory 1)
 ;; Exports the memory created in the previous line,
 ;; allowing it to be used by other functions.
 (export "memory" (memory 0))
 ;; Creates the puts function, which takes a single parameter of type i32.
 ;; The 'puts' function is called from the 'print' function,
 ;; printing out 'hello world'.
 (func (export "puts") (param $0 i32)
 ;;Creates a local variable called $1 of type i32.
  (local $1 i32)
  ;; Sets the value of the local variable $1 to 4.
  (set_local $1
    (i32.const (i32.const 4))
  )
  ;; Stores the 8-bit value of 0x68 "h" at the address
  ;; specified by the parameter $0.
  (i32.store8
    (get_local $0)
    (i32.const (i32.const 0x68))
  )
  ;; Stores the 8-bit value of 0x65 "e" at the address calculated
  ;; by adding the parameter $0 and the value 1.
  (i32.store8
    (i32.add
      (get_local $0)
      (i32.const (i32.const 1))
    )
    (i32.const (i32.const 0x65))
  )
  ;; Stores the 8-bit value of 0x6c "l" at the address calculated
  ;; by adding the parameter $0 and the value 1.
    (i32.store8
    (i32.add
      (get_local $0)
      (i32.const (i32.const 1))
    )
    (i32.const (i32.const 0x6c))
  )
  ...
  ...
 )
 ;; Sets the start function to be the "puts" function defined earlier.
 (start $puts)
)
```

---

#### Wasm Architecture

<img rounded style="width: 1400px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/wasm/WebAssembly-high-level-architecture.png" />

Notes:

WebAssembly has only four primitive types: i32, i64, f32 and f64.
The first two represent integers with 32 and 64 bits respectively, whereas the last two denote 32 and 64-bit floating-point data.
Global variables, local variables, and return addresses are managed in the stack.
All non-scalar types, such as strings, arrays, and other buffers, must be stored in linear memory, which is a contiguous, untyped, byte-addressable array, multiple of 64Kib.
A program can load/store values from/to linear memory at any byte address.
A trap occurs if an access is not within the bounds of the current memory size.

</div>

---

<!-- .slide: data-background-color="#4A2439" -->

<img rounded style="width: 300px" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/question-mark.svg" />

---

## References

<pba-flex center>

- [Frontier Repository](https://github.com/paritytech/frontier)
- [Sub0 Workshop](https://www.youtube.com/watch?v=V9KfvhoqLJ4)
- [SputnikVM](https://github.com/rust-blockchain/evm)
- [Wasm Docs](https://developer.mozilla.org/en-US/docs/WebAssembly)
