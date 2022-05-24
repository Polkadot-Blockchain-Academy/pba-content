---
title: Lesson Slide Template # Also update the h1 header on the first slide to the same name
description: Describe your slides here
# PBA has a theme: "reveal-md/PBA-theme.css", alternatively, you can use a named default like "night" from this list: https://github.com/hakimel/reveal.js/tree/master/css/theme/source
theme: "reveal-md/PBA-theme.css"
# Add custom css files for your slides here, comma separated:
css: ["reveal-md/custom-classes.css"]
separator: "\r?\n---\r?\n"
verticalSeparator: "\r?\n---\r?\n"
# Below can be any of these: https://revealjs.com/config/
revealOptions:
    transition: "slide" # animation between slides = none/fade/slide/convex/concave/zoom
	backgroundTransition: "fade" # background swap between slides = none/fade/slide/convex/concave/zoom
	slideNumber: true
	controls: true
	progress: true
---

# Template Slides

_{Delete this slide}_

The following slides are for use in your lessons.

---

# Lesson Title 

## Lecture 1, Module 4

### Instructors:  <!-- TODO: fill this in  -->

<hr>

***At the end of this lecture, you will be able to:***

- Describe ... <!-- TODO: fill this in  -->
- Navigate ... <!-- TODO: fill this in  -->
- Justify ... <!-- TODO: fill this in  -->

---

## Meet your Instructor

<br>

<div class="left text-right">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

<img style="width: 40%" src="../assets/img/place-holder/profile.png"/>

</div>

<!-- Put no content here -->

<div style="margin-top:20px" class="right text-left">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Instructor Name

<!-- TODO: fill this in  -->

- A bit about me
- My Day Job
- I am a Subject matter in X
- _[Twitter](https://twitter.com) // [LinkedIn](https://linkedin.com) // [Email](mailto:)_

</div>

---

# Basic text

- Use some bullets
- To make a few points

Notes:
Speaker view ONLY notes

---

## Pictures

<img style="width: 50vw" src="../assets/img/place-holder/Landscape_mountain.jpg" alt="Some Pic">

---

## Columns

<div class="left text-center">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Left side

- some
- points
- you
- make

</div>

<!-- Put no content here -->

<div class="right text-center">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Right side

- more
- stuff
- here

</div>

---

<div class="left text-center">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Colum + Code

- Some
- Observations
- Others

</div>

<!-- Put no content here -->

<div class="right text-center">
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

KaTeX/LaTeX rendered within blocks with `$$` delimiters

$$
J(\theta_0,\theta_1) = \sum_{i=0}
$$

More info: https://revealjs.com/math/

---

## Code Highlight & Transitions

Syntax for many langs is possible, and very easy to style. You can _and should_ use highlighting of
lines in a large snippets of code.

_Examples are down from here in the slides_

<!-- .element: class="fragment" -->

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

<br>

Including line highlights

---

# More help needed?

_Please reach out to the academy content & docs team on element for support!_
