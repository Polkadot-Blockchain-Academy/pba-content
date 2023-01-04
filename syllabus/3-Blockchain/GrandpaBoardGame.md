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

## Setup

Select one participant to act as the "author" who is responsible for creating the blockchain structure, but will not actually play the grandpa protocol. The remaining participants are all players in the grandpa protocol.

Give one marker to each participant including the author. Each player should have their own marker color. Avoid colors that are hard to distinguish such as light red and pink. If you have colorblind players, take special care when choosing marker colors.


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

When the author creates a block they should gossip it to all players by drawing it on each player's view. The author should take care to vary to order in which they place new blocks on various players' views. In fact, the author may even gossip multiple blocks to a single player before going back and gossiping any of them to other players. However the author should ensure that all blocks are eventually gossiped to all players.

TODO Figures

In some ways the author acts as a "party host" or "dungeon master" for the game. They should observe the players progress, struggles, and enthusiasm, and author accordingly. If players are struggling to keep up or getting frustrated or overwhelmed the author should slow down the authoring rate or build a simpler chain with fewer forks. If players are easily finalizing blocks or getting bored the author should speed up or create more complex tree structures with many forks.

When playing the unified view mode, the author should only create blocks that are descendants of the latest finalized block. In the more realistic mode, the notion of latest finalized block is a little more vague, but still the author should avoid extending chains that have been ruled out by all or even most players.

TODO Figure

## Game Play

TODO

## Byzantine Actors

Once you have played a few rounds of the game and are able reliably finalize new blocks, you can spice things up by assigning one or more players to be Byzantine. Byzantine players are not required to follow the protocol rules. For example they may:

- Prevote for chains that do not extend the latest finalized chain
- Precommit for blocks other than the ones indicated by the prevote
- Go back and cast votes in previous rounds
- Fail to participate at all.

When first adding Byzantine players, you may assign the Byzantine roles such that everyone knows who is Byzantine. Or, for a more realistic experience, you may assign it blindly by eg drawing straws. Remember that in order for Grandpa to work you must have strictly less than one third of grandpa players Byzantine.

For the most realistic experience, allow players to self select whether they are Byzantine. By doing this there is no guarantee that the honest super majority criteria is met and you experience safety faults where different players finalize conflicting chains.