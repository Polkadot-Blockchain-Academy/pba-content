---
title: Design System
description: A design system to use within slides of a presentation
---

# Design System

### _For the polkadot academy slides._

<widget-speaker small name="Speaker Name Surname" position="Position/Department" image="/assets/img/0-Shared/placeholder/profile.png"></widget-speaker>

---v

```html
# Design system For the polkadot academy slides.

<widget-speaker
  small
  name="Speaker Name Surname"
  position="Position/Department"
  image="/assets/img/0-Shared/placeholder/profile.png"
></widget-speaker>
```

Note that in `.md` files, you have to keep the widget element all in a single line to work

---

## Design system examples

Use `o` to open the overview mode and explore slides here.

You can see the source of these slides to copy&paste<br>
as slide templates in your slides!

<br>

<widget-text center>

1. Images
   - full screen backgrounds
1. Columns
   - split screen backgrounds 
   - images in columns
1. MarkDown examples
   - code `fragment` transitions

</widget-text>

---

# Mosasaurus

---

<widget-text>

Mosasaurus is a genus of mosasaurs, an extinct group of aquatic scaly reptiles.

It lived from about 82 to 66 million years ago during the Late Cretaceous. Its earliest fossils were found as skulls near the River Meuse (Mosa in Latin).

</widget-text>

---v

```html
<widget-text>

Mosasaurus is a genus of mosasaurs, an extinct group of aquatic scaly reptiles.

It lived from about 82 to 66 million years ago during the Late Cretaceous. Its earliest fossils were found as skulls near the River Meuse (Mosa in Latin).

</widget-text>
```

---

<widget-text color>

Mosasaurus is a genus of mosasaurs, an extinct group of aquatic scaly reptiles.

It lived from about 82 to 66 million years ago during the Late Cretaceous.
Its earliest fossils were found as skulls near the River Meuse (Mosa in Latin).

</widget-text>

---v

```html
<widget-text color>

Mosasaurus is a genus of mosasaurs, an extinct group of aquatic scaly reptiles.

It lived from about 82 to 66 million years ago during the Late Cretaceous.
Its earliest fossils were found as skulls near the River Meuse (Mosa in Latin).

</widget-text>
```

---

<widget-speaker name="Gavin Wood" position="Founder Parity & Web3 Foundation" image="/assets/img/0-Shared/people/gav.png" github="gavofyork" twitter="gavofyork" linkedin="gavin-wood-88843316" matrix="gav:matrix.parity.io"></widget-speaker>

---v

```html
<widget-speaker
  name="Speaker Name Surname"
  position="Position/Department"
  image="/assets/img/0-Shared/people/gav.png"
  github="gavofyork"
  twitter="gavofyork"
  linkedin="gavin-wood"
  telegram="PolkadotOfficial"
></widget-speaker>
```

Note that in `.md` files, you have to keep the widget element all in a single line to work

---

## We believe in a decentralized and fair internet where users control their own data, identity and destiny.

---

<widget-columns>
<widget-column>

### This column has a bit of a statement to make.

</widget-column>
<widget-column>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</widget-column>
</widget-columns>

---v

```html
<widget-columns>
<widget-column> 

### Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.

</widget-column>
<widget-column>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit - Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in - Excepteur sint occaecat cupidatat non proident, sunt in

</widget-column>
</widget-columns>
```

---v

<widget-columns>
<widget-column color>

### Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.

</widget-column>
<widget-column>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</widget-column>
</widget-columns>

---v

```html
<widget-columns>
<widget-column color>

### Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.

</widget-column>
<widget-column>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</widget-column>
</widget-columns>
```

---

<widget-image src="/assets/img/4-Substrate/WebAssembly.png"></widget-image>

---v

```html
<widget-image src="/assets/img/4-Substrate/WebAssembly.png"></widget-image>
```

---

<widget-image fullscreen src="/assets/img/0-Shared/dynamic-wang-bg-2.png"></widget-image>

---v

```html
<widget-image fullscreen src="/assets/img/0-Shared/dynamic-wang-bg-2.png"></widget-image>
```

---
<!-- .slide: data-background="/assets/img/0-Shared/bg-circles-image.png" -->
<widget-text>

More info on reveal/reveal-md backgrounds:

- https://revealjs.com/backgrounds/
- https://www.npmjs.com/package/reveal-md custom styles attributes

</widget-text>

---

<widget-columns>
<widget-column></widget-column>
<widget-column>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
- Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
- Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
- Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum

</widget-column>
</widget-columns>

---v

```html
<widget-columns>
<widget-column></widget-column>
<widget-column>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
- Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
- Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
- Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum

</widget-column>
</widget-columns>
```

---
<!-- .slide: data-background="/assets/img/0-Shared/bg-circles-image-transparent.png" -->

<widget-columns padded>
<widget-column>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</widget-column>
</widget-columns>

---v

```html
<!-- .slide: data-background="/assets/img/0-Shared/bg-circles-image-transparent.png" -->

<widget-columns padded>
<widget-column>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</widget-column>
</widget-columns>
```

---
<!-- .slide: data-background="/assets/img/0-Shared/bg-circles-color-transparent.png" -->

<widget-columns padded>
<widget-column color>

### Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.

</widget-column>
<widget-column contrast>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</widget-column>
</widget-columns>

---v

```html
<!-- .slide: data-background="/assets/img/0-Shared/bg-circles-color-transparent.png" -->

<widget-columns padded>
<widget-column color>

### Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.

</widget-column>
<widget-column contrast>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</widget-column>
</widget-columns>
```

---

<!-- .slide: data-background-color="#8D3AED" -->

# Section title

---v

<!-- .slide: data-background-color="#8D3AED" -->

```html
<!-- .slide: data-background-color="#8D3AED" -->
# Section title
```

---

## Slide Title

- Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
- Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
- Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
- Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum

---

> A quote of myself, saying great stuff, as always.

Source: me™ at the last event

---

Testing **bold** and _italic_ markdown texts!

```html
Testing **bold** and *italic* markdown texts!
```

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

---v

~~~

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

~~~

---

<widget-columns>
<widget-column>

### Column + Code

- Some
- Observations
- Others

</widget-column>
<widget-column>

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

</widget-column>
</widget-columns>

---v
````html
<widget-columns>
<widget-column>

### Column + Code

- Some
- Observations
- Others

</widget-column>
<widget-column>


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

</widget-column>
</widget-columns>
```` 

---

## Tables

| Tables              |         Are         |                    Cool |
| ------------------- | :-----------------: | ----------------------: |
| col 1 is            |    left-aligned     |                   $1600 |
| col 2 is            |      centered       |                     $12 |
| col 3 is            |    right-aligned    |                      $1 |
| This row sure has a | _lot_ of text so it | spaces the columns outs |

---v

```
| Tables              |         Are         |                    Cool |
| ------------------- | :-----------------: | ----------------------: |
| col 1 is            |    left-aligned     |                   $1600 |
| col 2 is            |      centered       |                     $12 |
| col 3 is            |    right-aligned    |                      $1 |
| This row sure has a | _lot_ of text so it | spaces the columns outs |
```

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
