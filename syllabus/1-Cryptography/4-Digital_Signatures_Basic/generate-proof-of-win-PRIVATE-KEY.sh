#!/bin/sh
# #####################################################################
# Polkadot Blockchain Academy Proof-of-Winning tooling
# Generate `proof-of-winning.json` payload
#
# Dependencies:
#
# - subkey, via cargo: https://github.com/paritytech/substrate/tree/master/bin/utils/subkey#install-with-cargo
# - jq, via package manager of your OS: https://stedolan.github.io/jq/
# - sha512sum, via package manager of your OS: https://unix.stackexchange.com/questions/426837/no-sha256sum-in-macos
# 
# Polkadot Blockchain Academy - UNLICENSE - 2023-02-01
# #####################################################################


echo "//////////////////////////////////////////////////////////////////////////////////////////////////////////"
echo "////////////////           ///////////////////////////////////////////////////////////////////////////////"
echo "//////////////               /////////////////////////////////////////////////////////////////////////////"
echo "////////    ////           ////    ////////////////////////////////*  *///////////////////////////////////"
echo "/////        /////////////////        ////////////////////////////.   ,///////////////////////***/////////"
echo "////         /////////////////          /////////////////////////*    ///////////////////////.   ,////////"
echo "///         ////////////////////        /////////////////////////.   ,//////////////////////*.   /////////"
echo "///      /////////////////////////      //////////////*,.      .,   .//////*..      .,..,.            *///"
echo "////////////////////////////////////////////////////,.   ..,**,.    ,///*,    .,,,.    ***,.    ,,,,,*////"
echo "/////  ///////////////////////////////////////////*.   ,*//////,   ,*//*    *//////,   ,*/*.   ,//////////"
echo "///       ///////////////////////       /////////*.   *///////,    *//*    *///////,   ,//*   .///////////"
echo "///         ///////////////////         ////////*     ///////,    .*//,   .///////,    */*.   ,/////**////"
echo "////         /////////////////         /////////*     ,///*,.     ,**,     *////*.   .///*.   ,//*.   ,///"
echo "//////       /////////////////        ///////////,.         ,,       .,.          ,*/////,         .*////"
echo "///////////////             ////////////////////////**,.,,,*///*,,,,*/////*,,,,**///////////*,,,,**///////"
echo "///////////////             //////////////////////////////////////////////////////////////////////////////"
echo "//////////////////       /////////////////////////////////////////////////////////////////////////////////"
echo "//////////////////////////////////////////////////////////////////////////////////////////////////////////"
echo ""
echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
echo "                  Make sure to read and understand what this script does before you use it!"
echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n"
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


echo -n "  📝 A pubic, psudononomous, message for the Academy class (any text, without \"quotes\"): " 
# debug:
MESSAGE="I LIKE WINNING! BOOOOO YAAAAAA!"
# read MESSAGE

echo "  🙈 Your  provided secret is hashed for you by the script,"
echo "     not exposed in the output."
echo -n "  🏆 Your prize secret: "
echo -n "  (three words, space separated) used in the Econ games: "
# debug:
SECRET="some thee words"
# read SECRET
SECRET_HASH="0x""$(printf "$SECRET" | sha512sum | awk '{print $1}')"

# DELETE SECRET
unset SECRET

echo    "  🔑 Your PRIVATE KEY (hex encoding *or* mnemonic & derived path)"
echo -n "  💸 THE PRIZE WILL BE SENT HERE (0x..... *or* [12|24 words here]//HD-Wallet///Path): "
# debug:
PRIVATE="middle harsh axis absurd message meadow kick soccer empty left adult giraffe//some///path"
# read PRIVATE
  
echo -n "  🕸️ The network for the SS58 address (polkadot, kusama, some parachain...): "
# debug:
NETWORK="polkadot"
# read NETWORK

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
echo -n "                            🗑 Press [ENNTER] key to clear the screen..."
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
#   "signature": "0x5261afc255c4b5863dee0ff9199dbc321c2c79460cac0e46ac81a21171744a183d517c51822bcd3adc0e249412a27594ab26665ff85d95ed503d56781ef22683"
# }