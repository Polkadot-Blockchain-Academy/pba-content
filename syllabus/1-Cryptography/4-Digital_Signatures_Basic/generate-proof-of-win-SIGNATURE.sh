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


echo "     🎉🔐🔏 Polkadot Blockchain Academy Proof-of-Win (PWN) 🔏🔐🎉\n"
echo "============================================================================\n"
echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
echo "   Make sure to read and understand what this script does before you use it!"
echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n"
echo "                            This script will read your:"
echo "                           -      ⚠PRIZE SECRET⚠"
echo "                           - *Signed* Matrix username"
echo "                 *without* writing to disk or terminal history.\n"
echo "                It outputs a \"PWN-<your address>.json\" to send"
echo "                       to the Academy team to verify 🕵️\n"
echo "              You could use any Substrate compatible wallet to sign,"
echo "             and instead use the \"generate-proof-of-win-SIGNATURE.sh\"\n"


echo -n "  🗪 Your https://matrix.org contact (@some-dude:matrix.io): " 
# debug:
MATRIX="@some-dude:matrix.io"
# read MATRIX

echo "  🔏 Your need to sign a message that is *exatly* your matrix username:\n"
echo "  Polkadot.js UI & connected wallet -> sign \"" "$MATRIX" "\" *!!without quotes!!*"
echo "  📝 if you want to sign with \'subkey\', instead use \"generate-proof-of-win-PRIVATE-KEY.sh\""

echo "  🙈 Your  provided secret is hashed for you by the script,"
echo "     not exposed in the json message."
echo -n "  🏆 Your prize secret: "
echo -n "  (three words, space separated) used in the Econ games: "
# debug:
SECRET="some thee words"
# read SECRET
SECRET_HASH="0x""$(printf "$SECRET" | sha512sum | awk '{print $1}')"

# DELETE SECRET
unset SECRET

echo -n "  🔏 Your SIGNATURE for the message \"" "$MATRIX" "\" (raw hex): "
# USING SEED: subkey inspect "middle harsh axis absurd message meadow kick soccer empty left adult giraffe//some///path"
# debug:
SIGNATURE="0x1234567890"
# read SIGNATURE

echo    "  🙋 Your PUBLIC KEY (ss58 address or raw hex) used to sign the above"
echo -n "  💸 THE PRIZE WILL BE SENT HERE (0x..... *or* 14VJA6...): "
# debug:
PUB="14VJA6QWfE7iEXsvrcE8vmF5wnEqEfimG8s35VfWU1TJYPVR"
# read PUB
  
echo -n "The network for the SS58 address (polkadot, kusama, some parachain...): "
# debug:
NETWORK="polkadot"
# read NETWORK

# convert network
ADDRESS="$(subkey inspect "$PUB" --network "$NETWORK" --output-type json | jq '.ss58Address' -rj)"

echo -n "  🙋 Your Pub Key (SS58) for ""$NETWORK"" =""$ADDRESS"

echo "\n\n            👇 🔐 Generated \"PWN-""$ADDRESS"".json\" 🔐 👇"
# Write PWN-$ADDRESS.json
jq -n --arg matrix "$MATRIX" --arg ss58Address "$ADDRESS" --arg secretHash "$SECRET_HASH" --arg signature "$SIGNATURE" '.matrix = $matrix | .ss58Address = $ss58Address | .secretHash = $secretHash | .signature = $signature' > PWN-$ADDRESS.json 

jq < PWN-$ADDRESS.json

echo "           📬📬📬 Send \"PWN-""$ADDRESS"".json\" to your TA! 📬📬📬"
echo "                You must use the "$MATRIX" account to qualify.\n"
echo "                                Less Trust."
echo "                                More Truth.\n"