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

<blockquote style="text-align: left; font-size: 0.9em;">
Use as little gas as possible to paint as many fields as possible.<br/><br/>
Stay within your gas budget.<br/><br/>
The later you manage to still paint a field, the better you score.<br/><br/>
No overpainting! First player to paint a field owns it.<br/>
</blockquote>

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

- Worst 😱
  - Cross-contract call to `game`<br/><br/>
```rust
#[ink(message)]
pub fn dimensions(&self) -> (u32, u32)
```
<br/><br/>

<!-- .element: class="fragment" -->

- Best 👍️
  - `const width: u32`
  - `new(width: u32, height: u32)`

<!-- .element: class="fragment" -->

---

## More Pitfalls

<img rounded style="margin-top: 25px; width: 400px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/oopsie.gif" />

- Forgetting `--release`

<!-- .element: class="fragment" -->

- Iterating over a datastructure in your contract

<!-- .element: class="fragment" -->

---

## Avoid iteration

<pba-cols>
<pba-col center>

```
#[ink(message)]
fn pay_winner()
  let winner = self.players.find(…);

  self.transfer(winner, …);
}
```
</pba-col>
<!-- .element: class="fragment" -->
<pba-col center>

```rust
#[ink(message)]
fn pay_winner(
    winner: AccountId
) {
  assert!(is_winner(winner));

  self.transfer(winner, …);
}
```

</pba-col>
<!-- .element: class="fragment" -->
</pba-cols>
---

## Strategy 1<br/>Return Random Numbers

<img rounded style="margin-top: 25px; width: 500px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/0.png" />

---

## Strategy 1<br/>Return Random Number

- Wasm-compatible RNG
<!-- .element: class="fragment" -->
- Use Storage to hold seed for random number
<!-- .element: class="fragment" -->
- 📈 Uses little Gas
<!-- .element: class="fragment" -->
- 📉 Quickly runs into collisions
<!-- .element: class="fragment" -->
- 📉 Score function rewards players that late in game still paint fields
<!-- .element: class="fragment" -->

---

## Strategy 2<br/>Paint only free fields

<img rounded style="margin-top: 25px; width: 500px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/1.png" />

---

## Strategy 2<br/>Paint only free fields

- Query board for free fields
- 📈 Succeeds late in game

<!-- .element: class="fragment" -->

- 📉 Cross-contract call 💰️
- 📉 Need to iterate over `Mapping`: `O(n)`

<!-- .element: class="fragment" -->

---

## Strategy 3<br/>Shift computation off-chain

<img rounded style="margin-top: 25px; width: 500px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/2.png" />

---

## Strategy 3<br/>Shift computation off-chain

- Off-chain Script
  - Query board ➜ Search free field<br/><br/>

<!-- .element: class="fragment" -->

- ```rust[1-2|1-7]
  #[ink(message)]
  fn set_next_turn(turn: …) {}

  #[ink(message, selector = 0)]
  pub fn your_turn(&mut self) -> {
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
#[ink(message)]
fn submit_turn(&mut self) {
    // -- snip --

    for (idx, player) in players.iter_mut().enumerate() {
        …
    }

  // -- snip --
}
```

<!-- .element: class="fragment"  -->

---

## Strategy 4<br/>Exploit player sorting in game loop

```rust
impl<T: Config> AddressGenerator<T> for DefaultAddressGenerator {
	fn generate_address(
		deploying_address: &T::AccountId,
		code_hash: &CodeHash<T>,
		input_data: &[u8],
		salt: &[u8],
	) -> T::AccountId {

    // -- snip --

	}
}
```

➜ All inputs are known
<!-- .element: class="fragment"  -->

➜ Generate low `T::AccountId` with known inputs
<!-- .element: class="fragment"  -->

---

## Strategy 5<br/>Checking these slides already yesterday

<img rounded style="margin-top: 25px; width: 500px;" src="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/4.png" />

---

<!-- .slide: data-background="../../assets/img/6-FRAME/6.5-Smart_Contracts/ink/Questions_2.svg"" -->
