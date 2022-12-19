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
    use xcm::latest::prelude::*;
    use xcm::VersionedXcm;
	use xcm_simulator_for_exercises::{
		MockNet,
        ParaA,
		TestExt,
        ParachainPalletXcm,
        ParachainPalletBalances,
        ALICE, BOB, INITIAL_BALANCE, parachain,
	};
    use codec::Encode;
    use frame_support::assert_ok;

    #[test]
    fn execute_withdraw_asset() {
        MockNet::reset();

        let withdraw_amount = 100;
        let location = MultiLocation {
            parents: 1,
            interior: Here
        };

        ParaA::execute_with(|| {
            let message: Xcm<parachain::RuntimeCall> = Xcm(vec![
                WithdrawAsset((location, withdraw_amount).into()),
            ]);
            assert_ok!(
                ParachainPalletXcm::execute(
                    parachain::RuntimeOrigin::signed(ALICE),
                    Box::new(VersionedXcm::V2(message.into())),
                    100_000_000_000
                )
            );

            assert_eq!(
                ParachainPalletBalances::free_balance(ALICE),
                INITIAL_BALANCE - withdraw_amount
            );
        });
    }

    #[test]
    fn execute_buy_execution() {
        MockNet::reset();

        // Task
        // Create a BuyExecution Instruction
        // 1.) Who is buying the execution?
        // 2.) How to verify we executed this instruction correctly?

        ParaA::execute_with(|| {
           // Insert here the appropriate code to execute the XCM message asked for.
        });
    }

    #[test]
    fn execute_send_funds_to_bob() {
        MockNet::reset();

        // Task
        // Send 100 from Alice to Bob from a parachain locally and verify it.
        // 1.) Where to send it from?
        // 2.) how to verify Bob received the funds?

        ParaA::execute_with(|| {
            // Insert here the appropriate code to execute the XCM message asked for.
         });
    }

    #[test]
    fn execute_transact_message() {
        MockNet::reset();

        // Task
        // Create a call which we can dispatch locally
        // 1.) What kinds of calls do we have to choose from?
        //      (Hint system pallet might be useful here but any call can do(Which we can verify))
        // 2.) How can we verify the call was dispatched correctly?(Hint events could be a way :)

        ParaA::execute_with(|| {
            // Insert Here the appropriate code to execute the XCM message asked for.
        });
    }

    #[test]
    fn execute_transact_on_own_dispatchable() {
        MockNet::reset();

        // Task
        // 1.) Add your own dispatchable in the `parachain` module via your own custom pallet or in an
        //      existing pallet
        // 2.) Construct the call for your new dispatchable
        // 3.) Send a Transact instruction for your new dispatchable
        // 4.) Verify the call was dispatched

        ParaA::execute_with(|| {
            // Insert Here the appropriate code to execute the XCM message asked for.
        });
    }
}
