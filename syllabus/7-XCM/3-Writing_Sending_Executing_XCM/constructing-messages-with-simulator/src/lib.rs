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
		// ParaB, // Might be useful for one of the exercises :)
		TestExt,
        ALICE, INITIAL_BALANCE, para_account_id,
	};

    #[test]
    fn send_ump_withdraw_deposit_alice() {
        /* ------------------------------------------------------------------------- */
        // In this section we do some initialization for our XCM exercise
        // 1.) Create a new Test net scenario
        MockNet::reset();

        // 2.) Declare an amount to send to ALICE's account on the relaychain
        let withdraw_amount = 100;

        // 3.) Create 2-tuple with item 1 being the junction/s to send from and item 2 the withdraw_amount
        // let withdraw_assets_from = $CREATE_TUPLE_HERE;

        /*  ------------------------------------------------------------------------ */
        // In this next section we focus on constructing the instructions necessary create our XCM message
        // 1.) Takes some assets and place in the holding register.
        // First we need to use an Instruction which can withdraw assets and place them in the holding register
        // let instruction_1: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

        // 2.) Purchase execution for this message to be executed appropriately on the destination chain
        // Second we need to use an Instruction which can obtain execution some execution
        // let instruction_2: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

        // 3.) Drain assets from the holding register and specify to whome receives this asset
        // Lastly we need an instruction which can take assets from the holding register and give them to ALICE
        // let instruction_3: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

        // 4.) XCM Message for parachain
        // Create the XCM message for the given instructions above
        // let message = $INSERT_CONSTRUCTED_XCM_MESSAGE_HERE

        ParaA::execute_with(|| {
        // 5.) Execute send_xcm from the XCM pallet
        //     assert_ok!(
        //         // ParachainPalletXcm::send_xcm(
                        /* $INSERT_CORRECT_PARAMS_TO_SEND_XCM */
        //         // )
        //     );
        });

        Relay::execute_with(|| {
            assert_eq!(relay_chain::Balances::free_balance(ALICE), INITIAL_BALANCE + withdraw_amount);
            assert_eq!(relay_chain::Balances::free_balance(para_account_id(1)), INITIAL_BALANCE - withdraw_amount);
        })
    }

    #[test]
    fn withdraw_and_deposit_and_query_holding_xcmp() {
        /* ----------------------------------------------------- */
        // Now do it on your own!!!
        // 1.) Construct XCM Instructions
        // 2.) Send correct XCM Message
        // 3.) Verify correct message was received and state was updated correctly on destination

        MockNet::reset();

        let withdraw_amount = 100;

        /* Fill in here */
    }
}
