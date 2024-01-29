---
title: Price Finding Mechanisms
description: Auction Basics for Web3 engineers
duration: 1 hour
---

# Price Finding Mechanisms

---

## Introduction

- A central topic of economics is _price finding_ in markets.
- How does a buyer and a seller agree on the price of a good?

---

## Supply / demand for BIG markets

<img rounded style="width: 800px; margin-right: 200px;" src="./img/2.1-supply-and-demand.png" />

---

## Supply-demand for BIG markets

- Supply-demand model works well for _big_ markets.
- We have lots of buyers, sellers, lots of info.
- Predictable _market price_, balancing supply and demand.
- Only sensible price at which a buyer and seller can trade.

---

## What about _difficult_ markets?

<img rounded style="width: 900px;" src="./img/2.3-auction-platform.png" />

---

## What about _difficult_ markets?

Today we focus on markets with few buyers or sellers,
few trades, or non-standardized goods.

<pba-flex center>

- Gov't sells radio frequencies to broadcasters.
- A painting by a famous artist trades hands.
- An oil well goes on sale with unknown amount of oil.

</pba-flex>

---

## A simple model

<pba-flex center>

- Selena wants to sell one item of a good.
- Two buyers, Alice and Bob.
- Each buyer has a secret _valuation_ $v_A$ and $v_B$:<br/>
  how much they are willing to pay for the item.

</pba-flex>

How does Selena _optimally_ sell the item?

---

## Let's assume...

<pba-flex center>

- All of Selena's income is profit.
- Alice and Bob each have random valuations between 0 and 1.
- The valuations are secret but their distribution is well-known.

</pba-flex>

---

## Price posting

- Simplest solution: Selena posts a price $p$, sells item to first taker.
- What is her _optimal price_ $p$?
  One maximizing her _expected revenue_.
- Expected revenue: price $\times$ prob. of making a sale.
- For price $p$, prob. of making a sale is $(1-p^2)$.
- Hence her expected revenue is $p \cdot ( 1-p^2)$.

Notes:

- If both buyers accept the posted price, we assume the firm selects a random buyer to trade with.
- The proof of this and all other math formulas can be found in the slides, for anyone interested.
- Assuming that 100% of the sale price is profit for the seller, this profit is

  - p in case of selling, 0 otherwise, where
  - (1-p^2) is the probability of selling the item.
  - To see this probability, consider conversely that the probability that none of the buyers accepts the price must be $p^2$, because each valuation must be below $p$, and these are two individual events each happening with probability $p$ (recall: valuations are uniformly distributed between 0 and 1).

- $Pr\{v_i \leq x\}$ is the probability that the randomly drawn value of valuation $v_i$ is below $x$.
- Independent values will be important: when is this (not) a sensible assumption?
  (Example: oil drilling rights auctions, ...)
- Uniform distribution is just for pedagogical reasons; standard model allows for more general distributions.

---

## Price posting

<pba-cols>
<pba-col>

- At the optimum point, expected revenue is $\approx 0.38$.
- We will see that **an auction can do better**.

</pba-col>

<pba-col>

<img rounded style="width: 600px; margin-right: 10px;" src="./img/2.3-price-posting.png" />

</pba-col>
</pba-cols>

Notes:

- Prices, valuations, and revenue are given in the same currency unit, say, USD.
- How do we maximize the expected revenue function f(p)=p (1-p^2)?
  The function has a unique peak, so at the maximum, the first derivative must be zero.
- The function is maximized for a $p$ satisfying $$ f'(p)=1-p^2 - 2p^2 = 0 \implies p = \sqrt{1/3}.$$
- Expected revenue is then $f(\sqrt{1/3})=\frac{2}{3}\sqrt{1/3}\approx 0.38.$

---

## Auctions

<img rounded style="width: 500px;" src="./img/2.3-auctioneer.gif" />

---

## Lesson summary

<pba-flex center>

- Auctions can be superior to posting a price.
- We discuss four important auction formats.
- How to make buyers reveal their secret valuations.
- Bid shilling.
- The revenue-equivalence theorem.
- Guides on what auction format to use in practice.

</pba-flex>

---

## Auctions

- Alice and Bob (bidders) submit bids based on own valuations.
- Selena (auctioneer) selects winner to allocate the item to.
- The winner's payment is a function of the bids.

_An auction is a competitive game for buyers, where the seller makes the rules._

---

## Auction formats

<pba-flex center>

1. There are two broad classes:
   1. _Static Auctions:_ bidders submit their bids at the same time.
   1. _Dynamic Auctions:_ bidders submit bids over time.

- The difference is whether or not bidders can react to the bids of others and adjust their own.
- Static auctions are also called _sealed-bid auctions_.

</pba-flex>

---

## Auction formats

Four auction formats we discuss today:

<pba-flex center>

- Static auctions
  - with a first-price payment rule
  - with a second-price payment rule
- Dynamic Auctions
  - with ascending price (a.k.a. English auctions)
  - with descending price (a.k.a. Dutch auction)

</pba-flex>

---

## Second-price auction

- Why make the winner pay the second highest bid?
- Least intuitive format, but strategically simplest for bidders.
- Simply bid your valuation truthfully!
- A bidder wants to maximize their _expected profit_:
  - own valuation - price paid, in case they win
  - zero, otherwise

---

## Second-price auction

- **Truthful equilibrium**: a _dominant strategy_ to bid truthfully.
- **Dominant strategy**: outcome is at least as good as the outcome of any other strategy, _no matter what the other bidder bids_.

<img rounded style="width: 1300px;" src="./img/second-price-auction.svg" />

Notes:

- We can immediately say that you should never overbid (i.e., bid above your valuation), because in that case your profit can only be zero (if you lose) or negative (if you win).
  So the key question is whether you should ever underbid.
- Look at Situation 1: In the case that Bob's bid is higher than your valuation, bidding truthfully or underbidding both lead to losing and having a revenue of zero.
- Look at Situation 2: But if Bob's bid is lower than your valuation, any bid between these two values leads to you winning and having the same profit (namely the difference of these values), hence there is no gain in underbidding.
  However if you underbid too much, you run the risk of losing and having zero profit.
- Hence in both cases we see that underbidding does not increase your profit, and can only decrease your chance of winning.
  So it is better to bid exactly your valuation.
- The proof can be easily generalized to any number of bidders (to obtain that in a second-price auction, it is a dominant strategy for each bidder to bid truthfully.

TIP: use arrow chars in unicode for style: https://www.htmlsymbols.xyz/arrow-symbols

---

## Second-price auction

**Expected revenue**

- Selena's expected revenue is expected value of 2nd highest valuation.
- For two independent variables uniformly sampled from $[0,1]$,
  the expected value of the minimum is $1/3\approx 0.33$.
- This is not quite as good as posting a price
  (which provided expected revenue $0.38$).
  Why not?

---

## Reserve price

- Because the format we considered is not optimal for the auctioneer!
- The optimal auction involves a _reserve price_ $r>0$:
  - If no bid is above $r$, nobody wins.
  - If one bid is above $r$, the payment is $r$.
  - If both bids are above $r$, the payment is the second-highest bid.

Notes:

- Selena is basically participating as a bidder bidding r

---

## Reserve price

**Fact:** Under any reserve price $r$, it is still optimal to bid truthfully, and if Selena sets $r=0.5$, her expected revenue is $\approx 0.42$, so it is better than posted price (where expected revenue was $\approx 0.38$).

---

## Reserve price

<img rounded style="width: 1400px;" src="./img/reserve-price.svg" />

Notes:

- Basically there are four cases equally likely. In one case she makes a loss, the other doesn't matter but in two cases she makes a gain.
- The proof idea is that if $r=0.5$, half of the time one valuation is above it and the other is below it, and the reserve price increases the paying price.
  On the other hand, if both valuations are below $r$ then there is no sale, which decreases Selena's revenue, but this case only happens one quarter of the time.
  Finally, if both valuations are above $r$, the presence of a reserve price does no affect the result.
  Overall, there is a positive net gain in Selena's expected revenue.
  Below we present the formal proof.
- The probability that both valuations are below $r$ is $r^2$, the prob. that $r$ is in between the valuations is $2r(1-r)$, and the prob.that both valuations are above $r$ is $(1-r)^2$.
  You can check that these probabilities sum up to one.
- In the first case Selena's revenue is zero, and in the second case it is $r$.
  Hence the expected revenue for the first two cases is $2r^2(1-r)$.
- To compute the expected revenue of the third case, we simply copy the integral formula for the expected value (from a couple of slides ago), but we restrict the integration limits to $[r, 1]$:
  \begin{align}
  \int_r^1 x\cdot f(x) dx &= 2\int_r^1 (x-x^2)dx\\
  &=2\cdot \left[\frac{x^2}{2}-\frac{x^3} {3}\right]\_r^1 \\&=\frac{1}{3} - r^2 + \frac{2}{3}r^3.
  \end{align}
- Putting all terms together, we obtain that the expected revenue is
  $$R(r)=0+2r^2(1-r)+\left(\frac{1}{3} - r^2 + \frac{2}{3}r^3\right)=\frac{1}{3}+r^2-\frac{4}{3}r^3.$$
- To maximize the auctioneer's expected revenue function $R(r)$ above:
  - We evaluate the function at all the local extrema (minima and maxima).
  - Since the function is continuous and differentiable, the local extrema are found at the borders $r=0$ and $r=1$, and at the roots of the derivative $R'(r)=2r-4r^2=0$.
    These roots are $r=0$ and $r=1/2$.
  - By inspecting all these points, we find that the global maximum is found at $r=1/2$, with a value of $R(1/2)=5/12\approx 0.42$.
- Literature: Myerson, Roger B. "Optimal auction design." Mathematics of Operations Research 6, No. 1 (1981): 58-73.

---

## English clock auction

Recall the rules:

<pba-flex center>

- Selena continually raises the price.
- At any price, you decide whether to stay or leave.
- If you leave, you may not return.
- If you are the last one in the auction you win<br/>
  and pay the price at which the second-to-last<br/> bidder left.

</pba-flex>

Notes:

- Next we move to English clock auctions, which have a very similar analysis.
- Notice it has a flavor of a second-price auction: the price you pay is decided by another bidder, not you.
- In a sense, these two auction formats are strategically equivalent!

---

## English clock auction

- English clock auction is **strategically equivalent** to static second-price auction.
- It is a dominant strategy to stay until the price reaches one's valuation.
- The expected revenue for Selena is the also the same!
- Consequently, these two formats are also **revenue equivalent**.

---

## Shill bidding

- Second-price and English auctions popular among theorists, not so popular among practitioners.
- One reason is that they are prone to _shill-bidding_: bidder that acts on behalf of the auctioneer to drive up the price.

Notes:

- You could argue its similar like a reserve price, but the difference is that the other bidders don't know the value.
- Both second-price auctions and English auctions have the truthful bidding property, which is very satisfactory for theorists.
  However, as you can probably tell, they are not so popular in practice.
- A shill bidder has no intention of winning, but just increasing the price that the winner must pay.
  They do it to benefit the auctioneer.
- Shill bidding is frowned upon, sometimes illegal, but it is usually hard or impossible to prove and to prevent.

---

## First-price auction

<pba-flex center>

- The winning bidder pays her bid.
- Other rules same as in the second-price auction; i.e.,
  - all bidders submit their bids simultaneously,
  - the highest bid wins.

</pba-flex>

---

## First-price auction

Bidding in the first-price auction is **not** truthful.

- Bidding truthfully can never be optimal:
  if you win, you earn nothing.
- Underbidding is strictly better, you win sometimes
  and when you do you have a positive utility.

---

## First-price auction

**Equilibrium strategy:** It is a _Nash equilibrium_ for each bidder to bid **half** their own valuation.

_Nash equilibrium:_ A set of strategies, one per player, where no one has an incentive to change their strategy.

---

## First-price auction

**Intuition:** suppose you are Alice

- If you bid 0, winning prob. is zero.
- If you bid your valuation, profit is zero.
- Hence, there is a sweet spot between 0 and your valuation
  where your expected profit is maximal.
- It turns out this is bidding half your valuation,
  at which point you and Bob each wins _half of the time_.

---

## First-price auction

**Expected revenue**

- Reasonable to assume each bidder bids half their valuation.
- Hence, Selena's revenue is $\frac{1}{2}\max\{v_A, v_B\}$.
- The expected value of $\max\{v_A, v_B\}$ is $2/3$.
- Hence, her expected revenue is $1/3$.

The same as in second-price auction!

---

## Revenue Equivalence

**Fact:** When valuations are secret and independent,
there is no reserve price, and item goes to highest bidder,
then _all auction mechanisms_ are _revenue equivalent_.

---

## Dutch auctions

<pba-flex center>

- Selena continually lowers the price.
- As soon as a bidder accepts the price,<br/>
  they are declared winners and auction is over.
- Winner pays the price they accepted.

</pba-flex>

---

## Dutch Auction

Recall the rules:

- The auctioneer continually lowers the price.
- At any price, you can decide whether or not to accept the price.
- If you are the first to accept the price, you win and pay the price you just accepted.

---

## Dutch Auction

- It turns out that the Dutch auction is strategically equivalent and revenue equivalent to the static first-price auction.
- The price that you accept in the Dutch auction corresponds to the price that you'd bid in a static first-price auction.
- The tradeoffs that the bidders face are very similar: take the current price or wait a bit at the risk of another bidder accepting first.
  It is an equilibrium to wait till the price is half your valuation.

---

## Recap

<pba-cols>
<pba-col>

**Analyzed important auction formats:**

- Static first-price auction.
- Static second-price auction.
- English clock auction.
- Dutch auction.

</pba-col>
<pba-col>

**Learned under standard assumptions:**

- First-price and Dutch auctions are strategy equivalent.
- Second-price and English clock auctions are strategy equivalent.
- All four auctions are revenue equivalent.
- Having a reserve price increases the expected revenue,
  and it beats posting a price.

</pba-col>
</pba-cols>

---

<!-- .slide: data-background-color="#000" -->

## Break (10 minutes)

---

<!-- .slide: data-background-color="#4A2439" -->

## Discussion

---

## Independence of valuations

In our analysis, it was important to assume that
bidders' valuations are independent from one another.

_Can you think of examples where this assumption isn't sensible?_

---

## Independence of valuations

_Answer:_

**Sensible:** - a piece of art, where the bidders are final clients.

**Not sensible:** drilling rights to an oil well.
Bidders will have similar estimates of amount of oil,
hence valuations are highly correlated.

---

## Common value auctions

Special scenario: there is a unique valuation of item,
but each bidder only has a private estimate of it.

In these cases, it is observed that sealed-bid auctions
tend to give higher revenue than dynamic auctions.

_Why do you think this is the case?_

---

## Common value auctions

_The auction may be used as a means of gathering information from other participants to triangulate a price_

_Answer_: In a dynamic auction, a bidder can use the bids of others as additional signals of the correct valuation.
If bids so far seem high, my initial estimate must be low, and vice versa, so I can adjust my personal estimate.
Hence estimates converge.

In a static auction, there is no convergence of estimates, so it is more likely that some bidders keep unusually high estimates.
As a result, there is a higher chance that the winner ends up paying more than the correct valuation.
This is known as the _winner's curse_.

---

## Equivalence of revenues

It is observed in practice that first-price auctions lead to higher revenue than second-price auctions.

This _violates_ the equivalence of revenues, so an assumption in our analysis fails consistently.

_What do you think it is?_

---

## Equivalence of revenues

_Answer:_ **Risk aversion.** People prefer lower uncertainty games, even if this means lower expected profits.

_Would you rather win a million dollars with a 50% chance, or 300 thousand with a 90% chance?_

In Nash equilibrium analysis for first-price auctions, we claimed that if Bob bids half his valuation, then Alice should bid half hers, so each wins 50% of time.
But we implicitly assumed that Alice is risk neutral.
Yet she might prefer to bid more and win more often.

---

## Front Running

Computer systems may have _front runners_: special nodes can see an incoming bid, react by creating their own bid, and make it look like their bid was created first.

If you run an auction on a system with front runners, which of the four auctions would you use?
Why?

---

## Front Running

_Answer:_ Meet front runner _Fred_.

In a Dutch auction, if Fred is a bidder he waits for first bidder to signal accepting the price, and Fred makes the signal first.
He's guaranteed to win with least possible price.

In second-price auction, if Fred is auctioneer he can shill bid successfully: when a highest bid arrives, he creates bid slightly under it and makes it appear as if it was created first.

---

## Front Running

_Answer:_ Meet front runner _Fred_.

In a first-price auction, if Fred is bidder and if he can "open the seals" he can win by slightly outbidding highest bid.
(Poor data privacy, a common issue in computer systems)

Hence, it might be best to run an English auction.

---

## Sniping

In a dynamic auction with a fixed bidding time window, _sniping_ is placing a highest bid as late as possible, so other bidders can't react and overbid you.
The practice leads to low revenue.
Particularly bad when done by a front runner (microseconds).

How to protect an auction from snipers?

---

## Sniping

_Answer:_ **candle auctions.**<br/>
Dynamic first-price auction with _random ending time._

<img rounded style="width: 600px;" src="./img/2.3-candle-auction-photo.jpg" />

---

## Sniping

_Answer:_ **candle auctions.**<br/>
Dynamic first-price auction with _random ending time._

- Similar to first-price auction
- except that ending time is unpredictable.
- At ending time, current highest bidder wins, pays own bid.
- Sniping protection: the longer you wait to bid, the higher the chances the auction ends.

---

<!-- .slide: data-background-color="#4A2439" -->

# Workshop: Auction Games

---

## NFT Auction

- You have the chance to bid on one professionally created NFTs by an experienced artist.
- There are 13 unique NFTs which are minted up to each three times (random process).
- Use your budget that you accumulated during the last Academy Games.
- Everything that you will not use for bidding (or if your bid was lower than your budget), you will receive at the end of the Academy.
- **100% of the revenue of the auctions goes to the artist.**
- You are randomly assigned to one of three auction formats.

---

# The Artist & NFTs!

---

## Sharmaine Kwan

- **Inspiration**: Hong Kong artist, is inspired by the city's historic neon signs and is dedicated to preserving their essence through her art.
- **Contemporary Approach**: She uses diverse media, including traditional and LED neon lights, digital art, and sculpture, to give a modern twist to the classic neon art style.
- **Themes**: Kwan's work reflects on the urban environment and cultural changes, merging nostalgia with future possibilities.
- **Education and Publications**: She has a Fine Art degree from the UK and authored books on traditional Chinese painting.
- **Global Recognition**: Kwan's art has been exhibited internationally, and she has collaborated with various brands and organizations.

---

## Lumina Celestia

<img rounded style="width:600px" src="./img/2.3-collection-preview.jpg" />

---

### Format 1: Activity Rule Auction

- The initial bidding phase lasts 30 seconds.
- Every valid bid resets the timer.
- You need to bid at least 30 cents more than the previous highest bid.
- Whoever has the highest bid at the end, wins.
  Winners pay their bids.

---

### Format 2: Candle Auction

- Auction Format from the 16th Century.
- The auction lasts for exactly 4 minutes.
- A “candle mechanism” randomly determines, after the auction, when the auction ended
- Grace-period of 1 minute.
- Candle Phase of 3 minutes.
- Whoever had the highest bid when the auction actually ended, wins.

---

### Format 3: Hard Close Auction

- Similar to eBay auctions.
- Auction lasts for 4 minutes.
- Whoever has the highest bid at the end of the auction, wins.
- Winners pay their bids.

---

<!-- .slide: data-background-color="#4A2439" -->

## Auction 2: Questions?

---

## Auction 2: NFT Auction

Link will be distributed!

---

## Auction 2: Discussion

---

## Auction 2: Results!

---

## Further Reading

Polkadot & Kusama Auctions<br/>Data & Background information:

<pba-flex center>

- [Kusama Auctions 1-5](https://polkadot.network/blog/making-history-an-overview-of-the-first-five-parachain-slot-auctions-on-kusama/)
- [Kusama Auctions 6-10](https://polkadot.network/blog/kusama-batch-2-auctions-report/)
- [Polkadot Auctions 1-5](https://polkadot.network/blog/making-history-again-polkadot-auctions-1-5/)

</pba-flex>

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions?

---

# Thank you!
