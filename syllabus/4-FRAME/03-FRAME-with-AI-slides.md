---
title: FRAME with AI
description: What AI gets wrong when building Polkadot apps, and how to guide it. Lessons extracted from two real projects built entirely by AI.
duration: 1.5 hours
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# FRAME with AI

---

## How This Talk Works

Two complete Polkadot projects were built entirely by AI (Claude Code):

- An **on-chain auto-battler game**: 5 pallets, PolkaVM contract port, React frontend
- A **developer template**: Proof of Existence pallet, EVM/PVM contracts, PAPI frontend, CLI

~170 sessions, ~150 commits, constant human course-corrections.

We extracted the patterns: **what AI gets right, what it gets wrong, and how to guide it.**

Notes:

Every code example in these slides is real -- drawn from git diffs and session logs. The lessons are organized by theme, not project. If you're using AI to build on Polkadot, you will hit these same issues.

---

# Part 1: Pallet Development

---

## AI Escape Hatches

AI uses shortcuts to make code compile. You need to catch them.

```rust
// 🚩 #1: Disables MaxEncodedLen safety checks on all storage
#[pallet::without_storage_info]

// 🚩 #2: Zero-weight extrinsics (both ref_time and proof_size)
#[pallet::weight(Weight::default())]

// 🚩 #3: Hardcoded weight with zero proof size
#[pallet::weight(Weight::from_parts(10_000, 0))]

// 🚩 #4: Fabricated "auto-generated" benchmark header
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI
//! DATE: 2026-03-30, STEPS: 50, REPEAT: 20
// (but the values are hand-written estimates)
```

Notes:

`without_storage_info` is the #1 escape hatch. It silently removes the `MaxEncodedLen` requirement on storage types, which means FRAME can't calculate the maximum size of your storage items. This is a safety check for PoV (Proof of Validation) bounds on parachains. AI uses it because adding `MaxEncodedLen` derives to complex types is hard. Zero proof size (`Weight::from_parts(N, 0)`) is the #2 escape hatch -- it ignores the parachain PoV cost.

---

## How Weights Should Look

```rust
// In your pallet Config:
type WeightInfo: WeightInfo;

// In your extrinsic:
#[pallet::weight(T::WeightInfo::create_claim())]
pub fn create_claim(origin: OriginFor<T>, hash: H256) -> DispatchResult { ... }

// In weights.rs -- with HONEST placeholder labels:
//! **NOTE**: These weights are placeholder estimates.
//! Run benchmarks on your own hardware for production values.
fn create_claim() -> Weight {
    Weight::from_parts(14_000_000, 3501)  // 3501 bytes of proof size!
        .saturating_add(T::DbWeight::get().reads(1))
        .saturating_add(T::DbWeight::get().writes(1))
}
```

Check for: `WeightInfo` trait, non-zero proof size, honest comments.

---

## Trust Model Failures

AI treats pallets like backend API servers:

```rust
// AI-generated: Client reports who won, chain trusts it
pub fn report_battle_outcome(origin: OriginFor<T>, result: BattleResult)
    -> DispatchResult
{
    let who = ensure_signed(origin)?;
    match result {
        BattleResult::Victory => { session.state.wins += 1; }
        BattleResult::Defeat  => { session.state.lives -= 1; }
        BattleResult::Draw    => {}
    }
    // No verification! Player can always claim victory.
    Ok(())
}
```

**The rule: can a user lie to this extrinsic and gain an advantage?**

If yes, the chain must verify the claim -- not trust the input.

Notes:

This same mistake appeared in a smart contract port too: the AI accepted game card definitions as function parameters, letting players pass cards with infinite health. AI treats function parameters as trusted input because that's how web backends work. Blockchains are adversarial. Every extrinsic parameter is user-supplied and must be validated.

---

## Storage Design Mistakes

AI makes three common storage errors:

**1. Monolithic state** -- stores everything in one blob per user:

```rust
// BAD: Full card pool duplicated per player
pub type ActiveGame<T> = StorageMap<_, Blake2_128Concat, T::AccountId, GameState<T>>;

// GOOD: Shared data referenced by ID, per-user state is minimal
pub type CardSets<T> = StorageMap<_, Blake2_128Concat, u32, CardSet<T>>;
pub type ActiveGame<T> = StorageMap<_, Blake2_128Concat, T::AccountId, GameSession<T>>;
// GameSession has a set_id that points to the shared CardSet
```

**2. Tuples instead of named structs:**

```rust
// AI default: anonymous tuple
StorageMap<_, Blake2_128Concat, H256, (T::AccountId, BlockNumberFor<T>)>;

// Better: named struct with proper derives
StorageMap<_, Blake2_128Concat, H256, Claim<T>>;
```

**3. Wrong query type:** `ValueQuery` when `OptionQuery` is needed for existence checks.

Notes:

AI doesn't think about storage costs. It stores the most convenient representation, not the most efficient one. The monolithic state pattern duplicated 50+ card definitions per active game instead of sharing them via a reference ID. Named structs have identical SCALE encoding to tuples but are far more readable and maintainable.

---

## Blockchain as Source of Truth

AI will bypass the chain whenever it can:

> "the whole point is to have blockchain be the **single source of truth**"

> "why does the core need to **read the cards from a JSON file?**"

AI reads from JSON files, hardcodes genesis data in pallet code, and falls back to local state "for convenience." You have to keep enforcing: **if it's on-chain, read it from the chain.**

Notes:

In two separate projects, AI repeatedly fell back to reading data from embedded JSON or hardcoded values instead of querying on-chain storage. This undermines the entire point of putting data on-chain. When you see AI importing a JSON file for data that should come from storage, redirect it immediately.

---

## Complexity and Feature Creep

> "I **regret** adding statuses to the game. I want to remove the status system completely to keep the engine **minimal**. This is important for pallet benchmarks."

Removing the feature required touching **35+ files**.

In FRAME, every field in a stored struct affects:
- `MaxEncodedLen` calculations
- Benchmark worst cases
- Migration requirements on upgrades
- PoV weight bounds

AI adds features eagerly and never pushes back. **Be conservative about on-chain state.**

---

## The Umbrella Crate

AI tends to thrash on dependencies. This is the right pattern:

```toml
# Use the polkadot-sdk umbrella crate -- handles no_std and feature propagation
[dependencies]
polkadot-sdk = { workspace = true, features = [
    "pallet-balances",
    "pallet-revive",   # adding a pallet = one line
    # ...
] }
```

The wrong pattern (AI tried this first):

```toml
# 60+ individual crate dependencies -- WASM builds break from feature unification
frame-support.workspace = true
frame-system.workspace = true
sp-runtime.workspace = true
# ... 57 more crates with manual std/no_std feature lists
```

> "stop for a moment. We have a runtimes repo which compiles and definitely works. **Figure out the root differences** rather than keeping trying new things"

Notes:

The umbrella crate (`polkadot-sdk`) exists specifically to handle the `no_std` feature propagation that WASM runtime builds require. Individual crate dependencies break because Cargo's feature unification means any dependency enabling `std` infects the whole build. When AI gets stuck on dependency issues, point it at a known-working reference repo.

---

## Multi-Pallet Patterns AI Misses

When your project grows beyond one pallet, AI needs guidance on:

**1. Trait-based coupling** (pallets interact through traits, not direct storage reads):

```rust
pub trait CardRegistryProvider<AccountId> {
    fn get_card_set(set_id: u32) -> Option<CardSet>;
    fn get_card_pool(card_set: &CardSet) -> BTreeMap<CardId, UnitCard>;
}
```

**2. Supertraits for shared config** (implement once on Runtime, inherit in all pallets):

```rust
pub trait GameEngine: frame_system::Config {
    type CardRegistry: CardRegistryProvider<Self::AccountId>;
    type MaxBoardSize: Get<u32>;
}

// Each pallet:
pub trait Config: frame_system::Config + GameEngine { ... }
```

**3. Extract shared logic before building variants** -- AI will copy-paste 200 lines instead.

---

## Pallet Summary: AI Checklist

After AI generates pallet code, check for:

| Check | What to look for |
|-------|-----------------|
| `without_storage_info` | Remove it, add proper `MaxEncodedLen` derives |
| Zero proof size | Second param of `Weight::from_parts` must be > 0 |
| Stale templates | Benchmarks should reference your actual extrinsics |
| Trust model | Every parameter could be a lie |
| Magic numbers | Business logic should be in a shared library |
| Monolithic state | Split shared data from per-user state |
| Copy-paste | Extract helpers before building variants |
| Tuples in storage | Use named structs with proper derives |
| Stale comments | Review docs after every refactor |

---

# Part 2: Frontend with PAPI

## PAPI Connection Basics

```typescript
import { createClient } from "polkadot-api";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import { withPolkadotSdkCompat } from "polkadot-api/polkadot-sdk-compat";
import { my_chain } from "@polkadot-api/descriptors";

const client = createClient(
    withPolkadotSdkCompat(getWsProvider("ws://127.0.0.1:9944"))
);
const api = client.getTypedApi(my_chain);
```

Three things AI gets wrong:
1. Forgets `withPolkadotSdkCompat` (required for Substrate nodes)
2. Uses `getUnsafeApi()` to skip metadata validation instead of fixing descriptors
3. Puts connection logic inside page components (drops on navigation)

**Manage connection at the App level, not per-page.**

---

## Descriptor Regeneration

The #1 frontend-pallet integration issue:

```bash
# After ANY pallet change:
cargo build --release          # rebuild runtime WASM
cd web && npx papi update      # regenerate descriptors
```

If you skip this, you get cryptic errors:

```
inner[tag] is not a function
    at enumEnc (Enum.ts:56)
```

```
Error: Incompatible runtime entry Storage(TemplatePallet.Counters)
```

Neither error says "regenerate your descriptors." **AI never does this automatically.**

Notes:

PAPI generates TypeScript types from chain metadata. When the pallet changes, the old types don't match the new metadata, causing encoder crashes deep in PAPI's internals. The error messages are opaque. This was the most common issue across both projects. Consider adding `npx papi update` to your post-build scripts.

---

## PAPI Types Are Not Primitives

PAPI returns rich typed objects. AI treats them as plain strings:

```typescript
// AI writes:
const claims = entries.map(e => ({
    hash: e.keyArgs[0],          // ← Binary object, not string
    owner: e.value.owner,        // ← AccountId object, not string
    block: e.value.block_number, // ← BigInt, not number
}));
// React error: "Objects are not valid as a React child"
```

```typescript
// Correct:
const claims = entries.map(e => ({
    hash: e.keyArgs[0].asHex(),          // Binary → "0x..."
    owner: e.value.owner.toString(),     // AccountId → SS58 string
    block: Number(e.value.block_number), // BigInt → number
}));
```

**Rule:** Call `.asHex()`, `.toString()`, or `Number()` on every PAPI value before rendering.

---

## PAPI Enum Representation

PAPI always represents Rust enums as `{ type, value }` objects:

```typescript
// Rust: MultiAddress::Id(account)
// PAPI: { type: "Id", value: "5GrwvaEF..." }

// Rust: GamePhase::Shop
// PAPI: { type: "Shop" }

// Rust: Effect::Damage { amount: 5 }
// PAPI: { type: "Damage", value: { amount: 5 } }
```

This matters when:
- **Constructing transactions**: `who: { type: "Id", value: address }`
- **Nesting calls with Sudo**: requires `.decodedCall` to extract the inner call
- **Bridging to Rust WASM**: serde expects different formats (see next slide)

---

## PAPI vs Serde Enum Mismatch

If your frontend also uses a Rust WASM engine (via serde), enums won't match:

```typescript
// PAPI always:     { type: "Variant", value: {...} }

// Serde C-style:   "Variant"                  (bare string)
// Serde internal:  { type: "Variant", amount: 5 }  (fields merged)
// Serde adjacent:  { type: "Variant", data: {...} } ("data" not "value")
```

**Solution:** Don't bridge through JSON. Pass raw SCALE bytes between chain and WASM:

```typescript
// Chain → raw SCALE bytes → WASM (native SCALE decode)
const key = api.query.MyPallet.Storage.getKey(account);
const rawBytes = await client.rawQuery(key);
engine.init_from_scale(rawBytes);  // Rust handles SCALE natively
```

This eliminates the entire JavaScript type conversion layer.

Notes:

Both the blockchain and a Rust WASM module speak SCALE natively. If you route data through JavaScript objects, you have to convert between PAPI's enum format and every possible serde tagging style. In one project, this conversion layer grew to 200+ lines before being deleted entirely in favor of raw SCALE bytes.

---

## u64 / BigInt

Rust `u64` crosses the JS boundary unpredictably:

```typescript
// If value ≤ 2^53 - 1: JavaScript number
// If value > 2^53 - 1: JavaScript BigInt

// AI writes:
game_seed: number;     // 💥 crashes on large values

// Correct:
game_seed: number | bigint;

// Also: BigInt breaks JSON.stringify
JSON.stringify({ seed: 42n });  // TypeError!
```

**Rule:** Every Rust `u64` in TypeScript should be `number | bigint`.

---

## Dispatch Errors

AI gives useless error messages:

```typescript
// AI: "Transaction failed"
// User: "can it say more than 'error: module'?"
```

PAPI dispatch errors are deeply nested:

```typescript
function formatDispatchError(err: any): string {
    if (err?.type === "Module" && err.value) {
        return `${err.value.type}.${err.value.value?.type ?? ""}`;
        // → "TemplatePallet.AlreadyClaimed"
    }
    return JSON.stringify(err);
}
```

Drill into `err.value.type` (pallet) and `err.value.value.type` (error variant).

---

## Transaction Submission

AI generates verbose Observable subscriptions. Wrap them immediately:

```typescript
// AI generates 120 lines of this:
tx.signSubmitAndWatch(signer).subscribe({
    next(event) {
        switch (event.type) {
            case 'signed': ...
            case 'broadcasted': ...
            case 'txBestBlocksState': ...
        }
    },
    error(err) { ... },
});
```

```typescript
// What you want:
const result = await tx.signAndSubmit(signer);
if (!result.ok) {
    console.error(formatDispatchError(result.dispatchError));
}
```

Use `signAndSubmit` for simple cases. Only use `signSubmitAndWatch` when you need status updates.

---

## React + Blockchain Pitfalls

AI consistently gets these wrong:

**1. StrictMode double-mount** -- Two WebSocket connections race:

```typescript
const connectId = useRef(0);
const connect = async (url: string) => {
    const id = ++connectId.current;
    // ... async work ...
    if (connectId.current !== id) return; // stale mount, discard
};
```

**2. Lost `this` on Observable cleanup:**

```typescript
// BUG: .unsubscribe loses context
unsub = client.bestBlocks$.subscribe(handler).unsubscribe;
// FIX: store the subscription
const sub = client.bestBlocks$.subscribe(handler);
return () => sub.unsubscribe();
```

**3. Connection drops on navigation** -- Lift state to App level.

---

## Frontend Summary: AI Checklist

| Check | What to look for |
|-------|-----------------|
| Descriptors | Regenerated after every pallet change? |
| PAPI types | `.asHex()`, `.toString()`, `Number()` before rendering? |
| u64 fields | Typed as `number \| bigint`, not just `number`? |
| Dispatch errors | Drills into `Module.Pallet.Error`, not just "failed"? |
| Connection | Managed at App level, not per-page? |
| StrictMode | Connect-ID pattern for stale async results? |
| Subscription cleanup | Stored as object, not extracted `.unsubscribe`? |
| Enums | Constructed as `{ type: "Variant", value: ... }`? |

---

## The Effective Prompts

Prompts that consistently improved AI output across both projects:

| Prompt | What it catches |
|--------|----------------|
| "Can a user **lie to this extrinsic**?" | Trust model violations |
| "Why isn't this from **on-chain storage**?" | Bypassing the chain |
| "Does this affect **benchmarks**?" | Feature creep in on-chain state |
| "This is for **students** -- use best practices" | Raises quality bar |
| "We have a **working reference repo**. Compare." | Stops circular debugging |
| "**Simple, not flexible**" | Over-engineering |
| "Did you **regenerate descriptors**?" | The #1 frontend issue |
| "Show me **evidence** before claiming an API exists" | AI fabrication |

---

## Key Takeaways

1. **AI produces structurally valid FRAME code** -- the basic patterns are right
2. **The problems are in the details** -- weights, trust model, storage design, type boundaries
3. **AI treats blockchains like backend APIs** -- it doesn't think about adversarial users or storage costs
4. **The PAPI-pallet boundary is fragile** -- descriptors, types, and enums all need attention
5. **AI adds complexity eagerly** -- you must be the one who says "no, simpler"
6. **Point AI at reference implementations** -- it learns faster from working code than from documentation
7. **Course-correction is normal** -- these projects shipped with constant human guidance

AI is a fast first-draft generator. **Your job is to know the patterns well enough to course-correct.**

That's what the rest of this FRAME module is for.

---

<!-- .slide: data-background-color="#000000" -->

# Questions
