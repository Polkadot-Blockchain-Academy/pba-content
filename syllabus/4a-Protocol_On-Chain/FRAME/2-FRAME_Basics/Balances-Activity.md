# Balance Management Activities

Complete these exercises to practice working with modern fungible traits. Each activity builds on concepts from the lecture.

## Setup

Create a new pallet or use the provided template with the following imports:

```rust
use frame_support::{
    pallet_prelude::*,
    traits::fungible::{self, Inspect, Mutate, hold::*, freeze::*},
};
use frame_system::pallet_prelude::*;
```

## Activity 1: Enhanced Staking (30 minutes)

Building on the staking example from the lecture, add these features:

### Task 1.1: Implement Compound Staking
Allow users to stake additional amounts on top of their existing stake.

```rust
pub fn stake_extra(origin: OriginFor<T>, additional: BalanceOf<T>) -> DispatchResult {
    // TODO: 
    // 1. Get existing stake amount
    // 2. Add to existing hold (don't create new one)
    // 3. Update storage
}
```

### Task 1.2: Implement Partial Unstaking
Allow users to unstake a portion of their staked amount.

```rust
pub fn unstake_partial(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
    // TODO:
    // 1. Ensure user has enough staked
    // 2. Release partial hold
    // 3. Update storage accordingly
}
```

### Task 1.3: Implement Slashing with Treasury
When slashing occurs, send 50% to treasury, burn the rest.

```rust
pub fn slash_with_treasury(
    staker: &T::AccountId, 
    percent: Percent
) -> (BalanceOf<T>, BalanceOf<T>) {
    // TODO:
    // 1. Calculate slash amount
    // 2. Slash the hold
    // 3. Send 50% to treasury account
    // 4. Burn the remaining 50%
    // Return (treasury_amount, burned_amount)
}
```

### Testing Your Implementation
```rust
#[test]
fn compound_staking_works() {
    // Test that multiple stakes accumulate correctly
}

#[test]
fn partial_unstaking_works() {
    // Test that partial unstaking leaves remaining stake intact
}

#[test]
fn slash_split_works() {
    // Test that slashing splits correctly between treasury and burn
}
```

## Activity 2: Advanced Voting (30 minutes)

Enhance the voting system with unlock schedules and vote updates.

### Task 2.1: Implement Vote Removal with Unlock Period
Add time-based unlocking based on conviction level.

```rust
pub struct UnlockSchedule<BlockNumber> {
    pub amount: Balance,
    pub unlock_at: BlockNumber,
}

pub fn remove_vote(origin: OriginFor<T>, proposal: ProposalId) -> DispatchResult {
    // TODO:
    // 1. Get vote record and conviction
    // 2. Calculate unlock period based on conviction:
    //    - None: immediate
    //    - Locked1x: current_block + 100
    //    - Locked2x: current_block + 200
    //    - Locked3x: current_block + 400
    // 3. Create unlock schedule
    // 4. Don't release freeze yet, schedule it
}

pub fn unlock_expired(origin: OriginFor<T>) -> DispatchResult {
    // TODO:
    // 1. Check all unlock schedules for caller
    // 2. Process any that have expired
    // 3. Reduce freeze by unlocked amount
}
```

### Task 2.2: Implement Vote Updates
Allow changing vote without removing the freeze.

```rust
pub fn update_vote(
    origin: OriginFor<T>,
    proposal: ProposalId,
    new_approve: bool,
    new_conviction: Conviction,
) -> DispatchResult {
    // TODO:
    // 1. Ensure vote exists
    // 2. Calculate new voting power
    // 3. Update freeze if new conviction requires more
    // 4. Update vote record
}
```

### Task 2.3: Multi-Proposal Voting
Handle multiple concurrent votes with different convictions.

```rust
pub fn get_max_freeze_needed(who: &T::AccountId) -> BalanceOf<T> {
    // TODO:
    // 1. Iterate through all active votes for account
    // 2. Calculate freeze needed for each based on conviction
    // 3. Return the maximum
}
```

## Activity 3: Dynamic Storage Deposits (30 minutes)

Implement a more sophisticated storage system with updates and transfers.

### Task 3.1: Update Storage with Deposit Adjustment
Allow updating stored data with automatic deposit adjustment.

```rust
pub fn update_data(origin: OriginFor<T>, new_data: Vec<u8>) -> DispatchResult {
    let who = ensure_signed(origin)?;
    
    // TODO:
    // 1. Get existing data and consideration
    // 2. Calculate new footprint
    // 3. Use consideration.update() to adjust deposit
    // 4. Store updated data with new consideration
}
```

### Task 3.2: Transfer Storage Ownership
Allow transferring stored data to another account (with deposit).

```rust
pub fn transfer_storage(
    origin: OriginFor<T>, 
    to: T::AccountId
) -> DispatchResult {
    // TODO:
    // 1. Take data and old consideration from sender
    // 2. Drop old consideration (refund to sender)
    // 3. Create new consideration for recipient
    // 4. Store under new owner
}
```

### Task 3.3: Implement Storage Quotas
Add per-account storage limits using both item count and total size.

```rust
pub struct StorageQuota {
    pub max_items: u32,
    pub max_bytes: u64,
}

pub fn check_quota(who: &T::AccountId, additional_size: usize) -> DispatchResult {
    // TODO:
    // 1. Count existing items for account
    // 2. Sum existing storage size
    // 3. Check if new storage would exceed quotas
    // 4. Return appropriate error if exceeded
}
```

## Activity 4: Integration Challenge (30 minutes)

Combine all three patterns in a mini-DAO system.

### Requirements:
1. Users stake tokens to become members
2. Members can create proposals (requires storage deposit)
3. Members vote with conviction (voting power = stake � conviction)
4. Proposals can slash misbehaving members
5. Storage deposits refunded when proposal completes

### Skeleton Code:
```rust
pub struct Proposal<AccountId, BlockNumber> {
    pub proposer: AccountId,
    pub action: ProposalAction,
    pub votes_for: Balance,
    pub votes_against: Balance,
    pub ends_at: BlockNumber,
}

pub enum ProposalAction {
    SlashMember { who: AccountId, percent: Percent },
    UpdateParameters { /* ... */ },
}

impl<T: Config> Pallet<T> {
    pub fn create_proposal(
        origin: OriginFor<T>,
        action: ProposalAction,
    ) -> DispatchResult {
        // TODO: Implement using storage deposits
    }
    
    pub fn vote_on_proposal(
        origin: OriginFor<T>,
        proposal_id: ProposalId,
        approve: bool,
        conviction: Conviction,
    ) -> DispatchResult {
        // TODO: Implement using freezes + stake amount
    }
    
    pub fn execute_proposal(
        origin: OriginFor<T>,
        proposal_id: ProposalId,
    ) -> DispatchResult {
        // TODO: Execute if passed, refund storage deposit
    }
}
```

## Bonus Challenges

1. **Nested Holds**: Implement a system where holds can have "child holds" (e.g., stake � delegate � sub-delegate)

2. **Freeze Priorities**: Add priority levels to freezes where higher priority freezes can override lower ones

3. **Consideration Splitting**: Allow splitting a storage deposit consideration between multiple owners

4. **Time-locked Holds**: Create holds that automatically release after a certain block number

## Submission Guidelines

1. Implement at least 3 activities
2. Include comprehensive tests
3. Document any design decisions
4. Explain your approach to error handling

## Common Mistakes to Avoid

- L Using `Precision::Exact` when `BestEffort` is more appropriate
- L Forgetting to check `reducible_balance` before operations
- L Not handling the existential deposit properly
- L Creating multiple holds/freezes when you should extend existing ones
- L Forgetting to emit events for important state changes

## Resources

- [Fungible Traits Documentation](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/fungible/index.html)
- [Example: Staking Pallet](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/frame/staking/src/lib.rs)
- [Example: Conviction Voting](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/frame/conviction-voting/src/lib.rs)
- [Example: Preimage Pallet](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/frame/preimage/src/lib.rs)