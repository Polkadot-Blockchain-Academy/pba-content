# Week 1 Assignment - Cryptography, Economics, Game Theory

This week's assignment is a set of exercise. Complete all of these exercises.

## Exercise 1: Encryption CLI Utility

Write a CLI utility in Rust that allows users to exchange encrypted messages. The utility does not need to do any communicating or networking. It just needs to do the encrypting and decrypting. Then the user can communicate ciphertext over email, element, etc.

- Support Symmetric encryption where the user supplies the shared key
- Support asymmetric encryption
- Support key pair generation for asymetric encryption

idea: we could give students starter code that is the main CLI part, and they fill in the library. Then to grade we run unit tests against the library

## Exercise 2: Implement merkle tree or MMR

See merkle tree activity already in module 1

## Exercise 3: Implement CBC and CTR modes for AES

See activity already in module 1

## Exercise 4: Commit Reveal Scheme

Implement a hash-based commit reveal-scheme. The user takes these actions:

1. Enter some data they want to commit to.
2. Receive a Salt and a Commitment in exchange.
3. Commit to data by sharing the commitment publicly
4. Reveal by sharing the Salt publicly
3. Check others' commitments by providing a commitment and a salt

## Exercise 5: Auctions

```rust
/// A bid cast in an auction
struct Bid {
    bidder: String,
    price: u32,
    time: Duration,
}

/// The result of an auction
struct AuctionWinner {
    bidder: String,
    price: u32,
}
```

Implement three function that takes in a bunch of bids and return the winner and the price they pay.

### English Auction

The highest bidder wins and pays the price they bid

```rust
fn english_auction(Vec<Bid>) -> AuctionWinner;
```

### Dutch Auction

The highest bidder wins and pays the price bid by the second highest bidder

```rust
fn dutch_auction(Vec<Bid>) -> AuctionWinner;
```

### Candle Auction

Only the bids that are placed before the ending time are actually counted. Of these bids, the highest bidder wins and pays the price they bid

```rust
fn candle_auction(Vec<Bid>, end_time: Duration) -> AuctionWinner
```