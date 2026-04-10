---
title: FRAME with AI
description: Lessons learned from building a full blockchain game with AI, covering common mistakes, design evolution, and best practices for AI-assisted FRAME development.
duration: 3 hours
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

## Part 2: Frontend Integration with PAPI

### Building a TypeScript Frontend for FRAME Pallets

---

## The Stack

The frontend is a React/TypeScript app that talks to the chain via PAPI (Polkadot API):

```
┌─────────────────────────────────┐
│  React UI (TypeScript)          │
├─────────────────────────────────┤
│  Game Store (Zustand)           │
├──────────────┬──────────────────┤
│  PAPI        │  WASM Engine     │
│  (chain I/O) │  (game logic)    │
├──────────────┴──────────────────┤
│  Polkadot Node (WebSocket)      │
└─────────────────────────────────┘
```

Two serialization formats in play:
- **SCALE**: chain storage and pallet communication
- **Serde/JSON**: browser WASM engine communication

Notes:

PAPI (polkadot-api) is the modern TypeScript client for Polkadot chains. It generates typed APIs from chain metadata, giving you TypeScript types that match your pallet's storage, calls, and events. The challenge is bridging PAPI's TypeScript types with a Rust WASM engine that speaks Serde JSON.

---

## Lesson 19: WASM Initialization in React

AI's first attempt at WASM init:

```typescript
// App.tsx
useEffect(() => {
    await wasm.default('/manalimit_core_bg.wasm');
    // ... use engine
}, []);
```

Three problems emerged:
1. React StrictMode calls `useEffect` twice -- **double init crashes WASM**
2. Multiple components racing to init -- **concurrent init crashes**
3. The hardcoded `.wasm` path didn't work with Vite bundling

Notes:

WASM modules can only be initialized once. React's StrictMode (which runs effects twice in dev) and concurrent component mounting mean you need a singleton guard. AI doesn't account for React's initialization quirks when setting up WASM modules.

---

## The Triple-Guard Pattern

The fix required three layers of defense:

```typescript
// Module-level singleton (survives React re-renders)
let wasmInitialized = false;
let initPromise: Promise<void> | null = null;

async function initWasm() {
    if (wasmInitialized) return;             // Guard 1: already done
    if (initPromise) return initPromise;      // Guard 2: already in progress
    initPromise = wasm.default().then(() => { // Guard 3: do it once
        wasmInitialized = true;
    });
    return initPromise;
}
```

```typescript
// Component level
const initCalled = useRef(false);
useEffect(() => {
    if (initCalled.current) return;  // Guard 4: StrictMode protection
    initCalled.current = true;
    initWasm();
}, []);
```

Notes:

This pattern -- module-level promise deduplication plus React ref guard -- became the standard for all WASM-in-React usage. AI initially only added one guard (the boolean), which wasn't enough. The promise dedup is the key insight: if two callers race, they share the same promise instead of both calling `wasm.default()`.

---

## Lesson 20: The WASM Reentrance Bug

A crash that took **4 sessions** (77+ messages) to resolve:

```
Error: recursive use of an object detected which would lead
to unsafe aliasing in rust
```

The cause: the game engine used `RefCell` for interior mutability. When `init_from_scale()` was called, it triggered a React state update, which caused a re-render, which called `get_state()` on the engine -- while the `RefCell` was still borrowed.

```
init_from_scale() → RefCell.borrow_mut() → state update
    → React re-render → get_state() → RefCell.borrow() → 💥
```

The fix: **never call engine methods from within React render paths triggered by engine state updates.**

Notes:

This is the most time-consuming bug in the entire project. Rust's RefCell enforces single-borrow rules at runtime, but in WASM, "runtime" means the browser's single thread. If a WASM call triggers a JavaScript callback that re-enters the WASM module, you get this error. AI doesn't model these cross-language reentrance paths.

---

## Lesson 21: The JSON Bridge Disaster

AI's first approach to passing chain data to the WASM engine:

```typescript
// chainConvert.ts -- grew to 200+ lines
function chainStateToWasm(chainState: PapiGameState): WasmGameState {
    return {
        bag: chainState.bag.map(id => Number(id)),
        board: chainState.board.map(slot =>
            slot ? convertBoardUnit(slot) : null
        ),
        hand: chainState.hand.map(card => ({
            ...convertCard(card),
            name: new TextDecoder().decode(card.name),  // BoundedVec<u8> → string
            abilities: card.abilities.map(a => ({
                trigger: papiEnumToSerde(a.trigger),     // {type:"OnPlay"} → "OnPlay"
                effect: flattenTaggedEnum(a.effect),     // {type:"Damage",value:{amount:5}}
                                                         //   → {type:"Damage",amount:5}
                target: convertAdjacentEnum(a.target),   // different serde tag style
                // ... 20 more fields
            })),
        })),
        // ... more nested conversions
    };
}
```

Every new game feature required updating this conversion layer.

Notes:

The fundamental problem: PAPI decodes SCALE into JavaScript objects using its own enum representation (`{type: "Variant", value: {...}}`), but the Rust WASM engine's serde expects different formats depending on the Rust enum's serde attributes. Each Rust enum can use different serde tagging: externally tagged, internally tagged, adjacently tagged, or untagged. AI kept adding conversion functions without realizing the approach was unsustainable.

---

## The Three Enum Mismatches

PAPI always returns enums as `{type: "Variant", value: {...}}`.

But Rust serde has three different enum formats:

```typescript
// 1. Simple C-style enum: serde expects a bare string
// PAPI: {type: "Front"}  →  Serde: "Front"

// 2. Internally tagged (#[serde(tag = "type")]):
// PAPI: {type: "Damage", value: {amount: 5}}
// Serde: {type: "Damage", amount: 5}  ← value fields merged up

// 3. Adjacently tagged (#[serde(tag = "type", content = "data")]):
// PAPI: {type: "All", value: {scope: "Board"}}
// Serde: {type: "All", data: {scope: "Board"}}  ← value renamed to "data"
```

AI wrote separate conversion functions for each style but never recognized the pattern.

Notes:

This is the #1 type conversion pitfall when bridging PAPI and Rust WASM. The fix was to stop converting through JSON entirely and use raw SCALE bytes instead. But if you must display chain data in the UI, you'll need to handle these enum mismatches. Know your Rust serde attributes.

---

## Lesson 22: The SCALE Bytes Breakthrough

From a session log:

> "i am **unhappy** with the way we currently handle the blockchainStore and the chainConvert files in the front end. there are lots of issues regarding getting types working from the SCALE encoded data"

The solution: **skip JavaScript entirely** for the read path.

```typescript
// BEFORE: Chain → PAPI decode → JS objects → JSON convert → WASM serde
const game = await api.query.OabArena.ActiveGame.getValue(address);
const wasmState = chainStateToWasm(game);  // 200 lines of conversion
engine.init_from_json(JSON.stringify(wasmState));

// AFTER: Chain → raw SCALE bytes → WASM SCALE decode
const gameKey = api.query.OabArena.ActiveGame.getKey(address);
const rawBytes = await client.rawQuery(gameKey);
engine.init_from_scale(rawBytes, cardSetBytes);  // Rust handles SCALE natively
```

`chainConvert.ts` was **deleted entirely**.

Notes:

Both the blockchain and the Rust WASM module speak SCALE natively. Decoding to JavaScript objects just to re-encode for Rust was a round-trip through an impedance mismatch. The breakthrough was realizing you can query raw SCALE bytes from the chain and pass them directly to WASM. This eliminated 200+ lines of buggy conversion code.

---

## Lesson 23: SCALE Bytes for Writes Too

The write path got the same treatment:

```typescript
// BEFORE: Engine → JSON → manual conversion → PAPI typed API
const action = JSON.parse(engine.get_commit_action());
const chainAction = wasmActionToChain(action);  // another conversion layer
const tx = api.tx.OabArena.submit_turn(chainAction);

// AFTER: Engine → SCALE bytes → PAPI codec decode → typed API
const actionScale: Uint8Array = engine.get_commit_action_scale();
const codecs = await getTypedCodecs(auto_battle);
const action = codecs.tx.OabArena.submit_turn.dec(actionScale);
const tx = api.tx.OabArena.submit_turn(action);
```

PAPI's `getTypedCodecs()` can decode raw SCALE bytes into the exact TypeScript types the typed API expects.

Notes:

`getTypedCodecs(descriptors)` returns a codec tree mirroring the chain's type registry. `codecs.tx.PalletName.call_name.dec(scaleBytes)` decodes SCALE bytes into PAPI-typed objects. This means the WASM engine produces SCALE, PAPI decodes it, and the typed API sends it -- no manual JavaScript conversion anywhere.

---

## The Final Architecture

```
READ PATH (chain → UI):
  Chain storage → raw SCALE bytes → WASM engine (native SCALE decode)
                                  → game state for rendering

WRITE PATH (UI → chain):
  WASM engine → SCALE bytes → PAPI codec decode → typed API → chain

DISPLAY PATH (chain → UI, for card data shown in the UI):
  Chain storage → PAPI typed query → enum conversion → UI components
```

The display path still needs enum conversion for rendering card data. But the critical game state path (read + write) is **pure SCALE end-to-end**.

Notes:

This three-path architecture was the final design after 5 iterations. The read and write paths use raw SCALE bytes, completely bypassing JavaScript type conversion. Only the display path (showing card names, abilities, etc. in the UI) needs PAPI-to-serde enum conversion. This minimizes the surface area for type bugs.

---

## Lesson 24: u64 / BigInt Pain

Rust `u64` values cross the JS boundary in two forms:

```typescript
// If value <= Number.MAX_SAFE_INTEGER (2^53 - 1): returns number
// If value > Number.MAX_SAFE_INTEGER: returns BigInt

// AI wrote:
game_seed: number;  // 💥 crashes when seed > 2^53

// Fixed:
game_seed: number | bigint;  // handles both cases
```

Also: `BigInt` can't be serialized to JSON:

```typescript
JSON.stringify({ seed: 42n });  // TypeError: BigInt not serializable

// Need explicit conversion
JSON.stringify({ seed: Number(42n) });  // only safe if value fits
```

Notes:

Every Rust `u64` in your TypeScript interfaces should be `number | bigint`. AI consistently types these as just `number` because that's what it sees in most TypeScript code. The BigInt serialization issue also trips up AI when it tries to pass chain data through `JSON.stringify`.

---

## Lesson 25: Descriptors Must Be Regenerated

When you change a pallet (add storage, modify a call, change types), the frontend won't see the changes until you regenerate PAPI descriptors:

```bash
# After modifying the pallet:
cd blockchain && cargo build --release  # rebuild runtime WASM
cd ../web && npx papi update            # regenerate descriptors
```

This updates `web/.papi/metadata/auto_battle.scale` and `web/.papi/descriptors/`.

AI frequently changed pallet code without regenerating descriptors, causing **cryptic encoder errors**:

```
inner[tag] is not a function
    at enumEnc (Enum.ts:56)
    at VariantEnc (Variant.ts:51)
```

Notes:

This error message gives no clue that the real problem is stale descriptors. PAPI's encoder expects type structures that match the chain metadata. If the metadata is outdated, the encoder crashes with internal errors. Always regenerate descriptors after pallet changes. AI never does this automatically.

---

## Lesson 26: Don't Break Offline Mode

After the blockchain frontend was working:

> "we recently got the end to end blockchain front-end working, but as a result, **the main page /#/ which is supposed to work without blockchain doesn't load anything**"

The AI had made the WASM engine's initialization depend on blockchain connection. The card pool was only loaded from the chain, breaking the offline/sandbox mode.

The fix required a dual initialization path:

```typescript
// Blockchain mode: cards from chain
const cards = await api.query.OabCardRegistry.UserCards.getEntries();
engine.load_cards(cards);

// Offline mode: cards from embedded JSON
import cardsJson from '../data/cards.json';
engine.load_cards(cardsJson);
```

Notes:

When adding blockchain integration to an existing app, AI tends to replace the existing initialization path rather than adding a second one alongside it. Always verify that non-blockchain functionality still works after adding chain integration. This is a PAPI-specific trap: once you have typed APIs, it's tempting to use them for everything.

---

## Lesson 27: PAPI Event Extraction

Reading events from a finalized transaction:

```typescript
const txResult = await submitAndWatch(tx, signer, { waitFor: 'finalized' });

// PAPI events: e.type is pallet name, e.value.type is event variant
const battleEvent = txResult.events.find(
    (e: any) => e.type === 'OabArena' && e.value?.type === 'BattleReported'
);

// Access event fields through e.value.value
const { battle_seed, opponent_board, result } = battleEvent.value.value;
```

AI initially used `.events[0]` (positional access) which broke when System events appeared before the pallet event.

Notes:

PAPI events have a double-nested structure: `event.type` is the pallet name, `event.value.type` is the event variant, and `event.value.value` holds the actual fields. Always filter by pallet name + event variant, never by position. AI gets this wrong because it assumes events come in a predictable order.

---

## Lesson 28: Transaction Submission Evolution

AI's first approach to submitting transactions:

```typescript
// v1: 120+ lines of manual observable subscription
tx.signSubmitAndWatch(signer).subscribe({
    next(event: any) {
        switch (event.type) {
            case 'signed': console.log('Signed');
            case 'broadcasted': console.log('Broadcasted');
            case 'txBestBlocksState':
                if (event.found) { /* check finalized */ }
                break;
            case 'finalized': resolve(event);
        }
    },
    error(err) { reject(err); },
});
```

This was eventually replaced with a promise-based helper:

```typescript
// Final: clean promise-based API
const result = await submitAndWatch(tx, signer, {
    waitFor: 'finalized',
    timeoutMs: 30_000,
});
```

Notes:

PAPI's raw `signSubmitAndWatch` returns an Observable with multiple event types. AI generates the verbose switch-case pattern every time, often with incorrect state tracking. Wrapping this in a promise-based helper should be one of the first things you do. The Observable is powerful for advanced use cases but overkill for simple submit-and-wait.

---

## Lesson 29: The GameBackend Abstraction

When the project added a PolkaVM contract alongside the pallet, a backend interface emerged:

```typescript
interface GameBackend {
    connect(): Promise<void>;
    startGame(setId: number): Promise<{ seed: bigint }>;
    submitTurn(actionScale: Uint8Array): Promise<TurnResult>;
    getGameState(): Promise<GameStateRaw | null>;
    getCards(): Promise<CardData[]>;
    // ...
}

interface GameStateRaw {
    stateBytes: Uint8Array;   // SCALE-encoded game state
    cardSetBytes: Uint8Array; // SCALE-encoded card set
}
```

Both backends return **raw SCALE bytes** -- the WASM engine doesn't care whether it came from a pallet or contract.

Notes:

The key design insight: using SCALE bytes as the interchange format means the GameBackend interface is thin. The pallet backend uses PAPI to fetch raw bytes; the contract backend uses viem to fetch raw bytes; the WASM engine consumes the same bytes either way. AI didn't propose this abstraction -- it only emerged when the contract port forced the question "how do we support both backends?"

---

## Lesson 30: Contract Backend Differences

The pallet backend and contract backend have fundamentally different patterns:

```typescript
// Pallet: PAPI typed codecs decode SCALE for the typed API
const action = codecs.tx.OabArena.submit_turn.dec(actionScale);
const tx = api.tx.OabArena.submit_turn(action);
await submitAndWatch(tx, signer);

// Contract: SCALE bytes wrapped in ABI encoding for EVM calldata
const data = '0x20fa4907' + encodeAbiBytes(actionScale);  // selector + ABI
const returnData = await publicClient.call({ to: contract, data });  // simulate
await walletClient.sendTransaction({ to: contract, data });           // execute
```

The contract requires **simulate-then-send**: call `eth_call` first to get the return value, then `eth_sendTransaction` to actually execute.

Notes:

PolkaVM contracts use Ethereum-compatible ABI encoding, so the frontend wraps SCALE bytes inside ABI `bytes` parameters. This "SCALE-in-ABI" pattern is unique to PolkaVM. The simulate-then-send pattern exists because Ethereum transactions don't return values -- you need a view call first. AI wrote both calls but forgot the `from` address on the view call, causing a session of debugging.

---

## Lesson 31: The Missing `from` Address

A one-line bug that caused an entire debugging session:

```typescript
// BUGGY: view call without `from` address
const result = await publicClient.call({
    to: contractAddress,
    data,
    // `account` missing! Contract sees caller as 0x0000...
});

// FIXED: include the caller
const result = await publicClient.call({
    to: contractAddress,
    data,
    account: selectedAccount.address,  // Contract needs msg.sender
});
```

The contract uses `api::caller()` to look up per-player game sessions. Without `from`, the caller is the zero address.

Notes:

View calls (eth_call) in Ethereum-compatible chains need the `from` field when the contract uses msg.sender for storage lookups. AI consistently forgets this because view calls "don't need a signer." They don't need a signer, but they still need a sender address for contracts that behave differently per caller.

---

## Frontend Summary: Type Boundary Patterns

| Boundary | Format | Common AI Bug |
|----------|--------|--------------|
| Chain → WASM engine | Raw SCALE bytes | Tries JSON bridge instead |
| WASM → Chain (submit) | SCALE → PAPI codec | Builds manual conversion |
| Chain → UI (display) | PAPI typed query | Ignores enum mismatches |
| WASM → UI (display) | Serde JSON | Forgets `BigInt` handling |
| UI → Chain (contract) | SCALE in ABI bytes | Forgets `from` address |

---

## Frontend Summary: What AI Gets Wrong

1. **WASM init**: Doesn't handle React StrictMode or concurrent mounting
2. **Type conversion**: Builds a JSON bridge instead of using raw SCALE bytes
3. **Enums**: PAPI and serde use different enum representations -- AI doesn't know this
4. **u64**: Types as `number` instead of `number | bigint`
5. **Descriptors**: Changes pallet code without regenerating descriptors
6. **Events**: Uses positional access instead of filtering by pallet + variant
7. **Observables**: Generates 120-line switch-case instead of a promise wrapper
8. **Offline mode**: Replaces local init path instead of adding blockchain alongside it
9. **View calls**: Forgets the `from` address for per-user contract queries

---

---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# Polkadot Stack Template

### A Teaching Template Built Entirely by AI

---

## Context: Polkadot Stack Template

A full-stack Polkadot developer template, 100% AI-generated:

- Proof of Existence pallet with tests, benchmarks, and weights
- The same logic as a Solidity contract (EVM + PVM dual deployment)
- React/TypeScript frontend using PAPI
- Rust CLI using subxt
- Docker, Zombienet, CI/CD pipelines

The developer's guidance:

> "This repo should be simple, clear, well documented, no bloat, not over-complicated, easy to pick up and modify, easy for AI agents to navigate and understand"

---

## Pallet Development

---

## Lesson 32: The Umbrella Crate Dance

The AI went through 4 dependency approaches in a single night:

1. Start with `polkadot-sdk` umbrella crate (stable2503) -- works
2. Switch to ~60 individual crate deps to add `pallet-revive` -- **WASM build breaks**
3. Revert to umbrella crate, remove `pallet-revive` -- works again
4. Re-add `pallet-revive` as one line in the umbrella features -- works

```toml
# The fix was ONE line in the umbrella features:
polkadot-sdk = { workspace = true, features = [
    "pallet-revive",  # <-- this is all it took
] }
```

From a session log, after the AI tried approach #2:

> "stop for a moment, and think about the root cause of the problem. We have a runtimes repo which compiles and definitely works. **Figure out the root differences and solve it** rather than keeping trying new things"

Notes:

The umbrella crate (`polkadot-sdk`) handles no_std feature propagation correctly. Individual crates break WASM builds because Cargo feature unification means any dependency enabling `std` infects the whole build. When AI gets stuck on dependency issues, point it at a known-working reference repo.

---

## Lesson 33: Inline Tests to Canonical Structure

AI's first attempt -- everything in `lib.rs`:

```rust
// lib.rs -- 200+ lines: pallet logic + test module + mock runtime
#[frame::pallet]
pub mod pallet { /* ... */ }

#[cfg(test)]
mod mock { /* ... */ }

#[cfg(test)]
mod tests { /* ... */ }
```

The developer's requirement:

> "this repo will be used as a template for builders and students. the pallet needs **benchmarks and tests and mock runtime**"

Refactored to canonical polkadot-sdk structure:

```
pallets/template/src/
├── lib.rs           # pallet logic only
├── mock.rs          # test runtime
├── tests.rs         # unit tests
├── benchmarking.rs  # frame_benchmarking v2
└── weights.rs       # WeightInfo trait
```

Notes:

AI generates everything in one file because it's the path of least resistance. The polkadot-sdk convention of separate files for mock/tests/benchmarks/weights is not something AI will produce without explicit instruction. For a teaching template, the canonical structure matters -- students will copy this layout.

---

## Lesson 34: Fabricated Benchmark Headers

The AI generated `weights.rs` with this header:

```rust
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2026-03-30
//! STEPS: `50`, REPEAT: `20`
```

But the weights were hand-estimated placeholders. A later fix added:

```rust
//! **NOTE**: These weights are placeholder estimates. Run benchmarks
//! on your own hardware for accurate production values.
```

Notes:

AI copies the benchmark header format from training data but generates fake values. Students copying this template might assume the weights are real. Always mark placeholder weights clearly. This also applies to any "auto-generated" comment in AI output -- verify the claim.

---

## Lesson 35: Zero Proof Size Weights

The initial weight annotation:

```rust
#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
```

The `0` in `Weight::from_parts(10_000, 0)` means **zero proof size**. For parachains, this is wrong -- every storage read/write contributes to the Proof of Validation (PoV), which is bounded.

Fixed with the WeightInfo pattern:

```rust
fn create_claim() -> Weight {
    Weight::from_parts(14_000_000, 3501)  // 3501 bytes of proof size
        .saturating_add(T::DbWeight::get().reads(1_u64))
        .saturating_add(T::DbWeight::get().writes(1_u64))
}
```

Notes:

AI consistently sets proof_size to zero because most training examples are from solo chains where PoV doesn't matter. For parachain pallets, proof_size is critical -- the relay chain enforces PoV bounds. Always check the second parameter of `Weight::from_parts`.

---

## Lesson 36: Counter to Proof of Existence

The design pivot from session logs:

> "okay, lets switch the design of the pallets and contracts from a counter to a 'proof of existence' system. the logic should have two functions: **create claim and revoke claim**"

Storage changed fundamentally:

```rust
// Counter (simple, no ownership):
pub type Counters<T: Config> =
    StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

// Proof of Existence (ownership + timestamp):
pub type Claims<T: Config> =
    StorageMap<_, Blake2_128Concat, H256, Claim<T>, OptionQuery>;
```

Key differences:
- `ValueQuery` (counter defaults to 0) vs `OptionQuery` (claim either exists or doesn't)
- `AccountId` key (per-user) vs `H256` key (per-file-hash)
- `u32` value vs struct with owner + block_number

Notes:

The storage type choice (`ValueQuery` vs `OptionQuery`) has real implications for your extrinsic logic. With `ValueQuery`, you always get a value back (the default). With `OptionQuery`, you get `None` when nothing is stored, which lets you `ensure!` that a claim doesn't already exist.

---

## Lesson 37: Tuples to Named Structs

From a session log:

> "in the pallet, lets use a **named struct rather than tuple** for: `(T::AccountId, BlockNumberFor<T>)`. Make sure that any breaking changes in the front-end are fixed"

```rust
// BEFORE: Anonymous tuple
StorageMap<_, Blake2_128Concat, H256, (T::AccountId, BlockNumberFor<T>), OptionQuery>;

// AFTER: Named struct
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Claim<T: Config> {
    pub owner: T::AccountId,
    pub block_number: BlockNumberFor<T>,
}

StorageMap<_, Blake2_128Concat, H256, Claim<T>, OptionQuery>;
```

SCALE encoding is identical (struct fields encode in order, same as a tuple). The derive list and `skip_type_params` are required boilerplate.

Notes:

AI defaults to tuples for compound storage values because they're shorter. Named structs are always better for teaching and maintenance -- the derive boilerplate is a one-time cost. The `skip_type_params(T)` attribute is necessary when your struct is generic over `T: Config` but the metadata system shouldn't track it.

---

## Lesson 38: Stale Comments Survive Refactors

When the AI removed `RuntimeEvent` from the Config trait (modern FRAME derives it automatically), the doc comment was left behind:

```rust
pub trait Config: frame_system::Config {
    /// The overarching runtime event type.  // <-- stale, nothing here anymore
    type WeightInfo: WeightInfo;
}
```

Similarly, when the pallet was renamed from `Counter` to `ProofOfExistence`, internal doc comments still referenced "counter."

Notes:

AI doesn't clean up documentation during code changes. It modifies the code but leaves surrounding comments referencing the old design. For a teaching template, stale comments are especially harmful -- students trust them. Always review comments after refactors.

---

## Lesson 39: SDK Upgrade API Changes

The upgrade from stable2503 to stable2512 required several API changes the AI had to handle:

```rust
// pallet_session gained new associated types:
impl pallet_session::Config for Runtime {
    type Currency = Balances;      // NEW in 2512
    type KeyDeposit = ();          // NEW in 2512
}

// parachain_system renamed a type:
type SelectCore = ...;             // REMOVED in 2512
type RelayParentOffset = ConstU32<0>;  // ADDED in 2512
```

And `pallet-revive` required replacing the standard `UncheckedExtrinsic`:

```rust
// Standard:
type UncheckedExtrinsic = generic::UncheckedExtrinsic<...>;

// With pallet-revive (supports both Substrate and Ethereum tx formats):
type UncheckedExtrinsic = pallet_revive::evm::runtime::UncheckedExtrinsic<...>;
```

Notes:

SDK version upgrades are where AI struggles most. The API changes are not in training data, and error messages can be cryptic. The session logs show the developer pointing AI at known-working repos (`../runtimes`, `../polkadot-sdk`) rather than letting it guess. When upgrading, always give AI a reference implementation.

---

## Lesson 40: "Best Practices" Audit Prompt

From a session log:

> "is there any other bad practices or manual handling of things which the polkadot sdk can handle through the whole repo? **remember this is a template for students so our code should be best practices**"

This prompt caught:
- `RuntimeEvent` still in Config (now auto-derived)
- Hardcoded weight estimates without clear placeholder labels
- Manual SCALE decoding in the CLI that could use subxt metadata
- Stale doc comments from the Counter era
- Missing `pallet-xcm` weight warning

Notes:

After initial generation, explicitly ask AI to audit for anti-patterns. The "this is a template for students" framing is powerful -- it raises AI's quality bar because it knows the code will be copied.

---

## Frontend Integration with PAPI

---

## Lesson 41: PAPI Connection Setup

The base pattern for connecting to a Polkadot node:

```typescript
import { createClient } from "polkadot-api";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import { withPolkadotSdkCompat } from "polkadot-api/polkadot-sdk-compat";
import { stack_template } from "@polkadot-api/descriptors";

const client = createClient(
    withPolkadotSdkCompat(getWsProvider("ws://127.0.0.1:9944"))
);
const api = client.getTypedApi(stack_template);
```

AI initially used `getUnsafeApi()` to skip metadata validation, then had to switch back to `getTypedApi()` once descriptors were properly generated.

Notes:

`withPolkadotSdkCompat` is required for Substrate nodes. `getTypedApi` validates that the descriptors match the connected chain's metadata. `getUnsafeApi` skips validation but can cause runtime encoder crashes. Always regenerate descriptors rather than bypassing validation.

---

## Lesson 42: PAPI Type Gotchas

AI's first query returned raw PAPI types that React couldn't render:

```typescript
// AI wrote:
const entries = await api.query.TemplatePallet.Claims.getEntries();
const claims = entries.map(e => ({
    hash: e.keyArgs[0],    // Binary object, not string!
    owner: e.value.owner,  // AccountId object, not string!
    block: e.value.block_number,  // BigInt, not number!
}));
```

```
Error: Objects are not valid as a React child
(found: object with keys {asText, asHex, asOpaqueHex, asBytes, asOpaqueBytes})
```

Fix:

```typescript
const claims = entries.map(e => ({
    hash: e.keyArgs[0].asHex(),        // Binary → hex string
    owner: e.value.owner.toString(),   // AccountId → SS58 string
    block: Number(e.value.block_number),  // BigInt → number
}));
```

Notes:

PAPI returns rich typed objects, not primitive strings. `H256` comes back as a `Binary` with methods like `.asHex()`. `AccountId` needs `.toString()` for SS58 format. `BlockNumber` is `bigint`, not `number`. AI consistently treats query results as plain strings because that's what polkadot.js returned.

---

## Lesson 43: Dispatch Error Introspection

AI's first transaction submission had no error details:

```typescript
const result = await tx.signAndSubmit(signer);
if (!result.ok) {
    setStatus("Transaction failed");  // useless for debugging
}
```

From a session log:

> "can it say more than 'error: module'? we should have the **full error information**"

The fix drills into PAPI's dispatch error structure:

```typescript
function formatDispatchError(err: any): string {
    if (err?.type === "Module" && err.value) {
        return `${err.value.type}.${err.value.value?.type ?? ""}`;
        // e.g. "TemplatePallet.AlreadyClaimed"
    }
    return JSON.stringify(err);
}
```

Notes:

PAPI's dispatch errors have a nested structure: `{type: "Module", value: {type: "PalletName", value: {type: "ErrorVariant"}}}`. AI initially just stringified the top-level error or showed "Module." Always drill into `err.value.type` and `err.value.value.type` to get the actual pallet error name.

---

## Lesson 44: React Strict Mode Connection Races

The connection logic had two bugs:

**Bug 1: Lost `this` context on subscription cleanup:**

```typescript
// BUG: .unsubscribe loses 'this' context
unsub = client.finalizedBlock$.subscribe(handler).unsubscribe;

// FIX: store the subscription object
const sub = client.finalizedBlock$.subscribe(handler);
return () => sub.unsubscribe();
```

**Bug 2: Stale async results from StrictMode double-mount:**

```typescript
const connectIdRef = useRef(0);
const connect = async (url: string) => {
    const id = ++connectIdRef.current;
    // ... async connection work ...
    if (connectIdRef.current !== id) return;  // stale mount, discard
    setConnected(true);
};
```

Notes:

React StrictMode double-mounts components in development, causing two connections to race. The connect-ID pattern discards results from the first (unmounted) connection attempt. AI consistently gets this wrong because it doesn't model React's lifecycle during async operations.

---

## Lesson 45: Pallet Detection

The template detects which pallets are available on the connected chain:

```typescript
// Try querying the pallet -- if it throws, the pallet doesn't exist
try {
    await api.query.TemplatePallet.Claims.getEntries();
    detected.templatePallet = true;
} catch {
    detected.templatePallet = false;
}

try {
    await api.constants.Revive.DepositPerByte();
    detected.revive = true;
} catch {
    detected.revive = false;
}
```

From a session log:

> "We should also **detect on connection** to the node, if it has the pallets we expect. If not, we should **disable those buttons** and put a note saying that the pallet was not found"

Notes:

PAPI throws when you query a pallet that doesn't exist in the runtime. Wrapping queries in try/catch is the standard detection pattern. AI initially showed all navigation links regardless of runtime composition. For a template that students might customize (removing pallets), detection prevents confusing errors.

---

## Lesson 46: Sudo Transactions in PAPI

Funding accounts with Sudo requires the `.decodedCall` pattern:

```typescript
const tx = api.tx.Sudo.sudo({
    call: api.tx.Balances.force_set_balance({
        who: { type: "Id", value: ss58Address },
        new_free: amount,
    }).decodedCall,  // <-- extracts the inner RuntimeCall
});
await tx.signAndSubmit(aliceSigner);
```

Note the enum encoding for `who`: `{ type: "Id", value: address }`. PAPI represents Rust enums as objects with `type` and `value` fields.

Notes:

The `.decodedCall` method on a PAPI transaction extracts the inner `RuntimeCall` data without submitting it. This is how you nest one extrinsic inside another (like Sudo wrapping Balances). AI initially tried to pass the full transaction object to Sudo, which doesn't work. The enum `{ type: "Id", value: ... }` pattern for `MultiAddress` is also a common PAPI gotcha.

---

## Lesson 47: Contract Pages -- Same Code, Two Backends

The EVM and PVM contract pages share identical frontend code:

```typescript
// Both use viem with the same ABI
const hash = await walletClient.writeContract({
    address: contractAddress,
    abi: proofOfExistenceAbi,
    functionName: "createClaim",
    args: [fileHash],
});
await publicClient.waitForTransactionReceipt({ hash });
```

The only difference is the contract address -- the `eth-rpc` proxy provides the same Ethereum JSON-RPC interface for both EVM and PVM backends.

Notes:

This is a powerful teaching point: the same Solidity contract compiles to both EVM (via solc) and PVM (via resolc), and the same frontend code interacts with both. The eth-rpc adapter translates Ethereum JSON-RPC calls to Substrate extrinsics. Students only need to learn one API for both contract execution environments.

---

## Lesson 48: PAPI Descriptor Regeneration (Again)

From session logs, persistent descriptor issues:

> "Error: Incompatible runtime entry Storage(TemplatePallet.Counters)"

> "i restarted the node and the front end using scripts, which should have regenerated the descriptors, and **i still get error**"

> "it is still not working. why does the metadata hash change? find the root cause. **compare to the OAB repo, which is definitely working end to end**"

The descriptor regeneration workflow:

```bash
cd web && npx papi update && npx papi generate
```

This must happen every time the pallet changes. The postinstall hook (`"postinstall": "papi generate || true"`) auto-generates on `npm install`, but the `|| true` silently swallows failures.

Notes:

Descriptor mismatches are the #1 frontend-pallet integration issue. The error messages (`Incompatible runtime entry`) don't say "regenerate your descriptors." AI frequently changes pallet code without running the regeneration step. In the template, the postinstall hook helps but the `|| true` mask means you won't see if generation failed.

---

## Lesson 49: Connection State and Navigation

From session logs:

> "when I go to a different page, then back to home, I see the error: **Could not connect**. But I am successfully connected"

> "attempting to connect **should happen on any page**"

The fix: lift connection state to a top-level hook (`useConnection`) so it persists across page navigation:

```typescript
// App.tsx -- connection managed at the top
function App() {
    useConnectionManagement();  // connects once, persists across routes
    return <Routes>...</Routes>;
}
```

Notes:

AI initially put connection logic inside each page component. When React Router navigated between pages, the component unmounted, dropping the WebSocket connection. The fix is managing connection at the App level, not the page level. This is a React architecture issue, not a PAPI issue, but AI consistently puts state too low in the component tree.

---

## PST Summary: Key Differences from OAB

| Aspect | Open Auto Battler | Polkadot Stack Template |
|--------|-------------------|------------------------|
| Pallet complexity | 5 pallets, game logic | 1 simple PoE pallet |
| PAPI usage | Raw SCALE bytes | Typed API queries |
| Frontend complexity | WASM engine + SCALE bridge | Direct PAPI reads |
| Contract integration | Custom ABI encoding | Standard viem/Hardhat |
| Key challenge | Trust model, state splitting | SDK upgrades, descriptors |
| Session count | ~50 sessions | ~22 sessions |

The template project is simpler but surfaces a different set of issues: SDK dependency management, pallet detection, dispatch error handling, and making code exemplary for students.

---

## The Good News

Both projects -- a complex game and a simple template -- were built entirely by AI across ~170 combined sessions.

The key insight: **AI is a fast first-draft generator, not a FRAME expert.**

Your job is to know the patterns well enough to course-correct.

That's what the rest of this FRAME module is for.

---

<!-- .slide: data-background-color="#000000" -->

# Questions
