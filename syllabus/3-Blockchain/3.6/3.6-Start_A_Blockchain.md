# Start your own blockchain

In this activity, students will get hands on experience operating a blockchain node and seeing it interact with other blockchain nodes.
In particular, they will use the Substrate node template software.

This activity is basically a guided walkthrough of the start a [private network (trusted node) tutorial](https://docs.substrate.io/tutorials/get-started/trusted-network/) combined with a classwide network a la the [old Substrate beginner workshop](https://github.com/substrate-developer-hub/substrate-beginner-workshop)

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

### Alice and Bob Network

For the first network, we will just have two authorities `Alice` and `Bob`.

- Choose two volunteers to run with the `--alice` and `--bob` flags.
  Everyone else is a full node
- Start nodes and wait for peer connections.
- Discuss the log messages
- Look at telemetry
- Kill your node for ~30 sec and then bring it back up to see it catch up syncing

### Different Authorities

- Here we instill the core ideas of a genesis configuration.
- We start the network over and give more students a chance to be authorities.
- We choose a new alice and bob.
- But we want to do more than two at a time, so we add Charlie, DAve, et al to the genesis config.

### Key Leak

- Now we simulate what happens when a session key is leaked.
- We choose a few more people to restart their nodes with --alice and --bob.
- This drives home that these keys are only for learning, and also motivates trying it again with our own keys.

### Own keys

Optional, as this probably will not fit at this stage in the course.
But if there is free time, it's good to have more activities to fill it.
Also this could be done at _any time_ later in the course if we need to fill time.

Here we generate our own keys with the node template's `key` subcommand, and instert tham into the genesis config.

## Discussion Topic

### How can we have authorities change over time?
