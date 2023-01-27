// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

#[cfg(test)]
mod tests {
	use xcm_simulator_for_exercises::{
		relay_chain,
		MockNet,
		Relay,
		ParaA,
        parachain,
        ParachainPalletXcm,
        RelayChainPalletXcm,
        
		// ParaB, // Might be useful for one of the exercises :)
		TestExt,
        ALICE, INITIAL_BALANCE, para_account_id,
	};
    use frame_support::assert_ok;
    use xcm::latest::prelude::*;
    use xcm::{VersionedXcm, VersionedMultiLocation};

    #[test]
    fn execute_initiate_teleport_to_para_a() {
      /* ------------------------------------------------------------------------- */
        // In this section we do some initialization for our XCM exercise
        // 1.) Create a new Test net scenario
        MockNet::reset();

        // 2.) Declare an amount to withdraw and teleport
        let withdraw_amount = 100;

        let asset_location = MultiLocation {
            parents: 0,
            interior: Here
        };

        // 3.) Create 2-tuple with item 1 being the junction/s to send from and item 2 the withdraw_amount
        // let withdraw_assets_from = $CREATE_TUPLE_HERE;

        /*  ------------------------------------------------------------------------ */
        // In this next section we focus on constructing the instructions necessary create our XCM message
        // 1.) Takes some assets and place in the holding register.
        // First we need to use an Instruction which can withdraw assets and place them in the holding register
        // let instruction_1: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

        // 2.) Inform a destination chain that we are placing the previously withdrawn reserve assets
        // Second we need to use an Instruction which informs the other chain about the assets being teleported
        // let instruction_2: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

        // 4.) XCM Message for parachain
        // Create the XCM message for the given instructions above
        // let message = $INSERT_CONSTRUCTED_XCM_MESSAGE_HERE

        let message: Xcm<parachain::RuntimeCall> = Xcm(vec![
            WithdrawAsset((asset_location.clone(), withdraw_amount).into()),
            InitiateTeleport {
                assets: Wild(All),
                dest: MultiLocation::new(0, X1(Parachain(1))),
                xcm: Xcm(vec![])
            }
        ]);

        Relay::execute_with(|| {
              assert_ok!(
                RelayChainPalletXcm::execute(
                    relay_chain::RuntimeOrigin::signed(ALICE),
                    Box::new(VersionedXcm::V2(message.into())),
                    100_000_000_000
                )
              );
        });

        ParaA::execute_with(|| {
            let expected_message_received: Xcm<parachain::RuntimeCall> = Xcm(vec![
                ReceiveTeleportedAsset(vec![MultiAsset { id: Concrete(MultiLocation { parents: 1, interior: Here }), fun: Fungible(100) }].into()),
                ClearOrigin
            ]);
    
            assert_eq!(parachain::MsgQueue::received_dmp(), vec![expected_message_received]);
        })
    }

    #[test]
    fn execute_initiate_reserve_withdraw_to_para_a() {
            /* ------------------------------------------------------------------------- */
        // In this section we do some initialization for our XCM exercise
        // 1.) Create a new Test net scenario
        MockNet::reset();

        // 2.) Declare an amount to send to ALICE's account on the relaychain
        let withdraw_amount = 100;

        let asset_location = MultiLocation {
            parents: 0,
            interior: Here
        };

        // 3.) Create 2-tuple with item 1 being the junction/s to send from and item 2 the withdraw_amount
        // let withdraw_assets_from = $CREATE_TUPLE_HERE;

        /*  ------------------------------------------------------------------------ */
        // In this next section we focus on constructing the instructions necessary create our XCM message
        // 1.) Takes some assets and place in the holding register.
        // First we need to use an Instruction which can withdraw assets and place them in the holding register
        // let instruction_1: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE
        // HINT: when a message is sent from Para A to the relay, the para A multilocation origin is converted
        // to:
        // let para_a_account = xcm_simulator_for_exercises::para_account_id(Para_A_para_id)

        // 2.) Purchase execution for this message to be executed appropriately on the destination chain
        // Second we need to use an Instruction which can obtain execution some execution
        // let instruction_2: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

        // 3.) Drain assets from the holding register and specify to whome receives this asset
        // Lastly we need an instruction which can take assets from the holding register and give them to ALICE
        // let instruction_3: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

        // 4.) XCM Message for parachain
        // Create the XCM message for the given instructions above
        // let message = $INSERT_CONSTRUCTED_XCM_MESSAGE_HERE

        let message: Xcm<parachain::RuntimeCall> = Xcm(vec![
            WithdrawAsset((asset_location.clone(), withdraw_amount).into()),
            DepositReserveAsset {
                assets: Wild(All),
                max_assets: 1,
                dest: MultiLocation::new(0, X1(Parachain(1))),
                xcm: Xcm(vec![])
            }
        ]);

        Relay::execute_with(|| {
            assert_ok!(RelayChainPalletXcm::execute(
                relay_chain::RuntimeOrigin::signed(ALICE),
                Box::new(VersionedXcm::V2(message.into())),
                100_000_000_000
            ));
        });

        ParaA::execute_with(|| {
            let expected_message_received: Xcm<parachain::RuntimeCall> = Xcm(vec![
                ReserveAssetDeposited(vec![MultiAsset { id: Concrete(MultiLocation { parents: 1, interior: Here }), fun: Fungible(100) }].into()),
                ClearOrigin
            ]);
    
            assert_eq!(parachain::MsgQueue::received_dmp(), vec![expected_message_received]);
        });

        Relay::execute_with(|| {
            assert_eq!(relay_chain::Balances::free_balance(ALICE), INITIAL_BALANCE - withdraw_amount);
            assert_eq!(relay_chain::Balances::free_balance(para_account_id(1)), INITIAL_BALANCE + withdraw_amount);
        })
    }     

    #[test]
    fn send_ump_withdraw_deposit_alice() {
        /* ------------------------------------------------------------------------- */
        // In this section we do some initialization for our XCM exercise
        // 1.) Create a new Test net scenario
        MockNet::reset();

        // 2.) Declare an amount to send to ALICE's account on the relaychain
        let withdraw_amount = 100;

        let asset_location = MultiLocation {
            parents: 0,
            interior: Here
        };

        // 3.) Create 2-tuple with item 1 being the junction/s to send from and item 2 the withdraw_amount
        // let withdraw_assets_from = $CREATE_TUPLE_HERE;

        /*  ------------------------------------------------------------------------ */
        // In this next section we focus on constructing the instructions necessary create our XCM message
        // 1.) Takes some assets and place in the holding register.
        // First we need to use an Instruction which can withdraw assets and place them in the holding register
        // let instruction_1: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE
        // HINT: when a message is sent from Para A to the relay, the para A multilocation origin is converted
        // to:
        // let para_a_account = xcm_simulator_for_exercises::para_account_id(Para_A_para_id)

        // 2.) Purchase execution for this message to be executed appropriately on the destination chain
        // Second we need to use an Instruction which can obtain execution some execution
        // let instruction_2: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

        // 3.) Drain assets from the holding register and specify to whome receives this asset
        // Lastly we need an instruction which can take assets from the holding register and give them to ALICE
        // let instruction_3: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

        // 4.) XCM Message for parachain
        // Create the XCM message for the given instructions above
        // let message = $INSERT_CONSTRUCTED_XCM_MESSAGE_HERE

        let message: Xcm<parachain::RuntimeCall> = Xcm(vec![
            WithdrawAsset((asset_location.clone(), withdraw_amount).into()),
            DepositAsset {
                assets: Wild(All),
                max_assets: 1,
                beneficiary: MultiLocation::new(0, X1(AccountId32{ network: NetworkId::Any, id: ALICE.into()})),
            }
        ]);

        ParaA::execute_with(|| {
            assert_ok!(ParachainPalletXcm::send(
                parachain::RuntimeOrigin::root(),
                Box::new(VersionedMultiLocation::V1(MultiLocation::parent())),
                Box::new(VersionedXcm::V2(message.into())),
            ));
        });

        Relay::execute_with(|| {
            assert_eq!(relay_chain::Balances::free_balance(para_account_id(1)), INITIAL_BALANCE - withdraw_amount);
            assert_eq!(relay_chain::Balances::free_balance(ALICE), INITIAL_BALANCE + withdraw_amount);
        })
    }

    #[test]
    fn withdraw_and_query_holding_xcmp() {
        /* ----------------------------------------------------- */
        // Now do it on your own!!!
        // 1.) Construct XCM Instructions
        // 2.) Send correct XCM Message from Para_A to Relay
        // 3.) Verify correct message was received and state was updated correctly on destination
        // 4.) Verify a response was received by the request originating chain

        MockNet::reset();

        let withdraw_amount = 100;

        let asset_location = MultiLocation {
            parents: 0,
            interior: Here
        };

        let message: Xcm<parachain::RuntimeCall> = Xcm(vec![
            WithdrawAsset((asset_location.clone(), withdraw_amount).into()),
            QueryHolding {
                query_id: 1,
                dest: MultiLocation::new(0, X1(Parachain(1))),
                assets: Wild(All),
                max_response_weight: 1000000000
            }
        ]);

        ParaA::execute_with(|| {
            assert_ok!(ParachainPalletXcm::send(
                parachain::RuntimeOrigin::root(),
                Box::new(VersionedMultiLocation::V1(MultiLocation::parent())),
                Box::new(VersionedXcm::V2(message.into())),
            ));
        });

        ParaA::execute_with(|| {
          let expected_received_message = Xcm(vec![
            QueryResponse {
                query_id: 1,
                response: Response::Assets(
                    vec![MultiAsset { id: Concrete(MultiLocation { parents: 1, interior: Here }), fun: Fungible(100) }].into()),
                max_weight: 1000000000
            }]);
            assert_eq!(parachain::MsgQueue::received_dmp(), vec![expected_received_message]);

        });  
        /* Fill in here */
    }

    #[test]
	fn test_automatic_versioning_on_runtime_upgrade_with_relay() {
		MockNet::reset();

		ParaA::execute_with(|| {
			// Set version 1 to parachain
            // HINT: you can use parachain::XcmVersioner
		});

		Relay::execute_with(|| {
			// This sets the default version in the relay to 2, for not known destinations
			assert_ok!(RelayChainPalletXcm::force_default_xcm_version(
				relay_chain::RuntimeOrigin::root(),
				Some(2)
			));

			// Wrap version, which sets VersionedStorage
			// This is necessary because the mock router does not use wrap_version, but
			// this is not necessary in prod
			// This triggers note_unknown_version in pallet-xcm
			// And also version negotiation
			assert_ok!(<RelayChainPalletXcm as xcm::WrapVersion>::wrap_version(
				&Parachain(1).into(),
				Xcm::<()>(vec![])
			));

			// Let's advance the relay. This should trigger the subscription message
			relay_chain::relay_roll_to(2);

			// queries should have been updated
			// we received a query back from the para indicating of the version
			assert!(RelayChainPalletXcm::query(0).is_some());
		});

		/*  FILL in class
		let expected_supported_version: relay_chain::RuntimeEvent =
			pallet_xcm::Event::SupportedVersionChanged(
				// parachain multilocation as seen by the relay,
				// version that the relay received?
			)
			.into();

		Relay::execute_with(|| {
			// Assert that the events vector contains the version change
			assert!(relay_chain::relay_events().contains(&expected_supported_version));
		});*/

		/*  FILL in class
		let expected_version_notified: parachain::RuntimeEvent =
			pallet_xcm::Event::VersionChangeNotified(
				// Relay as seen by the para,
				// version that the para received?
			)
			.into();*/

		// ParaA changes version to 2, and calls on_runtime_upgrade. This should notify the targets
		// of the new version change
		ParaA::execute_with(|| {
			// Set version 2
			// Do runtime upgrade
			parachain::on_runtime_upgrade();
			// Initialize block, to call on_initialize and notify targets
			parachain::para_roll_to(2);
			// Expect the event in the parachain
			// assert!(parachain::para_events().contains(&expected_version_notified));
		});

		/*  FILL in class
		// This event should have been seen in the relay
		let expected_supported_version_after: relay_chain::RuntimeEvent =
			pallet_xcm::Event::SupportedVersionChanged(
				// Para multilocation as seen by the relay,
				// version that the relay received?
			)
			.into();

		Relay::execute_with(|| {
			// Assert that the events vector contains the new version change
			assert!(relay_chain::relay_events().contains(&expected_supported_version_after));
		});*/
	}
}
