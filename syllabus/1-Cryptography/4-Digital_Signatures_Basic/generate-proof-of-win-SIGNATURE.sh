#!/bin/sh
######################################################################
# Polkadot Blockchain Academy Proof-of-Winning tooling
# Generate `proof-of-winning.json` payload
#
# For Polkadot.js API and <Bytes /> wrapped messages
# 
# Polkadot Blockchain Academy - UNLICENSE - 2023-02-01
# #####################################################################

## TODO ONCE RESOLVED UPDATE TO MATCH UNIFORM BEHAVIOR:
## https://github.com/polkadot-js/apps/issues/8930

echo    "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
echo    "                  Make sure to read and understand what this script does before you use it!"
echo    "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n"
echo    " Dependencies:\n"
echo    " - sha512sum (OS package manager): https://unix.stackexchange.com/questions/426837/no-sha256sum-in-macos"
echo    " - subkey (cargo): https://github.com/paritytech/substrate/tree/master/bin/utils/subkey#install-with-cargo"
echo    " - jq (OS package manager): https://stedolan.github.io/jq/\n"

echo -n "                                     👌 Press [ENTER] to start..."
read START
clear

echo "//////////////////////////////////////////////////////////////////////////////////////////////////////////"
echo "////////////////           ///////////////////////////////////////////////////////////////////////////////"
echo "//////////////               /////////////////////////////////////////////////////////////////////////////"
echo "////////    ////           ////    ////////////////////////////////*  *///////////////////////////////////"
echo "/////        /////////////////        ////////////////////////////.   ,///////////////////////***/////////"
echo "////         /////////////////          /////////////////////////*    ///////////////////////.   ,////////"
echo "///         ////////////////////        /////////////////////////.   ,//////////////////////*.   /////////"
echo "///      /////////////////////////      //////////////*             .//////*.         .*//*           *///"
echo "////////////////////////////////////////////////////,.   .,,**,.    ,///*,    .,,,.    ***,.    ,,,,,*////"
echo "/////  ///////////////////////////////////////////*.   ,*//////,   ,*//*    *//////,   ,//*.   ,//////////"
echo "///       ///////////////////////       /////////*.   *///////,    *//*    *///////,   ,//*   .///////////"
echo "///         ///////////////////         ////////*     ///////,    .*//,   .///////,    */*.   ,/////**////"
echo "////         /////////////////         /////////*     ,///*,.     ,**,     *////*.   .///*.   ,//*.   ,///"
echo "//////       /////////////////       ////////////,.         ,,       .,.           ,*/////,         .*////"
echo "///////////////             ////////////////////////**,,,,,*///*,,,,*/////*,,,,**///////////*,,,,**///////"
echo "///////////////             //////////////////////////////////////////////////////////////////////////////"
echo "//////////////////       /////////////////////////////////////////////////////////////////////////////////"
echo "//////////////////////////////////////////////////////////////////////////////////////////////////////////"
echo ""
echo "==========================================================================================================\n"
echo "                           🎉🔐🔏 Blockchain Academy Proof-of-Win (PWN) 🔏🔐🎉\n"
echo "==========================================================================================================\n"
echo "                                         This script processes a:"
echo "                                            - ⚠PRIZE SECRET⚠"
echo "                                            - SIGNED MESSAGE"
echo "                              *without* writing them to disk or terminal history.\n"
echo "                               It outputs a \"PWN-<your address>.json\" to submit"
echo "                                      to the Academy team to verify 🕵️\n"
echo "                           Alternatively, you could subkey to sign, providing a secret"
echo "                            Use \"generate-proof-of-win-PRIVATE-KEY.sh\" instead\n"

echo "  🕸️ The network for the SS58 address (polkadot, kusama, some parachain...): "
read NETWORK
# debug, uncomment to override:
NETWORK="polkadot"

echo -n "  🔏 Your need to sign a message out of band from this script.\Provide a"
echo   "  📝 A pubic, pseudononymous, message for the Academy class (any text, without \"quotes\"):\n"

read UNWRAPPED_MESSAGE
UNWRAPED_MESSAGE="I LIKE WINNING! BOOOOO YAAAAAA!"

echo "  🪄 Copy&Paste -> sign this with a PJ.js tool (no padding or spaces):\n"
echo "$UNWRAPED_MESSAGE\n"
echo "               🔏 Open up a wallet using Polkadot.js API to sign, maybe via https://polkadot.js.org/apps/#/signing/ ..."
echo "                                     ⏳ Press [ENTER] after you have a signature..."
read CRAP

MESSAGE="<Bytes>$UNWRAPED_MESSAGE</Bytes>"
echo "  🤦 You message is opaquely wrapped by Polkadot.j s (https://github.com/polkadot-js/apps/issues/8930) so the message you *actually* signed is:\n"
echo "$MESSAGE\n" 
# debug, uncomment to override:
# using Polkadot JS API/Apps, wrapped

echo    "  😅 But we can deal with that 👍 let's continue...\n"
echo -n "  🔏 Your SIGNATURE for the message (raw hex): "
read SIGNATURE
# debug, uncomment to override:
# USING SEED: subkey inspect "middle harsh axis absurd message meadow kick soccer empty left adult giraffe"
SIGNATURE="0x78bea5e6ae9973c9842e33c1f37109fd5a8dc4f954cd22a133756a7590fffd0363f956afd24a16a6bcb00a3ce7bfdcc8045dad80b421bd01a8948ff9d2853e8a"

echo    "  🙋 Your PUBLIC KEY (ss58 address or raw hex) used to sign the above"
echo -n "  💸 THE PRIZE WILL BE SENT HERE (0x..... *or* 14VJA6...): "
read PUB
# debug, uncomment to override:
PUB="14XeJg226wvHG6PWmhKUsrv5PmeccjbXwFe9pVrBbryEWeZc"

echo "  🙈 Your  provided secret is hashed for you by the script,"
echo "     not exposed in the output.\n"
echo "  🏆 Your prize secret (three words, space separated):"
read SECRET
# debug, uncomment to override:
SECRET="some thee words"

SECRET_HASH="0x$(printf "$SECRET" | sha512sum | awk '{print $1}')"

# DELETE SECRET
unset SECRET

# convert network
ADDRESS="$(subkey inspect "$PUB" --network "$NETWORK" --output-type json | jq '.ss58Address' -rj)"

echo -n "  🙋 Your Pub Key (SS58) for ""$NETWORK"" =""$ADDRESS"

FILE="PWN-""$ADDRESS"".json"

echo "\n\n                    👇 🔐 $FILE 🔐 👇"

# Write PWN-$ADDRESS.json
jq -n --arg message "$MESSAGE" --arg ss58Address "$ADDRESS" --arg secretHash "$SECRET_HASH" --arg signature "$SIGNATURE" '.message = $message | .ss58Address = $ss58Address | .secretHash = $secretHash | .signature = $signature' > $FILE

jq < $FILE

echo "                   📬 Deliver $FILE 📬"
echo "                   The Academy team will provide a link to upload or paste this json into.\n"
echo -n "                                 🗑 Press [ENTER] to clear the screen..."
read LESS
clear
echo -n "\n\n\n\n                                        Less Trust."
read MORE
echo            "                                        More Truth.\n\n\n\n"
read MIC_DROP

# debug, no HD path, most wallets:
# {
#   "message": "<Bytes>I LIKE WINNING! BOOOOO YAAAAAA!</Bytes>",
#   "ss58Address": "14XeJg226wvHG6PWmhKUsrv5PmeccjbXwFe9pVrBbryEWeZc",
#   "secretHash": "0x58cf16bcdceec9bce18246eeaa2f3358a2cdfdb7dc98a3d5f61da18f841b057369c58e64a456e236e853d853ef088a0eb57551a2a2b124c3060d5f402a2bf0a3",
#   "signature": "0x78bea5e6ae9973c9842e33c1f37109fd5a8dc4f954cd22a133756a7590fffd0363f956afd24a16a6bcb00a3ce7bfdcc8045dad80b421bd01a8948ff9d2853e8a"
# }
# Tested to verify correctly using the UNWRAPPED message "I LIKE WINNING! BOOOOO YAAAAAA!" on https://polkadot.js.org/apps/#/signing/verify
