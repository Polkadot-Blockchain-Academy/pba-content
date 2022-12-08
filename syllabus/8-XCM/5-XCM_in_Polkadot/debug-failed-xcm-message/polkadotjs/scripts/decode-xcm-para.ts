// Import
import { ApiPromise, WsProvider } from "@polkadot/api";
import yargs from "yargs";
import { decodeXCMGeneric } from "./decode-xcm-generic";

import { blake2AsU8a } from "@polkadot/util-crypto";
import { u8aToHex } from "@polkadot/util";

const args = yargs.options({
  "para-ws-provider": { type: "string", demandOption: true, alias: "wr" },
  "block-number": { type: "number", demandOption: true, alias: "b" },
  channel: { choices: ["dmp", "hrmp"], demandOption: true, alias: "channel" },
  "para-id": { type: "number", demandOption: false, alias: "p" },
}).argv;

// Construct
const paraProvider = new WsProvider(args["para-ws-provider"]);

async function main() {
  // Provider
  const paraApi = await ApiPromise.create({ provider: paraProvider });

  // Get block hash
  const blockHash = (await paraApi.rpc.chain.getBlockHash(args["block-number"])) as Uint8Array;
  // Get block by hash
  const signedBlock = (await paraApi.rpc.chain.getBlock(blockHash)) as any;

  // the hash for each extrinsic in the block
  signedBlock.block.extrinsics.forEach((ex, index) => {
    // Parachain Inherent set validation data.
    // Probably needs to be mapped to pallet index too
    if (ex.method._meta["name"] == "set_validation_data") {
      if (args["channel"] == "dmp") {
        // Check downward messages (from relay chain to parachain)
        // Go through each message
        ex.method.args[0].downwardMessages.forEach((message) => {
          // We recover all instructions
          decodeXCMGeneric(paraApi, message, 1); //Type 1 is DMP
        });
      } else {
        // Check hrmp messages (from parachain to parachain)
        // Types
        const para = paraApi.createType("ParaId", args["para-id"]) as any;
        ex.method.args[0].horizontalMessages.forEach((messages, paraId) => {
          // Filter by the paraId that we want
          if (paraId.eq(para)) {
            // Go through each message
            messages.forEach((message) => {
              // We recover all instructions
              decodeXCMGeneric(paraApi, message, 2); //Type 2 is HRMP
            });
          }
        });
      }
    }
  });
}

main()
  .catch(console.error)
  .finally(() => process.exit());
