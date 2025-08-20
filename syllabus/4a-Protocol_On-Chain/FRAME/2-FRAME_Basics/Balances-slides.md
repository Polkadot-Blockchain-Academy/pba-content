---
title: Balance Management in FRAME
description: Modern patterns for working with native token balances using fungible traits
duration: 2 hours
---

# Balance Management in FRAME

---

## Why This Workshop?

Working with balances is fundamental to blockchain development:

- 💰 Every pallet deals with tokens somehow
- 🔒 Need to lock funds for staking, voting, deposits
- 🎯 Must handle these patterns correctly and securely

Let's master modern balance management! 🚀

---

## Agenda

<pba-flex center>

1. Core concepts: Holds vs Freezes
1. The fungible trait hierarchy
1. Hands-on: Staking scenario
1. Hands-on: Voting scenario  
1. Hands-on: Storage deposits
1. Best practices & common patterns

</pba-flex>

---

## Quick Note on Legacy Code

<pba-flex center>

⚠️ **You might see these in older code:**

```rust
use frame_support::traits::{Currency, ReservableCurrency, LockableCurrency};
```

**These are deprecated!** If you see them:
- Don't use them in new code
- Use `fungible` traits instead
- Ask if unsure

We'll focus on the modern approach 🎯

</pba-flex>

---

## Core Concepts: Balance Arithmetic

<pba-flex center>

```text
┌─────────────────────────────────────┐
│         Total Balance               │
│  ┌─────────────┬─────────────────┐  │
│  │   Free      │     Held        │  │
│  │  Balance    │    Balance      │  │
│  └─────────────┴─────────────────┘  │
└─────────────────────────────────────┘

Spendable = Free - max(Freezes) - ED
```

**Key Insight**: Holds affect total balance, Freezes only affect spendability

</pba-flex>

---

## Holds vs Freezes

<pba-cols>
<pba-col>

### Holds 📌

- **Stack** (cumulative)
- **Slashable**
- **Named reasons**
- **Affect total balance**

Use for:
- Staking bonds
- Storage deposits
- Collateral

</pba-col>
<pba-col>

### Freezes 🧊

- **Overlap** (take maximum)
- **Not slashable**
- **Named reasons**
- **Only affect free balance**

Use for:
- Voting locks
- Vesting schedules
- Minimum balance requirements

</pba-col>
</pba-cols>

---

## Visual: How They Work

```text
Account with 1000 tokens:

Hold 300 for staking:     Total: 1000, Free: 700, Held: 300
Hold 200 for storage:     Total: 1000, Free: 500, Held: 500

Freeze 400 for voting:    Total: 1000, Free: 500, Held: 500
                         Spendable: 100 (500 - 400)

Freeze 600 for vesting:   Total: 1000, Free: 500, Held: 500
                         Spendable: 0 (500 - max(400,600) = -100)
```

**Remember**: Holds stack, Freezes take the maximum!

---

## The Trait Hierarchy

```rust[1-4|5-8|9-12|13-16]
// 1. Inspect - Read balance information
pub trait Inspect<AccountId> {
    fn total_balance(who: &AccountId) -> Self::Balance;
    fn reducible_balance(who: &AccountId, preserve: Preservation, force: Fortitude) -> Self::Balance;
}

// 2. Mutate - Safely modify balances
pub trait Mutate<AccountId>: Inspect<AccountId> {
    fn transfer(source: &AccountId, dest: &AccountId, amount: Self::Balance, preservation: Preservation) -> Result<Self::Balance, DispatchError>;
}

// 3. Unbalanced - Low-level operations (use carefully!)
pub trait Unbalanced<AccountId>: Inspect<AccountId> {
    fn set_total_issuance(amount: Self::Balance);
}

// 4. Balanced - Operations that return imbalances
pub trait Balanced<AccountId>: Inspect<AccountId> + Unbalanced<AccountId> {
    fn deposit(who: &AccountId, value: Self::Balance, precision: Precision) -> Result<Imbalance, DispatchError>;
}
```

---

## Scenario 1: Staking Pattern

Let's build a simple staking mechanism using holds!

---

## Step 1: Define Hold Reason

```rust[1-7|9-15]
// In your pallet
use frame_support::pallet_prelude::*;

#[pallet::composite_enum]
pub enum HoldReason {
    /// Funds held for staking
    Staking,
}

// Configure in pallet
#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeHoldReason: From<HoldReason>;
    
    type Currency: fungible::Inspect<Self::AccountId>
        + fungible::Mutate<Self::AccountId>
        + fungible::hold::Inspect<Self::AccountId>
        + fungible::hold::Mutate<Self::AccountId, Reason = Self::RuntimeHoldReason>;
}
```

---

## Step 2: Implement Staking

```rust[1-20|4-6|8-10|12-15|17-18]
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn stake(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
        let who = ensure_signed(origin)?;
        
        // Hold the tokens
        T::Currency::hold(&HoldReason::Staking.into(), &who, amount)?;
        
        // Update staking info
        StakedAmount::<T>::insert(&who, amount);
        TotalStaked::<T>::mutate(|total| *total = total.saturating_add(amount));
        
        // Emit event
        Self::deposit_event(Event::Staked { who, amount });
        
        Ok(())
    }
    
    pub fn unstake(origin: OriginFor<T>) -> DispatchResult {
        let who = ensure_signed(origin)?;
        let amount = StakedAmount::<T>::take(&who).ok_or(Error::<T>::NotStaking)?;
        
        // Release the hold
        T::Currency::release(&HoldReason::Staking.into(), &who, amount, Precision::BestEffort)?;
        
        // Update total
        TotalStaked::<T>::mutate(|total| *total = total.saturating_sub(amount));
        
        Self::deposit_event(Event::Unstaked { who, amount });
        Ok(())
    }
}
```

---

## Step 3: Slashing (Holds are Slashable!)

```rust[1-15|5-8|10-12]
impl<T: Config> Pallet<T> {
    pub fn slash_staker(staker: &T::AccountId, percent: Percent) -> BalanceOf<T> {
        let staked = StakedAmount::<T>::get(staker).unwrap_or_default();
        let slash_amount = percent * staked;
        
        // Slash from held balance
        let actual_slash = T::Currency::burn_held(
            &HoldReason::Staking.into(),
            staker,
            slash_amount,
            Precision::BestEffort,
            Fortitude::Force,
        ).unwrap_or_default();
        
        // Update records
        StakedAmount::<T>::mutate(staker, |amt| {
            *amt = amt.map(|a| a.saturating_sub(actual_slash))
        });
        
        actual_slash
    }
}
```

---

## Scenario 2: Voting Pattern

Now let's implement conviction voting using freezes!

---

## Step 1: Define Freeze Reason

```rust[1-7|9-14]
#[pallet::composite_enum]
pub enum FreezeReason {
    /// Funds frozen for voting
    Voting,
}

// Add to Config
#[pallet::config]
pub trait Config: frame_system::Config {
    // ... existing config
    
    type RuntimeFreezeReason: From<FreezeReason>;
    
    type Currency: fungible::freeze::Inspect<Self::AccountId>
        + fungible::freeze::Mutate<Self::AccountId, Id = Self::RuntimeFreezeReason>;
}
```

---

## Step 2: Conviction Multiplier

```rust[1-20|2-10|12-20]
#[derive(Copy, Clone, Encode, Decode, TypeInfo)]
pub enum Conviction {
    /// 0.1x votes, unlocked
    None,
    /// 1x votes, locked for 1x period
    Locked1x,
    /// 2x votes, locked for 2x period
    Locked2x,
    /// 3x votes, locked for 4x period
    Locked3x,
}

impl Conviction {
    pub fn votes<B: From<u8> + Mul<Output = B>>(&self, balance: B) -> B {
        match self {
            Conviction::None => balance / 10u8.into(),
            Conviction::Locked1x => balance,
            Conviction::Locked2x => balance * 2u8.into(),
            Conviction::Locked3x => balance * 3u8.into(),
        }
    }
}
```

---

## Step 3: Implement Voting

```rust[1-25|7-9|11-17|19-23]
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn vote(
        origin: OriginFor<T>,
        proposal: ProposalId,
        approve: bool,
        conviction: Conviction,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;
        let balance = T::Currency::reducible_balance(&who, Preservation::Preserve, Fortitude::Polite);
        
        // Calculate voting power
        let votes = conviction.votes(balance);
        
        // Extend freeze based on conviction
        T::Currency::extend_freeze(
            &FreezeReason::Voting.into(),
            &who,
            balance, // Freeze the full balance used for voting
        )?;
        
        // Record vote
        Votes::<T>::insert((&who, proposal), VoteRecord {
            approve,
            votes,
            conviction,
        });
        
        Self::deposit_event(Event::Voted { who, proposal, approve, votes });
        Ok(())
    }
}
```

---

## Key Insight: Multiple Votes

```rust[1-15|6-7|9-10|12-13]
// User has 1000 tokens

// Vote 1: 500 tokens with 2x conviction
vote(proposal_1, true, Conviction::Locked2x);
// Freeze: 500 tokens
// Voting power: 1000 votes

// Vote 2: 800 tokens with 1x conviction  
vote(proposal_2, false, Conviction::Locked1x);
// Freeze extended to: 800 tokens (max of 500, 800)
// Voting power: 800 votes

// User can still spend: 1000 - 800 = 200 tokens
// Both votes are active with different powers!
```

Freezes overlap perfectly for voting scenarios! 🎯

---

## Scenario 3: Storage Deposits

Using the `Consideration` trait for elegant deposit handling!

---

## Understanding Considerations

```rust
/// A trait for creating and managing storage deposits
pub trait Consideration<AccountId>: Sized {
    /// Create a new consideration, reserving funds
    fn new(who: &AccountId, footprint: Footprint) -> Result<Self, DispatchError>;
    
    /// Update the footprint and adjust funds accordingly
    fn update(self, who: &AccountId, footprint: Footprint) -> Result<Self, DispatchError>;
    
    /// Drop the consideration, refunding any remaining funds
    fn drop(self, who: &AccountId) -> Result<(), DispatchError>;
}
```

**Key Feature**: Dropping a consideration automatically refunds! 🪄

---

## Step 1: Configure Storage Deposits

```rust[1-15|7-13]
use frame_support::traits::fungible::{
    HoldConsideration,
    LinearStoragePrice,
};

#[pallet::config]
pub trait Config: frame_system::Config {
    /// Base deposit for any storage
    #[pallet::constant]
    type BaseDeposit: Get<BalanceOf<Self>>;
    
    /// Per-byte deposit
    #[pallet::constant]
    type ByteDeposit: Get<BalanceOf<Self>>;
    
    /// Storage deposit helper
    type Consideration: Consideration<Self::AccountId, Footprint>;
}
```

---

## Step 2: Implement Storage Operations

```rust[1-30|6-11|13-17|19-20|22-28]
#[pallet::storage]
pub type DataStorage<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (Vec<u8>, T::Consideration)>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn store_data(origin: OriginFor<T>, data: Vec<u8>) -> DispatchResult {
        let who = ensure_signed(origin)?;
        
        ensure!(data.len() <= 10_000, Error::<T>::DataTooLarge);
        
        // Calculate storage footprint
        let footprint = Footprint {
            count: 1,
            size: data.len() as u64,
        };
        
        // Create consideration (automatically holds deposit)
        let consideration = T::Consideration::new(&who, footprint)?;
        
        // Store with consideration
        DataStorage::<T>::insert(&who, (data.clone(), consideration));
        
        Self::deposit_event(Event::DataStored { who, size: data.len() as u32 });
        Ok(())
    }
    
    pub fn remove_data(origin: OriginFor<T>) -> DispatchResult {
        let who = ensure_signed(origin)?;
        
        // Take data and consideration
        let (_data, consideration) = DataStorage::<T>::take(&who)
            .ok_or(Error::<T>::NoDataStored)?;
        
        // Dropping consideration automatically refunds!
        drop(consideration);
        
        Self::deposit_event(Event::DataRemoved { who });
        Ok(())
    }
}
```

---

## Step 3: Configure in Runtime

```rust[1-20|7-8|10-16]
// In runtime configuration
parameter_types! {
    pub const BaseDeposit: Balance = 10 * UNIT;
    pub const ByteDeposit: Balance = UNIT / 1000; // 0.001 per byte
}

impl pallet_storage::Config for Runtime {
    type BaseDeposit = BaseDeposit;
    type ByteDeposit = ByteDeposit;
    
    // Use linear pricing with holds
    type Consideration = HoldConsideration<
        AccountId,
        Balances,
        HoldReason,
        LinearStoragePrice<BaseDeposit, ByteDeposit>,
    >;
}
```

Storage deposit = 10 UNIT + (0.001 UNIT × bytes)

---

## Consideration Types Available

<pba-cols>
<pba-col>

### Hold-based
- `HoldConsideration`
- `LoneHoldConsideration`

Use when:
- Need slashing
- Multiple holds per account
- Storage is long-term

</pba-col>
<pba-col>

### Freeze-based
- `FreezeConsideration`  
- `LoneFreezeConsideration`

Use when:
- Just preventing spending
- Simpler use cases
- Single freeze per account

</pba-col>
</pba-cols>


---

## Common Patterns & Best Practices

```rust[1-20|1-8|10-15|17-20]
// 1. Always use Precision::BestEffort for user operations
T::Currency::release(
    &HoldReason::Staking.into(),
    &who,
    amount,
    Precision::BestEffort, // Don't fail if we can't release exact amount
)?;

// 2. Check reducible_balance before operations
let available = T::Currency::reducible_balance(
    &who,
    Preservation::Preserve, // Keep account alive
    Fortitude::Polite,     // Respect holds/freezes
);
ensure!(available >= amount, Error::<T>::InsufficientBalance);

// 3. Use ensure_can_withdraw for pre-checks
T::Currency::ensure_can_withdraw(&who, amount, WithdrawReasons::all(), balance)?;
```

---

## Testing Your Balance Logic

```rust[1-25|4-8|10-14|16-20|22-24]
#[test]
fn staking_with_holds_works() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let stake_amount = 500;
        
        // Setup: Give Alice some balance
        Balances::set_balance(&alice, 1000);
        
        // Stake tokens
        assert_ok!(Staking::stake(RuntimeOrigin::signed(alice), stake_amount));
        
        // Verify hold
        assert_eq!(Balances::balance_on_hold(&HoldReason::Staking.into(), &alice), stake_amount);
        assert_eq!(Balances::free_balance(&alice), 1000); // Free stays same
        assert_eq!(Balances::total_balance(&alice), 1000); // Total unchanged
        
        // Try to spend held funds (should fail)
        assert_noop!(
            Balances::transfer(RuntimeOrigin::signed(alice), 2, 600),
            TokenError::FundsUnavailable
        );
        
        // Can spend non-held funds
        assert_ok!(Balances::transfer(RuntimeOrigin::signed(alice), 2, 400));
    });
}
```

---

## Activity Time! 🛠️

Let's practice these patterns:

1. **Staking Exercise**: Implement slash functionality
2. **Voting Exercise**: Add vote removal with unlock periods  
3. **Storage Exercise**: Update data size with deposit adjustment

Check `Balances-Activity.md` for detailed instructions!

---

## Summary: When to Use What?

<pba-flex center>

| Use Case | Pattern | Reason |
|----------|---------|---------|
| Staking | Hold | Slashable, trackable |
| Voting | Freeze | Overlapping locks |
| Storage Deposits | Hold + Consideration | Auto-refund |
| Vesting | Freeze | Time-based release |
| Collateral | Hold | May need slashing |
| Min Balance | Freeze | Account restriction |

</pba-flex>

---

## Key Takeaways

<pba-flex center>

1. 🎯 **Always use** `fungible` traits for balance management
2. 📌 **Holds**: Stack, slashable, for bonds/deposits
3. 🧊 **Freezes**: Overlap, for locks/restrictions
4. 💎 **Considerations**: Elegant storage deposit pattern
5. ✅ **Best practices**: Use `BestEffort`, check `reducible_balance`
6. 🚀 **You're ready** to build production pallets!

</pba-flex>

---

## Additional Resources

<pba-flex center>

- [Fungible Trait Docs](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/fungible/index.html)
- [PR #12951: Fungible Trait Introduction](https://github.com/paritytech/substrate/pull/12951)
- [Storage Deposits Guide](https://docs.substrate.io/build/storage-deposits/)
- Example Pallets:
  - `pallet-staking` (holds)
  - `pallet-conviction-voting` (freezes)
  - `pallet-preimage` (considerations)

</pba-flex>

---

<!-- .slide: data-background-color="#000" -->

# Questions?

</content>
