---
title: ink! Workshop (Second Day)
description: An continuation of the ink! workshop.
duration: 20 min
---

<img rounded style="width: 1400px; padding-top:15px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/advanced-workshop.jpg" alt="ink!" />

---

<pba-cols>
<pba-col center>

### Now

We help you debug!

</pba-col>
<pba-col center>

### Then

🕹️🎮🕹️🎮

</pba-col>
<pba-col center>

### Thereafter

Solutions Explainer

</pba-col>
</pba-cols>

<br/>
<br/>

- Use as little gas as possible to paint as many fields as possible.
- Stay within your gas budget.
- The later you manage to still paint a field, the better you score.

<br/>
<br/>

[paritytech/squink-splash-advanced](https://github.com/paritytech/squink-splash-advanced)

---

## Frontend

[https://splash.use.ink](https://splash.use.ink)

---

## Questions

- What strategy did the winner choose?

<!-- .element: class="fragment" -->

- What strategies did the others choose?

<!-- .element: class="fragment" -->

- What do you think would be the perfect strategy?

<!-- .element: class="fragment" -->

---

## Board Dimensions

- Best:
  - `const width: u32` or
  - `new(width: u32, height: u32)`
- Worst: Cross-contract call

---

## More Pitfalls

* Forgetting `--release`

---

## Strategy 1<br/>Return Random Numbers

<img rounded style="margin-top: 25px; width: 500px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/0.png" />

---

## Strategy 1<br/>Return Random Number

- Wasm-compatible RNG
<!-- .element: class="fragment" -->
- Use Storage to hold seed for random number
<!-- .element: class="fragment" -->
- \+ Uses little Gas
<!-- .element: class="fragment" -->
- \- Quickly runs into collisions
<!-- .element: class="fragment" -->
- \- Score function rewards players that late in game still paint fields
<!-- .element: class="fragment" -->

---

## Strategy 2<br/>Paint only free fields

<img rounded style="margin-top: 25px; width: 500px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/1.png" />

---

## Strategy 2<br/>Paint only free fields

- Query board for free fields,
- Cross-contract call
- Need to iterate over `Mapping`: `O(n)`
- Expensive

---

## Strategy 3<br/>Shift computation off-chain

<img rounded style="margin-top: 25px; width: 500px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/2.png" />

---

## Strategy 3<br/>Shift computation off-chain

- Off-chain Script
  - Query board
  - Search free field
  - `fn set_next_turn(Option<(u32, u32)>)`

<!-- .element: class="fragment" -->

- ```rust
  #[ink(message, selector = 0)]
  pub fn your_turn(&mut self) -> Option<(u32, u32)> {
    self.next_turn
  }
  ```

<!-- .element: class="fragment"  -->

---

## Strategy 4<br/>Exploit player sorting in game loop

<img rounded style="margin-top: 25px; width: 500px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/3.png" />

---

## Strategy 4<br/>Exploit player sorting in game loop

- On top of Strategy 3 (off-chain computation).
<!-- .element: class="fragment"  -->

- Game loop calls players in same order each time.
<!-- .element: class="fragment"  -->

```rust
// fn submit_turn()

for (idx, player) in players.iter_mut().enumerate() {
    …
}
```
<!-- .element: class="fragment"  -->

---

## Strategy 5<br/>Checking these slides already yesterday

<img rounded style="margin-top: 25px; width: 500px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/4.png" />

---

<img rounded style="width: 1100px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/Questions_2.svg" />
