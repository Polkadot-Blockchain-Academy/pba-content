---
title: Introduction to Polkadot SDK
description: Building blockchains with Substrate and FRAME
duration: 60 minutes
---

# Introduction to Polkadot SDK

Building Production Blockchains with Substrate & FRAME

---

## What We'll Cover

<pba-flex center>

1. Why Rust for blockchains?
2. Why building blockchains is hard
3. The Substrate solution
4. Runtime architecture & upgradability
5. FRAME: Making it manageable
6. Your path forward

</pba-flex>

---

## Why Rust? 🦀

<pba-cols>
<pba-col>

### The Numbers Don't Lie

**Microsoft Study (2019)**:
- 70% of security vulnerabilities are memory safety issues
- Rust eliminates entire classes of bugs:
  - Buffer overflows
  - Use after free
  - Data races
  - Null pointer dereferencing

</pba-col>
<pba-col>

### Perfect for Blockchains

✅ **Memory safe** without garbage collection
✅ **Deterministic** execution
✅ **Zero-cost abstractions**
✅ **Fearless concurrency**
✅ **Great WASM support**

*"If it compiles, it works"* - mostly true!

</pba-col>
</pba-cols>

---

## Real World Impact

<pba-flex center>

### Google's Android Team (2022)

> "Memory safety vulnerabilities dropped from 76% to 35% after adopting Rust"

### In Blockchain Context

- **Solana**: ~500k lines of Rust
- **Polkadot**: ~1M lines of Rust
- **Critical bugs**: Dramatically lower than C++ chains

</pba-flex>

---

## Building a Blockchain is HARD

<pba-flex center>

To build a production blockchain, you need expertise in **8 disciplines**:

</pba-flex>

---

## The 8 Disciplines of Blockchain

<pba-cols>
<pba-col>

### Infrastructure Layer
1. **🌐 Networking**: P2P, libp2p, gossip protocols
2. **🔐 Cryptography**: Hashing, signatures, VRFs
3. **🤝 Consensus**: BFT, longest chain, finality
4. **💾 Database**: State management, pruning

</pba-col>
<pba-col>

### Application Layer
5. **⚙️ Execution**: VM design, WASM, metering
6. **💰 Economics**: Fees, rewards, slashing
7. **🗳️ Governance**: Upgrades, parameter changes
8. **🛠️ Dev Experience**: Finally, your business logic!

</pba-col>
</pba-cols>

Each discipline = months/years to master 😱

---

## The Traditional Approach

<pba-cols>
<pba-col>

### Option 1: Fork & Modify

```bash
git clone https://github.com/bitcoin/bitcoin
# or
git clone https://github.com/ethereum/go-ethereum
```

**Problems**:
- Inherit all technical debt
- Hard to upgrade
- Limited by original design
- Merge conflicts forever

</pba-col>
<pba-col>

### Option 2: Build from Scratch

```rust
fn main() {
    // TODO: Implement blockchain
    println!("Good luck!");
}
```

**Problems**:
- Years of development
- Reinvent every wheel
- Security vulnerabilities
- Never reach production

</pba-col>
</pba-cols>

---

## Enter Substrate

<pba-flex center>

> **Substrate** handles disciplines 1-7, so you can focus on #8

```text
┌─────────────────────────────────────────┐
│            Your Business Logic          │ ← You work here!
├─────────────────────────────────────────┤
│                FRAME                    │ ← We'll learn this
├─────────────────────────────────────────┤
│              Substrate Core             │ ← Given to you
│  (Networking, Consensus, Database...)   │
└─────────────────────────────────────────┘
```

**Before**: Build entire blockchain
**After**: Build just your features

</pba-flex>

---

## Still Not a Smart Contract Platform

<pba-cols>
<pba-col>

### Smart Contracts
- Deploy to existing chain
- Limited by platform
- Pay rent forever
- Share resources
- Can't customize consensus

</pba-col>
<pba-col>

### Substrate Chains
- **Your own blockchain**
- **Full customization**
- **No platform limits**
- **Dedicated resources**
- **Choose your consensus**

*"With great power comes great responsibility"*

</pba-col>
</pba-cols>

---

## The Power of Pallets

<pba-flex center>

Instead of building everything, **mix and match pre-built components**:

```rust
construct_runtime! {
    System: frame_system,            // ✅ Core blockchain logic
    Timestamp: pallet_timestamp,     // ✅ Time management
    Balances: pallet_balances,       // ✅ Token logic
    Staking: pallet_staking,         // ✅ Proof of Stake
    Democracy: pallet_democracy,     // ✅ Governance
    
    YourPallet: your_custom_logic,   // 🎯 Your innovation!
}
```

**Audited, production-tested code = safer, faster, cheaper**

</pba-flex>

---

## Runtime Architecture

<pba-flex center>

```text
┌─────────────────────────┐
│      Client/Node        │ ← Long-lived, hard to upgrade
│  - P2P Networking       │
│  - Block Production     │
│  - Storage Database     │
├─────────────────────────┤ ← Host API boundary
│       Runtime           │ ← Your logic lives here!
│  - Business Logic       │
│  - State Transition     │
│  - Stored as WASM       │
└─────────────────────────┘
```

**Key Insight**: Runtime is just data in the blockchain!

</pba-flex>

---

## The Magic of WASM

<pba-cols>
<pba-col>

### Traditional Blockchain
```text
Need upgrade?
→ Convince all nodes
→ Hard fork
→ Pray everyone updates
→ Chain splits if they don't
```

</pba-col>
<pba-col>

### Substrate Chain
```rust
// It's just a transaction!
#[pallet::call]
fn set_code(
    origin: OriginFor<T>,
    new_code: Vec<u8>,
) -> DispatchResult {
    ensure_root(origin)?;
    // Update WASM code
    storage::put(CODE_KEY, new_code);
    Ok(())
}
```

</pba-col>
</pba-cols>

---

## Analogy: Gaming Console 🎮

<pba-cols>
<pba-col>

### Traditional (Hard Fork)

<img style="width: 400px" src="./img/console-upgrade.png" />

Buy new console for new games

</pba-col>
<pba-col>

### Substrate (Runtime Upgrade)

<img style="width: 400px" src="./img/game-cartridge.png" />

Same console, just change the game!

</pba-col>
</pba-cols>

The "console" (node) rarely changes, but "games" (runtime) upgrade seamlessly

---

## When Do Nodes Need Upgrading?

<pba-flex center>

### Host API Changes

Rare but important:
- New crypto primitives
- Performance improvements  
- Bug fixes in node software

### Everything Else?

**Just runtime upgrades!** No node coordination needed

```rust
// Recent Polkadot stats:
// Runtime upgrades: ~100
// Required node upgrades: ~10
```

90% of upgrades are forkless! 🎉

</pba-flex>

---

## Enter FRAME

**Framework for Runtime Aggregation of Modularized Entities**

<pba-cols>
<pba-col>

### Without FRAME
```rust
// 1000s of lines of boilerplate
impl_storage! {
    // Manual storage management
}
impl_dispatch! {
    // Manual call routing  
}
impl_metadata! {
    // Manual metadata
}
// ... and much more
```

</pba-col>
<pba-col>

### With FRAME
```rust
#[pallet::storage]
pub type MyValue<T> = StorageValue<_, u32>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn do_something(
        origin: OriginFor<T>,
        value: u32,
    ) -> DispatchResult {
        MyValue::<T>::put(value);
        Ok(())
    }
}
```

</pba-col>
</pba-cols>

---

## FRAME in Action

<pba-flex center>

### A Complete Pallet in ~50 Lines

```rust
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    
    #[pallet::pallet]
    pub struct Pallet<T>(_);
    
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>>;
    }
    
    #[pallet::storage]
    pub type Counter<T> = StorageValue<_, u32, ValueQuery>;
    
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CounterIncremented { new_value: u32 },
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn increment(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;
            let new_value = Counter::<T>::mutate(|v| { *v += 1; *v });
            Self::deposit_event(Event::CounterIncremented { new_value });
            Ok(())
        }
    }
}
```

**That's it!** Storage, events, calls - all handled by FRAME

</pba-flex>

---

## Real Production Examples

<pba-cols>
<pba-col>

### Built with Substrate

- **Polkadot & Kusama**
- **Chainlink**
- **Compound Gateway**
- **Acala**
- **Moonbeam**
- **Astar**

100+ production chains

</pba-col>
<pba-col>

### Use Cases

- DeFi protocols
- Gaming chains
- Identity systems
- Supply chain
- IoT networks
- Private chains

*If you can imagine it, you can build it*

</pba-col>
</pba-cols>

---

## Your Learning Path

<pba-flex center>

### This Week at PBA

1. **Today**: Understand the architecture
2. **Day 1-2**: Master FRAME basics
3. **Day 3-4**: Advanced patterns
4. **Weekend**: Build your own pallet!

### Key Skills You'll Gain

✅ Design pallets with proper abstractions
✅ Use modern balance management (holds/freezes)
✅ Implement governance & upgrades
✅ Write secure, efficient runtime code

</pba-flex>

---

## The Bigger Picture

<pba-cols>
<pba-col>

### Substrate Gives You

- Battle-tested infrastructure
- Modular architecture
- Forkless upgrades
- Rich pallet ecosystem
- Rust's safety guarantees

</pba-col>
<pba-col>

### You Focus On

- Your unique value proposition
- Business logic
- User experience
- Going to market faster
- Innovation, not reimplementation

</pba-col>
</pba-cols>

---

## Why This Matters

<pba-flex center>

### The Old Way
> "It took us 3 years and $10M to launch our blockchain"

### The Substrate Way  
> "We went from idea to mainnet in 6 months with 3 developers"

**Real quote from a Substrate team** 🚀

</pba-flex>

---

## Summary

<pba-flex center>

1. **Rust** eliminates entire classes of bugs (70% reduction!)
2. **Building blockchains requires 8 disciplines** - it's genuinely hard
3. **Substrate** handles 7/8 disciplines for you
4. **WASM runtime** enables forkless upgrades
5. **FRAME** makes the remaining work manageable
6. **Production ready** - 100+ live chains prove it

</pba-flex>

---

## What's Next?

<pba-flex center>

### Immediate Next Steps

1. **Calls, Events & Errors** - How FRAME pallets work
2. **Frameless Exercise** - Appreciate what FRAME does
3. **Balance Management** - Modern token handling
4. **Build something!** - Theory → Practice

### Remember

You're not just learning a framework...
**You're joining a revolution in blockchain development!** 🚀

</pba-flex>

---

<!-- .slide: data-background-color="#000" -->

# Questions?

Let's build the future of Web3 together 🦀