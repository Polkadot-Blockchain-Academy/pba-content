# Instructions for Template

Please first view the [CONTRIBUTING.md](https://github.com/paritytech/polkadot-blockchain-academy/blob/main/CONTRIBUTING.md) guide on how to use these slides.

> See the source for syntax and to copy Markdown for these sides in your lessons!.
> Avalible [on Github](](https://github.com/paritytech/polkadot-blockchain-academy/blob/main/reveal-md/INSTRUCTIONS-HOW-TO-USE-REVEAL-MD-Slides.md))

---

## What are we going to see:

<div class="small-text">

- How to use Reveal.js Features
  - Useful `reveal.js` tips
  - Speaker Notes 

- Template Slides
  - Columns
  - Tables
  - Images

- Code Highlight & Transitions
  - Rust Examples

</div>
---

## How to use Reveal.js Features 

> Press the `down/up` keys to navigate _vertical_ slides

_Try doing down a slide._
<!-- .element: class="fragment" -->

----

### Use the keybindings!

<div class="small-text">

- **Overview mode**: “O” to see a birds-eye view of your presentation, “ESC” to return to the highlighted slide (you can quickly navigate with arrows)

- **Fullscreen**: “F”, “ESC” to exit fullscreen mode

- **Speaker mode**: “S” it synchronizes 2 windows: one with the presentation, and another with a timer and all speaker notes!

- **Zoom-in**: ALT+click make the view zoom at the position of your mouse’s pointer; very useful to look closely at a picture or chart surrounded by too much bullet points.

</div>

----

## Speaker Notes & Viewer

> Press the `s` key to bring up a popup window with speaker view

_You need to unblock popups to have the window open_

Note:
This is a note just for you. Set under a line in your slide starting with "`Note`:" all
subsequent lines are just seen in speaker view.

---

# Template slides

The following slides are for use in your lessons.

_These are **vertically** below this slide_
<!-- .element: class="fragment" -->

----

## Columns

<div class="left text-center">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

Left
- some
- points
- you 
- make

</div>

<!-- Put no content here -->

<div class="right text-center">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

Right
- more
- stuff
- here

</div>

----

### Meet your Instructor

<div class="left text-center">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

![Profile Pic](assets/profile.png)

</div>

<!-- Put no content here -->

<div class="right text-center">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

<br>

### Instructor Name

- A bit about me
- more
- last

</div>

----

<div class="left text-center">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Colum + Code

- Some
- Observations
- Others

</div>

<!-- Put no content here -->

<div class="right text-center fill">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

```rust [0|1,13|4-8]
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');
}
```

</div>

----

## Tables

| Tables   |      Are      |  Cool |
|----------|:-------------:|------:|
| col 1 is |  left-aligned | $1600 |
| col 2 is |    centered   |   $12 |
| col 3 is | right-aligned |    $1 |

----

## Pictures

![Landscape](assets/Landscape_mountain.jpg)

---

## Code Highlight & Transitions

Syntax for many langs is possible, and very easy to style.
You can _and should_ use highlighting of lines in a large snippets of code.

_Examples are down from here in the slides_
<!-- .element: class="fragment" -->

----

<div class="small-text">Rust Example<div>


<div class="fill-vertical">

```rust [0|1,6|15-25|30-31]
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
```

<div>

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
