# Week 3 Assignment - FRAME

This assignment is a project covering the material you've learned about writing Substrate Runtimes with FRAME. To complete this project, select one of the following options, and implement the runtime described using FRAME.

## Option 1: Decentralized Exchange

- Create a simple multi-assets pallet (or use the existing one).
- Create a Uniswap style DEX to allow users to trustlessly exchange tokens with one another.
  - Be sure to implement Liquidity rewards.
  - Expose an API which acts as a “price oracle” based on the existing liquidity pools.
- Add a simple NFT pallet (like the kitties pallet we will do in class)
  - Allow users to mint or buy/sell new kitties with any token.

## Option 2: Quadratic Voting

- Create a simple Identity pallet, or use the existing identity pallet.
- Create a voting system where users with identities can **reserve** an amount of token, which then weights their vote on a quadratic scale.
- Proposals should be a simple on chain text or hash, votes can be simply aye or nay.
- As a bonus, create a more complex proposal system where users can vote on multiple things at once, and have to consider how they want to distribute their votes across them.

## Option 3: Direct Delegation Proof of Stake

- A pallet which manages the DPoS System
  - Where one set of users can register to be a validator by providing a session key for Aura / BABE.
  - Where any user can use their tokens to delegate (vote) for the set of validators.
  - Where every N blocks, the current “winners” are selected, and Aura / BABE is updated.
  - As a bonus, try to support delegation chains, where you can back a delegator who themselves pick the validator.
- A pallet which gives block rewards to the current block producer.
  - As a bonus, you can think about and implement some kind of slashing for validators if they “misbehave”

## Option 4: Liquid Staking Pool

- Create a new pallet which interfaces with the existing `pallet-staking` and `pallet-democracy` (you might need to make some alterations to them).
- For `pallet-staking`:
  - New users stake through your system by transferring funds to your pallet. Your pallet will control the staking behavior of the funds (i.e. who to nominate), and in return, it gives a liquid derivative back to the user.
  - There should be a simple voting system in the pallet where holders of the derivative token can influence the nomination strategy of the pool.
  - The derivative token must be transferrable, hence “liquid” staking.
  - Ensure the reward and slashing is accurate and reliable across this multi-pallet system.
- For `pallet-democracy`
  - Users should be able to vote on referenda using the tokens which are staked and managed by your pallet.

> In all projects, you are free to use everything that you have learned from Substrate, including existing pallets. Feel free to use a pallet as-is, or tweak it slightly to fit your needs.

> To the contrary, when needed, make a `Trait` that is supposed to deliver some functionality, and do a _mock_ implementation of it, to abstract away components that you want to interact with and are not readily available in FRAME.

## Grading rubric

Your implementation will be reviewed for code quality and implementation details.

- Implementation
  - Correctness and accuracy of implementation
  - Evidence of using various techniques used in class
  - As close to production ready as possible
- Code Quality
  - Tests and code coverage
  - Use of best practices and efficient code
  - Well documented, with considerations and compromises noted
- Bonus Points
  - Integrate this into a working node.
  - UX to interact with the runtime code.
    - Value functionality over beauty.
  - Integrate node as a working parachain on a relay chain.
  - Working cross chain scenarios for your runtime logic using XCM and XCMP.
- TODO: Add something about benchmarking
