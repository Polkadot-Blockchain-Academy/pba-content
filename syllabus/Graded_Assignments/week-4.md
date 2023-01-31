# Week 4 Assignment - Polkadot and XCM

This assignment will be focused on understanding and interacting with reserve chains and the reserve based transfer mechanism. The students will be provided with a relay chain and parachain nodes, this last one lacks some key configuration over the `xcm_config.rs` that will need to be completed as part of the assigment. Also using the reserve based transfer some tokens will need to be transferred as well.
## Build requirements.

For this assignment, you will be provided with:

- Rococo Relay Chain.

https://github.com/paritytech/polkadot

- Parachain template.

< SHARE THE LINK >

You will have to build both projects following these guideliness:

Please use Rococo 0.9.37 release for your relay chain.
## Requirements

### ROC transfer from Rococo to the parachain

The first part of this assigment is to send some ROC tokens (native Rococo token) to the parachain by executing a reserve based transfer.
For testing the whole interaction you will have to define your transfer as a new test entry in the XCM simulator.
Please take into account that you can use the existing extrinsic `limitedReservedTransferAssets` from the `xcmPallet` from sending ROC to the parachain.

Extra points: Instead of using the extrinsic `limitedReservedTransferAssets` from the `xcmPallet`, you can build the XCM message by your own.

### XCM Configuration (xcm_config.rs)

- We will provide you with a parachain template with some missing XCM configurations. Part of the assignment is to complete these configurations and make the process to work (the configuration quality will be graded). These configurations will be:

- Barrier:
  - You can configure the Barrier in the way you consider best. Please note that copying and pasting from other parachains might not work as expected for this assignment. So please be mindful of your Barrier setup.
 
- AssetTransactor:
  - The easiest way to configure this is by using the transactor related to the pallet_balance. This is the minimum accepted solution for this configuration.
  - Local transactor.
  - Extra points: 
      - Add the Fungible transactor and instead of minting the received token using the pallet_balances, use the `pallet_assets` to mint a derived (a.k.a wrapped) asset for ROC. You might call this asset in the parachain WROC

- IsReserve: Configure the parachain to recognize the relay chain as the valid reserve of assets.

- Filters: As the calls from pallet_xcm are not supposed to be used in the parachain, all the corresponding filters are set to `Nothing`. Please do not change anything from it.

### Extra points : Build a pallet that sends the ROC token back to Rococo.

For sending the ROC back from the parachain to the relay chain, you will have to create a new pallet with a specific call that commits this purpose. The logic for the pallet shouldn't be super complex as its only purpose is to send the XCM message only.
As a requirement for the pallet, when the message is dispatched please add a custom event that confirms the XCM message was attempted and also add your name in it.

### README.md

As previous assigments, please provide a README.md file with all the necessary information to understand your project and your reasoning behind it.

 
