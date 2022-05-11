# Instructions for Template

These are made with `reveal-md`.

See [the source](https://github.com/NukeManDan/pba-template-reveal-md/blob/main/SLIDES/instructions-how-to-use.md) for syntax.

---

## What are we going to see:

-   How to use this Template
    - Speaker Notes & Viewer
-   Code Highlight & Transitions

---

## How to use this Template

Press the `down/up` keys to navigate _vertical_ slides

> Press `Esc` or `o` to see an `overview` view that your arrow keys can navigate in, click one to open
at this slide.

_Try doing down a slide._
<!-- .element: class="fragment" data-fragment-index="2" -->

----

## Speaker Notes & Viewer

> Press the `s` key to bring up a popup window with speaker view

_You need to unblock popups to have the window open_

Note: This is a note just for you. Set under a line in your slide starting with "`Note`:" all
subsequent lines are just seen in speaker view.

---

## Code Highlight & Transitions

Syntax for many langs is possible, and very easy to style.
You can _and should_ use highlighting of lines in a large snippets of code.

_Examples are down from here in the slides_
<!-- .element: class="fragment" data-fragment-index="2" -->

----

## Rust Example

Example: a runtime `lib.rs` snippet

<pre><code style="font-size: 0.5em !important" data-trim data-noescape data-line-numbers="0|1|4|5|10-12" class="rust">
#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod weights;
pub mod xcm_config;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;
</pre></code>
<!-- .element: class="fragment" data-fragment-index="2" -->

This text appears after the code highlight views.

<!-- .element: class="fragment" data-fragment-index="3" -->

Note: Eurk. On va vite fais essayer de comprendre. WTF!

----

## Some other code blocks...

```typescript
function async getUser(id: string): Promise<User> {
  const user = await db.find({ user: id });
  await db.close();
  return user;
}
```

-   text after code is up
<!-- .element: class="fragment" data-fragment-index="2" -->

----

## Some other code blocks...

```javascript
let numbers = [1, 2, 3, 4, 5];
const firstThreeNumbers = numbers.splice(0, 3);
const lastThreeNumbers = numbers.splice(2, 5);
```

```javascript
let numbers = [1, 2, 3, 4, 5]; // 1,2,3,4,5

const firstThreeNumbers = numbers.splice(0, 3);
// firstThreeNumbers = 1,2,3
// AND numbers = 4,5

const lastThreeNumbers = numbers.splice(2, 5);
// lastThreeNumbers = undefined
```

<!-- .element: class="fragment" data-fragment-index="2" -->

-   text fragments can transition in, out-of-order on the slide.
<!-- .element: class="fragment" data-fragment-index="4" -->

```javascript
let numbers = [1, 2, 3, 4, 5]; // 1,2,3,4,5
```

<!-- .element: class="fragment" data-fragment-index="3" -->

---

# More help needed?

> Please reach out to the academy content & docs team on element for support!
