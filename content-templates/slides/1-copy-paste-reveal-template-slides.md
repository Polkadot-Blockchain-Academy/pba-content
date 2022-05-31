---
title: Copy and Paste Slide Templates # Also update the h1 header on the first slide to the same name
description: Use the `---` delineated slides here in your content!
# PBA has a theme: "reveal-md/PBA-theme.css", alternatively, you can use a named default like "night" from this list: https://github.com/hakimel/reveal.js/tree/master/css/theme/source
theme: "reveal-md/PBA-theme.css"
# Add custom css files for your slides here, comma separated:
css: ["reveal-md/custom-classes.css"]
separator: "\r?\n---\r?\n"
verticalSeparator: "\r?\n---v\r?\n"
# Below can be any of these: https://revealjs.com/config/
revealOptions:
    transition: "slide" # animation between slides = none/fade/slide/convex/concave/zoom
	backgroundTransition: "fade" # background swap between slides = none/fade/slide/convex/concave/zoom
	slideNumber: true
	controls: true
	progress: true
---

# Copy and Paste Slide Templates

### _Use the `---` delineated slides here in your content!_

---

<div class="flex-container">
<div class="left text-right"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

<!-- TODO: add a good circularly cropped head-shot of ou to the `assets/profile` folder  -->
<img style="width: 550px; float:right; margin-right:30px" src="../../assets/img/shared/profile.png"/>

</div>
<div style="margin-top:130px" class="right text-left"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Instructor Name
<!-- .element: style="margin-bottom: -30px;" -->

#### _Position or Title_
<!-- .element: style="margin-left: 20px;" -->

- I am a Subject matter in X
- A bit about me


_[Twitter](https://twitter.com) // [LinkedIn](https://linkedin.com) // [Email](mailto:)_

</div>
</div>

---

### _At the end of this lecture, you will be able to:_

- Describe ... <!-- TODO: fill this in  -->
- Navigate ... <!-- TODO: fill this in  -->
- Justify ... <!-- TODO: fill this in  -->

---

## Here is a topic

- Use some bullets
- To make a few points

Notes:
Speaker view ONLY notes

---

## Here is an important point

#### _Make it clear_

Notes:
Stuff you should remember to say

---

## Pictures

<!-- set height*width in px, where full screen is 1920*1080 -->
<img style="width: 800px" src="../../assets/img/shared/Landscape_mountain.jpg" alt="Some Pic">

#### _Leave a note on why this one matters_

---

## Code Highlight & Transitions

Syntax for many langs is possible, and very easy to style.
You can _and should_ use highlighting of lines in a large snippets of code.

You an also add comments to make "fragments" for specific components

<!-- .element: class="fragment" data-fragment-index="1" -->

_They can ordered how you see fit!_

<!-- .element: class="fragment" data-fragment-index="3" -->

See the source for syntax

<!-- .element: class="fragment" data-fragment-index="2" -->

---

## Rust Example

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

---

## Columns

<div class="flex-container">
<div class="left centered"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Left side (centered)

- Some really important things to say
- Lots of great stuff
- Points here too

center justified text

This is the default for all but bullets

</div>
<div class="right"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Right side (non-centered)

- some
- more
- points
- you
- make

Left justified text with line brake <br>
use `<br>` <br>
(MarkDown comment below)
<!-- .element: style="text-align: left;" -->

</div>
</div>

---

<div class="flex-container">
<div class="left"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Colum + Code

- Some
- Observations
- Others

</div>

<!-- Put no content here -->

<div class="right">
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
</div>

---

## Tables

| Tables              |         Are         |                    Cool |
| ------------------- | :-----------------: | ----------------------: |
| col 1 is            |    left-aligned     |                   $1600 |
| col 2 is            |      centered       |                     $12 |
| col 3 is            |    right-aligned    |                      $1 |
| This row sure has a | _lot_ of text so it | spaces the columns outs |

---

## Math

KaTeX/LaTeX rendered within blocks with "`$$`" delimiters

$$J(\theta_0,\theta_1) = \sum_{i=0}$$

In line uses "`\\(`" and "`\\)`" to render: \\(\alpha\beta\gamma\\)
.

More info: https://revealjs.com/math/

---

# More help needed?

_Please reach out to the academy content & docs team on element for support!_
