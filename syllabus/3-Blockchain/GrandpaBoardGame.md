# Grandpa - The Board Game

- Players: 5+ (4 actual players, 1 author)
- Play time: 15 - 60 min
- Materials: A large whiteboard and many colored markers

## Overview

Grandpa is a Byzantine fault tolerant blockchain finality gadget ([formal spec](https://arxiv.org/pdf/2007.01560.pdf)). This collaborative board game allows players to learn and practice the Grandpa protocol while also having fun together.

Your goal in the game is to finalize blocks in an ever-growing blockchain data structure. You will work together to share information with other players and reach consensus. But watch out; some players may be Byzantine!

<figure>
    <img src="./img/generals-board-game.jpg" />
    <figcaption><b>Byzantine Generals solving distributed consensus via a board game.</b></figcaption>
</figure>

Some less important details of the grandpa protocol (such as the notion or primaries, and the timeout conditions) are omitted from the board-game for the sake of playability and clarity.

## Setup

Select one participant to act as the "author" who is responsible for creating the blockchain structure, but will not actually play the grandpa protocol. The remaining participants are all players in the grandpa protocol.

Give one marker to each participant including the author. Each player should have their own marker color. Avoid colors that are hard to distinguish such as light red and pink. If you have colorblind players, take special care when choosing marker colors.

Choose a goal number of blocks that you wish to finalize together as a team. The game will end when you reach this block number.

### Views and Gossip

The grandpa protocol operates in a decentralized asynchronous blockchain network. As such, there is no universal view of the blockchain data structure or the messages that are being passed between players in the protocol. Some players may see more information than others and information may arrive to the various players in different orders.

Divide the whiteboard into a dedicated space for each player in the protocol. Each player should have roughly 50cm X 50cm. The author does not need their own dedicated space.

TODO FIGURE

Throughout the game all participants including the author are responsible for communicating with other players by adding information _to other players dedicated spaces_. In fact, most of the marks that you make during the game will be on someone else's space rather than your own. For a more realistic game, take care to share information with other players in a different order each time.

### Genesis Block

Before game play begins, the author draws a single genesis block labeled `G` on each player's view. Each player marks the genesis block as final by shading it with their color in their own view.

TODO Figure

#### Simplified Version - A Universal View

If you are relatively new to the game or the grandpa protocol itself, it may be helpful to play a few rounds with a single universal view. While this approach is less realistic, it removes some complexity and opportunities for confusion. In this simplified mode, every player considers the single universal view to be their own view. Once you have played a few rounds, you should switch to the more realistic mode where each player has a unique view.

TODO Figure (including finalized genesis block)

## Authoring

The author is responsible for creating the blockchain data structure and gossiping it to the players. As the game progresses the author will grow the blockchain by creating descendant blocks of this genesis block. The author may create blocks anywhere in the chain they see fit. They may create forks, or linear runs without forks. They may create a long chain and then go back and create shorter forks from earlier in the chain.

When the author creates a block they should gossip it to all players by drawing it on each player's view. A block is drawn with a pointer to its parent block and a short unique block identifier like a few characters or digits. The author should take care to vary to order in which they place new blocks on various players' views. In fact, the author may even gossip multiple blocks to a single player before going back and gossiping any of them to other players. However the author should ensure that all blocks are eventually gossiped to all players.

TODO Figures

In some ways the author acts as a "party host" or "dungeon master" for the game. They should observe the players progress, struggles, and enthusiasm, and author accordingly. If players are struggling to keep up or getting frustrated or overwhelmed the author should slow down the authoring rate or build a simpler chain with fewer forks. If players are easily finalizing blocks or getting bored the author should speed up or create more complex tree structures with many forks.

When playing the unified view mode, the author should only create blocks that are descendants of the latest finalized block. In the more realistic mode, the notion of latest finalized block is a little more vague, but still the author should avoid extending chains that have been ruled out by all or even most players.

TODO Figure

## Game Play

The Grandpa protocols proceeds in rounds. Each player has their own view of what round they are on, and not all players will be on the same round at the same time. In each round, each player casts two votes known as the "prevote" and "precommit" in that order. Each player begins in round 1.

Like many other BFT protocols, Grandpa requires strictly greater than 2/3 of players (not counting the author) to be properly following the protocol. For the remainder of this section this will be referred to as a "threshold".

### Prevoting

Each player begins a round by casting their prevote. A prevote can be thought of as a non-binding signal for what the player hopes to finalize in this round. A player casts their prevote by writing the current round number off to the right of the block they are prevoting for first on their own view, and then on other players' views. Remember you should send your prevotes out to other players in a different order each time, and it is okay to allow some latency between sending it to each player.

TODO Figure

TODO what if a player hasn't yet seen the block I'm prevoting for. Some options:
1. Add the block to their view in addition to the prevote. Models gossip and block requests.
2. Add your prevote for the closest ancestor that they have
3. Don't add your prevote at all
4. Add your prevote off to the side somewhere so they can put it in place once they hear about the block.

### The Prevote Ghost

Once a player has seen a threshold of prevotes in the current round, they can mark the round's "Prevote Ghost" on their own view. The prevote ghost is defined as the highest block that has a threshold of prevotes, and it is marked by drawing the letters `PG` and a round number off to the left of the block. For example `PG4` for round four's prevote ghost. (Or optionally `👻4` if you are feeling artistic).

TODO Figure

Note that the prevote ghost may change over time as more votes come in. You may update your markings if you wish and doing so may help you finalize more blocks. However, the protocol does not required players to update it; after all, players may never even have seen those later votes.

### The Estimate

Once the prevote ghost is marked, each player marks the estimate on their own view to the left of the same block that is the prevote ghost with the letter `E` and a round number. For example, `E4` for round four's estimate.

TODO Figure

A round's estimate is defined as the highest block that is in the chain of the prevote ghost that could possibly achieve a threshold of precommits. So while the estimate _begins_ at the same block as the prevote ghost, it may change as more votes come in. It is especially likely to change if you are also willing to update your prevote ghost as described in the previous section.

### Precomitting

Once you have marked a prevote ghost and an estimate, you may wait a short time for any more prevotes to come in. Once you get tired of waiting (or when it is impossible for the prevote ghost to move any higher), you may cast your precommit for the block that you see as the prevote ghost. Mark your precommit first on your own view and then on other players' views by writing the round number off to the right of the block and circling it. Precommits are distinguished from prevotes by the circle. Remember not all players will agree on which block is the prevote ghost, so others may precommit for blocks different then you have. Mark your precommit for a given round by writing the round number in

TODO Figure

As you observe more prevotes appearing on your view, your estimate may change. Specifically it may move up the chain to ancestor blocks.

TODO Figure of estimate changing

### Completing a Round

We will decide that _some_ block is finalized in each round, although it may be a block that was already finalized in a previous round. We will only ever finalize an ancestor of the estimate. Once some ancestor of the estimate has achieved a threshold of precommits, you can declare that block finalized by shading it with your color on your view.

TODO Figure

After a round has completed, you may choose to erase the votes for that round from your view to keep the board tidy. But you are not required to do so. Be careful not to erase votes for _future_ rounds by accident as some players may have advanced to the next round before you.

Proceed to the next round

## Ending the Game

Grandpa is intended to continue finalizing blocks forever. Since you likely don't want to play this board game forever, the board game does have an end.

The honest players win when they all finalize the goal number of blocks chosen at the beginning without a safety violation.

The Byzantine players (if any; see next section) win when two honest players finalize conflicting blocks or the honest players get fed up and flip the whiteboard over.
## Byzantine Actors

Once you have played a few rounds of the game and are able reliably finalize new blocks, you can spice things up by assigning one or more players to be Byzantine. Byzantine players are not required to follow the protocol rules. For example they may:

- Prevote for chains that do not extend the latest finalized chain
- Precommit for blocks other than the ones indicated by the prevote
- Go back and cast votes in previous rounds
- Fail to participate at all.

When first adding Byzantine players, you may assign the Byzantine roles such that everyone knows who is Byzantine. Or, for a more realistic experience, you may assign it blindly by eg drawing straws. Remember that in order for Grandpa to work you must have strictly less than one third of grandpa players Byzantine.

For the most realistic experience, allow players to self select whether they are Byzantine. By doing this there is no guarantee that the honest super majority criteria is met and you experience safety faults where different players finalize conflicting chains.
