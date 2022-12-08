import { blake2AsU8a } from "@polkadot/util-crypto";
import { u8aToHex } from "@polkadot/util";

export function decodeXCMGeneric(provider: any, message: any, type: number) {
  let fragments;
  switch (type) {
    case 0:
      // XCM going to the Relay Chain (UMP)
      fragments = decodeMessageIntoFragmentVec(provider, message);
      break;
    case 1:
      // XCM going from the Relay Chain to a Parachain (DMP)
      fragments = decodeMessageIntoFragmentVec(provider, message.msg);
      break;
    case 2:
      // XCM going from a Parachain to another Parachain (HRMP/XCMP)
      // First byte is a format version that creates problme when decoding it as XcmVersionedXcm
      // We remove it

      fragments = decodeMessageIntoFragmentVec(provider, message.data.slice(1));
      break;
    default:
      console.error("Not supporting this particular scenario!");
      break;
  }

  for (let i = 0; i < fragments.length; i++) {
    let instructions = fragments[i];
    console.log(`Blake2 hash of fragment ${i} is: ${u8aToHex(blake2AsU8a(instructions.toU8a()))}\n`);
    console.log(`Instructions of fragment ${i} are: ${instructions.toString()}\n`);
  }
}


function decodeMessageIntoFragmentVec(provider: any, message: any) {
  let fragments = [];
  let remainingMessage = message;
  while (remainingMessage.length != 0) {
    let fragment = provider.createType("XcmVersionedXcm", remainingMessage) as any;
    fragments.push(fragment);
    remainingMessage = remainingMessage.slice(fragment.toU8a().length)
  }
  return fragments;
}