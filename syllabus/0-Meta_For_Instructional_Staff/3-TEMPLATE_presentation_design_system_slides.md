---
title: Design System
description: A design system to use within slides of a presentation
---

# Design System

### _For the polkadot academy slides._

---

## Design system examples

Use `o` to open the overview mode and explore slides here.

You can see the source of these slides to copy&paste<br>
as slide templates in your slides!

<br>

<pba-flex center>

1. Columns
1. Images
1. MarkDown examples

</pba-flex>

---

## Columns

<pba-cols>
<pba-col center>

### Center 1

Using<br>`<pba-col center>`

</pba-col>
<pba-col center>

### Center 2

Using<br>`<pba-col center>`

</pba-col>
</pba-cols>

---v

<!-- prettier-ignore -->
```html
<pba-cols>
<pba-col center>

### Center 1

Using<br>`<pba-col center>`

</pba-col>
<pba-col center>

### Center 2

Using<br>`<pba-col center>`

</pba-col>
</pba-cols>
```

---

## Columns

<pba-cols>
<pba-col left>

### Left

Using<br>`<pba-col left>`

</pba-col>
<pba-col center>

### Center

Using<br>`<pba-col center>`

</pba-col>
<pba-col right>

### Right

Using<br>`<pba-col right>`

</pba-col>
</pba-cols>

**_TODO: fix spacing here, use with caution!_**

---v

<!-- prettier-ignore -->
```html
## Columns

<pba-cols>
<pba-col left>

### Left

Using<br>`<pba-col left>`

</pba-col>
<pba-col center>

### Center

Using<br>`<pba-col center>`

</pba-col>
<pba-col right>

### Right

Using<br>`<pba-col right>`

</pba-col>
</pba-cols>
```

---

<pba-cols>
<pba-col>

### This column has a bit of a statement to make.

</pba-col>
<pba-col>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</pba-col>
</pba-cols>

---v

<!-- prettier-ignore -->
```html
<pba-cols>
<pba-col>

### This column has a bit of a statement to make.

</pba-col>
<pba-col>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</pba-col>
</pba-cols>
```

---

## Images

<img style="width: 50vw" src="../../assets/img/4-Substrate/WebAssembly.png"/>

---v

<!-- prettier-ignore -->
```html
<img style="width: 50vw" src="../../assets/img/4-Substrate/WebAssembly.png"/>
```

---

<!-- .slide: data-background="../../assets/img/0-Shared/bg-circles-image-transparent.png" -->
More info on reveal/reveal-md backgrounds:

- https://revealjs.com/backgrounds/
- https://www.npmjs.com/package/reveal-md custom styles attributes

---v

<!-- prettier-ignore -->
```html
<!-- .slide: data-background="../../assets/img/0-Shared/bg-circles-image-transparent.png" -->

More info on reveal/reveal-md backgrounds:

- https://revealjs.com/backgrounds/
- https://www.npmjs.com/package/reveal-md custom styles attributes
```

---v

<!-- .slide: data-background="../../assets/img/0-Shared/bg-circles-color-transparent.png" -->

<!-- prettier-ignore -->
```html
<!-- .slide: data-background="../../assets/img/0-Shared/bg-circles-color-transparent.png" -->

<pba-cols padded>
<pba-col color>

### Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.

</pba-col>
<pba-col contrast>

- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Ut enim ad minim veniam, quis nostrud exercitation
- Duis aute irure dolor in reprehenderit in
- Excepteur sint occaecat cupidatat non proident, sunt in

</pba-col>
</pba-cols>
```

---

<!-- .slide: data-background-color="#8D3AED" -->

# Section title

---v

<!-- .slide: data-background-color="#8D3AED" -->

<!-- prettier-ignore -->
```html
<!-- .slide: data-background-color="#8D3AED" -->

# Section title
```

---

> A quote of myself, saying great stuff, as always.

Source: me™ at the last event

---

Testing **bold** and _italic_ markdown texts!

<!-- prettier-ignore -->
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

<!-- prettier-ignore -->
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

<pba-cols>
<pba-col>

### Column + Code

- Some
- Observations
- Others

</pba-col>
<pba-col>

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

</pba-col>
</pba-cols>

---v

<!-- prettier-ignore -->
```html
<pba-cols>
<pba-col>

### Column + Code

- Some
- Observations
- Others

</pba-col>
<pba-col>


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

</pba-col>
</pba-cols>
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
