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
        ALICE, INITIAL_BALANCE, parachain,
	};
    use frame_support::assert_ok;

    #[test]
    fn execute_withdraw_asset() {
        MockNet::reset();

        let withdraw_amount = 100;

        ParaA::execute_with(|| {
            let message: Xcm<parachain::RuntimeCall> = Xcm(vec![
                WithdrawAsset((Here, withdraw_amount).into()),
            ]);
            assert_ok!(
                ParachainPalletXcm::execute(
                    parachain::RuntimeOrigin::signed(ALICE),
                    Box::new(VersionedXcm::V2(message.into())),
                    100_000_000_000
                )
            );

            // assert_eq!(
            //     ParachainPalletBalances::free_balance(ALICE),
            //     INITIAL_BALANCE - withdraw_amount
            // );
        });
    }
}