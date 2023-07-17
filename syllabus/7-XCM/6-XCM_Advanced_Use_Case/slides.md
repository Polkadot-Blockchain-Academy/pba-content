---
title: XCM Advanced Use Case # Also update the h1 header on the first slide to the same name
description: Leverage HRMP to remote compound on Substrate-based DEX
duration: 2 hours
---

# XCM Advanced Use Case

---

## _At the end of this lecture, you will be able to:_

<pba-flex center>

1. Perform HRMP transactions between parachains.
2. Gain familiarity with depositing assets into a liquidity pool and receiving LP tokens.
3. Understand the construction and execution of XCM messages.
4. Set up automated tasks triggered by compound_rewards.
5. Develop proficiency in debugging XCM messages using xcm-tools.

---

As we learned in Chapter 3, the XCM pallet serves as a bridge between the XCVM subsystem and the FRAME subsystem. It enables us to send and execute XCM and interact with the XCM executor. In this chapter, I, as the founder of OAK Network and a parachain developer, will demonstrate how to build products using XCM and dispatch them from both polkadot.js apps and Javascript code.

## Overview

In this session, I will teach you how to build a real use case of XCM from the perspective of a parachain developer. Since our primary code base will be a parachain, and not a relay chain like Polkadot, we will be focusing on HRMP messages most of the time, which means horizontal communication between two parachains.

We will use a real example to explain the configuration and composition of an XCM message. The product use is to send a message from Moonriver to Turing Network with the goal to set up an automation calling Turing’s timeAutomation extrinsic. 

The objectives are listed below:

- Target Chain: Turing Network
- Target Chain Version: XCM V3
- Target Extrinsic: `automationTime.scheduleXcmpTask`
- Source Chain: Moonriver
- Source Chain Version: XCM V3
- Source Extrinsic: `xcmTransactor.transactThroughSigned`

End Result: The `scheduleXcmpTask` event fires successfully on Turing Network, which means the remote call is executed successfully and creates an automation task.

The XCM call sets up a recurring task, such as an auto-transfer of MOVR every Monday. Turing Network is responsible for triggering the action when its condition is met. The overall flow of the entire product is shown in the diagram below.

<img style="width: 900px;" src="../../../assets/img/7-XCM/high-level-product-flow.png" alt="High level product flow - Moonriver and Turing Network" />

High level product flow - Moonriver and Turing Network

## Prerequisite

For this demo, we are using the existing xcmPallet built in Polkadot and Kusama. This pallet provides common extrinsic interfaces that developers can use to easily compose an XCM message. Moonriver has further encapsulated the function to make their own xcmTransactor. We will showcase their simple extrinsic `xcmTransactor.transactThroughSigned` first by calling it, and then explain its deeper implementation, the instruction sequence.

Before we get into the actual XCM message, there are some prerequisites that must be fulfilled:

1. **Configure an Allow Barrier,** `AllowTopLevelPaidExecutionFrom<Everything>`, on the recipient chain, in this case, Turing Network. We covered the Barrier topic in the previous chapter. Barriers are responsible for creating Allow or Deny rules for incoming messages. By adding this Barrier, we allow the DescendOrigin instruction in XCM, which will reassign the origination of the transaction on Turing from Moonriver's sovereign account to the user's proxy account.
   
2. **Create a proxy wallet.** The proxy wallet is an account abstraction that allows a blockchain to execute some code on behalf of a user. We need to use a user's sub-wallet for a specific extrinsic call, creating granular control from the user's wallet.

## XCM Message Composition

### XCM configs

In section #4 of the Chain Config in XCM document, we have reviewed various chain configurations. In this section, we will illustrate their usage through a real-world example. Although there are several variables to be decided, once you become familiar with them and establish a few templates, you can continue to use them.

The following are the parameters you need to decide before sending an XCM message:

1. **Version number**: Before composing an XCM message, you must first look up the XCM version number on the recipient chain and ensure that the version is supported on the source chain. In this case, both the recipient chain, Turing Network, and the source chain, Moonriver, support XCM v3.
2. **Weight**: The weight of an XCM instruction is defined with a different value on each chain. It specifies how much computational power is required for the execution of each instruction and determines the gas, or fee, for the XCM execution.
3. **Fee per Second**: In addition to the weight, if we use an asset other than the native token of the recipient chain, TUR in this case, to pay for the fee, the value of the asset must be converted in relation to the recipient chain's native token. The Fee per Second defines the conversion rate between MOVR and TUR, assuming we want to use MOVR to pay for all the fees in this transaction.

Once we decide the values of the above parameters, we can move forward to the next step, which is to construct the instruction sequence of the message.

### Message elements

In this section, we will construct the XCM message using Moonriver’s `xcmTransactor.transactThroughSigned` extrinsic. Its parameters include the following elements:

1. Destination: This refers to the target chain to which the message will be sent. Since we will be sending the message to the Turing Network (paraId 2114 on Kusama), we will set the value to {Relay, 2114}.
    
    !https://s3-us-west-2.amazonaws.com/secure.notion-static.com/f48b0fb6-7c4e-4340-961d-6e4c0125cec9/Untitled.png
    
2. InnerCall: This is the encoded call hash of the transaction on the destination chain. This value will be passed on to the Transact instruction within the XCM message, which we will discuss in the next section.
3. Fees: `transactRequiredWeightAtMost` sets a limit on the gas fee of the innerCall to prevent the transaction from costing too many fee tokens. Similarly, `overallWeight` sets an upper limit on the XCM execution plus the Transact hash.

Once the parameters are filled in, we can submit and sign the transaction. With all the inputs defined, we can kick off the XCM message directly from the extrinsic tab of polkadot.js apps.

<img style="width: 900px;" src="../../../assets/img/7-XCM/polkadot-xcm-call.png" alt="The polkadotXcm.send() extrinsic call to fire the XCM message" />

The polkadotXcm.send() extrinsic call to fire the XCM message

After firing the message, events from both the sender and recipient parachains should appear in the Polkadot.js app Network tab. Screenshots of this process will be demonstrated in the [Debugging Live](https://www.notion.so/PBA-Founders-Track-Product-Showcase-with-XCM-Automation-0ba978674d8c4a91b17a5237a762278d?pvs=21) section below.

### Sequence of Instruction

Once the transaction above is submitted and finalized on the chain, we can use the xcm-tools built by the Moonbeam team to inspect the XCM message. The code and scripts for the tool are listed in [this Github repo](https://github.com/Moonsong-Labs/xcm-tools). An example of the script is shown below:

`yarn xcm-decode-para --w wss://wss.api.moonbeam.network --b 1649282 --channel hrmp --p 2000`

Now let's take a closer look at the output of the script. Each line in the output represents an XCM instruction that we learned about in previous chapters. We will go through them in order:

1. DescendOrigin (descend_location): The first instruction in the XCM instruction array is DescendOrigin, which transfers authority to the user's proxy account on the destination chain.
2. WithdrawAsset and BuyExecution: The two instructions combined deduct XCM fees from the user's proxy wallet and reserve them for execution.
3. Transact(origin_type, require_weight_at_most, call): Transact is where the encoded innerCall is executed on the target chain. Remember that we specified a requireWeightAtMost during the call to ensure that its gas cost does not exceed the specified limit.
4. RefundSurplus and DepositAsset: If there is any leftover fee token after Transact execution, these instructions guarantee they will be refunded and transferred to the specified location, normally the user's wallet.

## Code from client (Javascript)

After proving that the XCM message above executes correctly, we can replicate the procedure from the client of a dApp. Below is a node.js code snippet we created for this particular demo.

URL: 

[xcm-demo Github Repo](https://github.com/OAK-Foundation/xcm-demo/blob/master/src/moonbeam/moonbase-alpha.js)

To run the program, clone it using git and execute the following command:
`PASS_PHRASE=<PASS_PHRASE> PASS_PHRASE_ETH=<PASS_PHRASE_ETH> npm run moonbase-alpha`

As you can see from the code, there are several preparation steps leading up to the main code block, which constructs the XCM message. With the help of the following code, we can easily dispatch the message repeatedly and test out different input values.

```bash
const transactExtrinsic = parachainHelper.api.tx.xcmTransactor.transactThroughSigned(
        {
            V3: {
                parents: 1,
                interior: {
                    X1: { Parachain: 2114 },
                },
            },
        },
        {
            currency: {
                AsCurrencyId: 'SelfReserve',
            },
            feeAmount: fungible,
        },
        encodedTaskViaProxy,
        {
            transactRequiredWeightAtMost: {
                refTime: transactRequiredWeightAtMost,
                proofSize: 0,
            },
            overallWeight: {
                refTime: overallWeight,
                proofSize: 0,
            },
        },
    );
```

## Debugging Live

There are two potential areas where issues may arise when using XCM messages. The first is in the construction of the message itself, and the second is within the execution of the transaction on the target chain.

1. Message formatting issues: The first possible issue is that the XCM message is malformed and is not correctly received by the recipient chain. We can use the xcm-tool covered in chapter 5 to interpret XCM messages that occurred on-chain with its block number and channel. For example, `npm run xcm-decode-para -- --wr wss://wss.api.moonriver.moonbeam.network --b 2391172 --channel hrmp --p 2000`. Below are two common problems we could encounter:
    1. Incorrect fee and weight inputs: As mentioned earlier, specifying the maximum weight allowed in the XCM call is crucial. If the actual weight is slightly above the limit, the recipient chain may deny the call because it exceeds the user's limit. In this case, the error will be seen from the `xcmQueue.fail` event recipient chain. We can increase the maximum weight parameter value and retry.
    2. Version mismatch: Another error that can occur is a VersionMismatch, which means that the recipient chain does not accept the Multi-location version specified in Destination or FeeAsset. In this case, we need to examine the version of the recipient XCM and change that of multi-location to V2 or V3.
2. Transaction formatting issues: In order to examine the execution of the transaction, we need to find the specific transaction, which will be an extrinsic record that occurred after `XcmMessageQueue.success`. Unfortunately, as far as I know, there is no great data tool to automatically correlate `XcmMessageQueue.success` with the transaction. However, we can manually examine it by correlating the message hash with the source chain.

## Summary

In this section, we discuss a real-life use case of the XCM between Moonriver and Turing Network.

To create an XCM message between chains, you need to prepare the following elements:

- Type: VRP or HMRP, which represent the two parties involved in the communication.
- Goal: What extrinsic to call, or what will be in the transaction.
- Details: Chain configurations are different, so you need to decide on the DescendOrigin, which is whether to descendOrigin to the user's account or use a global account, and the Sequence, which specifies what instructions will be in the message.

Finally, put all the elements together and troubleshoot the message. Once you have established a template, you can automate the construction process using the polkadot.js JavaScript library or even write a wrapper in the parachain's Rust code, such as `xtoken.reserveTransfer`.
