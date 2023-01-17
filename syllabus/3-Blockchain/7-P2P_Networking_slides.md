---
title: Peer-to-Peer (P2P) Networking
description: Peer-to-Peer (P2P) networking for Web3 engineers
---
# Peer-to-Peer Networking
Andrew Burger, Parity, Integritee, Blockchains Yay
---
## Introduction/Agenda
* Discuss the network layer and network conditions that blockchains operate on(Mostly)
* Talk about traditional web2 network overlays pros vs cons with web3 network overlays 
* Discuss attacks and how to address along with the underlying threat model
* Libp2p
---
## Centralized vs Decentralized networks
Notes:
    2.) Not all p2p clients must run the same software they can develop their own (BTC, Ethereum etc..) Further decentralization.
---
## Advantages to Decentralized networks
* No privledged nodes
* Less bottlenecks with bandwidth
* DOS resistent
* No centralized infrastructure necessary(Except internet for now..)
Notes: 
    1.) No single node or nodes(CDN) have access to all of the content or files or is critical for operating the network. Each node has a copy of the data.
    <br>
    2.) No central node carrying all of the load of traffic. Block production and Block peering/importing can be mentioned here
    <br>
    3.) Difficult to overload the network or DOS (Not single node is privledged)
    <br>
    4.) Although many nodes are run on Centralized cloud compute platforms they dont have to be(Typically)
---
## Difficulties or disadvantages
* Since it is permisionless a node can share malicious resources
* Latency
* Difficult to regulate illicit activity
* The network is limited by nodes with the weakest hardware
Notes:
    2.) Latency may be an issue if we need to wait for many peers to receive the data produced from a single node since everyone may not have a direct connection
    <br>
    3.) No central point to go and snoop all users data(for better or for worse)
    <br>
    4.) Why we have hardware requirements for blockchain networks
---
## Initial discovery
* Bootnode/bootnodes (More on this later in Substrate) 
Notes:
    1.) Must know someone who is participating in the network initially(Bootnode)
---
## Gossip Protocol
<img style="width: 700px" src="../../assets/img/3-Blockchain/3.7-p2p-gossip-1.svg">
Notes:
   1.) Talk about how we have and want block 45 being peered to others 
---v
## Gossip Protocol
<img style="width: 900px" src="../../assets/img/3-Blockchain/3.7-p2p-gossip-2.svg">
Notes:
    Talk about advertising vs just blind sending and how that can be inneficient 
---
## Discovery
* 1.) Connect to a peer
* 2.) Ask peer for a list of their known nodes(Addresses to fill DHT)
* 3.) Connect to random subset of peers from the list 
* 4.) Repeat steps 2 and 3 
---
## Paritions
<img style="width: 800px" src="../../assets/img/3-Blockchain/3.7-p2p-partition.svg">
Notes: 
    Talk about how when a partition happens in P2P vs Centralized
    In p2p only one node needs to have a full copy in order for the file to
    beable to be distributed accross the network
---v
## Partitions cont..
<img style="width: 500px" src="../../assets/img/3-Blockchain/3.7-p2p-partition2.svg">
Notes:
    1.) This is horrible and means all nodes are totally screwed
---
## Attacks
Notes: Show picture of something scary and devious here 
---
## Eclipse Attack
<img style="width: 800px" src="../../assets/img/3-Blockchain/3.7-p2p-eclipse-topology.svg">
Notes:
    1.) Distorts view of the healthy normal honest state of the network 
    2.) Transaction confirmations can be fictious
---v
## Executing the Attack
- 1.) Flood a target node with a bunch of malicious peer addresses
- 2.) The targeted node then stores these maliciious peers and utilizes them when resyncing on next bootup
- 3.) DOS targeted node to take it offline to force a resync with these new malicious peers 
---
## Preventing Attacks
* Restrict inbound connections in some way and be wary of connections with other nodes
* Random selection of peers to connect with 
* Deterministic node selection, "Bootnodes" with higher credibility and trust (Can be a bottleneck)
* Restricting new nodes (Probably not what we want..)
Notes: Concise but revealing here..
       Important to remember Bootnodes are subject as well and should be rotated
---
## libp2p
* Toolbox for developing systems built ontop of the p2p networking 
* Simply put helpful in establishing encrypted and authenticated channels between two peers
Notes:
What is libp2p
---
## Addressing(MultiAddress)
- Generalization of an IP
- Multiaddress is to an IP address what a transport is to TCP/IP
- EX
    - /ip4/127.0.0.1/tcp/30333
    - /dns/example.com/udp/5015/quic
    - /ip6/fe80::0202:b3ff:fe1e:8329/tcp/10350/ws
Notes:
    Show example here, it is important for looking at chain-spec
---
## Protocols (Generic Protocol negotiation) 
* You can change your encryption protocol via the protocol negotiation!
* Ping
* Kad-dht 
Notes: Overview of a few protocols from docs 
---
## KAD-DHT
* Simply put a hash table containing a set of data entries these data entries are distributed accross the network
* There is no central registry where to obtain everything
* When we want some piece of data offered by the network we search for its distance to specific peers
---v
## DHT Operations in libp2p
* `FIND_NODE`: given a key, find the closest nodes to the key 
* `PUT_VALUE`: add a `key-value` mapping to the DHT
* `GET_VALUE`: given a key, retrieve the value
* `ADD_PROVIDERS`: advertising in the network that a peer is providing a given key
* `GET_PROVIDERS`: finding out what peers can provide the value for a specified key
---v
## Example Findkey(k=Block45)
<img style="width: 500px" src="../../assets/img/3-Blockchain/3.7-p2p-find-node-libp2p.svg">
---
## Peers
* PeerId
* PeerStore
* PeerDiscovery
Notes: Give some information on each 
---
## Transports
* TCP
* UDP(Cannot use UDP alone transports must be reliable and ordered)
* QUIC and more..
Notes: Talk here about how you can use not only TCP but various other transports..
---
## Stream Multiplexing
Notes: Very Brief overview maybe..
---
## Security and Maliciousness
* From Game theory.. Just because a particular type of attack is theoretically possible/feasible does not mean that it is practical..
---v
## Identity and Trust
* Every node has an public private key pair or PeerId which allows to verify who we are talking too.
* Authorization is NOT default. (Some systems may not require any authorization from a peer you can think of this as a tuning on permission..) 
* Reputation systems are one way to identify bad actors(Blacklist IP for instance, we use reputation in substrate)
    - i.e. duplicate messages
    - Try to maintain connections with the nodes that have the highest reputation (With some randomness to allow new nodes to join)
---
## DOS
* KAD-DHT are vulnerable to sybils.
* A DHT query may need to be routed through several peers before the query is fulfilled.(Those peers can be malicious and attempt to lie)
* If a malicious actor wants to target a specific key they can improve their chances of being in the lookup path.
* Do this by generating Ids close to the target key based on the DHT distance metric
---v
## Sybils
- Sybils are hard to defend against and precautions can be taken at the application level to mitigate(Proof of work perhaps?)
---v
## S/Kademlia paper in libp2p
* Query multiple disjoint lookup paths(Paths which dont share any routing peers) in parallel
---
## Additional Resources
* https://curriculum.pl-launchpad.io/curriculum/libp2p/
* https://docs.libp2p.io/concepts/
---
