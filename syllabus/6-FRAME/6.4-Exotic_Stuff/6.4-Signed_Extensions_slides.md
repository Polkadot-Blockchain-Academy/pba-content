---
title: Signed Extensions
description: Signed Extensions, Transaction Priority.
instructors: ["Shawn Tabrizi, Kian Paimani"]
teaching-assistants: ["Sacha Lansky"]
---

# Signed Extensions

---v

- In this lecture you will learn above one of the most advance FRAME concepts, _Signed Extensions_.
  They allow for a multitude of custom features to be added to FRAME transactions.

---

## History

- Signed Extensions originally where added to implement tipping in a reasonable way.

> Originally, your dumb instructor (@kianenigma) had the idea of hard-coding it into the
> `UncheckedExtrinsic`, until @gavofyork jumped in with the idea of signed extensions.
> [Tipped Transaction Type. by kianenigma · Pull Request #2930 · paritytech/substrate](https://github.com/paritytech/substrate/pull/2930/files) > [Extensible transactions (and tips) by gavofyork · Pull Request #3102 · paritytech/substrate](https://github.com/paritytech/substrate/pull/3102/files)

---v

- In essence, they are a generic way to **extend** the transaction. Moreover, if they have
  additional payload, it is signed, therefore _`SignedExtension`_.

---

## Anatomy

A signed extension can be either combination of the following things:

1. Some additional data that is attached to the transaction.
   - The tip!
2. Some hooks that are executed before and after the transaction is executed.
   - Before each transaction is executed, it must pay its fee upfront.
3. Some additional validation logic that is used to validate the transaction, and give feedback to
   the pool.
   - Set priority of transaction priority based on some metric!

- Some additional data that must be present in the signed payload of each transaction
  - Spec version is part of all transaction!

---

## Let's Peek at the Trait

Notes:
just a breeze over the trait.

---

## Tuple of Signed Extension

- Is also a signed extension itself!

> Look at the tuple implementation.

- Main takeaways:
  - `type AdditionalSigned = (SE1::AdditionalSigned, SE2::AdditionalSigned)`,
  - `type Pre = (SE1::Pre, SE2::Pre)`,
  - all of hooks:
    - Executes each individually, combines results

Notes:

TODO: we need a lecture in module 4 around `impl_for_tuples`.
TODO: how `TransactionValidity` is `combined_with` is super important here, but probably something
to cover more in 4.3 and recap here.

---

## Usage In The Runtime

- Each runtime has a bunch of signed extensions. They can be grouped as a tuple

```rust
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>,
);
```

---v

> Recall that a tuple of `SignedExtension` is itself a `SignedExtension`.

> We will get to this later as well, but recall that SignedExtensions are not a _FRAME/Pallet_
> concept per se. FRAME just implements them. This also implies that everything regarding signed
> extensions is applied to **all transactions**, throughout the runtime.

- This type is passed to the `UncheckedExtrinsic` and `CheckedExtrinsic`. Let's go into each and see
  what each do with these extensions!

---

## `UncheckedExtrinsic`: Decoding.

- The first important detail is that when `UncheckedExtrinsic` is being decoded, if it has a
  signature, then it decodes the signed extension as well.

  For example:

  ```rust
  struct Foo(u32, u32);
  impl SignedExtension for Foo { .. }
  ```

  implies that two `u32`s are added to the end of the signature field of **all transactions**.

---

## `UncheckedExtrinsic`: Checking.

- Taken from `fn check`:

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

## `UncheckedExtrinsic`

- in `fn validate` of `Applyable`, we use `validate` of signed extensions
  - Also for unsigned
  - Historical: `ValidateUnsigned` and `SignedExtension` have an overlap, someday the former
    might become deprecated. TODO: find the github issue about this.
- in `fn apply` of `Applyable`, we
  - `pre_dispatch` or `pre_dispatch_unsigned` accordingly.
  - `post_dispatch` accordingly
    - TODO: `type Pre` and passing data to post-dispatch needs its own section.

---

## Recap: Transaction Queue Validation

- `fn validate` of `Applyable` is actually used in --you guessed it-- the Executive module for transaction validation.

> walk over the `fn validate_transaction` in executive, which is used directly in the runtime API.

- See how it is doing the minimum amount of validation, NOT dispatching anything.

> Transaction queue is not part of the consensus system. Validation of transaction are _free_. Doing
> too much work in validation of transactions is essentially opening a door to be DOS-ed.

---

## Putting It All Together: `ChargeTransactionPayment`

- Let's now walk over this

---

## Notable Signed Extensions

- These are some of the default signed extensions that come in FRAME
- See if you can predict how they are made!

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
- discovered by XLC
- TODO: link to examples being signed with this, and the first one.

- uses `pre_dispatch` and `validate` to ensure the signing account is not `0x00`.

---v

### `check_nonce`

- `pre_dispatch`: check nonce and actually update it.
- `validate`: check the nonce, DO NOT WRITE ANYTHING, set `provides` and `requires`.

  - remember that:
    - `validate` is only for lightweight checks, no read/write.
    - anything you write to storage is reverted anyhow. TODO: fact check this statement

---v

### `check_weight`

- Check there is enough weight in `validate`.
- Check there is enough weight, and update the consumed weight in `pre_dispatch`.
- Updated consumed weight in `post_dispatch`.

---

## Big Picture: Pipeline of Extension

- Signed extensions (or at least the `pre_dispatch` and `validate` part) remind me of the extension
  system of `express.js`, if any of you know what that is

TODO: figure

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
