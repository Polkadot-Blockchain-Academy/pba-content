---
title: Transaction Extensions
description: Transaction Extensions, Transaction Priority.
---

# Transaction Extensions

---v

# Transaction Extensions
## Or: Lifecycle of a Transaction

---v

## Summary

- In this lecture you will learn above one of the more advanced FRAME concepts, _Transaction Extensions_.

- They allow for a multitude of custom features to be added to FRAME transactions.

- They allow hooking into and configuring the transaction lifecycle in the transaction pool and block execution.

---

## Recap: Transaction Pool

The transaction pool has two jobs:
1. Transaction Validation
2. Transaction Ordering

---v

### 1. Transaction Validation

Moving transactions from one list to the other.

<diagram class="mermaid" style="display: flex; width: 80%">
graph LR
    W["🤠 Wild West"] --"😈"--> T["🗑️"]
    W --"😇 ⌛️"--> R["✅ Ready"]
    W --"😇 ⏳"--> F["⏰ Future"]
</diagram>

Notes:
Greatly simplified from the actual tx pool of course. Feel free to check it out.

---v

### 1. Transaction Validation

- Transaction validity is exclusively outside of the transaction pool, and is **100% determined by the Runtime**.
- Transaction validation should be **cheap** to perform.
- Transaction pool is entirely an **offchain operation**.
  - --> No state change

Notes:

Important, must pause and ask!

- Why is it from the runtime? because the transaction format is opaque and the node doesn't even know what to do with it.
- Why does it have to be cheap? wild west, unpaid, DoS!
- Pesky question: but be aware that from the runtime's perspective, the node could be malicious. The runtime cannot trust the node to obey.
  ** THE RUNTIME MUST RE-VALIDATE TRANSACTIONS LATER in block building and import as well **

---v

### Shower Thought: Runtime vs STF

<img style="width: 1100px;" src="../../Substrate/img/tx-pool/peter-parker-glasses-off.png" />

---v

### Shower Thought: Runtime vs STF

<img style="width: 1100px;" src="../../Substrate/img/tx-pool/peter-parker-glasses-on.png" />

---v

### 1. Transaction Validation

```rust[1-100|5-6|8-9|11-12|14-100|1-100]
pub type TransactionValidity = Result<ValidTransaction, TransactionValidityError>;

/// This is going into `Ready` or `Future`
pub struct ValidTransaction {
  /// If in "Ready", what is the priority?
  pub priority: u64,

  /// For how long is this valid?
  pub longevity: u64,

  /// Should we propagate it?
  pub propagate: bool,

  /// Does this require any other tag to be present in ready?
  ///
  /// This determines "Ready" or "Future".
  pub requires: Vec<Tag>,
  /// Does this provide any tags?
  pub provides: Vec<Tag>,
}

type Tag = Vec<u8>
```

Notes:

So some code in our Runtime will have to provide this data!

---v

### 2. Transaction Ordering

- `provides` and `requires` is a very flexible mechanism; it allows you to:
  - Specify if a transaction is "Ready" or "Future"
  - Within each, what transactions should ge before which.

<diagram class="mermaid" style="display: flex; width: 40%">
graph LR
    W["🤠 Wild West"] --"😈"--> T["🗑️"]
    W --"😇 ⌛️"--> R["✅ Ready"]
    W --"😇 ⏳"--> F["⏰ Future"]
</diagram>

Note: it essentially forms a graph.

---v

### 2. Transaction Ordering: `priority`

From the **Ready pool**, when all requirements are met, then `priority` dictates the order.

Further tie breakers:

2. ttl: shortest `longevity` goes first
3. time in the queue: longest to have waited goes first

<!-- .element: class="fragment" -->

Note:

https://github.com/paritytech/polkadot-sdk/blob/bc53b9a03a742f8b658806a01a7bf853cb9a86cd/substrate/client/transaction-pool/src/graph/ready.rs#L146

---

## History

- Transaction Extensions are an evolution of Signed Extensions.
  - See [the introducing PR](https://github.com/paritytech/polkadot-sdk/pull/3685).
- In essence, they are
  - used to provide ordering and validity information for a transaction as discussed earlier.
  - a generic way to **extend** the transaction.

---

## Flow

<diagram class="mermaid" style="display: flex; width: 90%">
graph TD
    subgraph "Transaction Extension Pipeline"
        A[Transaction Input] --> B[Extension 1]
        B --> C[Extension 2]
        C --> D[Extension N]
        D --> E[Call Dispatch]
        E --> F[Post Dispatch Pipeline]
    end

    subgraph "Extension Data Types"
        G[Implicit Data<br/>Runtime-derived] --> H[Val Data<br/>validate to prepare]
        H --> I[Pre Data<br/>prepare to post_dispatch]
    end

    subgraph "Extension Phases"
        J[1. implicit]
        K[2. validate]
        L[3. prepare]
        M[4. post_dispatch]
        
        J --> K
        K --> L
        L --> M
    end

    subgraph "Data Flow Through Phases"
        N[Origin In] --> K
        O[Inherited Implication] --> K
        K --> P[ValidTransaction + Val + Origin Out]
        P --> L
        L --> Q[Pre Data]
        Q --> M
        M --> R[Weight Refund]
    end

    subgraph "Pipeline Composition Tuples"
        S[Ext A] --> T[Ext B]
        T --> U[Ext C]
        
        S1[Origin0] --> S
        S --> S2[Origin1]
        S2 --> T
        T --> T2[Origin2]
        T2 --> U
        U --> U2[Origin_final]
        
        I1[Implication0] --> S
        S --> I2[Implication1 + A data]
        I2 --> T
        T --> I3[Implication2 + B data]
        I3 --> U
    end

    subgraph "Implication Structure"
        V[Base Implication<br/>Call + Version] --> W[Explicit Implications<br/>Extension Data]
        W --> X[Implicit Implications<br/>Runtime-derived]
        Y[ImplicationParts] --> V
        Y --> W  
        Y --> X
    end

    A --> J
    B -.-> S
    F --> M
    G -.-> J
    
    classDef phase fill:#4a148c,stroke:#fff,stroke-width:2px,color:#fff
classDef data fill:#6a1b9a,stroke:#e1bee7,stroke-width:2px,color:#fff
classDef flow fill:#7b1fa2,stroke:#ce93d8,stroke-width:2px,color:#fff
    
    class J,K,L,M phase
    class G,H,I,V,W,X,Y data
    class N,O,P,Q,R,S1,S2,T2,U2,I1,I2,I3 flow
</diagram>

---

## Anatomy

A transaction extension can be either or both of the following things:

- Some additional data that is attached to the transaction.
  - The tip!

<!-- .element: class="fragment" -->

- Some hooks that are executed before and after the transaction is executed.
  - Before each transaction is executed, it must pay its fee upfront.
  - Perhaps refund the fee partially 🤑.

<!-- .element: class="fragment" -->

---v

### Anatomy

- Some additional validation logic that is used to validate the transaction, and give feedback to the pool.
  - Set priority of the transaction based on some metric!

<!-- .element: class="fragment" -->

- Some additional data that must be present in the signed payload of each transaction.
  - Data that the sender has, the chain also has, it is not communicated itself, but it is part of the signature payload.
  - Spec version and genesis hash is part of all transactions' signature payload!

<!-- .element: class="fragment" -->

---v

### Anatomy: Let's Peek at the Trait

```rust [1-100|4-6|11|12-21|22-29|32-38]
pub trait TransactionExtension<Call>: /* snip required traits */
where Call: Dispatchable,
{
    type Implicit: Codec + StaticTypeInfo;
    type Val;
    type Pre;

    const IDENTIFIER: &'static str;

    // Required methods
    fn weight(&self, call: &Call) -> Weight;
    fn validate(
        &self,
        origin: <Call as Dispatchable>::RuntimeOrigin,
        call: &Call,
        info: &<Call as Dispatchable>::Info,
        len: usize,
        self_implicit: Self::Implicit,
        inherited_implication: &impl Implication,
        source: TransactionSource,
    ) -> Result<(ValidTransaction, Self::Val, <Call as Dispatchable>::RuntimeOrigin), TransactionValidityError>;
    fn prepare(
        self,
        val: Self::Val,
        origin: &<Call as Dispatchable>::RuntimeOrigin,
        call: &Call,
        info: &<Call as Dispatchable>::Info,
        len: usize,
    ) -> Result<Self::Pre, TransactionValidityError>;

	// Provided (but might implement)
	fn post_dispatch(
        pre: Self::Pre,
        info: &<Call as Dispatchable>::Info,
        post_info: &mut <Call as Dispatchable>::PostInfo,
        len: usize,
        result: &Result<(), DispatchError>,
    ) -> Result<(), TransactionValidityError> { ... }

    // Provided methods
    fn implicit(&self) -> Result<Self::Implicit, TransactionValidityError> { ... }
    fn metadata() -> Vec<TransactionExtensionMetadata> { ... }
    /* snip */
}
```

---

## Grouping Transaction Extension

- Is also a transaction extension itself!
- You can look at the implementation yourself.. but the TLDR is:
  - Executes each individually
  - Passes resulting `Origin` from one to the next
  - Combines results

Notes:

TODO: how `TransactionValidity` is `combined_with` is super important here, but probably something to cover more somewhere else and recap here.

---v

## Usage In The Runtime

- Each runtime has a bunch of signed extensions. They can be grouped as a tuple

```rust
pub type TxExtension = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>,
);

type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, TxExtension>;
```

- Transaction extensions might originate from a pallet, but are applied to ALL EXTRINSICS 😮‍💨!

Notes:

We will get to this later as well, but recall that TransactionExtensions are not a _FRAME/Pallet_
concept per se. FRAME just implements them. This also implies that everything regarding transaction
extensions is applied to **all transactions**, throughout the runtime.

---

## Encoding

```rust
struct Foo(u32, u32);
impl TransactionExtension for Foo {
  type Implicit = u32;
  fn implicit(&self) -> Result<Self::Implicit, TransactionValidityError> {
    Ok(42u32)
  }
  /* snip */
}

pub struct UncheckedExtrinsic<Address, Call, Signature, (Foo)>
{
	pub preamble: Preamble<(Address, Signature, Extension)>,
	pub function: Call,
}
```

- Two `u32` are decoded as `self`, `42u32` is expected to be in the signature payload.

---v

## Encoding

Here's the `check` function of `CheckedExtrinsic` to hint at this:

```rust
// UncheckedExtrinsic::check
fn check(self, lookup: &Lookup) -> Result<Self::Checked, TransactionValidityError> {
  Ok(match self.preamble {
    Preamble::Signed(signed, signature, tx_ext) => {
      let signed = lookup.lookup(signed)?;
      // The `Implicit` is "implicitly" included in the payload.
      let raw_payload = SignedPayload::new(self.function, tx_ext)?;
      if !raw_payload.using_encoded(|payload| signature.verify(payload, &signed)) {
        return Err(InvalidTransaction::BadProof.into())
      }
      let (function, tx_ext, _) = raw_payload.deconstruct();
      CheckedExtrinsic { format: ExtrinsicFormat::Signed(signed, tx_ext), function }
    },
    /* snip */
  })
}
```

---

## Transaction Pool Validation

- Each pallet also has `#[pallet::validate_unsigned]`.
- This kind of overlaps with creating a transaction extension and implementing `bare_validate`.
- Substrate is in the process of migrating to transaction extensions.

Notes:

https://github.com/paritytech/substrate/issues/6102
https://github.com/paritytech/substrate/issues/4419

---v

### Transaction Pool Validation

- Recall that transaction pool validation should be minimum effort and static.
- In `executive`, we only do the following:
  - check signature.
  - call `TransactionExtension::validate`/`TransactionExtension::bare_validate`
  - call `ValidateUnsigned::validate`, if unsigned.

Notes:

> Transaction queue is not part of the consensus system. Validation of transaction are _free_. Doing
> too much work in validation of transactions is essentially opening a door to be DOS-ed.

---

## Post Dispatch

- The dispatch result, plus generic type (`type Pre`) returned from `prepare` is passed to `post_dispatch`.

---

## Notable Transaction Extensions

- These are some of the default transaction extensions that come in FRAME.
- See if you can predict how they are made!

---v

### `ChargeAssetTxPayment`

Charge payments, refund if `Pays::No`.

```rust
pub enum Pre<T: Config> {
	Charge {
		tip: BalanceOf<T>,
		// who paid the fee
		who: T::AccountId,
		// imbalance resulting from withdrawing the fee
		initial_payment: InitialPayment<T>,
		// asset_id for the transaction payment
		asset_id: Option<ChargeAssetIdOf<T>>,
		// weight used by the extension
		weight: Weight,
	},
	NoCharge {
		// weight initially estimated by the extension, to be refunded
		refund: Weight,
	},
}
```

<!-- .element: class="fragment" -->

---v

### `CheckGenesis`

Wants to make sure you are signing against the right chain.

Put the genesis hash in `implicit`.

<!-- .element: class="fragment" -->

`CheckSpecVersion` and `CheckTxVersion` work very similarly.

<!-- .element: class="fragment" -->

---v

### `CheckNonZeroSender`

- interesting story: any account can sign on behalf of the `0x00` account.
- discovered by [@xlc](https://github.com/xlc).
- uses `validate` to ensure the signing account is not `0x00`.
  - Note: used to check in both `validate` and `pre_dispatch` in the old `SignedExtension`.

Notes:

https://github.com/paritytech/substrate/issues/10413

---v

### `CheckNonce`

- `validate`: check the nonce, DO NOT WRITE ANYTHING, returns `provides` and `requires`.
- `prepare`: check nonce and actually update it.

<!-- .element: class="fragment" -->

<div>

- remember that:
  - `validate` is only for lightweight checks, no read/write.
  - anything you write to storage is reverted anyhow.

</div>

<!-- .element: class="fragment" -->

---v

### `CheckWeight`

- Check there is enough weight in `validate`.
- Calculate and update the consumed weight in `prepare`.
- Adjust consumed weight in `post_dispatch` based on unspent weight.

<!-- .element: class="fragment" -->

---

## Big Picture: Pipeline of Extension

- Transaction extensions remind me of other pipelines in computer graphics or data processing where you pass data from one stage to another.

---

## Exercises

- Walk over the notable transaction extensions above and riddle each other about how they work.
- TransactionExtensions are an important part of the transaction encoding. Try and encode a correct
  transaction against a template runtime in any language that you want, using only a scale-codec
  library.
- TransactionExtension that logs something on each transaction
- TransactionExtension that keeps a counter of all transactions
- TransactionExtension that keeps a counter of all successful/failed transactions
- TransactionExtension that tries to refund the transaction from each account as long as they submit less
  than 1tx/day.
