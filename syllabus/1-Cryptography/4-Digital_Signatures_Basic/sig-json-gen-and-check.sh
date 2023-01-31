#!/bin/sh

# Message to send
# Update to include your actual handle sending the message from that accout to the team
MSG="I am @some-dude:matrix.io from the Academy, here to clam my prize!"

# Temporary store of Secret Key or Mnemonic
# *** add a space before this command to omit from shell history!! ***
 # Secret phrase: argue gain dizzy flock weekend danger dilemma merit lake picnic genuine angry
 TMPSK="0x30b2b3fd43f50f04a8c4ffb3f6998d892dd4eeafd0a974380e251c3a1634545d"

# URI = address SS58 or pubkey-hex or privkey-hex
PUB="$(./target/release/subkey inspect --output-type json $TMPSK | jq '.publicKey' -rj)"

# User signs
# no new line is critical here, for compatibility w/ polkadot.js apps UI
 SIG="$(./target/release/subkey sign --suri "$TMPSK" --message "$MSG")"
## Known good
# 0x68c4dca7a33fab33e81f4e2a3a2addbebfbe95d41467ea9a30ebb9b465b1221c8f5679e9c33245e6a409a07e8bd544be2bc9f11f503476896cd1a55d071afc8c


# Write prize-verify.json
jq -n --arg msg "$MSG" --arg pub "$PUB" --arg sig "$SIG" '.message = $msg | .publicKey = $pub | .signature = $sig' > prize-verify.json 

# User sends prize-verify.json to team...

\cat prize-verify.json

echo "=============="

# Team validates with this command using prize-verify.json

./target/release/subkey verify --message "$(jq '.message' prize-verify.json -rj)" "$(jq '.signature' prize-verify.json -rj)" "$(jq '.publicKey' prize-verify.json -rj)" 
