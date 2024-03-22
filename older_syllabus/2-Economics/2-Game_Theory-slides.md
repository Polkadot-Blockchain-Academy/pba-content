---
title: Game Theory Basics
description: Game Theory Basics for Web3 engineers
duration: 1 hour lesson + 2 hours activities
---

# Game Theory Basics

Notes:

Game theory is a field of study at the intersection of mathematics and economics.
It considers an economic system to be a game and people to be players.
According to the rules of the game, it analyzes the players' best strategies and then uses this analysis to explain the observed behavior of the economic system.

Game theory is an interesting an powerful tool, because it's fairly simple and intuitive, yet extremely powerful to make predictions.
Hence, it's a good idea to learn game theoretical principles and keep them in the back of your mind when designing economic systems.

---

# Outline

<ul>
    <li class="fragment">Lesson
        <ul>
            <li class="fragment">What is Game Theory?</li>
            <li class="fragment">What is a Game?</li>
            <li class="fragment">Types of Games</li>
            <li class="fragment">Common Games</li>
            <li class="fragment">Nash Equilibrium</li>
            <li class="fragment">Equilibrium Selection</li>
        </ul>
    </li>
    <li class="fragment">Workshop & Activities
        <ul>
            <li class="fragment">Discussions & more games</li>
        </ul>
    </li>
</ul>

---

## What is Game Theory?

> Game theory studies strategic situations where the outcome for each participant or 'player' depends on the actions of all others. It formulates models to represent these scenarios and predicts the most likely or optimal outcomes.

---

## Game Theory in Web3

In the context of blockchains, game theoretic reasoning is used for <span style="font-style: italic;">modelling</span> and <span style="font-style: italic;">understanding</span>.

---

## Modelling

<ul>
<li class="fragment"><strong>Tokenomics:</strong> Macroeconomic design of a token (inflation, utility, etc.).</li>
<li class="fragment"><strong>Business Logic:</strong> Interaction of the token with different modules of a protocol.</li>
<li class="fragment"><strong>Consensus:</strong> Providing sufficient incentives to guarantee that participating nodes agree on a distributed state of the network.</li>
<li class="fragment"><strong>Collaboration:</strong> Nudging (aggregated) human behavior and their interaction with the protocol.</li>
</ul>

---

## Understanding

<ul>
<li class="fragment"><strong>Economics:</strong> Interaction between different protocols and how finite resources are allocated among all of them.</li>
<li class="fragment"><strong>Security:</strong> Testing economic security of protocols against various types of attacks.</li>
</ul>

---

## History of Game Theory

<ul>
<li class="fragment">Early “game theoretic” considerations going back to ancient times.</li>
<li class="fragment">“Game theoretic” research early 19th century, still relevant.</li>
<li class="fragment">The systematic study of games with mathematically and logically sound frameworks started in the 20th century.</li>
<li class="fragment">Modern game theory is used in economics, biology, sociology, political science, psychology, among others.</li>
<li class="fragment">In economics, game theory is used to analyze many different strategic situations like auctions, industrial economics, and business administration.</li>
</ul>

---

## Game theory is <span style="font-style: italic;">abstract</span>

<ul>
<li class="fragment">Game theoretic models aim to get at the essence of a given strategic problem.</li>
<li class="fragment">This often requires many simplifying assumptions.</li>
<li class="fragment">Pro: <em>Abstraction</em> makes the problem amenable to analysis and helps to identify the key incentives at work.</li>
<li class="fragment">Con: A certain lack of realism.</li>
<li class="fragment">In any case: Modeling a strategic situation always entails a tradeoff between tractability and realism.</li>
</ul>

---

# What is a Game?

---

## Definition: (Economic) Game

<ul>
<li class="fragment">A game is a strategic interaction among several players, that defines <em>common knowledge</em> about the following properties:</li>
<li class="fragment">all the possible <em>actions</em> of the players</li>
<li class="fragment">all the possible <em>outcomes</em></li>
<li class="fragment">how each combination of actions affects the outcome</li>
<li class="fragment">how the players value the different outcomes</li>
</ul>

---

## Definition: Common Knowledge

<ul>
<li class="fragment">An event $X$ is common knowledge if:</li>
<ul>
<li class="fragment">everyone knows $X$,</li>
<li class="fragment">everyone knows that everyone knows $X$,</li>
<li class="fragment">everyone knows that everyone knows that everyone knows $X$,</li>
<li class="fragment">... and so on ad infinitum.</li>
</ul>
</ul>

---

### Examples: Common Knowledge

<pba-cols>
<pba-col>

##### Auctions

- Actions: Bids.
- Outcome: Winner and Payment.

</pba-col>
<pba-col>

##### Price-competition<br/>between firms

- Actions: Price charged.
- Outcome: Demand for each firm, profit of each firm.

</pba-col>
</pba-cols>

---

## Types of games

Game theory distinguishes between:

<ul>
<li class="fragment">static & dynamic games</li>
<li class="fragment">complete & incomplete information games</li>
</ul>

---

## Static and Dynamic Games

|                  | Static Game                                                          | Dynamic Game                                                                                           |
| ---------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| Definition       | All players take their actions at the same time                      | Players move sequentially and possibly multiple times, (at least partially) observing previous actions |
| Simple Example   | Rock-Paper-Scissors                                                  | Tic-Tac-Toe                                                                                            |
| Economic Example | Sealed-bid auction.                                                  | English Auction.                                                                                       |
|                  | All bidders submit their bids simultaneously (in a sealed envelope). | Auctioneer publicly raises price if at least one bidder accepts the price.                             |
| Representation   | Payoff Matrix                                                        | Decision Tree                                                                                          |

---

## Completeness of Information in Games

|                       | Game of Complete Information                          | Game of Incomplete Information                             |
| --------------------- | ----------------------------------------------------- | ---------------------------------------------------------- |
| Information available | All information relevant to decision-making is known. | Not all information relevant to decision-making is known.  |
| Simple Example        | Chess                                                 | Poker                                                      |
| Economic Example      | Sealed auction for seized Bitcoin.                    | Used-car market: the resale value of a used car is opaque. |

Notes:

- There is also the notion of perfect and imperfect information which we should skip here.
  More info: https://economics.stackexchange.com/questions/13292/imperfect-vs-incomplete-information

---

### Three firms want to hire an engineer...

<pba-cols>
<pba-col>

- The engineer brings added value to each firm of 300,000 USD per year.
- The payoff of the firm is known by everyone to be 300,000 USD minus the salary.
- The payoff to the engineer is salary minus cost of working, which is known to everyone.
- All firms make a salary offer at the same time.

</pba-col>
<pba-col>

### Quiz Questions:

1.  Is this game static or dynamic?
    What would need to change in the description of the game such that it would fall in the other category?
1.  Is this game of complete or incomplete information?
    What would need to change in the description of the game such that it would fall in the other category?

</pba-col>
</pba-cols>

---

## Utility

<ul>
    <li class="fragment">Core concept of Game Theory.</li>
    <li class="fragment">Can transform any outcome to some value that is comparable.
        <ul>
            <li class="fragment">For example: What is better? Scenario A: Going for a vacation to France and drink Wine, or Scenario B: going to Germany and drink Beer?</li>
            <li class="fragment">Those dimensions are only comparable if we give both some value like U(A) = 5, U(B) = 3.</li>
        </ul>
    </li>
    <li class="fragment">Essential assumption: Agents are utility maximizers.</li>
    <li class="fragment">Monetary payouts can also be transformed to utility.
        <ul>
            <li class="fragment">Simplest assumption U(x) = x.</li>
            <li class="fragment">But that is likely not true in reality.</li>
        </ul>
    </li>
    <li class="fragment">Most things have <strong>diminishing rates of returns</strong>.</li>
</ul>

---

<!-- .slide: data-background-color="#4A2439" -->

# Common Games

---

## Prisoners' Dilemma

A fundamental problem:

> Even though everyone knows there is a socially optimal course of actions, no one will take it because they are rational utility maximizers.

It's a static game of complete information.

---

## Bonnie and Clyde

Bonnie and Clyde are accused of robbing two banks:

<ul>
<li class="fragment">The evidence for the first robbery is overwhelming and will certainly lead to a conviction with two years of jail.</li>
<li class="fragment">The evidence for the second robbery is not sufficient and they will be convicted only if at least one of them confesses to it.</li>
</ul>

---

## Bonnie and Clyde

In the interrogation they both are offered the following:

<ul>
<li class="fragment">If you both confess you both go to jail for 4 years.</li>
<li class="fragment">If you do not confess but your partner does, you go to jail for 5 years: 1 extra year for obstruction of justice.</li>
<li class="fragment">However, if you confess but your partner does not, we reduce your jail time to one year.</li>
</ul>

---

## Bonnie and Clyde

- **Cooperate** ($C$) with each other and not say anything
- **Defect** ($D$) and confess their crime

<div style="text-align: center;">
    <img style="width: 700px; margin: auto; display: block;" src="./img/Bonnie-clyde-1.png" />
</div>

---

## Nash-Equilibrium

<ul>
<li class="fragment">Fundamental concept in Game Theory</li>
<li class="fragment">A NE is a set of strategies, one for each player, such that no player can unilaterally improve their outcome by changing their strategy, assuming that the other player's strategy remains the same.</li>
</ul>

---

## Finding the NE

<div style="text-align: center;">
    <img style="width: 1200px; margin: auto; display: block;" src="./img/2.2-PD-NE-1.png" />
</div>

---

## Finding the NE

<div style="text-align: center;">
    <img style="width: 1200px; margin: auto; display: block;" src="./img/2.2-PD-NE-2.png" />
</div>

---

## Prisoners' Dilemma IRL

<div style="text-align: center;">
    <img style="width: 1200px; margin: auto; display: block;" src="./img/2.2-PD-NE-4.png" />
</div>

## Finding the NE

<div style="text-align: center;">
    <img style="width: 1200px; margin: auto; display: block;" src="./img/2.2-PD-NE-5.png" />
</div>

---

## Finding the NE

<div style="text-align: center;">
    <img style="width: 1200px; margin: auto; display: block;" src="./img/2.2-PD-NE-6.png" />
</div>

---

## Nash Equilibrium

- Remember: a Nash Equilibrium describes a set of strategies such that no player can unilaterally increase their payout assuming the other player's strategy is fixed.
- A Nash Equilibrium **does not mean** that payouts of players are maximized (either individually or collectively)!

---

## Dominant Strategy

<img rounded style="width: 800px; margin-right: 250px;" src="./img/Bonnie-clyde-2.png" />

Choosing D is a <span style="font-style: italic;">dominant strategy</span>: a strategy that is always optimal for a player, regardless of what the other players do.

---

## Prisoners' Dilemma IRL

<ul>
<li class="fragment"><strong>Nuclear Arms Race:</strong> <a href="https://www.history.com/topics/cold-war/arms-race">NATO and Russia</a> prefer no arms race to an arms race. Yet, having some arms is preferable to having no arms, irrespective of whether the other one is armed.</li>
<li class="fragment"><strong>OPEC:</strong> <a href="https://www.opec.org/opec_web/en/">Limiting oil supply</a> is in the best interest of all. However, given the high price that thus results, everyone has an incentive to increase individual oil supply to maximize profits.</li>
</ul>

---

## Ultimatum Game

<ul>
    <li class="fragment">We played it before.</li>
    <li class="fragment">Sequential game.</li>
    <li class="fragment">The Nash Equilibrium can be reasoned by <strong>backwards induction.</strong></li>
    <li class="fragment">The proposer has the following considerations:
    <ul>
        <li class="fragment">What would the recipient accept?</li>
        <li class="fragment">Answer: every payoff (larger than 0).</li>
        <li class="fragment">Therefore, I should offer, since I want to maximize my payout, something equal or slightly higher than 0.</li>
    </ul>
    </li>
    <li class="fragment">That means, the proposer offering something small and the recipient always accepting is the only NE.</li>
</ul>

---

## Coordination Game

<ul>
<li class="fragment">The prediction of play in the Prisoner's Dilemma was easy: both will defect.
  <ul>
  <li class="fragment">This is the optimal thing to do no matter what the other player does.</li>
  </ul>
</li>
<li class="fragment">In other games, predictions of play are not so clear.
  <ul>
  <li class="fragment">One instance is the coordination game.</li>
  </ul>
</li>
</ul>

---

## Coordination Game

> A coordination game is a type of static game in which a player will earn a higher payoff when they select the same course of action as another player.

---

## Coordination Game Example

<pba-cols>
<pba-col>

- Choose $L$ or $R$.
- The numbers represent the payoffs a player receives.
- The players only obtain utility if they coordinate their actions.

</pba-col>
<pba-col>

<img rounded style="width: 550px;" src="./img/Players.png" />

</pba-col>
</pba-cols>

---

## Coordination Game Example

<pba-cols>
<pba-col>

- The coordination game has two outcomes $(L,L)$ and $(R,R)$ that stand out.
- Clearly, if the other player chooses $L$ ($R$), then it is optimal for the other to do so also.
- So, in the outcomes $(L,L)$ and $(R,R)$ the players choose mutually optimal actions.

</pba-col>
<pba-col>

<img rounded style="width: 550px;" src="./img/Players.png" />

</pba-col>
</pba-cols>

---

## Coordination Game Example

<pba-cols>
<pba-col>

- Both $(L,L)$ and $(R,R)$ are instances of Nash equilibrium.
- By their very nature, coordination games always have multiple equilibria.
- The outcome $(D,D)$ in the Prisoner's dilemma is the unique Nash equilibrium.

</pba-col>
<pba-col>

<img rounded style="width: 550px;" src="./img/Players.png" />

</pba-col>
</pba-cols>

---

## Equilibrium selection

<ul>
    <li class="fragment">So, which outcome does the theory of Nash equilibrium predict in the coordination game?</li>
      <ul>
      <li class="fragment">None? Both?</li>
      </ul>
    <li class="fragment">Sometimes people switch between equilibria (if they are made to)...</li>
</ul>
<div class="fragment">
    <img rounded style="width: 550px;" src="./img/2.2-sweden-1967.jpg" /><br/>
    <p>Sweden, 1967.</p>
</div>

---

## Schelling Points

<ul>
<li class="fragment">Nash equilibrium <span style="font-style: italic;">does not</span> predict which strategies the players actually take.</li>
<li class="fragment">This is especially pronounced in games with multiple equilibria (e.g., coordination games).</li>
<li class="fragment">There are theories that offer insights into which strategies players actually take.</li>
</ul>

---

## Schelling Points

> If you are to meet a stranger in New York City, but you cannot communicate with the person, then when and where will you choose to meet?

<ul>
<li class="fragment">Literally any point and time is a Nash equilibrium...</li>
<ul>
<li class="fragment">However, most people responded: noon at (the information booth at) Grand Central Terminal.</li>
</ul>
<li class="fragment">Basic idea: in case of multiple equilibria, social norms may help to choose one.</li>
</ul>

---

# Summary (so far...)

<ul>
<li class="fragment">Typology of games: static/dynamic, complete/incomplete information.</li>
<li class="fragment">Three canonical games: Prisoner's Dilemma, Ultimatum-, and Coordination Game.</li>
<li class="fragment">The Prisoner's Dilemma has a unique Nash equilibrium, which is dominant, whereas the Coordination game has two Nash equilibria.</li>
<li class="fragment">To select among multiple equilibria, the concept of a <span style="font-style: italic;">Schelling Point</span> is sometimes used.</li>
</ul>

---

## Why are theories of equilibrium important?

<ul>
<li class="fragment">Nash Equilibria are used to predict the behavior of others in a closed system.</li>
<li class="fragment">If you can identify a unique Nash Equilibrium or the Schelling point in a system, you have a strong prediction of user behavior.</li>
<li class="fragment">So, you can begin to drive user behavior by designing incentives accordingly.</li>
</ul>

---

## Public Goods

<ul>
<li class="fragment"><strong>Non-excludable</strong> No-one can be excluded from consumption</li>
<li class="fragment"><strong>Non-rivalrous</strong> My consumption does not affect yours</li>
<li class="fragment">e.g., fireworks, street-lighting.</li>
</ul>

---

## Common Goods

<ul>
<li class="fragment"><strong>Non-excludable</strong> No-one can be excluded from consumption</li>
<li class="fragment"><strong>Rivalrous</strong> My consumption reduces your possibility to consume</li>
<li class="fragment">i.e., a public park, an office coffee machine.</li>
</ul>

---

## Public vs. Common Good

<ul>
<li class="fragment">Main difference is that in a <span style="font-style: italic;">common good</span> your consumption reduces the value of the good to others.</li>
<li class="fragment">This is called a <span style="font-style: italic;">consumption externality</span> that you impose on others (and others impose on you.)</li>
<li class="fragment"> The <span style="font-style: italic;">tragedy of the commons</span> is that, because you do not take this externality into account, consumption is higher than would be socially optimal. </li>
</ul>

---

## Stylized Public Good Game:

<ul>
<li class="fragment">$N$ players have 10 US dollars each, say, $N=4$.</li>
<li class="fragment">Each player can choose how much to place into a project.</li>
<li class="fragment">Funds in the project are magically multiplied by a factor $\alpha$, say, $\alpha=2$.</li>
<li class="fragment">Finally, the funds in the project are split equally among all players.</li>
<ul>
  <li class="fragment">What would be best for the individual?</li>
  <li class="fragment">What would be best for the collective?</li>
<ul>
</ul>

---

## Overfishing

<ul>
<li class="fragment">Fishing gives private benefit but might destroy the broader ecosystem, which has its own value for everyone (e.g., due to tourism).</li>
<li class="fragment">Because individual fishermen do not pay for the damage they cause to the broader ecosystem, they will fish too much.</li>
</ul>

---

## Air pollution

<ul>
<li class="fragment">Producing a good yields private profit but reduces air quality for everyone.</li>
<li class="fragment">Because there is no price attached to air quality, the firms do not have to pay for its reduction and, hence, will produce too much.</li>
</ul>

---

## But...

<ul>
<li class="fragment">There should be fishing/production/mining! After all, there are always benefits to these activities.</li>
<li class="fragment">The tragedy of the commons is that the externality is not priced into these activities, driving them to inefficiently high levels.</li>
</ul>

---

<!-- .slide: data-background-color="#000" -->

# Break (10 minutes)

---

<!-- .slide: data-background-color="#4A2439" -->

# Open Source

> Providing open-source software is like contributing to a public good and the community will therefore sooner or later collapse!

---

<!-- .slide: data-background-color="#4A2439" -->

# Design a 2x2 game

> Jack and Christine are rivals and keep taunting each other in front of others.
> At one time, Jack challenges Christine to a game of chicken.
> He proposes that they both get in their cars and drive towards each other on a road.
> In the middle of the distance between each other, there is a small bridge with a single lane.
> Whoever swerves away before the bridge <span style="font-style: italic;">chickened out</span>.
> If both keep straight, there is no way to avoid a strong collision between the two cars.
> All friends will be present to see the result.

Design this game in a 2x2 matrix and assign payoffs to the different outcomes.

---

## Design a 2x2 game

- What is/are the Nash Equilibrium/Equilibria here?
- Which type of games does this remind you of?
- How would you translate this game to real scenarios?

---

<!-- .slide: data-background-color="#4A2439" -->

# Workshop: Games

---

## Game 1: Guessing Game

- We divide the classroom into three groups and play a guessing game.
- The budget for this game is: $320.
- The game is simple: each player enters a number from 1 to 100.
- The player who guessed closest to 2/3 of the average number wins.
- You can only choose integer values.
- If multiple people win, the payoff is split equally.
- The game is repeated for ten rounds.

---

<!-- .slide: data-background-color="#4A2439" -->

## Game 1: Questions?

### Don't ask about strategies!

---

## Game 1: Guessing Game

Link will be distributed!

---

## Game 1: Discussion

- What number did you choose / what was your strategy?
  (which group were you in?)
- Did your strategy change over time?

---

---

## Game 1: Results!

---

## Game 2: Prisoner's Dilemma

- You play a Prisoner's Dilemma (groups of 2) over 10 rounds.
- You will be randomly matched to another student in the class.
- Budget for this game: $670
- You have the option to chat between rounds.
- Important: Keep the chat civil and do not reveal any identifying information about yourself.
- We will read the chat.

---

## Game 2: Payoffs

|           |             | **_The other participant_** |                        |
| --------- | ----------- | --------------------------- | ---------------------- |
|           |             | _Cooperate_                 | _Defect_               |
| **_You_** | _Cooperate_ | 200 points, 200 points      | 0 points, 300 points   |
|           | _Defect_    | 300 points, 0 points        | 100 points, 100 points |

---

<!-- .slide: data-background-color="#4A2439" -->

## Game 2: Questions?

### Don't ask about strategies!

---

## Game 2: Let's go!

Link will be distributed!

---

## Game 2: Results!

---

## Repeated Prisonner's Dilemma

<ul>
    <li class="fragment">Prof. Robert Axelrod held a tournament in the 1980s to find out which computer program ("strategy") would perform best against others.</li>
    <li class="fragment">In the first tournament, 14 strategies were submitted by leading Game Theorists and others in the field.</li>
    <li class="fragment">The game would be repeated 200 times.</li>
    <li class="fragment">Each strategy would play against every other strategy and against itself.</li>
</ul>

Notes:

- Based on https://ve42.co/Axelrod1980a and https://www.youtube.com/watch?v=mScpHTIi-kM

---

## Repeated Prisonner's Dilemma

<ul>
    Some famous strategies were:
    <li class="fragment">Being sneaky: Cooperate but smuggle in some defections.</li>
    <li class="fragment">Grim trigger: Always cooperate but retaliate and defect forever, if the other defect once.</li>
    <li class="fragment">Tit-For-Tat: Start with cooperate and copy the opponent's strategy of the last round.</li>
</ul>

---

## Repeated Prisonner's Dilemma

<ul>
    <li class="fragment">Tit-for-Tat was the best-performing strategy.</li>
    <li class="fragment">Prof. Axelrod was able to deduce four main aspects of well-performing strategies:
        <ul>
            <li class="fragment">First: Be nice (do not defect first).</li>
            <li class="fragment">Second: Be forgiving (retaliate but do not hold a grudge).</li>
            <li class="fragment">Third: Be retaliatory (strike back).</li>
            <li class="fragment">Fourth: Be clear (signal a simple strategy so the other knows who they are playing against).</li>
        </ul>
    </li>
</ul>

Notes:

- This was rather big surprise, especially since dominant strategy is being nasty.
- A lot of it is what we can observe in today's politics.

---

## Repeated Prisonner's Dilemma

<ul>
    <li class="fragment">There is no single best strategy.</li>
    <li class="fragment">It always depends on who you are playing against.</li>
    <li class="fragment">Simulations in a later study show that a cluster of tit-for-tat players within a defecting population can grow and overtake defecting types.</li>
</ul>

---

## Game 3: Public Good Game

- We will play a public good game as presented in the lesson.
- Budget for this game: $670
- Groups of 4 over 10 periods.
- Points in the project is multiplied by factor $1.6$.
- With one additional mechanism: After each round each player sees the contributions of the other players and can decide to deduct points from them (at own costs).

---

## Game 3: Instructions

<img rounded style="width: 1000px;" src="./img/2.2-pgg-instructions.png" />

---

## Game 3: Contribution

<img rounded style="width: 1000px;" src="./img/2.2-pgg-contribution.png" />

---

## Game 3: Punishment

<img rounded style="width: 900px;" src="./img/2.2-pgg-punishment.png" />

---

## Game 3: Payout

<img rounded style="width: 1000px;" src="./img/2.2-pgg-result.png" />

---

<!-- .slide: data-background-color="#4A2439" -->

## Game 3: Questions?

### Don't ask about strategies!

---

## Game 3: Let's go!

Link will be distributed!

---

## Game 3: Discussion

- What was your strategy?
- Were your groups able to sustain cooperation?
- Did you cooperate?
- Did you punish?

---

<!-- .slide: data-background-color="#4A2439" -->

## Game 3: Results!

---

## Game 3: Discussion

<ul>
    <li class="fragment">How could we characterize players to types?
     <ul>
            <li class="fragment">Freerider</li>
            <li class="fragment">Cooperators</li>
            <li class="fragment">Altruists</li>
        </ul>
    <li class="fragment">What do you think happens when playing this ...
        <ul>
            <li class="fragment">... for one round?</li>
            <li class="fragment">... for many rounds?</li>
            <li class="fragment">... when allowing for communication?</li>
            <li class="fragment">... with different group sizes?</li>
        </ul>
    </li>
</ul>

---

## Summary

- the basics of game theoretic concepts.
- different types of games.
- how games can be modeled.
- how to apply game theoretic thinking in our decision making in certain games.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions

---

## Further Reading

- [Comprehensive introduction to Game Theory](https://plato.stanford.edu/entries/game-theory/)
- [Great video about the Prisonner's Dilemma](https://www.youtube.com/watch?v=mScpHTIi-kM)
