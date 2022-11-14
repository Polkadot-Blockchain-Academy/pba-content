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
		parachain,
		MockNet,
		Relay, RelayChainPalletXcm,
		ParaA, ParachainPalletXcm,
		ParaB,
		TestExt,
	};

	use frame_support::assert_ok;
	use xcm::latest::prelude::*;

	// Test Questions:
    #[test]
    fn send_dmp_withdraw_deposit_alice() {
        // 1.) Create a new Test net scenario
        // Answer:
        // MockNet::reset();

        // 2.) Declare an amount to send to Alices account on the relaychain
        // Answer:
        // let withdraw_amount = 100;

        // 3.) Create UMP XCM Message for parachain
        // Answer:
        // let relay_message = Xcm(vec![
                // TODO: Fill in
        // ]);

        // 4.) Create UMP message for the RelayChain
        // Answer:
        // Relay::execute_with(|| {
            // assert_ok!(
                // Call send_xcm() with the constructed xcm relay message
            // );
        // });

        // 5.) Verify on parachain
        // Answer:
        // ParaA::execute_with(|| {
            // 6.) Check Balances Pallet for relay chain
            // Answer:
            // assert_ok!(
                    // TODO: Fill in
            // );
            // 7.) Check Balances Pallet for parachain
            // Answer:
            // assert_ok!(
                    // TODO: Fill in
            // );
        // });
    }

    #[test]
    fn withdraw_and_deposit_with_query_xcmp() {
        // TODO: Implement me
    }

}
