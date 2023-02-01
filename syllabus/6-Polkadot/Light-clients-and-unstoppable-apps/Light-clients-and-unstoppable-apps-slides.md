---
title: Light Clients and Unstoppable Apps
description: Light Clients and Unstoppable Apps.
duration: 45+ mins
---
## Light clients
## and
## unstoppable apps

---
## Publicly accessible node

The dApp (UI) connects to a third-party-owned publicly-accessible node client

<p class="red bordered-box"><span class="font-weight-bold">Centralized and insecure:</span> Publicly-accessible node can be malicious</p>
<p class="green bordered-box"><span class="font-weight-bold">Convenient:</span> Works transparently</p>
---v
## So what one needs to do

Find the web-socket url of a 3rd party node that you trust;

---v
## In your dApp

<pre><code data-line-numbers="">import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://westend-rpc.polkadot.io");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---v
## In your dApp

<pre><code data-line-numbers="1">import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://westend-rpc.polkadot.io");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---v
## In your dApp

<pre><code data-line-numbers="3-5">import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://westend-rpc.polkadot.io");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---v
## In your dApp

<pre><code data-line-numbers="7-9">import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://westend-rpc.polkadot.io");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>

---
## User-controlled node

The dApp (UI) connects to a node client that the user has installed on their machine

<p class="green bordered-box"><span class="font-weight-bold">Secure Trustless:</span> connects to multiple nodes, verifies everything</p>
<p class="red bordered-box"><span class="font-weight-bold">Inconvenient:</span> Needs an installation process and having a node up and running, plus maintenance effort</p>
---v
## So what one needs to do

1. Install all needed dependencies in your local machine **(e.g. rust, openssl, cmake, llvm etc)**;
2. Clone from github the `polkadot` repo;
3. Build the node locally;
4. Start the node locally;
5. Wait for  node to synchronize;

---v
## In your dApp

<pre><code data-line-numbers="">import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://127.0.0.1:9944");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---v
## In your dApp

<pre><code data-line-numbers="1">import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://127.0.0.1:9944");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---v
## In your dApp

<pre><code data-line-numbers="3-5">import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://127.0.0.1:9944");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---v
## In your dApp

<pre><code data-line-numbers="7-9">import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://127.0.0.1:9944");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>

---
## Light Client Node bundled in the browser

The uApp (UI) connects to a light client that is directly integrated inside of it

<p class="green bordered-box"><span class="font-weight-bold">Secure Trustless:</span> connects to multiple nodes, verifies everything</p>
<p class="green bordered-box"><span class="font-weight-bold">Convenient:</span> Works transparently</p>
---
## So what one needs to do

1. Install and use substrate connect dependencies in the uApp
2. Write/Replace 2-3 lines of code

---
## With PolkadotJS API

<pre><code data-line-numbers="">import { ScProvider } from "@polkadot/rpc-provider/substrate-connect";
import * as Sc from '@substrate/connect';

// Maybe some more code that does some magic here
const provider = new ScProvider(Sc, Sc.WellKnownChain.westend2);
await provider.connect();
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---v
## With PolkadotJS API

<pre><code data-line-numbers="1,2">import { ScProvider } from "@polkadot/rpc-provider/substrate-connect";
import * as Sc from '@substrate/connect';

// Maybe some more code that does some magic here
const provider = new ScProvider(Sc, Sc.WellKnownChain.westend2);
await provider.connect();
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---v
## With PolkadotJS API

<pre><code data-line-numbers="4-7">import { ScProvider } from "@polkadot/rpc-provider/substrate-connect";
import * as Sc from '@substrate/connect';

// Maybe some more code that does some magic here
const provider = new ScProvider(Sc, Sc.WellKnownChain.westend2);
await provider.connect();
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---v
## With PolkadotJS API

<pre><code data-line-numbers="9-11">import { ScProvider } from "@polkadot/rpc-provider/substrate-connect";
import * as Sc from '@substrate/connect';

// Maybe some more code that does some magic here
const provider = new ScProvider(Sc, Sc.WellKnownChain.westend2);
await provider.connect();
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
</code></pre>
---
### Or even without PolkadotJS API

<pre><code data-line-numbers="">import { createScClient, WellKnownChain } from "@substrate/connect";

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
</code></pre>
---v
### Or even without polkadotJS

<pre><code data-line-numbers="1">import { createScClient, WellKnownChain } from "@substrate/connect";

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
</code></pre>
---v
### Or even without polkadotJS
<pre><code data-line-numbers="4">import { createScClient, WellKnownChain } from "@substrate/connect";

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
</code></pre>
---v
### Or even without polkadotJS
<pre><code data-line-numbers="5-10">import { createScClient, WellKnownChain } from "@substrate/connect";

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
</code></pre>
---v
### Or even without polkadotJS
<pre><code data-line-numbers="12-15">import { createScClient, WellKnownChain } from "@substrate/connect";

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
</code></pre>
---
### Or with a custom chainspec
<pre><code data-line-numbers="">import { createScClient, WellKnownChain } from "@substrate/connect";
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
</code></pre>
---v
### Or with a custom chainspec
<pre><code data-line-numbers="2, 4, 8-13, 15-18">import { createScClient, WellKnownChain } from "@substrate/connect";
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
</code></pre>
---
<img style="width: 800px" src="../assets/BATMAN3.jpg" alt="rollup-batch">
