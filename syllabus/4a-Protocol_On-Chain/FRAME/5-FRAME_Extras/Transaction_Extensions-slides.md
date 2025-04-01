---
title: Transaction Extensions
description: Transaction Extensions, Transaction Priority.
---

# Transaction Extensions

---v

- In this lecture you will learn above one of the more advanced FRAME concepts, _Transaction Extensions_.

* They allow for a multitude of custom features to be added to FRAME transactions.

---

## History

- Transaction Extensions are an evolution of Signed Extensions.
- See [the introducing PR](https://github.com/paritytech/polkadot-sdk/pull/3685)
- In essence, they are a generic way to **extend** the transaction.

---

## Anatomy

A transaction extension can either or both of the following things:

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
  - Set priority of transaction priority based on some metric!

<!-- .element: class="fragment" -->

- Some additional data that must be present in the signed payload of each transaction.
  - Data that the sender has, the chain also has, it is not communicated itself, but it is part of the signature payload.
  - Spec version and genesis hash is part of all transaction's signature payload!

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

TODO: how `TransactionValidity` is `combined_with` is super important here, but probably something to cover more in 4.3 and recap here.

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
concept per se. FRAME just implements them. This also implies that everything regarding signed
extensions is applied to **all transactions**, throughout the runtime.

---

## TODO: Encoding

```rust
struct Foo(u32, u32);
impl SignedExtension for Foo {
  type AdditionalSigned = u32;
  fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
    Ok(42u32)
  }
}

pub struct UncheckedExtrinsic<Address, Call, Signature, (Foo)>
{
	pub signature: Option<(Address, Signature, Extra)>,
	pub function: Call,
}
```

- 2 u32 are decoded as, `42u32` is expected to be in the signature payload.

Notes:

Here's the `check` function of `CheckedExtrinsic` extensively documented to demonstrate this:

```rust
// SignedPayload::new
pub fn new(call: Call, extra: Extra) -> Result<Self, TransactionValidityError> {
	// asks all signed extensions to give their additional signed data..
	let additional_signed = extra.additional_signed()?;
	// this essentially means: what needs to be signed in the signature of the transaction is:
	// 1. call
	// 2. signed extension data itself
	// 3. any additional signed data.
	let raw_payload = (call, extra, additional_signed);
	Ok(Self(raw_payload))
}

// UncheckedExtrinsic::check
fn check(self, lookup: &Lookup) -> Result<Self::Checked, TransactionValidityError> {
	Ok(match self.signature {
		Some((signed, signature, extra)) => {
			let signed = lookup.lookup(signed)?;
			// this is the payload that we expect to be signed, as explained above.
			let raw_payload = SignedPayload::new(self.function, extra)?;
			// encode the signed payload, and check it against the signature.
			if !raw_payload.using_encoded(|payload| signature.verify(payload, &signed)) {
				return Err(InvalidTransaction::BadProof.into())
			}

			// the extra is passed again to `CheckedExtrinsic`, see in the next section.
			let (function, extra, _) = raw_payload.deconstruct();
			CheckedExtrinsic { signed: Some((signed, extra)), function }
		},
		// we don't care about signed extensions at all.
		None => CheckedExtrinsic { signed: None, function: self.function },
	})
}
```

---

## Transaction Pool Validation

- Each pallet also has `#[pallet::validate_unsigned]`.
- This kind of overlaps with creating a signed extension and implementing `validate_unsigned`.

Notes:

https://github.com/paritytech/substrate/issues/6102
https://github.com/paritytech/substrate/issues/4419

---v

### Transaction Pool Validation

- Recall that transaction pool validation should be minimum effort and static.
- In `executive`, we only do the following:
  - check signature.
  - call `Extra::validate`/`Extra::validate_unsigned`
  - call `ValidateUnsigned::validate`, if unsigned.
  - NOTE dispatching ✅!

Notes:

> Transaction queue is not part of the consensus system. Validation of transaction are _free_. Doing
> too much work in validation of transactions is essentially opening a door to be DOS-ed.

---v

### Transaction Pool Validation

- Crucially, you should make sure that you re-execute anything that you do in transaction pool validation in dispatch as well:

```rust
/// Do any pre-flight stuff for a signed transaction.
///
/// Make sure to perform the same checks as in [`Self::validate`].
fn pre_dispatch() -> Result<Self::Pre, TransactionValidityError>;
```

- Because conditions that are not stateless might change over time!

---

## Post Dispatch

- The dispatch result, plus generic type (`type Pre`) returned from `pre_dispatch` is passed to `post_dispatch`.
- See [`impl Applyable for CheckedExtrinsic`](https://github.com/paritytech/substrate/blob/a47f200eebeb88a5bde6f1ed2be9728b82536dde/primitives/runtime/src/generic/checked_extrinsic.rs#L69) for more info.

---

## Notable Signed Extensions

- These are some of the default signed extensions that come in FRAME.
- See if you can predict how they are made!

---v

### `ChargeTransactionPayment`

Charge payments, refund if `Pays::Yes`.

```rust
type Pre = (
  // tip
  BalanceOf<T>,
  // who paid the fee - this is an option to allow for a Default impl.
  Self::AccountId,
  // imbalance resulting from withdrawing the fee
  <<T as Config>::OnChargeTransaction as OnChargeTransaction<T>>::LiquidityInfo,
);
```

<!-- .element: class="fragment" -->

---v

### `check_genesis`

Wants to make sure you are signing against the right chain.

Put the genesis hash in `additional_signed`.

<!-- .element: class="fragment" -->

`check_spec_version` and `check_tx_version` work very similarly.

<!-- .element: class="fragment" -->

---v

### `check_non_zero_sender`

- interesting story: any account can sign on behalf of the `0x00` account.
- discovered by [@xlc](https://github.com/xlc).
- uses `pre_dispatch` and `validate` to ensure the signing account is not `0x00`.

Notes:

https://github.com/paritytech/substrate/issues/10413

---v

### `check_nonce`

- `pre_dispatch`: check nonce and actually update it.
- `validate`: check the nonce, DO NOT WRITE ANYTHING, set `provides` and `requires`.

<!-- .element: class="fragment" -->

<div>

- remember that:
  - `validate` is only for lightweight checks, no read/write.
  - anything you write to storage is reverted anyhow.

</div>

<!-- .element: class="fragment" -->

---v

### `check_weight`

- Check there is enough weight in `validate`.
- Check there is enough weight, and update the consumed weight in `pre_dispatch`.
- Updated consumed weight in `post_dispatch`.

<!-- .element: class="fragment" -->

---

## Big Picture: Pipeline of Extension

- Signed extensions (or at least the `pre_dispatch` and `validate` part) remind me of the extension
  system of `express.js`, if any of you know what that is

---v

## Big Picture: Pipeline of Extension

<img rounded src="../../../../assets/img/5-Substrate/dev-5-x-signed-extensions.svg" />

---

## Exercises

- Walk over the notable signed extensions above and riddle each other about how they work.
- SignedExtensions are an important part of the transaction encoding. Try and encode a correct
  transaction against a template runtime in any language that you want, using only a scale-codec
  library.
- SignedExtensions that logs something on each transaction
- SignedExtension that keeps a counter of all transactions
- SignedExtensions that keeps a counter of all successful/failed transactions
- SignedExtension that tries to refund the transaction from each account as long as they submit less
  than 1tx/day.
