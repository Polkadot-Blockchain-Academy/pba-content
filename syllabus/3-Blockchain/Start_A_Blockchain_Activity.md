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
This manual approach allows students to get familiar with sending and receiving tokens in their wallet.

Optionally, you may install a faucet or UBI pallet in the chain to allow students to self service _after_ the initial manual distribution.

### Optional Treasure Hunt

You could have tokens stored at eg the Alice key or other leaked keys and challenge students to find and recover the tokens.
Make up a story/myth about some mysterious figure who lost their key and left clues, etc.

### Run Nodes

Students start their own nodes and join the network

### Start Mining

Node runners can contribute PoW hashrate to the network in exchange for tokens.

### Fork the Network

Our chain's PoW is based on md5 which is getting old and weak, so we upgrade to add the newer standard sha3 while keeping the original md5 in place to facilitate a smooth transition. Actually we decide to add both sha3 and keccak just in case the NSA fucked with sha3.

(If desired, insert the [Forks lesson](./7-Forks_slides.md) here.)

We find out about a practical attack against md5 and decide to remove it.

Finally, there is a contentious fork. Some of the community are keccak maxis, others are sha3 maxis, others don't have a preference and are okay to allow both until the dust settles.

There could be three chains after step seven. But we want a relatively stable chain for the contracts work in the afternoon, so we try to guide the class toward agreeing on a single chain if it is not happening naturally.

### Smart Contracts

The Smart Contracts module is run immediately after, it makes an excellent learning experience to use this same chain they just launched to also launch the smart contracts.
