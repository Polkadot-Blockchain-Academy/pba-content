## FRAMELess Assignment Retro -- PBA3 UCB

### Statistics

Looking at the test failures statistics, we can learn a great deal about which parts of the
assignment/module where better delivered and better understood, and which parts not. This can also
help you know which lectures you might want to revisit.

The full table of test and failures is as follows:

```
test,failures
tipping::optional::alice_with_100_transfer_10_to_treasury_with_tip_1,66
tipping::optional::alice_with_100_transfer_5_to_treasury_with_tip_5,66
nonce::optional::validate_ready,56
nonce::optional::validate_future,53
tipping::optional::validate_tx_alice_with_zero_tips_zero,51
currency::optional::alice_remarks_then_transfers_all,40
tipping::challenging::bob_with_20_sends_10_to_charlie_and_tip_5,37
tipping::challenging::alice_with_100_transfers_20_to_bob_with_5_tip,36
tipping::challenging::validate_tx_alice_with_100_tips_100,34
tipping::challenging::bob_with_100_stakes_90_and_tip_10,34
tipping::challenging::multi_tip_in_single_block,33
tipping::challenging::bob_with_100_stakes_89_and_tip_5,31
tipping::challenging::alice_with_100_transfers_all_to_bob_with_tip_5,31
tipping::challenging::validate_tx_alice_with_zero_tips_10,29
nonce::fundamentals::apply_future,29
nonce::optional::validate_stale,29
tipping::fundamentals::alice_with_u128_max_div2_tips_u128_max_div4,28
tipping::fundamentals::validate_tx_alice_with_100_tips_95,28
tipping::challenging::alice_with_20_transfers_10_to_bob_and_tips_10,28
nonce::fundamentals::chain_nonce_failures,28
tipping::challenging::bob_with_100_stakes_85_and_tip_10,27
tipping::fundamentals::bob_with_100_stakes_50_and_tips_10,26
tipping::fundamentals::alice_with_100_transfers_all_to_bob_and_tips_10,26
tipping::fundamentals::validate_tx_alice_with_100_tips_105,26
tipping::fundamentals::alice_with_100_transfers_20_to_bob_with_10_tip,25
tipping::fundamentals::validate_tx_alice_with_100_tips_5,25
tipping::fundamentals::validate_tx_alice_with_100_tips_15,24
nonce::fundamentals::apply_stale,24
nonce::fundamentals::nonce_is_set_after_successful_apply,24
staking::challenging::bob_with_100_stakes_100,24
staking::challenging::bob_with_100_stakes_20_then_transfers_all,23
currency::challenging::alice_mints_9_to_bob,21
currency::challenging::alice_mints_5_to_bob,21
currency::challenging::alice_mints_100_to_bob_bob_transfers_all_to_alice,21
currency::challenging::alice_mints_100_to_bob_bob_transfers_100_to_alice,19
currency::fundamentals::alice_mints_10_to_bob,17
currency::challenging::multiple_mints_in_single_block_success_and_failure,17
staking::challenging::bob_with_100_stakes_95,17
staking::challenging::bob_with_100_stakes_90,17
currency::fundamentals::alice_mints_100_to_bob_bob_transfers_91_to_alice,15
currency::challenging::alice_mints_100_to_bob_bob_transfers_90_to_alice,15
currency::challenging::multiple_mints_in_single_block,15
staking::fundamentals::bob_with_100_stakes_20,15
basics::challenging::apply_sudo_remark_by_bob_fails,14
currency::fundamentals::bob_cannot_mint_to_alice,14
currency::fundamentals::alice_mints_100_to_bob_bob_transfers_20_to_alice,14
staking::fundamentals::bob_with_100_stakes_120,14
tipping::challenging::validate_tx_alice_with_100_tips_zero,14
basics::fundamentals::remark,13
currency::fundamentals::alice_can_mint_to_bob,13
basics::fundamentals::apply_bad_signature_fails,9
basics::fundamentals::apply_unsigned_set_fails,9
basics::fundamentals::validate_bad_signature_fails,9
basics::fundamentals::validate_unsigned,9
basics::fundamentals::empty_block,9
basics::fundamentals::set_value,9
nonce::fundamentals::apply_ready,9
basics::fundamentals::validate_signed_set_value_okay,6
basics::challenging::validate_sudo_remark_by_bob,6
basics::challenging::apply_remark_okay,4
```

As expected, the most frequent failures are optional tests. These are tests that we acknowledged as
"not clear enough", and have no effect on the grade.

A few interesting edges cases exist about what to do if a transfer is itself setting treasury as the
destination, and it has a tip.

Next, `nonce::optional::validate_ready,56` and `nonce::optional::validate_future,53` is a large
wake-up call that the transaction pool validation and the way that nonce works was not fully
grasped. This might have to do with the pool lecture being delivered with some technical
difficulties and online.

As a short recap, you should know that a future nonce is `Invalid` in `apply_extrinsics`, but in
`validate_transaction`, it is simply a `ValidTransaction` with a specific `requires![]` tag.

> Given this, we made all tests related to nonce and tx-pool validation be optional. This reduces
> the nonce score to getting just the `apply` correct, which many more people did.

`tipping::challenging::bob_with_20_sends_10_to_charlie_and_tip_5,37` is a good excuse to recap the
two phases of extrinsic execution:

1. `Apply`
2. `Dispatch`

Apply is the outer shell of the execution. It involves deducting the tip, checking nonce and
signature. If these are fine, the transaction will 100% be in the block.

Dispatch is merely the inner result of executing the transaction, for example if the transfer was
successful or not.

You should really see these two as separate and distinct. If Apply is `Ok(_)`, tip, nonce and tx's
presence in the block is non-negotiable.

In this case, Bob's tip will be deducted, and the transfer will fail because it would put Bob does
not have enough free balance. This is `Ok(Err(_))`, nonce is bumped nonetheless, tip is deducted
nonetheless, but the inner dispatch failed.

`tipping::challenging::alice_with_100_transfers_20_to_bob_with_5_tip,36` is a basic test to check
that you burn funds when the tip is less than 10.


`tipping::challenging::validate_tx_alice_with_100_tips_100,34` will ensure that you are checking all
aspects of `Apply`, as noted above, in `validate` as well.

While in `mini_substrate.rs` we did, here we didn't have many tests for your safe arithmetic. We
only checked that you properly saturate the priority if needed, namely in
`tipping::fundamentals::alice_with_u128_max_div2_tips_u128_max_div4,28`.

All in all, the mix of tipping and staking yielded the most interesting edge cases. If you have any
further in mind, please share it with us in our feedback, and we will incorporate it in the future
iterations of the assignment.



### Manual Review

Some common Rust mistakes or improvements that I noticed in my manual review of your code:

1. Don't underestimate the importance of `Option<_>`, `Result<_, _>` and the `?` operator. You might
  think "oh well, I know them", but I rarely saw students using these APIs correctly. Both `Option`
  and `Result` have an extensive array of methods that allow you to mutate them on the fly. `?` will
  allow you to avoid nested code. I saw many students who use `Result` and `Option`, but at a vert
  rudimentary level, not using any of the advance APIs, and even worse, not using the `?` operator,
  leading to this:

![nested](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*YoTPCR_l1ApgGGfMp6ZzmQ.png)

Suggested reading:

- https://doc.rust-lang.org/std/option/enum.Option.html
- https://doc.rust-lang.org/std/result/enum.Result.html

2. Another pattern that I saw way too often, and I dislike way too much is a wildcard arm in a match
   statement. If you have `enum Call { Foo, Bar, Baz }`, and a match statement like:

```
match call {
  Call::Foo => { ... },
  Call::Bar => { ... },
  Call::Baz => { ... },
  _ => { ... } // Don't do this 🤮
}
```

You really want your code to NOT COMPILE if you add a new variant to the enum. Adding `_ => {}` will
open the door to all sorts of "silent failures" that you don't want to deal with, especially in
blockchain systems.
