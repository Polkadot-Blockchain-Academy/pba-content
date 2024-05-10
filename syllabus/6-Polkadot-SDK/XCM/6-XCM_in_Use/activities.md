## Activities

In this chapter, we will engage in different activities compared to the ones discussed in "Chapter 5 XCM in Polkadot," which focused on VRP messages between Rococo and it's AssetsHub parachain.
Here, our focus will be on HRMP transactions between Turing Staging and Mangata.

Please note that the practice of this activity is simpler than what is taught in the slides, and this can be attributed to two reasons:

1. No need to create a proxy wallet or configure special Barriers:
   Unlike the example shown in the slides, the current activity does not require the creation of a proxy wallet or the configuration of special Barriers.
   This simplifies the process and removes the additional steps involved in setting up those components.
1. Variation of HRMP message showcasing the endless possibility of XCM composition:
   The activity focuses on demonstrating a variation of the HRMP message.
   By showcasing this variation, it highlights the vast possibilities and flexibility of XCM composition.
   This serves as a valuable insight into the potential of XCM messaging and its adaptability for different use cases.

### Top up wallet with TUR and MGX

Follow the steps below to top up your wallet with TUR and MGX:

1. Go to Turing Staging's faucet and obtain some TUR.
1. Reserve transfer TUR to Mangata.
1. Swap TUR for MGX on Mangata's DEX.
   By completing these steps, you should have TUR on Turing Staging, as well as TUR and MGX on Mangata.

### Mint liquidity on Mangata DEX

The objective of this activity is to remotely call a `compound_reward` extrinsic and practice HRMP messaging.
To achieve this, follow the demo code provided to call `mintLiquidity` on Mangata.
This will allow you to deposit TUR and MGX into the TUR-MGX pool and receive LP tokens.

### Call compound_rewards via XCM

This is the crucial step where you construct an XCM message from the Turing Staging side and successfully execute it on Mangata.
To accomplish this, it is essential for developers to understand the usage of multi-location, weights, and fee calculation.

### Set up a task to call compound_rewards automatically

For more advanced usage, you can set up a task on Turing Staging to trigger the compound_rewards periodically.
Follow these steps:

1. Obtain the encoded hash of a working example from the previous step.
1. Pass the encoded hash as a payload into automationTime.scheduleXcmpTask.
1. Configure the correct weights for the task.

### Debug XCM messages with xcm-tools

Feel free to utilize [Moonbeam’s xcm-tools](https://github.com/Moonsong-Labs/xcm-tools) to debug your XCM messages, troubleshoot any issues, and gain a deeper understanding of the message details.

Please let me know if you need any further assistance or have any questions.

Happy coding!
