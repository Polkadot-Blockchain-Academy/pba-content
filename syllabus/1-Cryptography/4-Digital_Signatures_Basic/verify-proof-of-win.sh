#!/bin/sh
# #####################################################################
# Polkadot Blockchain Academy Proof-of-Winning tooling
# Verify a `PWN-<your address>.json` payload
# 
# Polkadot Blockchain Academy - UNLICENSE - 2023-02-01
# #####################################################################

# echo    " Dependencies:\n"
# echo    " - sha512sum (OS package manager): https://unix.stackexchange.com/questions/426837/no-sha256sum-in-macos"
# echo    " - subkey (cargo): https://github.com/paritytech/substrate/tree/master/bin/utils/subkey#install-with-cargo"
# echo    " - jq (OS package manager): https://stedolan.github.io/jq/\n"


# Gather payloads, and run on a single file with this script command:

subkey verify --message "$(jq '.message' "$1" -rj)" "$(jq '.signature' "$1" -rj)" "$(jq '.ss58Address' "$1" -rj)"

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