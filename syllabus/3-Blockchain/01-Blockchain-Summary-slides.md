---
title: Blockchain Summary
description: Overview of Blockchain
---

### Coordination And Shared Stories

- Importance of having trustless verifiable histories
- Coordinating with people who you dont trust or know is still important
  (Give example here)
- Trust and Centralization

---

### P2P Networking

- Traditional networks are setup for centralization
- P2P Networks decentralize

---

### State Machine

- Set of states & rules which codify how to transition states
- i.e. Laundry (Wash State, Drying state, Off State etc...)
- We in blockchains want state machine so we can do things on blockchains (like have applications)

---

### Hash List Combined with State machine

- Each block contains hash of previous block
- Each block in the Hash list contains state machine information
- Now we have combined coordination and shared histories with a computer science concept

---

### Forks

- Where one set of network participants disagree on the hash list
- Potentially also a malicious set of network participants are presenting a fake history to the rest of the network(Which benefits them)

---

### Account Abstractions

- How to represent users in the state machine? (i.e. An account?)
- Accounts & UTXO Model (Main models)

---

### Accounts

- User has an account with a balance
- State is comprised of a bunch of Accounts and assosciated balances
- To transfer you check to see if the sending account has enough funds

---

### UTXO

- User has a set of coins
- State is comprised of all of the coins for all users
- Think of several Dollar bills with your name written on it
- To transfer you must collect all the bills with your names on them and "Spend them" by writing different peoples names on them now

---

### Consensus

- How to coordinate with people trustlessly
- Byzantine Generals
- Leader election

---

### Authoring

- How to choose a leader?
  - PoW bitcoin
  - PoS systems
- Why need to choose a leader?
- How to verify the leader did something correct?

---

### Finality

- How to determine a point with which we will not dispute ever again?
  - Voting, 2/3 honest assumption
  - PoW utilizing probablistic finality (Likely you will never fork again from this block)
- What happens if finality is not reached?
- Decoupling finality from authoring

---

### Grandpa

- Voting on forks separate from authoring new blocks
- Time Complexity

---

### Tendermint / Hotstuff

- Vote on every block
- "Instant Finality"
- Possible missed slots..

---

### Attacks on Consensus

- Authoring (Someone takes control of all authoring)
- Finality (A malicious set controlling what is considered final and the "correct chain")
- Privacy

---

### Sassafras

- Blind authoring
- Not forkful

---

### Light Clients

- Syncing with just block headers
- Listening on p2p network to view votes on blocks
- Verifying state proofs and transitions "lightly"

---

### Bridging

- Why is it hard to get two separate consensus systems to sync?
- What are the gurantees one authority set has to another authority set?
- Where is the accountability?

---

### Bridging

- Bridging naively (Single signer verifying a transaction from one chain to another)
- Bridging a little better.. (Multisig of participants listening to both and the aggregate being trusted)
- "Trustless" Bridging using light clients (You verify the other chains state by listening and being a participant in their consensus)

---
