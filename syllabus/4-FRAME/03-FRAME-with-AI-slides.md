---
title: FRAME with AI
description: Lessons learned from building a full blockchain game with AI, covering common mistakes, design evolution, and best practices for AI-assisted FRAME development.
duration: 1.5 hours
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# FRAME with AI

### Lessons from Building an On-Chain Game

---

## Context: Open Auto Battler

A complete auto-battler game built on Polkadot, 100% AI-generated code:

- 5 custom pallets (card registry, arena, tournament, constructed, common)
- A PolkaVM smart contract port
- React/TypeScript frontend with WASM battle engine
- ~100 commits of iterative AI-driven development

We'll trace the code's evolution to learn what AI gets right, what it gets wrong, and how to guide it.

---

## What AI Gets Right From Day One

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::call_index(0)]
    #[pallet::weight(Weight::default())]
    pub fn start_game(origin: OriginFor<T>, set_id: u32) -> DispatchResult {
        let who = ensure_signed(origin)?;
        ensure!(!ActiveGame::<T>::contains_key(&who), Error::<T>::AlreadyInGame);
        // ...
        Self::deposit_event(Event::GameStarted { player: who });
        Ok(())
    }
}
```

- Proper `ensure_signed` origin checks
- Storage guards with `ensure!`
- Events emitted for state changes
- Error variants with descriptive names

Notes:

AI tools have been trained on enough FRAME code to produce structurally valid pallets. The basic patterns -- origin checking, storage access, event emission, error handling -- are generally correct from the first attempt.

---

## How This Project Was Built

Every line of code was written by AI (Claude Code). The developer's role was:

- Describing what to build in natural language
- Reviewing the output
- Correcting mistakes and redirecting

Actual prompts from the project:

> "i plan to integrate the core engine into a Substrate SDK blockchain pallet. For this, I need the whole project to be no-std and wasm compatible."

> "let's update the pallet so that the submit_shop_phase verifies the actions and executes the battle, and updates the user's state all on-chain."

Notes:

This is a realistic workflow -- not "AI writes perfect code" but "AI writes a first draft, human course-corrects." The session logs show ~50 sessions of iterative development with constant corrections. Understanding what corrections were needed is the lesson.

---

## Phase 1: Bootstrapping

### AI's First Move: Copy the Template

The AI started by copying the entire Substrate parachain template verbatim -- 47 files, ~24,000 lines.

Then it did a mechanical find-and-replace rename:

```rust
// In the runtime, after "renaming":
// Template                            <-- stale comment survived
#[runtime::pallet_index(50)]
pub type TemplatePallet = pallet_auto_battle;  // <-- stale alias
```

The AI renamed the _crate reference_ but missed the _semantic naming_.

This needed a separate commit to fix:

```rust
#[runtime::pallet_index(50)]
pub type AutoBattle = pallet_auto_battle;
```

Notes:

AI is good at mechanical renaming across many files but misses semantic renaming. Names that need to change for clarity but are not syntactically linked to the old name get left behind. Always review the runtime macro aliases after a rename.

---

## Lesson 1: The `without_storage_info` Escape Hatch

The very first pallet included this:

```rust
#[pallet::pallet]
#[pallet::without_storage_info]  // <-- "just make it compile"
pub struct Pallet<T>(_);
```

This disables `MaxEncodedLen` requirements on all storage items.

The AI used it because the game state structs didn't implement `MaxEncodedLen`.

It persisted for **weeks** until explicitly removed, which then required adding `MaxEncodedLen` derives to **every stored struct** and discovering a missing `MaxConditions` config parameter.

Notes:

`without_storage_info` is the #1 AI escape hatch. It makes the pallet compile by silently removing a critical safety check. When you see this in AI-generated code, treat it as a TODO: you need to properly bound every storage type.

---

## Fixing `without_storage_info`

Removing it revealed cascading issues:

```rust
// BEFORE: Missing MaxConditions parameter
pub type BoundedAbility<T> = CoreBoundedAbility<<T as Config>::MaxStringLen>;

// AFTER: Corrected with the missing bound
pub type BoundedAbility<T> =
    CoreBoundedAbility<<T as Config>::MaxStringLen, <T as Config>::MaxConditions>;
```

And then a **separate** follow-up commit to fix the runtime:

```rust
// The AI forgot to add this to the runtime config
type MaxConditions = ConstU32<5>;
```

AI consistently generates code that compiles in isolation but **breaks at the runtime integration boundary**.

---

## Lesson 2: Weight Annotations

The AI's weight evolution over time:

```rust
// Commit 1: Wrong type entirely
#[pallet::weight(10_000)] // TODO: Calculate weights

// Commit 2: Correct type, zero value
#[pallet::weight(Weight::default())]

// Commit 3 (weeks later): Hand-estimated values
#[pallet::weight(Weight::from_parts(50_000_000, 0))]

// Never reached: Actual benchmarked weights
```

The stale benchmarks file from the template still referenced `do_something()` and `cause_error()` for multiple commits after those functions were deleted.

Notes:

AI consistently underestimates weights. The progression from wrong type -> zero weight -> hand-estimated -> benchmarked is a common pattern. AI will also leave stale benchmark code from templates untouched. Always check that your benchmarks reference actual extrinsics.

---

## Lesson 3: Keep the Core Engine Clean

From a session log:

> "in /core/ i need to set up my battle engine to support the Polkadot-SDK / Substrate. For that I need types which are bounded (i.e. using BoundedVec instead of Vec). **I don't want to make my engine bounded**, so we should create a new file `bounded.rs` which maintains bounded versions of the unbounded types."

The architecture decision: **core engine uses `Vec`, pallet uses `BoundedVec`, conversion layer bridges them.**

```rust
// core engine (clean, no FRAME dependency)
pub struct GameState {
    pub bag: Vec<CardId>,
    pub board: Vec<Option<BoardUnit>>,
}

// bounded.rs (FRAME wrapper with conversions)
pub type BoundedLocalGameState<T> = /* BoundedVec versions with From/Into impls */
```

Notes:

The developer had to explicitly tell the AI to keep FRAME types out of the core engine. AI's instinct was to use BoundedVec everywhere, which would have coupled the game logic to Substrate. The bounded wrapper pattern -- keeping a clean core with a conversion layer -- was a deliberate human design decision that AI did not suggest.

---

## Lesson 4: Hardcoded Business Logic

The AI duplicated game logic directly into the pallet:

```rust
// Hardcoded in the pallet extrinsic:
session.state.mana_limit = (2 + session.state.round).min(10);

if session.state.wins >= 10 {  // magic number from core engine
    // game over logic
}
```

Over several commits, this was progressively extracted:

```rust
// Final version: delegated to core engine
session.state.mana_limit = session.state.calculate_mana_limit();
```

Each round of extraction required re-prompting the AI.

Notes:

AI tends to inline business logic rather than calling shared functions. This creates synchronization hazards when the same logic exists in multiple places. When you see magic numbers or duplicated formulas in pallet code, push the AI to extract them into the domain library.

---

## Phase 2: Storage Design Evolution

### The Genesis Bag Story

```rust
// Commit 1: Hardcoded 10 rats
fn get_mock_genesis_bag() -> Vec<UnitCard> {
    let mut bag = Vec::new();
    for i in 0..10 {
        bag.push(UnitCard::new(i + 1, "rat", "Rat", 1, 1, 1, 1, false));
    }
    bag
}
```

```rust
// Commit 2: 100 cards from templates (still in the pallet)
let templates = get_starter_templates();
for i in 0..100 { /* ... */ }
```

```rust
// Commit 3: Delegated to core engine
state.bag = create_genesis_bag();
```

```rust
// Commit 4: Seed-dependent, fully parameterized
state.bag = create_genesis_bag(set_id, seed);
```

Notes:

The AI's instinct was to hardcode data directly in the pallet. Over four iterations, the genesis bag moved from hardcoded values to a properly parameterized core engine function. Each step required explicitly prompting the AI to extract the logic.

---

## Lesson 5: Blockchain as Source of Truth

From session logs, the developer had to repeatedly correct the AI:

> "the client wasm engine should be getting the information about the card pool **from the blockchain**. the whole point is to have blockchain be the single source of truth"

> "the whole point of our last task was to prevent stuff like this, where the blockchain cards were not the source of truth. **audit any more mistakes like this**"

> "before we continue, **why does oab-core need to read the cards json?**"

The AI kept falling back to hardcoded JSON files or local state instead of reading from on-chain storage.

Notes:

AI treats the blockchain like a database it can bypass. It will happily read card definitions from a JSON file "for convenience" rather than querying on-chain storage. This fundamentally undermines the point of putting data on-chain. You need to keep enforcing: if it's on-chain, read it from the chain.

---

## Lesson 6: Monolithic State to Split State

The single most important architectural change:

```rust
// BEFORE: Everything in one blob per player
pub struct GameSession<T: Config> {
    pub state: BoundedGameState<T>,  // Full card pool + player state
    pub current_seed: u64,
    pub owner: T::AccountId,
}
```

```rust
// AFTER: Shared reference data + per-player delta
pub struct GameSession<T: Config> {
    pub state: BoundedLocalGameState<T>,  // Only mutable player state
    pub set_id: u32,                      // Points to shared card definitions
    pub owner: T::AccountId,
}

#[pallet::storage]
pub type CardSets<T: Config> =
    StorageMap<_, Blake2_128Concat, u32, BoundedCardSet<T>, OptionQuery>;
```

Notes:

The AI initially stored the entire game state (including all card definitions) per player. This meant every card's full data was duplicated for every active game. The split into shared card sets + per-player mutable state was a critical storage optimization that the AI didn't propose -- it had to be prompted. This is a common pattern: AI doesn't think about storage costs.

---

## The Reconstruct/Decompose Pattern

After splitting state, every extrinsic needed this boilerplate:

```rust
// Read partial state from storage
let session = ActiveGame::<T>::get(&who).ok_or(Error::<T>::NoActiveGame)?;
let card_set = CardSets::<T>::get(session.set_id).ok_or(Error::<T>::CardSetNotFound)?;

// Reconstruct full state for game logic
let mut core_state = GameState::reconstruct(card_set.card_pool, session.state.into());

// ... run game logic on core_state ...

// Decompose back to partial state for storage
let (_, local_state) = core_state.decompose();
ActiveGame::<T>::insert(&who, GameSession { state: local_state.into(), /* ... */ });
```

The AI duplicated this pattern 3 times across extrinsics instead of extracting a helper.

Notes:

When pallets need to reconstruct rich domain objects from storage, extract a helper. AI tends to copy-paste the reconstruct/decompose boilerplate into every extrinsic rather than abstracting it.

---

## Lesson 7: Content-Addressed vs Sequential IDs

```rust
// BEFORE: Cards keyed by content hash (like Git)
pub type UserCards<T: Config> =
    StorageMap<_, Blake2_128Concat, T::Hash, UserCardEntry<T>, OptionQuery>;
```

```rust
// AFTER: Sequential IDs with a separate dedup index
pub type NextUserCardId<T: Config> = StorageValue<_, u32, ValueQuery>;
pub type UserCardHashes<T: Config> =
    StorageMap<_, Blake2_128Concat, T::Hash, u32, OptionQuery>;
pub type UserCards<T: Config> =
    StorageMap<_, Blake2_128Concat, u32, UserCardEntry<T>, OptionQuery>;
```

The AI initially chose content-addressable hashing, but the game needed stable integer IDs for card references. The dual-index pattern (hash map for dedup + ID map for lookup) was not in the AI's initial design.

Notes:

AI tends to reach for content-addressed storage (hash-keyed) because it's a common pattern in distributed systems. But on-chain, you usually want sequential IDs for predictable references. The dual-index pattern (keeping a hash dedup map alongside) is something you'll likely need to guide AI toward.

---

## Lesson 8: Trust Model Failures

The AI generated an "optimistic" battle reporting flow:

```rust
// AI-generated: Client reports who won, chain trusts it
pub fn report_battle_outcome(
    origin: OriginFor<T>,
    result: BattleResult,
) -> DispatchResult {
    let who = ensure_signed(origin)?;
    match result {
        BattleResult::Victory => { session.state.wins += 1; }
        BattleResult::Defeat => { session.state.lives -= 1; }
        BattleResult::Draw => {}
    }
    // No verification!
    Ok(())
}
```

This was replaced with fully on-chain battle resolution -- the chain runs the simulation itself.

Notes:

This is the most fundamental mistake AI made: it did not understand that a blockchain game cannot trust the client. The AI treated the pallet like a backend API server rather than a trustless execution environment. Always question AI-generated extrinsics: "Can a user lie to this function and gain an advantage?"

---

## Lesson 9: "Keep the APIs Simple"

From a 77-message debugging session (the longest in the project):

> "can we reduce the individual parameter complexity, and not have these random skip serializing stuff? wouldn't that be better?"

> **"i want to keep the apis stupid simple"**

The AI was adding `#[serde(skip_serializing)]` attributes, complex conditional fields, and multi-step serialization to work around type conversion issues. The developer pushed back: simplify the types, don't add escape hatches.

Notes:

AI solves type errors by adding complexity -- skip annotations, wrapper types, conditional serialization. The better solution is almost always to simplify the types themselves. When you see AI adding serialization workarounds, step back and ask: can the data structure be simpler?

---

## Lesson 10: Complexity Regret

After AI added a Status system (Shield, Poison, Guard) across ~35 files:

> "no. I **regret** adding things like statuses to the game. I think it might just be unnecessary complexity for the beta"

> "I want to remove the status system completely to keep the engine as **minimal and least complex as possible**. This is important for pallet benchmarks"

The removal required a dedicated session touching 35+ files. Features that took one prompt to add took much longer to remove.

Notes:

AI is eager to add features. It won't push back and say "this adds complexity to your benchmarks and storage." With FRAME, every new field in a stored struct affects: bounded type bounds, MaxEncodedLen calculations, benchmark worst cases, and migration requirements. Be conservative about what you let AI add to on-chain state.

---

## Phase 3: Multi-Pallet Architecture

### Splitting the Monolith

The single pallet was eventually split into 5 crates:

| Crate | Role |
|-------|------|
| `oab-common` | Shared traits, types, game logic functions |
| `pallet-oab-card-registry` | Card/set storage and metadata |
| `pallet-oab-arena` | Standard game mode |
| `pallet-oab-tournament` | Tournament mode with prize pools |
| `pallet-oab-constructed` | Deck-building mode |

Notes:

The AI initially built everything in one pallet. The split into multiple pallets with a shared common crate happened only after the code grew too large. This is a natural FRAME pattern -- start with one pallet, extract when you have clear boundaries. But AI won't propose the split on its own.

---

## Lesson 11: Loose Coupling via Traits

The extracted `CardRegistryProvider` trait:

```rust
pub trait CardRegistryProvider<AccountId> {
    fn get_card_set(set_id: u32) -> Option<CardSet>;
    fn card_set_exists(set_id: u32) -> bool;
    fn get_card_pool(card_set: &CardSet) -> BTreeMap<CardId, UnitCard>;
    fn get_set_creator(set_id: u32) -> Option<AccountId>;
    fn get_achievements(who: &AccountId, card_id: u32) -> u8;
    fn set_achievements(who: &AccountId, card_id: u32, bitmap: u8);
}
```

Game pallets access cards through the trait, not direct storage reads:

```rust
let card_set = T::CardRegistry::get_card_set(set_id)
    .ok_or(Error::<T>::CardSetNotFound)?;
```

Notes:

This is the FRAME "pallet coupling" pattern -- pallets interact through traits, not by reading each other's storage directly. The AI did not produce this pattern initially. It was extracting functions and only introduced the trait boundary when forced by the multi-pallet split. Guide AI toward trait-based coupling early.

---

## Lesson 12: Supertrait for Shared Configuration

Instead of duplicating config across 4 game pallets:

```rust
// Shared game engine config, implemented once on Runtime
pub trait GameEngine: frame_system::Config {
    type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
    type CardRegistry: CardRegistryProvider<Self::AccountId>;
    type MaxBagSize: Get<u32>;
    type MaxBoardSize: Get<u32>;
    type MaxHandActions: Get<u32>;
    type MaxGhostsPerBracket: Get<u32>;
}

// Each pallet inherits it
#[pallet::config]
pub trait Config: frame_system::Config + oab_common::GameEngine {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type WeightInfo: WeightInfo;
    type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
}
```

Notes:

Without the supertrait, every pallet would need its own `MaxBagSize`, `MaxBoardSize`, etc. The AI initially duplicated these across pallets and then needed to be guided toward the supertrait pattern. The GameEngine trait was implemented once on Runtime, and all game pallets inherited the shared constants.

---

## Lesson 13: Copy-Paste Over Abstraction

The AI generated the tournament system by copying the entire arena game flow:

```rust
// In tournament pallet -- nearly identical to arena pallet
pub fn submit_tournament_turn(origin: OriginFor<T>, /* ... */) -> DispatchResult {
    // ~200 lines copy-pasted from arena's submit_turn
    // with minor tournament-specific changes
}
```

This was immediately refactored into shared helper functions:

```rust
// Extracted to oab-common
pub fn prepare_battle<T: GameEngine>(/* ... */) -> Result<PreparedBattle, Error> { ... }
pub fn execute_and_advance<T: GameEngine>(/* ... */) -> TurnResult { ... }
```

Notes:

When AI needs to create a variant of existing logic, it copy-pastes rather than abstracting. The tournament pallet was a near-duplicate of the arena pallet. Prompting "extract shared logic first, then build the variant" produces much better results. But the AI needed to be explicitly told to refactor.

---

## Phase 4: Smart Contract Port

### What Translated Well

The core battle engine was reused directly:

```rust
// Same imports in both pallet and contract
use oab_battle::{resolve_battle, verify_and_apply_turn};
```

Game state structure mapped almost 1:1:

```rust
// Pallet
pub state: BoundedLocalGameState<T>,

// Contract
pub state: LocalGameState,  // Same shape, no BoundedVec
```

The business logic didn't change -- only the storage and dispatch layers.

---

## What Didn't Translate

| Pallet | Contract |
|--------|----------|
| `BoundedVec<T, MaxLen>` | `Vec<T>` |
| `#[pallet::storage]` macros | Manual keccak256 key hashing |
| `T::Randomness::random()` | `keccak256(addr ++ context ++ nonce)` |
| `Event<T>` deposits | ABI-encoded return values |
| `EnsureOrigin` | `if caller != admin { return; }` |
| Type-safe dispatch | Hand-written ABI selector matching |

Notes:

FRAME provides a huge amount of infrastructure that you take for granted until you port to a contract. Bounded types, storage macros, randomness traits, event system, origin system -- all of these need manual equivalents in a contract. The contract port really shows the value FRAME provides.

---

## Lesson 14: Security Flaw in Contract Port

The AI's initial contract accepted the card pool in the calldata:

```rust
// AI-generated: caller sends the card definitions
pub fn start_game(card_set: CardSet, all_cards: Vec<UnitCard>) {
    // Uses caller-provided card data directly!
    let mut session = ArenaSession::new(card_set, all_cards);
    // Player could pass cards with infinite health...
}
```

Fixed by storing cards on-chain and reading from contract storage:

```rust
// Fixed: cards are admin-registered and immutable
pub fn start_game(set_id: u16) {
    let card_set = Storage::card_sets().get(&set_id).expect("set exists");
    let card_pool = build_card_pool_from_storage(&card_set);
    let mut session = ArenaSession::new(card_pool);
}
```

Notes:

The same trust model mistake from Lesson 8, but in the contract world. AI treats function parameters as trusted input. In a pallet, this was "client reports battle results." In a contract, this was "client sends card definitions." Both are the same class of error: trusting user-supplied data for game-critical state.

---

## Lesson 15: Ghost Ordering Bug

The AI ported the ghost matchmaking but got the ordering wrong:

```rust
// AI-generated contract (BUGGY):
// Store player's ghost BEFORE selecting opponent
let player_ghost = create_ghost_from_board(&board);
push_ghost(bracket, player_ghost);
// Select opponent -- can match against yourself!
let enemy = select_ghost(bracket);
```

```rust
// Fixed (1-commit fix):
// Select opponent FIRST, then store
let enemy = select_ghost(bracket);
let player_ghost = create_ghost_from_board(&board);
push_ghost(bracket, player_ghost);
```

The pallet already had the correct order. The AI ported the logic but scrambled the sequencing.

Notes:

When AI ports code from one context to another, it can get the ordering of operations wrong. This is especially dangerous for read-then-write patterns where the order matters. The pallet had this right, but the contract port introduced a subtle bug that allowed players to battle their own ghost board.

---

## Lesson 16: Type Optimization

For the contract, every byte matters. The AI initially used the same types as the pallet:

```rust
// BEFORE: Desktop-sized types
pub struct CardId(pub u32);
pub type StatValue = i32;
pub type ManaValue = i32;
pub type RoundValue = i32;
```

After optimization (~53% storage reduction):

```rust
// AFTER: Contract-optimized types
pub struct CardId(pub u16);    // max 65k cards is plenty
pub type StatValue = i16;      // stats fit in [-32768, 32767]
pub type ManaValue = u8;       // max 255 mana
pub type RoundValue = u8;      // max 255 rounds
```

This was a 31-file, cross-crate change. Defining types as aliases in one place made this a manageable refactor.

Notes:

AI uses the "comfortable" types (i32, u32) by default because they're what it sees most in training data. For contracts where storage is expensive, you need to explicitly prompt for type optimization. The key insight: define semantic type aliases early so you can resize later.

---

## Lesson 17: AI Over-Engineers Solutions

Actual corrections from session logs:

> "rather than remove it entirely, just **hide it behind a feature flag**, keep the rest the same"
>
> "we don't need a real feature flag, **a hardcoded bool is fine**"

> "rather than 'detecting it' and adding it dynamically, **can it always exist in the background?**"

The AI's instinct was to build runtime detection, dynamic configuration, and conditional logic. The developer wanted a hardcoded boolean.

Notes:

AI over-engineers because it optimizes for flexibility and "good engineering." But in iterative development, a hardcoded `false` you can flip to `true` later is better than a feature flag system with configuration parsing. Tell AI explicitly: "simple, not flexible."

---

## Lesson 18: Verify AI Claims

From a session where AI described a user flow:

> "how did you know the specific flow? **did you make up any of this data?**"

> "I want to know how we can build out this flow with **100% accuracy**"

> "how about starting over, and **only using data you know 100% exists**"

Notes:

AI will confidently describe APIs, storage layouts, and runtime behaviors that don't exist. In the Polkadot SDK ecosystem, this is especially dangerous because the API surface changes frequently. Always verify AI claims against the actual codebase. If it describes a pallet config item or storage map, grep for it before building on that assumption.

---

## Summary: AI Mistake Patterns

| Pattern | Example | How to Catch |
|---------|---------|-------------|
| **Escape hatches** | `without_storage_info` | Search for it in generated code |
| **Zero weights** | `Weight::default()` | Grep for `default()` in weight annotations |
| **Trust model** | Client reports results | Ask "can a user lie to this?" |
| **Bypasses chain** | Reads JSON instead of storage | Ask "why isn't this from on-chain?" |
| **Copy-paste** | Tournament = Arena copy | Look for duplicate code blocks |
| **Stale templates** | `do_something` benchmarks | Check benchmarks match extrinsics |
| **Missing config** | `MaxConditions` not in runtime | Always compile after pallet changes |
| **Hardcoded logic** | Magic numbers in extrinsics | Look for numeric literals |
| **Wrong sequencing** | Ghost self-matching | Review read-then-write ordering |
| **Over-engineers** | Feature flag for a boolean | Ask "is there a simpler way?" |
| **Fabricates APIs** | Claims a trait method exists | Grep the actual codebase |
| **Eager complexity** | Adds features without pushback | Ask "does this affect benchmarks?" |

---

## Summary: How to Guide AI

1. **Start with the template, but rename semantically** -- don't just find-and-replace
2. **Remove `without_storage_info` early** -- it hides real problems
3. **Keep FRAME types out of the core engine** -- use a conversion layer
4. **Question every extrinsic parameter** -- can the user lie?
5. **Enforce blockchain as source of truth** -- AI will bypass the chain
6. **Extract business logic to a shared crate** -- don't let AI inline it
7. **Say "simple, not flexible"** -- AI over-engineers by default
8. **Be conservative about on-chain state** -- features are easy to add, hard to remove
9. **Prompt for trait-based coupling** before the codebase grows
10. **Define type aliases early** -- they make refactors possible
11. **Always compile after generation** -- AI code compiles in isolation, breaks at boundaries
12. **Verify every API claim** -- AI fabricates confidently

---

## The Good News

Despite all these issues, AI got this project from zero to a deployed on-chain game across ~100 commits.

The key insight: **AI is a fast first-draft generator, not a FRAME expert.**

Your job is to know the patterns well enough to course-correct.

That's what the rest of this FRAME module is for.

---

<!-- .slide: data-background-color="#000000" -->

# Questions
