# Start your own blockchain

In this activity, students will get hands on experience operating a blockchain node and seeing it interact with other blockchain nodes.
In particular, they will use the [Academy PoW Node](https://github.com/Polkadot-Blockchain-Academy/Academy-PoW).

## Learning objectives:

1.  How to run a node
1.  What the log messages mean
1.  Preview: How to configure a network with a chain spec
1.  Experience peer connections coming and going
1.  Practice checking for common not-peering issues (different genesis, firewall)

## Outline of Activity

### Compile

Because the compile time is long, this should be done in advance.
If there is a notion of "homework" in the class, this should be given as homework on the day before.
If not, we could get the compile going before the lecture begins.
We should also have a docker image ready to go.

### Generate User Keys

With Polkadot JS

### Get Tokens

Students request tokens from the teacher by sharing their address with the teachers in a public channel.
After the first five students have tokens, they pass them on to their peers.

### Optional Treasure Hunt

You could have tokens stored at eg the Alice key or other leaked keys and challenge students to find and recover the tokens.
Make up a story/myth about some mysterious figure who lost their key and left clues, etc.

### Run Nodes

Students start their own nodes and join the network

### Start Mining

Node runners can contribute PoW hashrate to the network in exchange for tokens.


### Fork the Network

In BA we forked the difficulty, but maybe there is something more compelling. Maybe fork to adjust max block size. Nice because it happened for real in bitcoin. Maybe hard because I believe max block size is controlled in the runtime.

Another idea:
When we launch the chain, we allow PoW block to have either of two valid kinds of seals. For example, one based on sha3 and another based on keccak. Then we pretend there is a cultural war and some people like one function or the other wnad some people don't care. When the tiem comes to fork, we have three different versions of the node available: 1 only accepts sha3, 2 only accepts keccak, 3 is the original that accepts either. In this way we can see that there will be at least two viable chains (the sha 3 chain and the keccak chain). If there are a lot of nodes that still run the old version and accept either hash, they will form a third chain. But if there are only a few such nodes, they will re-org into one of the other two chains, and potentially reorg back and forth between them.

### Light Clients

TODO

### Other Infrastructure

Like block explorer or indexer?

### Smart Contracts

The Smart Contracts module has an activity about launching smart contracts on a running chain.
When these modules are run back-to-back, it makes an excellent learning experience to use this same chain we just launched to also launch the smart contracts.
