# Debug-failed-xcm-messages

A set of scripts to help XCM debugging

## Install dependencies

From this directory

`npm install`

### Decode XCM Relay

Script to specifically decode XCM messages sent to the relay chain via UMP.

The script accepts these input fields:

- `--relay-ws-provider` or `--wr`, which specifies the websocket provider of the relay chain in which the XCM will be decoded
- `--block-number` or `-b`, which specifies the block number where the XCM message to be decoded is contained
- `--para-id` or `-p`, which specifies the parachain ID from which the XCM message was sent from

For example:

`npm run xcm-decode-relay -- --wr wss://kusama-rpc.polkadot.io --b 12034878 --p 2023`

### Decode XCM Parachain

Script to specifically decode XCM messages sent to parachains either via DMP or HRMP/XCMP

The script accepts these input fields:

- `--para-ws-provider` or `--wr`, which specifies the websocket provider of the parachain in which the XCM will be decoded
- `--block-number` or `-b`, which specifies the block number where the XCM message to be decoded is contained
- `--channel`, which specifies the type of channel (or transport method) the XCM is being delivered through. Valid options are `dmp` and `hrmp`/`xcmp` (although anything different than `dmp` defaults to `hrmp` or `xcmp`)
- `--para-id` or `-p`, (optional if channel is hrmp/xcmp) which specifies the parachain ID from which the XCM message was sent from

For example:

`npm run xcm-decode-para -- --wr wss://wss.api.moonriver.moonbeam.network --b 2391172 --channel dmp`

`npm run xcm-decode-para -- --wr wss://wss.api.moonbeam.network --b 1649282 --channel hrmp --p 2000`
