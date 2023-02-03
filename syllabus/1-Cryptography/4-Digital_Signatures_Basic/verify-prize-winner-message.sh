#!/bin/sh
# #####################################################################
# Polkadot Blockchain Academy Proof-of-Winning tooling
# Verify a `PWN-<your address>.json` payload
# 
# Polkadot Blockchain Academy - UNLICENSE - 2023-02-01
# #####################################################################

# Gather payloads, and run on a single file with this script command:
echo    " Dependencies:\n"
echo    " - sha512sum (OS package manager): https://unix.stackexchange.com/questions/426837/no-sha256sum-in-macos"
echo    " - subkey (cargo): https://github.com/paritytech/substrate/tree/master/bin/utils/subkey#install-with-cargo"
echo    " - jq (OS package manager): https://stedolan.github.io/jq/\n"

jq '.signature' "$1" -rj

subkey verify --message "$(jq '.message' "$1" -rj)" "$(jq '.signature' "$1" -rj)" "$(jq '.ss58Address' "$1" -rj)"
