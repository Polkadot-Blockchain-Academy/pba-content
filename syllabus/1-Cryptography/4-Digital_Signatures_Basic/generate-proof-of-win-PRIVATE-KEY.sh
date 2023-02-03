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
read MESSAGE
# debug, uncommnet to override:
MESSAGE="I LIKE WINNING! BOOOOO YAAAAAA!"

echo "  🙈 Your  provided secret is hashed for you by the script,"
echo "     not exposed in the output.\n"
echo "  🏆 Your prize secret (three words, space separated):"
read SECRET
# debug, uncommnet to override:
SECRET="some thee words"

SECRET_HASH="0x$(printf "$SECRET" | sha512sum | awk '{print $1}')"

# DELETE SECRET
unset SECRET

echo "  🔑 Your PRIVATE KEY (hex encoding *or* mnemonic & derived path)"
echo "  💸 THE PRIZE WILL BE SENT HERE (0x..... *or* [12|24 words here]//HD-Wallet///Path):"
read PRIVATE
# debug, uncommnet to override:
PRIVATE="middle harsh axis absurd message meadow kick soccer empty left adult giraffe"
# HD path works, but not used in most wallets 😭 :
# PRIVATE="middle harsh axis absurd message meadow kick soccer empty left adult giraffe//some///path"
  
echo "  🕸️ The network for the SS58 address (polkadot, kusama, some parachain...): "
read NETWORK
# debug, uncommnet to override:
NETWORK="polkadot"

# subkey needs an sURI = address SS58 or pubkey-hex or privkey-hex
ADDRESS="$(subkey inspect "$PRIVATE" --network "$NETWORK" --output-type json | jq '.ss58Address' -rj)"

echo -n "  🙋 Your Pub Key (SS58) for $NETWORK = $ADDRESS"

# Sign your provided message username (only)
SIGNATURE="$(subkey sign --suri "$PRIVATE" --message "$MESSAGE")"

# DELETE PRIVATE
unset PRIVATE

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



# debug, no HD path, most wallets:
# {
#   "message": "I LIKE WINNING! BOOOOO YAAAAAA!",
#   "ss58Address": "14XeJg226wvHG6PWmhKUsrv5PmeccjbXwFe9pVrBbryEWeZc",
#   "secretHash": "0x58cf16bcdceec9bce18246eeaa2f3358a2cdfdb7dc98a3d5f61da18f841b057369c58e64a456e236e853d853ef088a0eb57551a2a2b124c3060d5f402a2bf0a3",
#   "signature": "0x683dc112821364f6201f5e6c231a156ae8a4bc10a931972825543c6e8f273e47b271756d70366ba154fb29ea15360d3210f8e05951d64dd27518c8fd3476a587"
# } 
# Tested to verify correctly on https://polkadot.js.org/apps/#/signing/verify
# Also another good signature:
# "0x3ef12aa93dc7eca7d60616f9b2535f967c202ba77ca80451847d528025bde836280744bf9ca643413c93ce3702841414ef5b9ff4d0bb9c63f1b48dc0a2e6ff8e"

# debug, HD "//some///path":
# {
#   "message": "I LIKE WINNING! BOOOOO YAAAAAA!",
#   "ss58Address": "14VJA6QWfE7iEXsvrcE8vmF5wnEqEfimG8s35VfWU1TJYPVR",
#   "secretHash": "0x58cf16bcdceec9bce18246eeaa2f3358a2cdfdb7dc98a3d5f61da18f841b057369c58e64a456e236e853d853ef088a0eb57551a2a2b124c3060d5f402a2bf0a3",
#   "signature": "0xde6b453a72d65de1661305af10595c9126e72bc75a475d299635229bc69ac20e3aff52e0cb5c8059224f16d2231ede92e8ff37d3739099d9fe20df0c0863bb84"
# }