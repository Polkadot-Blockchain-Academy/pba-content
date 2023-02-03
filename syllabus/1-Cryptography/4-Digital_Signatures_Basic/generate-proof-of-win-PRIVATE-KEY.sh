#!/bin/sh
######################################################################
# Polkadot Blockchain Academy Proof-of-Winning tooling
# Generate `proof-of-winning.json` payload
# 
# Polkadot Blockchain Academy - UNLICENSE - 2023-02-01
# #####################################################################

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
echo "                                            - ⚠PRIVATE  KEY⚠"
echo "                              *without* writing them to disk or terminal history.\n"
echo "                               It outputs a \"PWN-<your address>.json\" to submit"
echo "                                      to the Academy team to verify 🕵️\n"
echo "                             You could use any Substrate compatible wallet to sign,"
echo "                            and instead use the \"generate-proof-of-win-SIGNATURE.sh\"\n"


echo "  📝 A pubic, pseudononymous, message for the Academy class (any text, without \"quotes\"):\n" 
# debug:
# MESSAGE="I LIKE WINNING! BOOOOO YAAAAAA!"
read MESSAGE

echo "  🙈 Your  provided secret is hashed for you by the script,"
echo "     not exposed in the output.\n"
echo "  🏆 Your prize secret (three words, space separated):"
# debug:
# SECRET="some thee words"
read SECRET
SECRET_HASH="0x""$(printf "$SECRET" | sha512sum | awk '{print $1}')"

# DELETE SECRET
unset SECRET

echo "  🔑 Your PRIVATE KEY (hex encoding *or* mnemonic & derived path)"
echo "  💸 THE PRIZE WILL BE SENT HERE (0x..... *or* [12|24 words here]//HD-Wallet///Path):"
# debug:
# PRIVATE="middle harsh axis absurd message meadow kick soccer empty left adult giraffe//some///path"
read PRIVATE
  
echo "  🕸️ The network for the SS58 address (polkadot, kusama, some parachain...): "
# debug:
# NETWORK="polkadot"
read NETWORK

# subkey needs an sURI = address SS58 or pubkey-hex or privkey-hex
ADDRESS="$(subkey inspect "$PRIVATE" --network "$NETWORK" --output-type json | jq '.ss58Address' -rj)"

# DELETE PRIVATE
unset PRIVATE

echo -n "  🙋 Your Pub Key (SS58) for $NETWORK = $ADDRESS"

# Sign your provided message username (only)
SIGNATURE="$(subkey sign --suri "$PRIVATE" --message "$MESSAGE")"

FILE="PWN-""$ADDRESS"".json"

echo "\n\n                    👇 🔐 $FILE 🔐 👇"

# Write PWN-$ADDRESS.json
jq -n --arg message "$MESSAGE" --arg ss58Address "$ADDRESS" --arg secretHash "$SECRET_HASH" --arg signature "$SIGNATURE" '.message = $message | .ss58Address = $ss58Address | .secretHash = $secretHash | .signature = $signature' > $FILE

jq < $FILE

echo "                   📬 Send us $FILE 📬"
echo "                   The Academy team will provide a link to upload or paste this json into.\n"
echo -n "                                 🗑 Press [ENTER] to clear the screen..."
read LESS
clear
echo -n "\n\n\n\n                                        Less Trust."
read MORE
echo            "                                        More Truth.\n\n\n\n"
read MIC_DROP

# debug:
# {
#   "message": "I LIKE WINNING! BOOOOO YAAAAAA!",
#   "ss58Address": "14VJA6QWfE7iEXsvrcE8vmF5wnEqEfimG8s35VfWU1TJYPVR",
#   "secretHash": "0x58cf16bcdceec9bce18246eeaa2f3358a2cdfdb7dc98a3d5f61da18f841b057369c58e64a456e236e853d853ef088a0eb57551a2a2b124c3060d5f402a2bf0a3",
#   "signature": "0x20204e4c9dedc55d6ea3140ffb9b2ad7acfc40809380fd8440f0944de6664305c250a88b64615425244ac1c9f21f12f832e12c422c9bfe2e41c1d01d50a33686"
# }

# manual
#  "signature": "0x445824db14fbe78e7c1fd45b9e63c40181ae6e349c88cad23139fb6e5658215d1325b9338bb1389a9dd63e3f5c312ca500f0527a732921d43c9e8b017ab8578d"
