---
title: Copy and Paste Slide Templates # Also update the h1 header on the first slide to the same name
description: A sentence for what these slides are about.
duration: 15 minuets
# PBA has a theme: "reveal-md/PBA-theme.css", alternatively, you can use a named default like "night" from this list: https://github.com/hakimel/reveal.js/tree/master/css/theme/source
# Add custom css files for your slides here, comma separated:
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
<img style="width: 800px" src="../../assets/img/0-Shared/placeholder/Landscape_mountain.jpg" alt="Some Pic" />

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

// Make the Wasm binary available.
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

## Design system examples

Use `o` to open the overview mode and explore slides here.

You can see the source of these slides to copy&paste<br/>
as slide templates in your slides!

<pba-flex center>

1. Columns
1. Images
1. MarkDown examples

</pba-flex>

---

## Two Column

<pba-cols>
<pba-col center>

### Center 1

Using<br/>`<pba-col center>`

</pba-col>
<pba-col center>

### Center 2

Using<br/>`<pba-col center>`

</pba-col>
</pba-cols>

---v

## Two Column

<!-- prettier-ignore -->
```html
<pba-cols>
<pba-col center>

### Center 1

Using<br/>`<pba-col center>`

</pba-col>
<pba-col center>

### Center 2

Using<br/>`<pba-col center>`

</pba-col>
</pba-cols>
```

---

## Three Columns

<pba-cols>
<pba-col left>

### Left

Using<br/>`<pba-col left>`

</pba-col>
<pba-col center>

### Center

Using<br/>`<pba-col center>`

</pba-col>
<pba-col right>

### Right

Using<br/>`<pba-col right>`

</pba-col>
</pba-cols>

---v

## Three Columns

<!-- prettier-ignore -->
```html
<pba-cols>
<pba-col left>

### Left

Using<br/>`<pba-col left>`

</pba-col>
<pba-col center>

### Center

Using<br/>`<pba-col center>`

</pba-col>
<pba-col right>

### Right

Using<br/>`<pba-col right>`

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

<img style="width: 50vw" src="../../assets/img/4-Substrate/WebAssembly.png" />

---v

<!-- prettier-ignore -->
```html
<img style="width: 50vw" src="../../assets/img/4-Substrate/WebAssembly.png" />
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

<!-- .slide: data-background-color="#4A2439" -->

# Section title

---v

<!-- .slide: data-background-color="#4A2439" -->

<!-- prettier-ignore -->
```html
<!-- .slide: data-background-color="#4A2439" -->

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

// Make the Wasm binary available.
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

// Make the Wasm binary available.
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

# How to use Mermaid Diagrams

[Mermaid](https://mermaid.js.org) lets you create diagrams and visualizations using text and code.

It is a JavaScript based diagramming and charting tool that renders Markdown-inspired text definitions to create and modify diagrams dynamically.

First of all lets see some examples of diagrams that Mermaid can show with its integration with revealJS;

---v

### [A Flowchart](https://mermaid.js.org/syntax/flowchart.html)

  <div class="mermaid">
    <pre>
      %%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
      flowchart TD
        A(Start) --> B{Is it?};
        B -- Yes --> C(OK);
        C --> D(Rethink);
        D --> B;
        B -- No ----> E(End);
      </pre>
  </div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
      %%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
      flowchart TD
        A(Start) --> B{Is it?};
        B -- Yes --> C(OK);
        C --> D(Rethink);
        D --> B;
        B -- No ----> E(End);
  </pre>
</div>
```

---v

### Entity relationship diagram

<div class="mermaid">
  <pre>
    erDiagram
    Node ||--o{ Wallet : places_order
    Wallet ||--|{ Account : owner
    Node }|..|{ Some-IP : uses
  </pre>
</div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
    erDiagram
    Node ||--o{ Wallet : places_order
    Wallet ||--|{ Account : owner
    Node }|..|{ Some-IP : uses
  </pre>
</div>
```

---v

### Sequence diagram

<div class="mermaid">
  <pre>
  sequenceDiagram
      Alice->>John: Hello John, how are you?
      John-->>Alice: Great!
      Alice-)John: See you later!
  </pre>
</div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
  sequenceDiagram
      Alice->>John: Hello John, how are you?
      John-->>Alice: Great!
      Alice-)John: See you later!
  </pre>
</div>
```

---v

### Class Diagram

<div class="mermaid">
  <pre>
    classDiagram
      note "From Duck till Zebra"
      Animal <|-- Duck
      note for Duck "can fly\ncan swim\ncan dive\ncan help in debugging"
      Animal <|-- Fish
      Animal <|-- Zebra
      Animal : +int age
      Animal : +String gender
      Animal: +isMammal()
      Animal: +mate()
      class Duck{
          +String beakColor
          +swim()
          +quack()
      }
      class Fish{
          -int sizeInFeet
          -canEat()
      }
      class Zebra{
          +bool is_wild
          +run()
      }
  </pre>
</div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
    classDiagram
      note "From Duck till Zebra"
      Animal <|-- Duck
      note for Duck "can fly\ncan swim\ncan dive\ncan help in debugging"
      Animal <|-- Fish
      Animal <|-- Zebra
      Animal : +int age
      Animal : +String gender
      Animal: +isMammal()
      Animal: +mate()
      class Duck{
          +String beakColor
          +swim()
          +quack()
      }
      class Fish{
          -int sizeInFeet
          -canEat()
      }
      class Zebra{
          +bool is_wild
          +run()
      }
  </pre>
</div>
```

---v

### State diagram (v2)

<div class="mermaid">
  <pre>
  stateDiagram-v2
    [*] --> Still
    Still --> [*]

    Still --> Moving
    Moving --> Still
    Moving --> Crash
    Crash --> [*]

  </pre>
</div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
  stateDiagram-v2
    [*] --> Still
    Still --> [*]

    Still --> Moving
    Moving --> Still
    Moving --> Crash
    Crash --> [*]
  </pre>
</div>
```

---v

### User Journey

<div class="mermaid">
  <pre>
  journey
    title My working day
    section Go to work
      Make tea: 5: Me
      Go upstairs: 3: Me
      Do work: 1: Me, Cat
    section Go home
      Go downstairs: 5: Me
      Sit down: 5: Me
  </pre>
</div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
    journey
    title My working day
    section Go to work
      Make tea: 5: Me
      Go upstairs: 3: Me
      Do work: 1: Me, Cat
    section Go home
      Go downstairs: 5: Me
      Sit down: 5: Me
  </pre>
</div>
```

---v

### Gantt

<div class="mermaid">
  <pre>
    gantt
      apple :a, 2017-07-20, 1w
      banana :crit, b, 2017-07-23, 1d
      cherry :active, c, after b a, 1d
  </pre>
</div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
    gantt
      apple :a, 2017-07-20, 1w
      banana :crit, b, 2017-07-23, 1d
      cherry :active, c, after b a, 1d
  </pre>
</div>
```

---v

### Pie Chart

<div class="mermaid">
  <pre>
    pie title Pets adopted by volunteers
      "Dogs" : 386
      "Cats" : 85
      "Rats" : 15
  </pre>
</div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
    pie title Pets adopted by volunteers
      "Dogs" : 386
      "Cats" : 85
      "Rats" : 15
  </pre>
</div>
```

---v

### Git Graph

<div class="mermaid">
  <pre>
    gitGraph
      commit
      commit
      branch develop
      checkout develop
      commit
      commit
      checkout main
      merge develop
      commit
      commit
  </pre>
</div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
    gitGraph
      commit
      commit
      branch develop
      checkout develop
      commit
      commit
      checkout main
      merge develop
      commit
      commit
  </pre>
</div>
```

---v

### Useful links

- [Mermaid Syntax](https://mermaid.js.org/syntax/flowchart.html)
- [Mermaid Live Editor with examples](https://mermaid.live/)

---

# More help needed?

_Please reach out to the academy content & docs team on element for support!_
