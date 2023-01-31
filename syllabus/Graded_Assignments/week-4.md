# Week 4 Assignment - Polkadot and XCM

This assignment will be focused on understanding and interacting with reserve chains and the reserve based transfer mechanism. The students will be provided with a relay chain and parachain nodes, this last one lacks some key configuration over the `xcm_config.rs` that will need to be completed as part of the assigment. Also a reserve based transfer of ROCs tokens from Rococo to the parachain will need to happen as well.
## Build requirements.

For this assignment, you will be provided with:

- Rococo Relay Chain.

https://github.com/paritytech/polkadot

- Parachain template.

< SHARE THE LINK >

Note: Please use Rococo 0.9.37 release for your relay chain.
## Requirements

### Parachain XCM Configuration (xcm_config.rs)

As we mentioned we will provide you with a parachain template with some missing XCM configurations. Please take into account that your configuration will need to support reserve based transfers from the relay chain and the configuration quality will be graded). 

Specifically you will need to set up the following configurations:

- Barrier:
  - You can configure the Barrier in the way you consider best. Please note that copying and pasting from other parachains might not work as expected for this assignment. So please be mindful of your Barrier setup.
 
- AssetTransactor:
  - The easiest way to configure this is by using the transactor related to the `pallet_balance`. This is the minimum accepted solution for this configuration.
  - Local transactor.
  - Extra points: 
      - Add the Fungible transactor so the received ROCs are minted as a derived (a.k.a wrapped) asset (WROC). The `pallet_assets` can be used for the miniting.

- IsReserve: Configure the parachain to recognize the relay chain as the valid reserve of assets.

- Filters: As the calls from pallet_xcm are not supposed to be used in the parachain, meaning that all the corresponding filters are set to `Nothing`. As a requirement the filters should not be changed.

### ROC transfer from Rococo to the parachain

The second part of this assigment is to send some ROC tokens (native Rococo token) to the parachain by executing a reserve based transfer.
For testing the whole interaction you will have to define your transfer as a new test entry in the XCM simulator.
Please take into account that you can use the existing extrinsic `limitedReservedTransferAssets` from the `xcmPallet` from sending ROC to the parachain.

Extra points: Instead of using the extrinsic `limitedReservedTransferAssets` from the `xcmPallet`, you can build the XCM message by your own.
### Extra points : Build a pallet that sends the ROC token back to Rococo.

Send the ROCs received back from the parachain to the relay chain by creating a new pallet with a specific call that commits this purpose. The logic for the pallet shouldn't be super complex as its only purpose is to send the XCM message only.
Also this execution should be added in the XCM simulator so we can validate the correct functioning of the pallet.

### README.md

As previous assigments, please provide a README.md file with all the necessary information to understand your project and your reasoning behind it.

 
