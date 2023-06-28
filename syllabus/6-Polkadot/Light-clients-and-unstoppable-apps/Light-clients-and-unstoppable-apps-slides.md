---
title: Light Clients and Unstoppable Apps
description: Light Clients and Unstoppable Apps, for Web3 engineers.
duration: 45+ mins
---

<style type="text/css">
.bordered-box {
  background-color: green;
  border-radius: 1rem;
  padding: 1rem;
  margin-top: 1rem !important;
}

.font-weight-bold {
  font-weight: bold;
}

.red {
  background-color: red;
}

.green {
  background-color: green;
}
</style>

# Light clients<br/>and<br/>Unstoppable Apps

---

## Traditional Web 2

<img rounded style="width: 80%;" src="../../../assets/img/Light-clients/web2.png" />

---

## The Web 3 vision

<img rounded style="width: 80%;" src="../../../assets/img/Light-clients/web3.png " />

---

## The Web 3 reality

<img rounded style="width: 80%;" src="../../../assets/img/Light-clients/web3_reality.png " />

---

<h1 style="font-size:7rem; font-weight: bold">Blockchain "decentralized” apps are still centralized</h1>

---

## The reality of blockchains today

<img rounded style="width: 100%;" src="../../../assets/img/Light-clients/reality_bc_today.png " />

---v

<h4>PUBLICLY-ACCESSIBLE NODE</h4>
<pba-cols>
  <pba-col left>
    <div>App connects to a third-party-owned publicly-accessible
node client</div>
    <div class="red bordered-box"><span class="font-weight-bold">Centralized and insecure:</span> Publicly-accessible node can be malicious</div>
<div class="green bordered-box"><span class="font-weight-bold">Convenient:</span> Works transparently</div>
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="2" -->
  <pba-col left>
    <img rounded src="../../../assets/img/Light-clients/PUBLICLY-ACCESSIBLE-NODE.png" />
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="1" -->
</pba-cols>

---v

<h4>USER-CONTROLLED NODE</h4>
<pba-cols>
  <pba-col left>
    <div>App connects to a node client that the user has installed on their machine</div>
    <div class="green bordered-box"><span class="font-weight-bold">Secure</span><br />Trustless: connects to multiple nodes, verifies everything</div>
<div class="red bordered-box"><span class="font-weight-bold">Convenient:</span> Works transparently</div>
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="2" -->
   <pba-col left>
      <img rounded src="../../../assets/img/Light-clients/USER-CONTROLLED-NODE.png" />
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="1" -->
</pba-cols>

---v

<h4>Why this needs fixing?</h4>
<pba-cols>
  <pba-col left>
      <h4>Reliability</h4>
      <div>"The middleman" can stop working for a reason or another, leaving end users incapable of interacting with the blockchain.</div>
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="1" -->
   <pba-col left>
   <h4>Possibility of censorship or hijacking</h4>
      <div>"The middleman" can decide to ban some end users or some transactions, or can be taken control of by an attacker.</div>
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="2" -->
   <pba-col left>
   <h4>Frontrunning problem</h4>
      <div>"The middleman" knows all the transactions that are submitted before they are actually applied, and can inject its own transactions ahead of time for its own monetary gains.</div>
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="3" -->
</pba-cols>
---v

# The solution:

# Light Clients

---

## NEXT SLIDES FOR LIGHT CLIENTS

---

## Publicly Accessible Node

The dApp (UI) connects to a third-party-owned publicly-accessible node client

<p class="red bordered-box"><span class="font-weight-bold">Centralized and insecure:</span> Publicly-accessible node can be malicious</p>
<p class="green bordered-box"><span class="font-weight-bold">Convenient:</span> Works transparently</p>

---v

## So what one needs to do

Find the web-socket url of a 3rd party node that you trust;

---v

## In your dApp

```javascript[0|1|3-5|7-9]
import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://westend-rpc.polkadot.io");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
```

---

## User-Controlled Node

The dApp (UI) connects to a node client that the user has installed on their machine

<p class="green bordered-box"><span class="font-weight-bold">Secure Trustless:</span> connects to multiple nodes, verifies everything</p>
<p class="red bordered-box"><span class="font-weight-bold">Inconvenient:</span> Needs an installation process and having a node up and running, plus maintenance effort</p>

---v

## So what one needs to do

<pba-flex center>

1. Install dependencies<br/>
   (e.g. rust, openssl, cmake, llvm etc);
1. Clone from github the `polkadot` repo;
1. Build the node locally;
1. Start the node locally;
1. Wait for node to synchronize;

<pba-flex>

---v

## In your dApp

```javascript[|1|3-5|7-9]
import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://127.0.0.1:9944");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
```

---

## Light Client in the Browser

The uApp (UI) connects to an _integrated_ light client

<p class="green bordered-box"><span class="font-weight-bold">Secure Trustless:</span> connects to multiple nodes, verifies everything</p>
<p class="green bordered-box"><span class="font-weight-bold">Convenient:</span> Works transparently</p>

---v

## So what one needs to do

<pba-flex center>

1. Install and use [Substrate Connect](https://github.com/paritytech/substrate-connect)<br/>
   dependencies in the uApp
1. Write/Replace 2-3 lines of code

</pba-flex>

---v

## With PolkadotJS API

```javascript[0|1-2|4-7|9-11]
import { ScProvider } from "@polkadot/rpc-provider/substrate-connect";
import * as Sc from '@substrate/connect';

// Maybe some more code that does some magic here
const provider = new ScProvider(Sc, Sc.WellKnownChain.westend2);
await provider.connect();
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
```

---v

### Or even without PolkadotJS API

```javascript[0|1|4|5-10|12-15]
import { createScClient, WellKnownChain } from "@substrate/connect";

// Maybe some more code that does some magic here
const scClient = createScClient();
const mainChain = await scClient.addWellKnownChain(
  WellKnownChain.polkadot,
  jsonRpcCallback = (response) {
    console.log(response);
  }
);

// Communicate with the network
mainChain.sendJsonRpc(
  '{"jsonrpc":"2.0","id":"1","method":"chainHead_unstable_follow","params":[true]}',
);
```

---v

### Or with a Custom Chainspec

```javascript[0|2, 4| 6-13| 9 | 15-18]
import { createScClient, WellKnownChain } from "@substrate/connect";
import myLovelyChainspec from './myLovelyChainspecFromSubstrateChain.json';

const myLovelyChainspecStringified = JSON.stringify(myLovelyChainspec);

// Maybe some more code that does some magic here
const scClient = createScClient();
const mainChain = await scClient.addChain(
  myLovelyChainspecStringified,
  jsonRpcCallback = (response) {
    console.log(response);
  }
);

// Communicate with the network
mainChain.sendJsonRpc(
  '{"jsonrpc":"2.0","id":"1","method":"chainHead_unstable_follow","params":[true]}',
);
```

---

<img rounded style="width: 800px" src="../assets/BATMAN3.jpg" alt="Back to the other slides, Batman!" />
