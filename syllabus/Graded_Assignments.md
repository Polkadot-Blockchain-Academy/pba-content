# Graded Projects

In order to assess student understanding and provide hands-on learning experience, we assign roughly one graded assignment per week throughout the course.

The exact assignment schedule will vary by cohort depending on content order, where the weekends fall, and instructors' preferences.
Some cohorts may include a pre-course assignment or a final assignment as well.
Some cohorts may choose to omit an assignment on a particular week.

## Assignment Style

Some assignments will be more like problem sets where students must complete each of several small exercises. Others will be like larger projects where students must choose one larger assignment and complete only that one.

## Grading

Currently each assignment is graded in its own bespoke way.
We are moving toward a more unified system where as much grading is automated as possible.
If you are creating a new assignment or grading one for the first time, checkout https://github.com/Polkadot-Blockchain-Academy/ucb-qualifier-grading/ as a tool that may help you in your endeavors.

## Menus of Assignments

This section contains links to individual assignments that may be useful throughout the course.

Typically an assignment covers content from more than one module.
Therefore some assignments may only be useful when content is taught in a particular order.

This list links to all graded assignments used in the PBA, roughly in order of where they will appear in the course.

Offered In | Link | Description
---------- | ---- | -----------
BA | Cryptography Capture The Flag, Merkle Tree, Nash Solver | https://github.com/Polkadot-Blockchain-Academy/pba-ctf-assignment--master
UCB  | On-chain Bridge Contract al la BTC-Relay   | https://github.com/Polkadot-Blockchain-Academy/bridge-assignment-master
BA | Accounts OR Utxo Cryptocurrency without runtime framework | https://github.com/Polkadot-Blockchain-Academy/frameless-node-template--master
Camb | Review and Critique a Whitepaper | https://github.com/Polkadot-Blockchain-Academy/pba-content/tree/cambridge-2022/syllabus/midterm
Camb, BA | Write a FRAME-based Runtime | https://github.com/Polkadot-Blockchain-Academy/frame-assignment--master
BA | Reserve Asset Transfers in XCM | https://github.com/Polkadot-Blockchain-Academy/xcm-assigment--master

## Idea Stubs

Sometimes you have an idea for an assignment but don't need to write it up formally right now.
We can brainstorm such ideas that may be useful in the future here.
If the list gets too long we can split it into files based on the part of the course.

### Commit Reveal Scheme

Implement a hash-based commit reveal-scheme.
The user takes these actions:

1. Enter some data they want to commit to.
1. Receive a Salt and a Commitment in exchange.
1. Commit to data by sharing the commitment publicly
1. Reveal by sharing the Salt publicly
1. Check others' commitments by providing a commitment and a salt

### Auctions

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

#### English Auction

The highest bidder wins and pays the price they bid

```rust
fn english_auction(Vec<Bid>) -> AuctionWinner;
```

#### Dutch Auction

The highest bidder wins and pays the price bid by the second highest bidder

```rust
fn dutch_auction(Vec<Bid>) -> AuctionWinner;
```

#### Candle Auction

Only the bids that are placed before the ending time are actually counted.
Of these bids, the highest bidder wins and pays the price they bid

```rust
fn candle_auction(Vec<Bid>, end_time: Duration) -> AuctionWinner