# Week 4 Assignment - Polkadot and XCM

This is the final graded assignment where we will apply everything we have learned about Polkadot & XCM.

# Description and overall goal.

For this assignment, you will be provided with:

- Rococo Relay Chain.
- Parachain template.

The overall goal for this assignment is to be able to reserve-based transfer some ROC (native asset of the Rococo relay chain) to a parachain built by you and then send them back to the relay chain.

## XCM Configuration (xcm_config.rs)

- We will provide you with a parachain template with some missing XCM configurations. Part of the assignment is to complete these configurations and make the process work (the configuration quality will be graded). These configurations will be:

- Barrier:

  - You can configure the Barrier in the way you consider best. Please note that copying and pasting from other parachains might not work as expected for this assignment. So please be mindful of your Barrier setup.
  - Extra points: Explain in your own words what each element of the Barrier config is doing (point out where and how all these are used in the xcm_builder).

- AssetTransactor:
  - The easiest way to configure this is by using the transactor related to the pallet_balance. This is the minimum accepted solution for this configuration.
  - Local transactor.
    Extra points: 
      - Configuring the Fungible transactor as well as the previous two.
      - If the ROCs arriving to the parachain are minted into the parachain as a derived (a.k.a wrapped) assets using something as the `pallet_assets`, you would add extra points since the Fungible transactor will be used for minting the assets.

- IsReserve: Configure the parachain to recognize the relay chain as the valid reserve of assets.

- Filters: As the calls from pallet_xcm are not supposed to be used in the parachain, all the corresponding filters are set to `Nothing`. Please do not change anything from it.

## Pallet

For sending the ROC back from the parachain to the relay chain, you will have to create a new pallet with a specific call that commits this purpose. The logic for the pallet shouldn't be super complex as its only purpose is to send the XCM message only.

Extra points: Provide the corresponding tests using the XCM Simulator.

## Considerations & Requirements

- For sending the ROC tokens from Rococo to the parachain, you can use the existing extrinsic `limitedReservedTransferAssets` from the `xcmPallet`.

- Use Rococo 0.9.37 release for your relay chain.

- Build both your chain and Rococo and set up a zombienet file (local) that launches.

- Write a Readme file guiding all your development and configuration criteria + a usage and testing description.
