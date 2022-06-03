# Lesson 7: Networking & RPCs

## Why is this topic Important?

- Substrate developers will not commonly interact with Networking, but should have a basic understanding of what is going on, how it works at a high level, and features that are provided in the case they run into errors or want to extend the default usage of LibP2P

## Core Ideas to Convey

- LibP2P
  - Multi-Address
  - Modular and Extensible
- Substrate Networking Protocols
  - We provide abstractions over LibP2P
    - BlockSync
    - Grandpa
    - Polkadot
    - Maybe more?
- Bootnodes
  - Future alternatives
- Peer Behaviors
  - Discovery
  - Banning
- Node Types
  - Light Client
  - Full Node
  - Block Author
- Network Topography
- Properly setting up different kinds of nodes
  - RPC Nodes
    - Firewall
    - Load Balancer
  - Block Producers
    - Isolated from everyone
- Common Developer Interactions
  - Failing to find peers
  - Connecting to a node which is not on the same computer; expose the websocket connections

## Prerequisite Knowledge or Skills

- Basic understanding of Peer to Peer networks
  - Limewire, Bittorrent
- Basics of Networking
  - UDP, TCP/IP
- Certificates

## Learning Outcome

- Be able to describe at a high level how networking works in Substrate, and how we have created new networking protocols for our needs.

## Learning Objectives

- Explain the trade-off of using P2P networking at all.
- Explain the advantages and disadvantages of LibP2P.
- Describe how LibP2P is utilized in Substrate, particularly with the extended protocols.

## Activities and Exercises

- Connect as a peer to Polkadot network
- Connect a front-end to a node in a docker environment or a separate machine
- Connect the whole class a single network, where someone random is selected as the bootnode.
  - Can they see a visualization of the network of nodes?
- ADVANCE: Enable IPFS node within Substrate
