---
title: Peer-to-Peer (P2P) Networking
description: Peer-to-Peer (P2P) networking for Web3 engineers
---
# Peer-to-Peer Networking
Andrew Burger, Parity, Integritee, Blockchains Yay
---
## Introduction

---
## Centralized vs Decentralized networks
Notes:
    1.) Picture here showing the two different node overlays
    2.) Not all p2p clients must run the same software they can develop their own (BTC, Ethereum etc..) Further decentralization.
    3.) No single node or nodes(CDN) have access to all of the content or files. Each node has a copy(Once the network is initialized)
---
## Advantages to Decentralized networks
* No single node or nodes(CDN) have access to all of the content or files or is critical for operating the network. Each node has a copy of the data.
* No bottlenecks with bandwidth
* Latency may be an issue if we need to wait for many peers to receive the data produced from a single node since everyone may not have a direct connection
* Difficult to overload the network or DOS (Not single node is privledged)
* No centralized infrastructure necessary(Except internet for now..)
Notes: 
    Block production and Block peering/importing can be mentioned here
---
## Difficulties or disadvantages
* Since it is permisionless a node can share malicious resources
* Difficult to regulate illicit activity
* The network is limited by nodes with the weakest hardware(Why we have hardware requirements for blockchain networks)
---

## Initial discovery
* Must know someone who is participating in the network initially
* Bootnode/bootnodes (More on this later in Substrate) 
---
## Gossip Protocol
Notes:
    Some picture showing one node looking for a packet via another node and asking its neighbors for packets
---
## Discovery
* 1.) Connect to a peer
* 2.) Ask peer for a list of their known nodes(Addresses to fill DHT)
* 3.) Connect to random subset of peers from the list 
* 4.) Repeat steps 2 and 3 
---
## Paritions
Notes: 
    Talk about how when a partition happens in P2P vs Centralized
    In p2p only one node needs to have a full copy in order for the file to
    beable to be distributed accross the network
---
## Attacks
Notes: Eclipse Attacks
---
## Preventing Attacks
Notes: Concise but revealing here..
---
## libp2p
Notes:
What is libp2p
---
## Addressing
---
## Protocols
* Ping
* Identify
* Kad-dht
Notes: Overview of a few protocols from docs 
---
## KAD-DHT
---
## Peers
* PeerId
* PeerStore
* PeerDiscovery
Notes: Give some information on each 
---
## Transports
* TCP
* UDP
* QUIC and more..
Notes: Talk here about how you can use not only TCP but various other transports..
---
## Stream Multiplexing
Notes: Very Brief overview
---
## NAT
Notes: Very Brief overview
---
## Security and Maliciousness
---
## DOS
---
## Additional Resources
* https://curriculum.pl-launchpad.io/curriculum/libp2p/
* https://docs.libp2p.io/concepts/
---
