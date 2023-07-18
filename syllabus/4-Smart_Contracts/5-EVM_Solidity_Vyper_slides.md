---
title: EVM, Solidity, and Vyper
description: Overview and history of the EVM and languages that compile to it.
  Focus on architecting real-world smart contracts.
---

# EVM

Ethereum Virtual Machine

* A VM designed specifically for the constraints and features of Ethereum

---v

## EVM Properties

* Deterministic: execution outcome easy to agree upon
* Spam-resistant: CPU and other resources are metered at a very granular level
* Turing-complete (with a caveat)
* Stack-based design
* Ethereum-specific (EVM can query block hash, accounts and balances, etc.)

Note:

It is critical that the EVM be 100% deterministic and that each implementation
produce the same outcome. Even the smallest discrepancy between two running
nodes would lead to different block hashes, violating consensus about the
results.

---

# History of Ethereum

(TODO)

ideas:
* Early concepts (IIRC, was attempted to be built on Bitcoin first)
* Shortcomings of Ethereum (storage bloat, ...?)
* DAO & hardfork
* Important updates
* The Merge
* Ice ages (aka difficulty bombs)

* Good resource: https://ethereum.org/en/history/
* Nov 27, 2013: Vitalik released WP
* Apr 21, 2014: Gav released YP
* July 22, 2014: $18M raised (ICO lasted 42 days)
* July 30, 2015: Frontier released -- bare bones
* Sept 7: 2015: Frontier "thawed" -- (TODO: only txns allowed)? also, difficulty bomb introduced
* Initial Launch: July 30, 2015
* TODO: show as timeline, include block numbers
* TODO: split into two slides (dev history, fork history)
  * maybe another for major changes?

---v

## DAO Hack

A DAO ("Decentralized Autonomous Organization") is much like a business entity
run by code rather than by humans. Like any other business entity, it has assets
and can carry out operations, but its legal status is unclear.

The earliest DAO ("The DAO") on Ethereum suffered a catastrophic hack due to a
bug in its code. The community disagreed on whether or not to hard-fork and
revert this, resulting in Ethereum splitting into two different chains.

Brief history:

* 2016: raised $150M worth of ETH
* later that year: 3.6M ETH drained
* reentrancy attack due to its `withdraw` function sending funds BEFORE recording them
* "Mainnet" Ethereum is the fork with the hack retroactively removed
* Ethereum Classic: code is law

---v

## Ethereum as first smart contracting platform

Ethereum has faced many challenges as the pioneer of smart contracts.

* Performance: Underpriced opcodes have been attacked as a spam or DoS vector
* High gas fees: Overwhelming demand low block-space supply
* Frontrunning: Inserting transactions before, after, or in place of others in order to economically benefit
* Hacks: Many hacks have exploited smart contract vulnerabilities
* Problems with aggregating smart contract protocols together

---v

## Idiosyncrasies

(TODO)

- Everything is 256bits
- No floating point arithmetic
- Revert
- Reentrancy
- ABI
- Exponential memory expansion cost

---

# Gas

## Turing completeness and the Halting Problem

* EVM: Turing-complete instruction set

* But what about the Halting Problem?
* Obviously cannot allow infinite loops
* Solution: Gasometer, a way to pre-pay for each opcode execution

Note:

The Halting Problem tells us that it's not possible to know that an arbitrary
program will properly stop. To prevent such abuse, we check that there is gas
remaining before every single opcode execution. Since gas is limited, this
ensures that no EVM execution will run forever and that all work performed is
properly paid for.

---v

## Gasometer

<img style="width: 100%; margin: 10px" src="img/frontier/GasometerDiagram.png" />

* Checks before each opcode to make sure gas can be paid
* Safe: prevents unpaid work from being done
* Deterministic: results are unambiguous
* Very inefficient: lots of branching and extra work

Note: This not only makes it possible to prevent abuse, but crucially allows
nodes to agree on doing so. A centralized service could easily impose a time
limit, but decentralized nodes wouldn't be able to agree on the outcome of such
a limit (or trust each other).

---v

## Gas limits and prices

The unit of account for EVM execution resources.

* `gas_limit`: specifies the maximum amount of gas a txn can pay
* `gas_price`: specifies the exact price a txn will pay per gas

A txn *must* be able to pay `gas_limit * gas_price` in order to be valid. This
amount is initially deducted from the txn's sender account and any remaining gas
is refunded after the txn has executed.

---v

## EIP-1559

An improvement to gas pricing mechanism. Introduced in London hard-fork.

```
gas_price --> max_base_fee_per_gas
          \-> max_priority_fee_per_gas
```

* Separates tip from gas price
* `base_fee` is an algorithmic gas price, this is exactly what is paid and is burned
* ...plus maybe tip if (`base_fee < max_base_fee + max_priority_fee`)
* Algorithmic, congestion-based multiplier controls `base_fee`

---v

## Gas Estimation

If a txn exhausts its `gas_limit` without finishing, it will produce an
out-of-gas error and all changes made in the EVM are reverted (except for fee
payment).

In order to estimate the amount of gas a txn will need, an RPC method
(`eth_estimateGas`) can perform a dry-run of the txn and record the amount
used.

However, there are a few caveats:

* Run against current state (state may change)
* The RPC node could lie to you
* This is expensive infrastructure overhead and can be a spam vector

---

## Contract accounts vs EOAs

An account is designated by a 160-bit account ID. These accounts can be controlled in
one of two ways: An `Externally-owned Account` or a `Contract Account`.

### Externally-owned Account (EOA)

* Traditional user-controlled account
* Can only be controlled via transactions signed with corresponding private keys
* Account ID generated by hashing public key

### Contract Account

* Controlled by code
* Created with immutable contract bytecode
* May only ever do precisely what the code specifies
* Account ID generated deterministically when bytecode is deployed

---

## Transactions

TODO: format this slide better

All interactions which potentially change EVM state must come from signed
transactions. The transaction's signature allows for the public key (and
thereby the account id) to be recovered. The transaction must specify how
much gas it is willing to pay and the sending account must be able to pay
for this up front.

Only an EoA can send a transaction since a contract account
does not have a private key.

#### Transactions can:

* Send 0 or more Ether to some other account

#### And optionally either:

* Call an external function on an existing contract
* OR Create a new contract

---v

## Transaction Structure

(TODO): redo this, point out what's useful, show image instead...?

```json
{ 
  hash: "0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060",
  blockHash: "0x4e3a3754410177e6937ef1f84bba68ea139e8d1a2258c5f85db9f1cd715a1bdd",
  blockNumber: 55555,
  chainId: null,

  gas: 21000,
  gasPrice: 10000000000000,

  nonce: 0,

  input: "0x1234567800000001",

  from: "0xa1e4380a3b1f749673e270229993ee55f35663b4",
  r: "0x88ff6cf0fefd94db46111149ae4bfc179e9b94721fffd821d38d16464b3f71d0",
  s: "0x45e0aff800961cfce805daef7016b9b675c137a6a41a548f7b60a3484c06a33a",
  v: "0x1c",

  to: "0x5df9b87991262f6ba471f09758cde1c0fc1de734",
  value: 12345,
}
```

---v

## Opcodes and Bytecode

An opcode is a single byte which represents an instruction for the VM to execute.
Functions compile down into a sequence of opcodes, which we call bytecode. This
bytecode is bundled together and becomes the on-chain contract code.

The EVM executes bytecode one opcode at a time until it is done, explicitly
halts, or the gasometer runs out of gas.

### ABI

ABI ("Application Binary Interface") describes the bytecode for a contract by
annotating where functions and other objects exist and how they are formatted.

---v

## Review Contract Code on Etherscan

https://etherscan.io/token/0x1f9840a85d5af5bf1d1762f925bdaddc4201f984?a=0xd0fc8ba7e267f2bc56044a7715a489d851dc6d78#code

---v

## Sandboxed Contract State

`Contract Account`s contain a sandboxed state, which stores everything that the
contract writes to storage. Contracts may not write to storage outside of their
own sandbox, but they can call other contracts whose bytecode *might* write to
their respective storage.

---v

## Calling Contracts

Contract functions can be invoked in two ways different ways:

* EoAs can call a contract functions directly
* Contracts can call other contracts (called "messaging")

#### Types of contract messaging

* Normal `call`: Another contract is called and can change its own state
* `staticcall`: A "safe" way to call another contract with no state changes
* `delegatecall`: A way to call another contract but modify our state instead

Note:

Transactions are the only means through which state changes happen. 

---v

## Message Object

Within the context of a contract call, we always have the `msg` object, which
lets us know how we were called.

```
msg.data (bytes): complete calldata (input data) of call
msg.gas (uint256): available gas
msg.sender (address): sender of the current message
msg.sig (bytes4) first 4 bytes of calldata (function signature)
msg.value (uint256) amount of Ether sent with this call
```

---v

## Ether Denominations

Ether is stored and operated on through integer math. In order to avoid
the complication of decimal math, it's stored as a very small integer: `Wei`.

```
1 Ether = 1_000_000_000_000_000_000 Wei (10^18)
```

Note: 

Integer math with such insignificant units mostly avoids truncation issues and
makes it easy to agree on outcomes.

---v

## Named Denominations

Other denominations have been officially named, but aren't as often used:

```
wei                 = 1 wei
kwei (babbage)      = 1_000 wei
mwei (lovelace)     = 1_000_000 wei
gwei (shannon)      = 1_000_000_000 wei
microether (szabo)  = 1_000_000_000_000 wei
milliether (finney) = 1_000_000_000_000_000 wei
ether               = 1_000_000_000_000_000_000 wei
```

`gwei` is often used when talking about gas.

---

# Programming the EVM

The EVM is ultimately programmed by creating bytecode. While it is possible to
write bytecode by hand or through low-level assembly language, it is much more
practical to use a higher-level language. We will look at two in particular:

* Solidity
* Vyper

---

# Solidity

* Designed for EVM
* Similar to C++, Java, etc.
* Includes inheritance (including MI)
* Criticized for being difficult to reason about security

---v

## Basics

```Solidity
// 'contract' is analogous to 'class' in other OO languages
contract Foo {
    // the member variables of a contract are stored on-chain
    public uint bar;

    constructor(uint value) {
        bar = value;
    }
}
```

---v

## Functions

```Solidity
contract Foo {
    function doSomething() public returns (bool) {
        return true;
    }
}
```

---v

## Modifiers

* A special function that can be run as a precondition for other functions

```Solidity
contract Foo {
    address deployer;

    constructor() {
        deployer = msg.sender;
    }

    // ensures that only the contract deployer can call a given function
    modifier onlyDeployer {
        require(msg.sender == deployer);
        _; // the original function is inserted here
    }

    function doSomeAdminThing() public onlyDeployer() {
        // this function can only be called if onlyDeployer() passes
    }
}
```

Note:

Although Modifiers can be an elegant way to require preconditions, they can do
entirely arbitrary things, and auditing code requires carefully reading them.


---v

## Payable

```Solidity
contract Foo {
    uint256 received;
    // this function can be called with value (Ether) given to it.
    // in this simple example, the contract would never do anything with
    // the Ether (effectively meaning it would be lost), but it will faithfully
    // track the amount paid to it
    function deposit() public payable {
        received += msg.value;
    }
}
```

---v

## Types: "Value" Types

```Solidity
contract Foo {
    // Value types are stored in-place in memory and require
    // a full copy during assignment.
    function valueTypes() public {
        bool b = false;

        // signed and unsigned ints
        int32 i = -1;
        int256 i2 = -10000;
        uint8 u1 = 255;
        uint16 u2 = 10000;
        uint256 u3 = 99999999999999;

        // fixed length byte sequence (from 1 to 32 bytes)
        // many bitwise operators can be performed on these
        bytes1 oneByte = 0x01;

        // address represents a 20-byte Ethereum address
        address a = 0x1010101010101010101010101010101010101010;
        uint256 balance = a.balance;

        // also: Enums

        // each variable is an independent copy
        int x = 1;
        int y = x;
        y = 2;
        require(x == 1);
        require(x == 2);
    }

}
```

---v

## Types: "Reference" Types

```Solidity
Contract Foo {
    // Reference types are stored as a reference to some other location. Only
    // their reference must be copied during assignment.
    contract Foo {
    mapping(uint => uint) forLater;

    // Reference types are stored as a reference to some other location. Only
    // their reference must be copied during assignment.
    function referenceTypes() public {
        // arrays
        uint8[3] memory arr = [1, 2, 3];

        // mapping: can only be initialized from state variables
        mapping(uint => uint) storage balances = forLater;
        balances[0] = 500;

        // dynamic length strings
        string memory foo = "<3 Solidity";

        // also: Structs

        // Two or more variables can share a reference, so be careful!
        uint8[3] memory arr2 = arr;
        arr2[0] = 42;
        require(arr2[0] == 42);
        require(arr[0] == 42); // arr and arr2 are the same thing, so mod to one affects the other
    }
}
}
```

---v

## Data Location

`Data Location` refers to the storage of `Reference Types`. As these are passed
by reference, it effectively dictates where this reference points to. It can be
one of 3 places:

* memory: Stored only in memory; cannot outlive a given external function call
* storage: Stored in the contract's permanent on-chain storage
* calldata: read-only data, using this can avoid copies

---v

## Data Location Sample

```Solidity
contract DataLocationSample {
    struct Foo {
        int i;
    }

    Foo storedFoo;

    // Data Location specifiers affect function arguments and return values...
    function test(Foo memory val) public returns (Foo memory) {
        // ...and also variables within a function
        Foo memory copy = val;

        // storage varables must be assigned befor use.
        Foo storage fooFromStorage = storedFoo;
        fooFromStorage.i = 1;
        require(storedFoo.i == 1, "writes to storage variables affect storage");

        // memory variables can be initialized from storage variables
        // (but not the other way around)
        copy = fooFromStorage;

        // but they are an independent copy
        copy.i = 2;
        require(copy.i == 2);
        require(storedFoo.i == 1, "writes to memory variables cannot affect storage");

        return fooFromStorage;
    }
}
```

---v

## Enums

```Solidity
contract Foo {
    enum Suite {
        Hearts,
        Diamonds,
        Clubs,
        Spades
    }

    function getHeartsSuite() public returns (Suite) {
        Suite hearts = Suite.Hearts;
        return hearts;
    }
}
```

---v

## Structs

```Solidity
contract Foo {
    struct Ballot {
        uint32 index;
        string name;
    }

    function makeSomeBallot() public returns (Ballot memory) {
        Ballot memory ballot;
        ballot.index = 1;
        ballot.name = "John Doe";
        return ballot;
    }
}
```

---

## Solidity Hands-On

---v

## Dev Environment

We will use the online [Remix IDE](https://remix.ethereum.org) for our sample
coding. It provides an editor, compiler, EVM, and debugger all within the
browser, making it trivial to get started.

---v

## Flipper Example

Code along and explain as you go

---v

## Student-exercise: Multiplier

Quick practice assigment:

Remix: https://remix.ethereum.org

* Write a contract which has a `uint256` storage value
* Write function(s) to multiply it with a user-specified value
* Interact with it: can you force an overflow?

Bonus:

* Prevent your multiplier function from overflowing
* Rewrite this prevention as a `modifier noOverflow()`

---

# Vyper

* Also designed for the EVM
* Similar to Python
* Intentionally lacks some features such as inheritance
* Auditable: "Simplicity for the reader is more important than simplicity for the writer"

---v

## Compared to Solidity

Vyper mostly lacks features found in Solidity, all in the spirit of improving
readability. Some examples:

* No Inheritance
* No modifiers
* No function overloading
* No recursive calling (!)
* No infinite-loops

---v

## Basics

```Python
# There is no `contract` keyword. Like Python Modules, a contract is implicitly
# scoped by the file in which it is found.

# storage variables are declared outside of any function
bar: uint

# init is used to deploy a contract and initialize its state
@external
def __init__(val):
    self.bar = val
```

---v

## Functions

```Python
@external
def doSomething() -> bool:
    return True
```

---v

## Decorators and Payable

```Python
# Vyper contains decorators for restricting functions:

@external # function can only be called externally
@internal # function can only be called within current context
@pure # cannot read state or environment vars
@view # cannot alter contract state
@payable # function may receive Ether

# also, to cover the most common use case for Solidity's modifiers:
@nonreentrant(<unique_key>) # prevents reentrancy for given id
```

source: https://docs.vyperlang.org/en/stable/control-structures.html#decorators-reference

---v

## Types

```Python
# value types are small and/or fixed size and are copied
@external
def valueTypes():
    b: bool = False

    # signed and unsigned ints
    i: int128 = -1
    i2: int256 = -10000
    u: uint128 = 42
    u2: uint256 = 42

    # fixed-point (base-10) decimal values with 10 decimal points of precision
    # this has the advantage that literals can be precisely expressed
    f: decimal = 0.1 + 0.3 + 0.6
    assert f == 1.0, "decimal literals are precise!"

    # address type for 20-byte Ethereum addresses
    a: address = 0x1010101010101010101010101010101010101010
    b = a.balance

    # fixed size byte arrays
    selector: bytes4 = 0x12345678

    # bounded byte arrays
    bytes: Bytes[123] = b"\x01"

    # dynamic-length strings
    name: String[16] = "Vyper"

# reference types are potentially large and/or dynamicly sized.
# they are copied-by-reference
@external
def referenceTypes():
    # fixed size list. can also be multidimensional.
    # all elements must be initialized
    list: int128[4] = [1, 2, 3, -4]

    # bounded, dynamic-size array. these have a max size but initialize to empty
    dynArray: DynArray[int128, 3]
    dynArray.append(1)
    dynArray.append(5)
    val: int128 = dynArray.pop() # == 5

    map: HashMap[int128, int128]
    map[0] = 0
    map[1] = 10
    map[2] = 20

```

---v

## Enums

```Python
enum Suite {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

# "hearts" would be considered a value type
hearts: Suite = Suite.Hearts
```

---v

## Structs

```Python
struct Ballot:
    index: uint256
    name: string

# "someBallot" would be considered a reference type
someBallot: Ballot = Ballot({index: 1, name: "John Doe"})
name: string = someBallot.name
```

---v

## Remix Plugin

Remix supports Vyper through a plugin, which can be easily enabled from within
the IDE. First, search for "Vyper" in the plugins tab:

<img style="width: 55%; margin: 10px" src="img/frontier/RemixInstallVyper1.png" />

---v

## Remix Plugin

Use Vyper through the new Vyper tab and use "Remote Compiler"

<img style="width: 55%; margin: 10px" src="img/frontier/RemixInstallVyper2.png" />

---

## Vyper Hands-On

---

# Storing or Secrets On-Chain

TODO: format this -- how to do the one-click-per-line thing?

Can we store secrets on-chain? What if we want to password-protect a particular
contract call?

Obviously we can't store any plaintext secrets on-chain, as doing so reveals
them.

---v

## Storing Hashed Secrets On-Chain

What about storing the hash of a password on chain and using this to verify some
user-input?

Accepting a pre-hash also reveals the secret. This reveal may occur in a txn
before it is executed and settled, allowing someone to frontrun it.

---v

## Verifying with commit-reveal

One potential solution is a commit-reveal scheme where we hash our reveal with
some salt, then later reveal it.

```
// stored on-chain:
secret_hash = hash(secret)
```

```
// first txn, this must execute and settle on chain before the final reveal.
// this associates a user with the soon-to-be-revealed secret
commitment = hash(salt, alleged_secret)
```

```
// final reveal, this must not be made public until commitment is recorded
reveal = alleged_secret, salt
verify(alleged_secret == secret)
verify(commitment == hash(salt, alleged_secret))
```

---v

## Alternative: Signature

Another approach is to use public-key cryptography. We can store the public key
of some keypair and then demand a signature from the corresponding private-key.

This can be expanded with multisig schemes and similar.

How does this differ from the commit-reveal scheme?

Note:

Commit-reveal requires that a specific secret be revealed at some point for
verification. A signature scheme provides a lot more flexibility for keeping the
secret(s) secure.

---

# TODO

##### Some things that I think are currently lacking:

* Clean up (merge slides, reorder)
* Visual aids

*  gas: mention alternative (hint at benchmarking?)

** [.] Transactions - should mention revert here
*  [ ] Precompiles (original ones, plus what moonbeam has done?)
*  [ ] link to Ethernaut, Zoombies, other resources, etc.

*  [ ] Non-transactions (view calls / RPC queries)
*  [ ] checksum cases
*  [ ] Loops - introduce in solidity but cover in more depth with vyper (no inf loops)
*  [ ] Tokenization, wrapped tokens, etc.

