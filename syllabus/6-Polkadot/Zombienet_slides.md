---
title: Zombienet
description: Zombienet workshop
duration: 1 hour
instructors: ["Javier Viola"]
---

<style>
    .colored {
        color: var(--r-link-color);
    }

    .colored-green {
        color: #18ffb2;
    }

    .colored-light-green {
        color: #c2ff33;
    }
</style>

# Zombienet workshop

---

## what is Zombienet?


Zombienet is an **integration testing tool** that allows users to **spawn** and **test** ephemeral substrate based networks.

---

## Why Zombienet?

Integration tests are always **complex**:

<br/>

- Setup Configuration
<!-- .element: class="fragment" -->
- Port management
<!-- .element: class="fragment" -->
- Ready state off all artifacts
<!-- .element: class="fragment" -->
- Observability
<!-- .element: class="fragment" -->
- Leaking resources
<!-- .element: class="fragment" -->

---v

## Friction to resolve

<br/>

- Config flexibility
- Local environment
- Maintenance
- CI friendly
- Scaling
- Test-runnner

---v

## Goals

<br/>

<pba-cols style="align-items:normal">
    <pba-col>

##### Hassle free setup

- __Toml__ / __Json__
- Nice defaults
- Templating language

</pba-col>
<pba-col>

##### Multiple envs

- **Local**
- **k8s**
- **podman**

</pba-col>
<pba-col>

##### Extensible

- Custom assertions
- Intuitive **D.S.L**
- Templating Lang.
</pba-col>
</pba-cols>

---

<!-- .slide: data-background-color="#4A2439" -->

# Phases

---

## Phases

<pba-cols style="align-items:normal">
<pba-col>

- Spawn
    - Custom chain-specs
    - Custom comand
    - Port-mapping
    - Parachains registration

</pba-col>
<!-- .element: class="fragment" -->
<pba-col>

- Test
    - Custom **D.S.L**
    - Multiple assertions
    - Extensible
    - Custom reporting

</pba-col>
<!-- .element: class="fragment" -->
</pba-cols>

NOTES:

---

## Zombienet Options

<pba-col center>

- As binary ([releases](https://github.com/paritytech/zombienet/releases))
- As library (@zombienet)
- As container (published in docker hub)
- From source ([zombienet](https://github.com/paritytech/zombienet) repo)
</pba-col>

NOTES:
 - As binary: Binaries for Linux and MacOS are available in each release in Github.
 - npm packages: cli, orchestrator, utils
 - image: docker.io/paritytech/zombienet
 code is available in GitHub with the instructions on how to build and run Zombienet. (https://github.com/paritytech/zombienet)

---v

### Download Zombienet

```sh[1-11|1-3|5-7|11]
# macOS
curl -L https://github.com/paritytech/zombienet/releases/download/v1.3.29/zombienet-macos
-o ./zombienet

# linux
curl -L https://github.com/paritytech/zombienet/releases/download/v1.3.29/zombienet-linux
-o ./zombienet

# make executable
chmod +x zombienet
```

---v

### Examples

<img style="" src="../../assets/img/5-Polkadot/zombienet/examples-qr.png">

[zombienet presentation examples](https://github.com/pepoviola/zombienet-presentation-examples)

---

<!-- .slide: data-background-color="#4A2439" -->

# Let’s spawn a new network!

---v

## But first, try manually…

<br/>

- Create chain-spec (parachain)

```sh
parachain-template-node build-spec --chain local --disable-default-bootnode > /tmp/para.json
```

<br/>

- Create chain-spec (relaychain)

```sh
polkadot build-spec --chain rococo-local --disable-default-bootnode > /tmp/relay.json
```

NOTE: Tutorial https://docs.substrate.io/tutorials/connect-relay-and-parachains/connect-a-local-parachain/

---v
### Add keys*

<br/>

When not using --alice or --bob, you need to provide additional `aura` and `grandpa` keys and inject them into the keystore! (**per node**)

```sh
  ./target/release/polkadot \
  key insert --base-path /tmp/node01 \
    --chain /tmp/relay.json \
    --scheme Sr25519 \
    --suri <your-secret-seed> \
    --password-interactive \
    --key-type aura
```

<br/>

```sh
  ./target/release/polkadot key insert \
    --base-path /tmp/node01 \
    --chain /tmp/relay.json \
    --scheme Ed25519 \
    --suri <your-secret-key> \
    --password-interactive \
    --key-type gran
```

NOTE: this step is optional if you use the devs accounts (e.g. alice, bob, charlie, dave, etc)

---v

- Start relay chain nodes

```sh[1-2|4-10|12-18]
# create nodes dirs
  mkdir -p /tmp/relay/{alice,bob}

  ./target/release/polkadot \
  --alice \
  --validator \
  --base-path /tmp/relay/alice \
  --chain /tmp/relay.json \
  --port 30333 \
  --ws-port 9944

  ./target/release/polkadot \
  --bob \
  --validator \
  --base-path /tmp/relay/bob \
  --chain /tmp/relay.json \
  --port 30334 \
  --ws-port 9945
```

NOTES:
why do we need to use different ports for Alice and Bob?

---v

- Start collator

```sh
# create nodes dirs
  mkdir -p /tmp/para/alice

 parachain-template-node \
--alice \
--collator \
--force-authoring \
--chain /tmp/para.json \
--base-path /tmp/para/alice \
--port 40333 \
--ws-port 8844 \
-- \
--execution wasm \
--chain /tmp/relay.json \
--port 30343 \
--ws-port 9977
```


---v

- Register parachain

    - Register ParaId <!-- .element: class="fragment" -->
    - Modify parachain chain-spec and create raw format. <!-- .element: class="fragment" -->
    - Generate genesis wasm and state
    <!-- .element: class="fragment" -->
    - Register parachain using sudo call <!-- .element: class="fragment" -->

    <br/>

    ```sh[|1|3|5]
    parachain-template-node build-spec --chain /tmp/para-raw.json --disable-default-bootnode --raw > /tmp/para-raw.json

    parachain-template-node export-genesis-wasm --chain /tmp/para-raw.json para-2000-wasm

    parachain-template-node export-genesis-state --chain /tmp/para-raw.json para-2000-genesis-state
    ```

---

<!-- .slide: data-background-color="#4A2439" -->

# Activity

Follow the [connect a local parachain](https://docs.substrate.io/tutorials/connect-relay-and-parachains/connect-a-local-parachain/) to launch your own network.

---

## Non-trivial chore

<pba-cols style="align-items:normal">
<pba-col>

- Error prone.

- Multiple commands.

- Port management.

- Multiple process.

</pba-col>

<pba-col>

<div class="fragment center" style="font-size:150%;padding-top:100px">Zombienet allow you to set everything in <span style="color: var(--r-link-color);font-weight:bold;">just</span> 1 file.</div>

</pba-col>
</pba-cols>

---

## Zombienet network definition

Zombienet allow to [define your network](https://paritytech.github.io/zombienet/network-definition-spec.html) with a simple configuration file.

NOTES:
https://paritytech.github.io/zombienet/network-definition-spec.html

---v

```toml[1|3-12|14-21]
# examples/0001-small-network.toml

[relaychain]
default_image = "docker.io/parity/polkadot:latest"
default_command = "polkadot"
chain = "rococo-local"

  [[relaychain.nodes]]
  name = "sub"

  [[relaychain.nodes]]
  name = "zero"

[[parachains]]
id = 1001
cumulus_based = true

  [parachains.collator]
  name = "collator01"
  image = "docker.io/parity/polkadot-parachain:latest"
  command = "polkadot-parachain"
```

NOTES: https://github.com/pepoviola/zombienet-presentation-examples/blob/main/examples/0001-small-network.toml

---v


### Spawn the network

```sh

./zombienet spawn examples/0001-small-network.toml

```

---

## Make the network config dynamic

The network definition supports using [nunjucks](https://mozilla.github.io/nunjucks/) templating language (similar to [tera](https://github.com/Keats/tera)). Where <span class="colored-green">{{variables}}</span> are replaced with <span class="colored-green">env vars</span> and you can use all the built-in features.

<br/>

```toml[2]
[relaychain]
default_image = "{{ZOMBIENET_INTEGRATION_IMG}}"
default_command = "polkadot"
```

---v

## Make the network config dynamic

<img style="" src="../../assets/img/5-Polkadot/zombienet/zombienet-env-vars.png">

---

## Providers

Zombienet <span class="colored">providers</span> allow to <span class="colored-green">spawn and test</span> networks with in different environments.

---v
<pba-cols style="align-items:normal">

<pba-col>

<span class="colored">Kubernetes</span>
<br/>

- Used internally, integrated with the [Grafana](https://grafana.com/oss/grafana/) stack.

- You need to provide your infra stack.

</pba-col>

<pba-col>

<span class="colored">Podman</span>
<br/>

- Automatically spawn and wire an instance of [Grafana](https://grafana.com/oss/grafana/) stack.

- Attach a jaeger instance if enabled in the network definition.

</pba-col>

<pba-col>

<span class="colored">Native</span>
<br/>

- Allow to attach to a running [Grafana](https://grafana.com/oss/grafana/) stack. **(wip)**

</pba-col>

</pba-cols>

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions?

---

## Meet the Test-runner

Zombienet’s built-in <span class="colored">test-runner</span> allows users to use a simple <span class="colored-green">D.S.L.</span> to easily and intuitively write tests. with a set of natural language expressions to make assertions.

---v

### Built-in assertions

<br/>


- <span class="colored">Prometheus</span>: Query the exposed metrics/histograms and assert on their values.

- <span class="colored">Chain</span>: Query/subscribe chain's storage/events.

- <span class="colored">Custom scripts</span>: Run custom js scripts or bash scripts (inside the pod).

- <span class="colored">Node's logs</span>: Match regex/glob patterns in the node's logs.

- <span class="colored">Integrations</span>: Zombienet supports multiple integrations, like jaeger spans, polkadot introspector and the backchannel.

---v

```[1-3|6|7|14-16|18-20|22-24|26-28]
Description: Small Network Paras
Network: ./0002-small-network-paras.toml
Creds: config # Only used with k8s

# well known functions
validator: is up # check all the validators in the group
validator-0: parachain 1000 is registered within 225 seconds
validator-0: parachain 1001 is registered within 225 seconds

# ensure parachains are producing blocks
validator-0: parachain 1000 block height is at least 5 within 300 seconds
validator-0: parachain 1001 block height is at least 5 within 300 seconds

# metrics
validator-0: reports node_roles is 4
validator-0: reports block height is at least 2 within 15 seconds

# logs (patterns are transformed to regex)
validator-1: log line matches glob "*rted #1*" within 10 seconds
validator-1: log line matches "Imported #[0-9]+" within 10 seconds

# system events (patterns are transformed to regex)
validator-2: system event contains "A candidate was included" within 10 seconds
validator-2: system event matches glob "*was backed*" within 10 seconds

# custom scripts
validator-0: js-script ./custom.js with "alice" within 200 seconds
validator-0: run ./custom.sh within 200 seconds
```

NOTES:
First three lines are the header

Each line represents an assertion

Each assertion is executed sequentially

Assertions on a group check each node

within keyword allows to keep-trying until time expires

---

## DSL extension

Learning a new D.S.L. can be tedious, but if you are using vscode we develop an [extension](https://github.com/paritytech/zombienet-vscode-extension) that can help you to write test easily.


NOTES:
Show the extension link
https://github.com/paritytech/zombienet-vscode-extension

---

# Demo time

```sh
./zombienet -p native test examples/0002-small-network-paras.zndsl
```

---

# Extensibility

<span class="colored">Zombienet</span> allow users to use the custom-js assertion to extend and run custom tests.

---v

## Custom-js

```[2]
# custom scripts
validator-0: js-script ./custom.js with "alice" within 200 seconds
```

```js
async function run(nodeName, networkInfo, args) {
    const {wsUri, userDefinedTypes} = networkInfo.nodesByName[nodeName];
    const api = await zombie.connect(wsUri, userDefinedTypes);
    const validator = await api.query.session.validators();
    return validator.length;
}

module.exports = { run }
```

NOTES:
Zombienet will load your script and call the run function.

Passing the node name, network info and an array of arguments from the assertion

Your function have access to the zombie object exposing utilities like connect, ApiPromise, Keyring, etc *

The assertions can validate the return value or the completions of your script.

*similar to the way that scripts are written in PolkadotJS apps - developer page (https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.polkadot.io#/js)

---

## More extensebility

<span class="colored">Zombienet</span> also allow users to use as a library to create their own interactions with the running network.

---v

### As a library

- <span class="colored">@zombienet/orchestrator</span> module expose the start function as entrypoint.

- Returning a [network](https://github.com/paritytech/zombienet/blob/main/javascript/packages/orchestrator/src/network.ts#L77) instance, with all the information about the running topology.

- You can also use the [test](https://github.com/paritytech/zombienet/blob/main/javascript/packages/orchestrator/src/orchestrator.ts#L853) function  passing a callback to run your test.

- <span class="colored">@zombienet/utils</span> module expose misc utils functions like _readNetworkConfig_.

---v

```js[1|2|6-7|10-12|14|]
import {start} from "@zombienet/orchestrator";
import { readNetworkConfig } from "@zombienet/utils";

const ZOMBIENET_CREDENTIALS = "";

// can be toml or json
const launchConfig = readNetworkConfig("../examples/0001-small-network.toml");

( async () => {
    const network = await start(ZOMBIENET_CREDENTIALS, launchConfig, {
        spawnConcurrency: 5,
    });

    // write your own test, `network` will have all the network info
})();
```

---

## acknowledgement & contributions


<span class="colored"><b>Zombienet</b></span> take inspiration and some patterns from <span class="colored-light-green">polkadot-launch</span> and <span class="colored-light-green">SimNet</span>.

We encourage everyone to test it, provide feedback, ask question and
contribute.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions?

---

## Activity

---













