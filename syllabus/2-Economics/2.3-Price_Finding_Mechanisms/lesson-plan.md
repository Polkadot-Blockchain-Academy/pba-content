## Lesson 4a: Price discovery methods (Auctions) (60min)

_Note: Can we have learners play Modern Art (the auction card game) as part of this section?_

The broad aim of Sessions 4a and 4b is to discuss price setting mechanisms, both traditional and on blockchains.

### Why is this important

Price discovery is one of the most important aspects in economics. Auctions, as most prominent device, are applied in Polkadot's parachain allocation system.

### Prerequisites

- Basic economic concepts
- Game theoretical concepts

### Learning Outcome

The goal is to give an introduction into traditional price-finding mechanisms: auctions and order books.

### Learning Objectives

Students...

- can design a simple auction and know why they are a very efficient way to discover prices.
- can distinguish between different auction designs and detect bad design decisions.
- evaluate the results of the real-world outcomes of the parachain candle auctions.

### Content

In the first section we cover the following topics:

- _Auctions_
  - Where were/are auctions used? (Some history of auctions.)
  - Discuss different types of auctions: first-price, second-price, Dutch, and English.
  - Why are auctions such a good means to sell a good: asymmetric information; i.e., if the seller does not know exactly what value the buyers attach to the good.
  - Derive equilibria in simple setting. Compare.
- _Order Books_
  - Order books can be used to discuss demand and supply more broadly.
  - Maybe: Discuss double auction (like a stock exchange).
- **Polkadot reference:**
  - Candle auction: discuss format, what specific problems does it solve (front-running, shill-bidding)? In particular: discuss in a non-technical way Alistair's and Samuel's paper on candle auctions.
  - Crowdsourcing, crowdloans [what do we mean here?]

### Activity (90-120min)

In this non-lesson part, we want to focus on classroom experiments.

- Playing a candle auction on your own (can also vary activity rules), also a hard-closing auction with discrete time steps (to see sniping behaviors).
- Experiment, Analysis of data, seeing if it was efficient - Compare it to the real polkadot auctions (graphs)

# Day 3

## Lesson 4b: Price discovery methods (DeFi) (60min)

_Note: Perhaps we reframe this section to be about 'exploits in game theory' or 'Price discovery/prediction markets'. We should not teach Defi in this module at all. Automated Market Makers could be used in this module._

### Why it is important

A key concept of traditional auctions, order books, are hard to implement on blockchains, which is why people have come up with, e.g., automated market makers. Decentralized finance is undoubtedly one of the most powerful and important areas of decentralized applications. As mentioned, it is imperative to build incentives correctly, otherwise users will find a way to exploit the system in any way possible to make money.

### Prerequisites

- Topics are heavily dependent on previous lesson 4a.

### Learning outcome

We introduce DeFi and some related topics in algorithmic game theory and mechanism design.

### Learning outcomes

Students...

- will have an overview on alternative price discovery methods.
- will understand the main types of DeFi apps that can be implemented, and good rules of thumb on how to develop them.

### Content

- Intro to decentralized finance. What and why? Advantages and technical limitations.
- Prediction markets (e.g. Iowa Electronic Market) and other belief aggregators, proper scores (e.g. logarithmic market scoring rule).
- Decentralized exchanges: automated market makers vs order books, liquidity providers, arbitrage, their use as price oracles.
- Experiments in DeFi: flash loans, HydraDX price finding mechanism, Squeeth leverage.

### Activity (90-120min)

- Examples of real life hacks & ill-designed incentives. We could split people in teams, each team analyzes one famous hack, does investigative work reading news from the Internet, finds what exactly went wrong, and presents to the other teams.
