#!/bin/sh

# Polkadot Blockchain Academy
# Verify a `proof-of-winning.json` payload

# Polkadot Blockchain Academy - Unlicence - 2023-02-01


# Gather payloads, and run on a single file with this script command:

subkey verify --message "$(jq '.message' prize-verify.json -rj)" "$(jq '.signature' prize-verify.json -rj)" "$(jq '.publicKey' prize-verify.json -rj)" 
