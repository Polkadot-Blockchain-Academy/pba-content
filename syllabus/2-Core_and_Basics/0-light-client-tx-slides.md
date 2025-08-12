---
title: Light Client Transaction
description: End to End Transaction Flow with a Light Client
---

# Light Client Transaction

---

In this presentation we will cover the end to end flow of a transaction.

- Starting with a light client.
- Being processed by a node.
- Updating the state of the blockchain.
- And finally state update being verified by the light client.

Along the way, we will remind you and touch on details you should already be familiar with.

---

# Part 0: What is a Light Client?

---

## Block Re-Execution

Blockchain systems are decentralized and trustless because anyone is able to fully re-execute the state transition function of the blockchain, across all blocks, and recreate the state of the chain at any point in time.

---

## Node Minimum Requirements

However, in order to "keep up" with the blockchain, you require minimum hardware, which is usually more powerful than the average phone, laptop, or other mobile device.

- Execution Speed
- SSD Requirements
- RAM Requirements
- Networking Speeds
- etc...

---

## Scalability Through Speed

As you have learned with Polkadot, we employ lots of advance engineering to achieve a secure, scalable, and resilient blockchain. However, the reality is, if you use a more powerful computer across the nodes in your network, your blockchian will inheriently perform better.

If you have a blockchain which can directly run on a phone, it is likely not performing at competitive speeds.

---

So does the trustless decentralized future exclude mobile devices and other similar lower power hardware?

---

## Introducing Light Clients

This is where Light Clients come into play.

Light Client Nodes are those which do not fully execute and sync the state of the blockchain, but use verifiable proofs to be able to communicate and receive information trustlessly from another full node.

---

## How Do Light Clients Work?

Light clients synchronize and verify **block headers** rather than the full blocks themselves.

These block headers tell

---

## Block Breakdown

A block is split into two main parts:

- Block Header
	- Parent Hash, Block Number, State Root, Extrinsic Root, Digest
- Block Body
	- Vector of Extrinsics

The block hash representing a unique block is simply the hash of the data inside the header. Since we already include the Extrinsic Root inside the block header, we need not know the block body to get the block hash.

---

## Block Header

Inside the block header is:

- **Parent Hash**: The hash of the preceding block's header. This is what links blocks together to form the blockchain.
- **Block Number**: The height of the block in the chain.
- **State Root**: This single merkle root hash represents all current data stored in the blockchain, and can be used to verify proofs that the blockchain contains some specific state.
- **Extrinsics Root**: This merkle root of the extrinsics found in the block's body, allowing a client to prove that a specific extrinsic was or was not included in the block without needing to download the entire body.
- **Digest**: A list of log items containing "auxiliary" information needed to verify the block. This is where consensus-related data lives.

For a light client, the digest is very important.

---

## Block Digest

- Consensus Logs: Data from the block production engine (e.g., BABE or AURA). This includes information like the slot number and the block author.

- Seal: A signature from the block author, proving they produced this block.

- GRANDPA Logs: This is how Polkadot's finality mechanism (GRANDPA) communicates validator set changes. A light client will parse the digest for logs like ScheduledChange or ForcedChange to know when the validator set will be updated. This is how a light client tracks the authority set without downloading the state.

---

## Light Client's Job

A light client's main goal is to verify the state of the blockchain with minimal resource usage. It achieves this by focusing on three critical tasks:

1. Following the Validator Set
2. Verifying Block Authorship
3. Confirming Finality

For the purposes of explaining how a light client works with these steps, we will go through them backwards.

---

## Confirming Finality

Light Clients will recieve new blocks from a full node, but why should it trust that this block is accurate and part of the cannonical chain?

For this, the full node will send to the light client a GRANDPA Justification, which is not part of the block, but something gossiped as part of the networking and consensus protocol.

Justification gossip happens in rounds, allowing the validators to give up to date votes on their view of the blockchain.

---

## GRANDPA Justification

The GRANDPA Justification includes signatures from current block producers that they believe some chain of blocks are part of the cannonical chain, and should be finalized. As soon as the Justification contains 2/3 of the validator signatures, the block is finalized.

Light Clients are responsible for keeping track of the current validator set, and are able to individually verify the signatures of each validator in the Justification.

By matching the block hash with the signatures from validators, they are then able to trust all the contents of the block header given to them by the full node.

---

## Block Author

Within the header itself are signatures and data from the block author who made the block.

Inside the digest is a **seal**, which is simply a signature from the block author for the block hash.

There is also a secret VRF which the block author reveals to show they are allowed to produce a block during that slot.

All of this can again be independently verified by the nodes on the network, including light clients.

---

## Following the Validator Set

Verifying the Justification assumes the light client knows the current validator set. Since it does not actually execute the blocks, it cannot simply query for that information.

Instead, this information is constantly updated in the block digest along with the other consensus critical data.

---

## Updating the Validator Set

The genesis block defines the initial state of the blockchain, including the initial validator set.

From there, based on the state transition function of the blockchain, a new validator set might be queued for some future block.

This will be pre-announced in the digest, and signed by the current validator set, showing that a new validator set will be active in the future.

With this announcement included in the digest, and backed by the GRANDPA Justification, light clients can always know who the active validators are, even as they change.

---

## Light Clients Verify Everything but the STF

As you can see, Light Clients are able to remain trustless within the blockchain ecosystem because they are able to verify all of the block headers of the canonical chain.

With this, it becomes simple to verify:

- a transaction has been included in the chain, from the extrinsics root.
- the state of the chain, from the state root.

And remember, the State Transition Function (Wasm) itself is stored on chain, but the light client is not expected to execute it.

---

# Part 1: Light Client Wants to Create a Transaction

---

So we have seen so far, in general, how a light client would trustlessly follow the canonical version of the blockchain.

But let's assume the light client wants to interact with the chain. Something simple like a balance transfer from Alice to Bob.

How would it do that?

---

## Fetching the Metadata

The first thing the light client will do is fetch the up to date metadata of the chain.

Remember that in the Polkadot ecosystem, chains can constantly upgrade and update their state transition function, changing the functionality of the chain and even what extrinsics are available.

---

## State Transition Function is Self Describing

To get the metadata itself, we must query the Runtime via the State Transition Function Code.

The Runtime exposes an API: `state_getMetadata`, which will return a SCALE encoded blob with all the data you need to know about the blockchain.

---

## Merkle Tree

TODO

---

## Merkle Proof

TODO

---

## SCALE

TODO

---

## FRAME Metadata Format

- Versioned, and occasionally changes over time.
- Defined by: https://github.com/paritytech/frame-metadata
- Automatically generated if you use FRAME macros.
- Output can be turned into JSON, which is easiest for human readability:
	- https://dev.papi.how/metadata/json

---

# Get Your Current Balance

---

## Get Your Current Balance

Using the Metadata we can see the following:

- System is pallet index 0.
	- With storage Prefix "System".
- There is a storage item called "Account".
	- With storage prefix is "Account"
- It uses hasher `Blake2128Concat`.
- It has a key using "type 0".
- It has a value using "type 3".

This tells Us everything we need to know to query and understand the blockchain.

---

## Key Type Lookup

Under the `lookup` section of the metadata, we can see all the Type Information, and how to serialize and deserialize this correctly in SCALE.

```JSON
{
  "0": {
    "id": 0,
    "path": [
      "sp_core",
      "crypto",
      "AccountId32"
    ],
    "params": [],
    "def": {
      "tag": "composite",
      "value": [
        {
          "name": null,
          "type": 1,
          "typeName": "[u8; 32]",
          "docs": []
        }
      ]
    },
    "docs": []
  }
}
```

---

## Alice Address

The address for Alice is:

```
5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

This can be generated using the "default mnemonic":

```text
bottom drive obey lake curtain smoke basket hold race lonely fit walk
```

Then adding the **hard key**:

```
//Alice
```

The same can be done for `//Bob`, `//Charlie`, etc...

---

## Storage Key

We can now generate the storage key for querying an account balance:

https://crates.parity.io/frame_support/pallet_prelude/struct.StorageMap.html

```
Twox128(Prefix::pallet_prefix())
	++ Twox128(Prefix::STORAGE_PREFIX)
	++ Hasher1(encode(key))
```

```
Twox128("System") = 0x26aa394eea5630e07c48ae0c9558cef7
Twox128("Account") = 0xb99d880ec681799c0cf30e8886371da9
encoded(5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY) = 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
Blake2128Concat(0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d) = 0xde1e86a9a8c739864cf3cc5ec2bea59fd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
```

Simple hashing tool: https://www.shawntabrizi.com/substrate-js-utilities/

```
0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9de1e86a9a8c739864cf3cc5ec2bea59fd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
```

---

## Query the Value from the Full Node

To get the value at that storage key from the full node, we call the `state_getStorage` RPC.

This will return a value like:

```
0x62030000010000000100000000000000fb2f65285e71000000000000000000008be5934d786800000000000000000000a243556c7f3e0000000000000000000000000000000000000000000000000080
```

---

## Value Type Lookup

We need to also understand the types of the returned value, and how to deserialize it.

```json
{
  "3": {
    "id": 3,
    "path": [
      "frame_system",
      "AccountInfo"
    ],
    "params": [
      {
        "name": "Nonce",
        "type": 4
      },
      {
        "name": "AccountData",
        "type": 5
      }
    ],
    "def": {
      "tag": "composite",
      "value": [
        {
          "name": "nonce",
          "type": 4,
          "typeName": "Nonce",
          "docs": []
        },
        {
          "name": "consumers",
          "type": 4,
          "typeName": "RefCount",
          "docs": []
        },
        {
          "name": "providers",
          "type": 4,
          "typeName": "RefCount",
          "docs": []
        },
        {
          "name": "sufficients",
          "type": 4,
          "typeName": "RefCount",
          "docs": []
        },
        {
          "name": "data",
          "type": 5,
          "typeName": "AccountData",
          "docs": []
        }
      ]
    },
    "docs": []
  },
  "4": {
    "id": 4,
    "path": [],
    "params": [],
    "def": {
      "tag": "primitive",
      "value": {
        "tag": "u32",
        "value": null
      }
    },
    "docs": []
  },
  "5": {
    "id": 5,
    "path": [
      "pallet_balances",
      "types",
      "AccountData"
    ],
    "params": [
      {
        "name": "Balance",
        "type": 6
      }
    ],
    "def": {
      "tag": "composite",
      "value": [
        {
          "name": "free",
          "type": 6,
          "typeName": "Balance",
          "docs": []
        },
        {
          "name": "reserved",
          "type": 6,
          "typeName": "Balance",
          "docs": []
        },
        {
          "name": "frozen",
          "type": 6,
          "typeName": "Balance",
          "docs": []
        },
        {
          "name": "flags",
          "type": 7,
          "typeName": "ExtraFlags",
          "docs": []
        }
      ]
    },
    "docs": []
  }
}
```

---

## Decoded Into Human Readable JSON

We can use this value type, and the returned hex value to construct a human readable JSON.

```json
{
  "nonce": 866,
  "consumers": 1,
  "providers": 1,
  "sufficients": 0,
  "data": {
    "free": "124649218584571n",
    "reserved": "114865906902411n",
    "frozen": "68716999295906n",
    "flags": "170141183460469231731687303715884105728n"
  }
}
```

---

## Request a Proof from the Full Node

The Light Client will then call a special RPC: `state_getReadProof`, which returns the data needed for a merkle proof for that value. All of this should be referencing the same block hash.

Using the storage key we want, and get a result like:

```json
{
  at: 0x2c64663a4821958604301c61d66a2ac4139bbfce2eab95649a03c30e9e7a2de6
  proof: [
    0x80c1c080c0ddf3b9ab1665f6fa9bc3bf96293ec812c5d3df0a08a3f8bd1f65f0330fc32e809db307c9a456e232cb899d362458aa4b3131a6adbe6266d09539f256de2d8f9f80257442a3193ebf64fb8a0019b2d9fc1d97734011eb5a8d2ea414f5ff9a27952b80e18c193fdffed4c3083a85c542cc746798e5a5e194470ce84b74196f908d139480d534098a3a09f8387c09e5f28879e7a5ef284340661b1ea59aa5c75ed775bed0
    0x80ffbe8009845321f24d42a661311c55e122b4722c0685022c714fba827a64e30435cf8080b63a95f8752dfdfc4b39fb7cb0cf7536eff04c383b8c26ffdff7a709b23f69a780b54f9d860044e2abdc09caaab05f5f416e4f9a8db09cd6991e8be107f81a866280123f56255f382b6fdc3b369e5bdc03cefae928d2e0f39516b4f969b7d2f4d8dd8020e632bf44a524b308544bad23b9997985cc1fbb976d8c325bb5096798d0125180bde6b2b2ff44787041683f5d287ee645c7fc473f24eaac5a19316babbb97fe5e8002ea08cbe73eadfbc3006fc9b72a5733cb1a91b747123d0ae15fb60e63732c3880ee479b0f2cd3903597508ca27329a3e29ae3718f141e5420672f53b5712dd2268036fac65ef7cd971c68205826ed69d42f3c36702e1583480734ae92c260662d83800688e3ce8f958838d13329d49e718ef7d8e8d3db43afdae4d9fe49fe5d703b1b8008f09865ee8e32230ba70bc5bc0c841f5ee3180b32181d7f28036a5f0535358a8049f7da1cd903209199994aca336a683f0946bd485142e186752655d02576354b80a2df8043566a7f47f887aa69e1c85c928961cb70abbf1f7ca99efc20508992f780fdf619ec1ff4b3a8f48b9ce57b6267346cbb56fea0c94b8bdb0963c02888fa61
    0x80ffff8037b0086d2f0f70d946b31d9744d2e42481ba510cd9e3fdc461a1ee062675bdd48096563b237937bf6e768d4642ea2ae12b92c80dcc71c5fb195edbc30cf44719d680e92371105b94ec13bfecc26fe5331192e2fcbfbe13e2427f970bfe79841ad53a808d878a88b99fe7b81369f75d08e6c7e57b0970a15a260d457d46aa7f9a67a649807bba1d275b005008d0c3d31761ef3a77874b11f7eeccb3db1351bcea418d4a10806f99e523c4bc7c4b579ba350f60b6d91c64a7dd602159707820b8f077fc4d26d8017e73721d14f70a12d37e31efc6e9a69e4e0d05aa5f0a9e5f79d988f647d6e3480fcfc20b7a6abd5bb07002ba1d44d68666d0037b4c19590146dcf1f047d9acfe780350429c711a11a2f7622f6627ac1901d3273e116bf1dbfb0f786a9b16987160d8093602b96f7123f3e9d202b94d539e1c0cd958dc5394f261314451df14bfc6d9e80553886d6c116044622c300e99e29f5da789a2be6ca7a3df92a8aa5acb4f25e4b801062eb7f5b289ea61044ae0b0e746bcbc53f08b1415b189ecf0c1f3d49a2613180f36f8bcb422c3069e3a8597b09253114054b75a9cd9914e32b623204ea907680800d4b9521f888dc100f182d77860f8086cda8668d961e8f246d80e352b392e30380ae7a246be77b08433628a0f4ead1787873f3a3f0119d87ebb1612882831585ec803a1776dc9d812899ee93434494ab99123c8f94516c6eac249df305b9a40df945
    0x80ffff807792e2b7ad8e961f38f625c57fece4c24ccab185c21daaf4c647dacd27d4f39f80828df66be6f6a489da88c20f2d00fea738a87e98d4b9a51d5b73e031614cb003801741e777721f0fb1f0662a3a6133b43ada7ff2057eae2dc977f1398fb14649d880ec3a40446446b847661b1823146a50db96ddff90120ba4bef19682a1cbdd09b18014ce1ceb8f950eff162e0a8b64a84a6706e411b516cf3c7ca14cc90ad5d3efab80c4a688fa32ec28fa879a2e7071eeb8661dc8401f6c5c62c2da11e49b4d8ca3c08080e03d36fb607505bfd98da3da9588b175a88110fd64b605583ca0dab3b084e680d3329308f44ac9559d431d4012cd42800c34d690c408e3d4331bbc41fc89d3b7809fe646e7796d1fc08f17ceac21bc5658a56346f3c64d4b58126913e7583089a3807861554ac122795ecf46090771fb1851b2a41c87c14bb1954c801aa603c73199803fabdbadc2fa8f9c33507206ac5b54af5594a1d9f06fa398289305b962d126ee8071e7431b48d09f1f8d78f9e994dade30e8864874d4d6d91c32012c65d8920cd780aaab4f45b894b059bb35404804fba20edbfb91892749940b2a52b4ad9fe2ca1180f641cd53c016b8fccdc88f2cdbdfe636d60382e4ff4bc94fae9cdda00006e95f80c5600be6b3d7fafd72fc24def89d379cd2bede3f496af138b8b83ff3b0ab4aa68080334a4156c3b067f7f605f12606e3117566d7b947f5b9c7d2fd6b6aacdbbe9f
    0x80ffff8088a4f6db81baa0e9c6be1376fe4f47ded48c49c18ca6d6273a62aa401cfbbfab805a04afa8d2a2eba8186db18693b6ad8491142b82f6164205afd90b855e0d3b378056e04008637e5121f1e8e71c7f527706b9522de1a66f5504ddf2bceb76c4676e8042fa574bb4e7c459d8e8fd5c768befa785162bdd38fc09925a9818cbd1d4185e8001fa50093f02faa29462aa570648aae001677c46fbf9dd13422be67a66c851ad80c9ee331f1da64b784ef9c1b920b251a6de973e0358343ddb040ad552164d965a808228450df2c589f3c3c2e66de7c818fbfff8bb19a3e4459addb91e6250181d708071b4b865b8e7d8e91c1946b8e34e9bd342ab64139c2e3ad7c9cc2af19ba0fe088019e3bac1bb74065cc56b59dd8339f7185226b0c562bd945513e34017a361b92d809bae3571722e8d001c4802dc32b9fd83d4b256063b8ccd876e88bcbfee75674c802ef01d560ddc1d3fe019cb5b8705477c00a5df7c60bfc2eeab8dbeae7446385f8092df40e1b2aed74942209bc781f7083e908c1052a8956caaad6568524e2ee5cf80c93b7475ae34f0f10b12d4ab6c3edbb24bec702f3f4a2c4cdac5b850b43b84848008f6b258c5b513bdff8328e2d47b677cd6e5b1400f30683f2194d8bc2abb1b408007ca1bb03edbeeede65ec2aa46e8c03bc49e6727ef9cdc2753e11c1ead83def1803ab8a2b87b5a8a071bb068e3aaa6acf99cc44f0bb85f800243e434f5f5e90d11
    0x80ffff80c8506376aa83fabe2d996fca70dd34f0c74f0912a9f334515d1e461c5c7ad3118013dea480b22b007ddf0f16db558125e9a0a74c1b952a8cdad3bc90503ce9ed7e80fc243e4f484da27c593fc81bf8ecb75f094a16baaeb9c053ec1d46d58e0fd749800cdf0de785b1fb25058968f7970ded305352281580404a3dae34966de0211ec980577be0182eaaa68dcfc23f658508685d9ba4817f184259b276c037565904611d80257cc14a50ba1f7140f0d662572372e3ee8c9d5ad73a5a216f0042895aaf412480cff328155a82749b9f93ec841f943b8dcadd7e462752316525942f113b2192ee80d701efc27b634794d0efe178f2b862c64e2e96b900a47b94f78bfecdceced16d8089ac4947cca4436c0b068141e86506c9363db3b1878a1664fa34f125993013ad806cefc717087408ad00151c7f7867e8102b8ccc45bba292d1586f8f0431d79ae780eac02fbd43feea54614464334293d92faf7449638fe3266c9b1b627b3654a74a80e7b9339761faecaa2b50baf11b217cc74e2b83e36c0c3906caaaa808efa8f78c8028fc48fd34aa32d65f62407acc62514578b0dc4ec913fe6e029c41ce2bf9b24e802fe3cde3be81d8cb59f77c39198bf7cd89750648e9fb333ec31a95e5647b70f280468de5d44d8d40b50a6eb99c0f5778ce651368b2f250b11b399626c9cdb921d7802cf8570a8b3ffb93ba8b0f240f7278646e8cdf5b874c43a26a300c31b7b40a59
    0x9eaa394eea5630e07c48ae0c9558cef7399f800ad50c0795d01ed308f6e40edf81fb78ab5a51c6cc6cd83f6cabda262cbdd9f3803d437268972d9be876a5e33fb726b96a7c50e7082e6279b2bc5ffa1e328edbc4505f0e7b9012096b41c4eb3aaf947f6ea4290800004c5f0684a022a34dd8bfa2baaf44f172b7100401804d997e9a85de536bdac02a4185da20fad3c2bd1f7e5f55fdacbbd2491d0c8f7280a98466cd4414a7996101b36f2b92e52edf1af89f1340971d33e17ac537608e498038c4bb151b38718f431e5c89f76407dcc9cbedf8f36572d98dc9b888dc75472f805d32741a49498917187fc21f9b8daea0eb94cfab8c3ee4b9c752b8fffa81b0794c5f021aab032aaa6e946ca50ad39ab6660304017c5f09cce9c888469bb1a0dceaa129672ef834c6663d0020706f6c6b61646f74
    0x9f099d880ec681799c0cf30e8886371da9ffff80c31638414fbe7cc191b3c6cd28cd8d03055ee0b5e4fe757f60df518093e928f38056fff4b67e1b3a6a09940f9e9eb1c5d7a5abfdf0b71830629720680df1db2faa80ef11b82aaca916ba891c7db49790cf05307c7d14feed076491cb9479b54414c580e2839d90ec7196c477857120fa357d2f183e4ddeffed6fb1006585b057ad8799808cf91663573a8059a11cbac2916ae252138f79000ea0a667fe113a08ee059e118084169f5b93e95704247df5d583c2a26201ca9ebbade66579b0510f4fda06a128809207464484613bad5c63cb18bfc165e490acd1b30b7a730420bcfb998940aec98031f95dbd307ab02d86589a949a5226b2554448963ab6b7db06ae8bd6ce83e2d980ac24d3857e48c207ddcf1b5a1423a0c053a2dff65f45e41291a4cb03073a54a580c8d420b81fa3970064adb603421ac6e8ed069783858fe996384584c24f17aab5803f83ed2ec2a88e62032a51e8fa76d51c1b84f94e85af26c993d6b0c38003a83880d9e36ad73849bd2719d0e46c94d41322d284d0fd108f3d926e8dccce3d58c7c980d488aae13a3b226f01e2fe2569e2493c00b0df4c89ebe021d97a69dc186d88c0808b3c28a128b4b26902bac7c9d987fa07e8abc284ca8b56491e07dd76ccb6c19180863b4ac70a24751452dcc8209009d92d77530ebf2418763eadf7c434bbfc20d58085788135dac7cde0039752937de974d817cab65545f3a368e406fc6b27240851
  ]
}
```

Magic computer science stuff verifies the proof.

---

# Construct a Transfer Extrinsic

---

## Full Extrinsic Payload

```
Extrinsic Version
	++ Preamble (Address, Signature, Signed Extensions)
	++ Function Call
```

See: `pub struct UncheckedExtrinsic`

---

## Metadata for the Transfer Call

From the Metadata, we can see:

- A pallet called "Balances".
	- With pallet "index 5". (check field, not order in the JSON)
- With Calls of "type 126".
	- This shows us the list of available Calls.
	- One of which is `transfer_keep_alive`,
		- With "call index 3". (check field, not order in the JSON)
	- With two parameters:
		1. `dest` with type `AccountIdLookupOf`
			- Which allows various address formats as input.
		2. `value` with type `T::Balance`
			- Which is `compact` + `u128`

---

## Construct the Transfer Call

```
pallet_index ++ call_index ++ parameters
```

```
pallet_index = 0x05
call_index = 0x03
```

Param 1:

```
AccountId Index = 0x00

Bob = 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
	= 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
```

Param 2:

```
Balance = 12345 = 0xe5c0
```

Final:

```
0x0503008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48e5c0
```

---

## Signed Extensions

Additional functions and information needed to validate an extrinsic.

```json
"signedExtensions": [
{
	"identifier": "CheckNonZeroSender",
	"type": 874,
	"additionalSigned": 36
},
{
	"identifier": "CheckSpecVersion",
	"type": 875,
	"additionalSigned": 4
},
{
	"identifier": "CheckTxVersion",
	"type": 876,
	"additionalSigned": 4
},
{
	"identifier": "CheckGenesis",
	"type": 877,
	"additionalSigned": 13
},
{
	"identifier": "CheckMortality",
	"type": 878,
	"additionalSigned": 13
},
{
	"identifier": "CheckNonce",
	"type": 880,
	"additionalSigned": 36
},
{
	"identifier": "CheckWeight",
	"type": 881,
	"additionalSigned": 36
},
{
	"identifier": "ChargeTransactionPayment",
	"type": 882,
	"additionalSigned": 36
},
{
	"identifier": "PrevalidateAttests",
	"type": 883,
	"additionalSigned": 36
},
{
	"identifier": "CheckMetadataHash",
	"type": 884,
	"additionalSigned": 34
}
]
```

---

## Specific Signed Extensions Explained Simply

```
[F] - Functional check, no data needed.
[H] - Hidden in signature, reintroduced by runtime.
[I] - Directly included, plus some functional logic.
```

<div class="text-small">

1. `CheckNonZeroSender`: [F] Ensures that we do not allow calls from the all `0` address.
2. `CheckSpecVersion`: [H] Verifies the transaction was created for the current runtime specification.
3. `CheckTxVersion`: [H] Confirms the runtime can understand the transaction payload.
4. `CheckGenesis`: [H] Makes sure the transaction is valid only for a specific blockchain.
5. `CheckMortality`: [I] Guarantees a transaction is only valid for a limited time, preventing replays.
6. `CheckNonce`: [I] Enables transaction ordering and prevents transaction replay.
7. `CheckWeight`: [F] Checks transaction's weight fits in the block.
8. `ChargeTransactionPayment`: [I] Allows tips and deducts the final transaction fee.
9. `PrevalidateAttests`: [F] Specifically used for Ethereum ICO claims of DOT.
10. `CheckMetadataHash`: [H] Ensures the transaction was created using the appropriate metadata.

</div>

---

TODO: more with signed extensions

---

# Submit and Track Transaction

---

## Submitting the Extrinsic

- The light client will then call the RPC `author_submitAndWatchExtrinsic` over a WebSocket connection.
- This will accept the extrinsic payload AND return a subscription id to track the progress of the extrinsic.
- From that point forward, the node will push updates back to the light client, with the `SubscriptionId`.

See: https://github.com/paritytech/jsonrpsee

---

## Transaction Pool

TODO

---

## Transaction Gossipping

TODO

---

## Signed Extension Verification

TODO

---

## Transaction Priority

---

## Client and Runtime

TODO

---

## Double Encoded

TODO

---
