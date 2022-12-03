# Polkadot Blockchain Academy Final Examination

Your final assignment is worth 50% of your final grade at the Polkadot Blockchain Academy. In this assignment, you will be putting to practice the things you have learnt in the remainder of the modules of the Academy.

> Module 7 (XCM) will not be covered until after the final exam’s submission due date. However, you are free to use XCM concepts in your project if you wanted to.

The goal of this assignment is to have you implement something from the proposed list of projects below:

- Create a simple multi-token DEX
  - Create a simple multi-assets pallet (or use the existing one).
  - Create a Uniswap style DEX to allow users to trustlessly exchange tokens with one another.
    - Be sure to implement Liquidity rewards.
    - Expose an API which acts as a “price oracle” based on the existing liquidity pools.
  - Add a simple NFT pallet (like the kitties pallet we will do in class)
    - Allow users to mint or buy/sell new kitties with any token.
- Build a Quadratic voting system
  - Create a simple Identity pallet, or use the existing identity pallet.
  - Create a voting system where users with identities can **reserve** an amount of token, which then weights their vote on a quadratic scale.
  - Proposals should be a simple on chain text or hash, votes can be simply aye or nay.
  - As a bonus, create a more complex proposal system where users can vote on multiple things at once, and have to consider how they want to distribute their votes across them.
- Basic Direct Delegation Proof of Stake system:
  - A pallet which manages the DPoS System
    - Where one set of users can register to be a validator by providing a session key for Aura / BABE.
    - Where any user can use their tokens to delegate (vote) for the set of validators.
    - Where every N blocks, the current “winners” are selected, and Aura / BABE is updated.
    - As a bonus, try to support delegation chains, where you can back a delegator who themselves pick the validator.
  - A pallet which gives block rewards to the current block producer.
    - As a bonus, you can think about and implement some kind of slashing for validators if they “misbehave”
- Build a Liquid Staking Pool.
  - Create a new pallet which interfaces with the existing `pallet-staking` and `pallet-democracy` (you might need to make some alterations to them).
  - For `pallet-staking`:
    - New users stake through your system by transferring funds to your pallet. Your pallet will control the staking behavior of the funds (i.e. who to nominate), and in return, it gives a liquid derivative back to the user.
    - There should be a simple voting system in the pallet where holders of the derivative token can influence the nomination strategy of the pool.
    - The derivative token must be transferrable, hence “liquid” staking.
    - Ensure the reward and slashing is accurate and reliable across this multi-pallet system.
  - For `pallet-democracy`
    - Users should be able to vote on referenda using the tokens which are staked and managed by your pallet.
- Or, “**bring your own idea**”! Needs approval from an Academy instructor who will grade your work.
  - Write a UTXO runtime without frame? (Speak with Joshy)
  - Tooling for Substrate / Polkadot / Parachains?
  - Front-end development for existing tools in the space?
  - Other ideas welcome!

> In all projects, you are free to use everything that you have learned from Substrate, including existing pallets. Feel free to use a pallet as-is, or tweak it slightly to fit your needs.

> To the contrary, when needed, make a `Trait` that is supposed to deliver some functionality, and do a _mock_ implementation of it, to abstract away components that you want to interact with and are not readily available in FRAME.

## Topic selection

You are expected to share your selected topic **no later than Sunday July 31st**.

If there is something you would like to implement that’s not in the proposed list, please come talk to either your TA, Shawn, or Kian for approval. Whomever approves your project will also grade your project, so keep them informed of your work.

It is your responsibility to make sure you have all your questions answered by your TA or other instructors before the final deadline.

## Deadline

The submission due date is **Wednesday, August 3rd at 12PM**.

## Submission

Send a link to a GitHub repository containing your implementation to your TA in order to submit. In fairness to all students, please refrain from updating your code past **12 PM of the due date**.

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
