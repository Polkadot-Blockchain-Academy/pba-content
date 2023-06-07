## Activities

### Grab funds in rococo

- Create a Rococo account and a Rockmine account.
- Go to the Rococo faucet and obtain some ROC.
  Follow https://substrate.io/developers/rococo-network/

### Perform a local XCM execution in rococo

- Use **PalletXCM** to Withdraw from your account and deposit in any other account using XCM locally.
  Does it work?
  Why can you do this'
- Use **PalletXCM** to issue a remark transaction with XCM.
  Does it work?
  Why can you do this?

### Teleport ROC to Rockmine

- Use **PalletXCM** to teleport ROC from Rococo to Rockmine.
  Can you do this?
  Why?
- Use both xcmPallet.limitedTeleportAssets and xcmPallet.execute to teleport

### Try teleporting back the ROC from Rockmine to Rococo.

- Use **PalletXCM** to teleport ROC from Rockmine to Rococo

### Try reserve transferring ROC from Rockmine to Rococo.

- Does it work?
  Why?

### Try sending a custom XCM message from Rockmine to Rococo.

- Use the **send** extrinsic from PalletXCM.
- Does Rococo process it?
  Why not?

### SubXT to debug failed messages

Messages to debug in Polkadot:

- Block failure: 13341503 | Message sent from parachain 2000
- Block failure: 10946380 | Message sent from parachain 2012
- Block failure: 10557896 | Message sent from parachain 1000
- Block failure: 11884750 | Message sent from parachain 2006
- Block failure: 13955421 | Message sent from parachain 2011

You can debug with either subxt or with Subscan.
