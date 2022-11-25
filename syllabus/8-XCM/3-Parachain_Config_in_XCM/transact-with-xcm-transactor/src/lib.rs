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
	use frame_support::{
		assert_ok, dispatch::Encode, sp_runtime::traits::Hash, traits::PalletInfo,
	};
	use xcm::latest::prelude::*;
	use xcm_executor::traits::Convert;
	use xcm_simulator_for_exercises::{
		para_account_id, parachain, relay_chain, MockNet, ParaA, Relay, RelayBalances, TestExt,
		ALICE, INITIAL_BALANCE,
	};
	use xcm_transactor_pallet::{Currency, CurrencyPayment, TransactWeights};

	#[test]
	fn test_xcm_transactor_pallet_from_sovereign_native_kind() {
		MockNet::reset();

		let remark = b"Testing Remark";
		// Get remark encoded
		let remark_encoded = create_remark_relay_call(remark);

		ParaA::execute_with(|| {
			// Transact through sovereig using Native as originKind
			// This wont work because remark accepts only signed origins
			// and Native does not convert to signed
			assert_ok!(parachain::XcmTransactor::transact_through_sovereign(
				parachain::RuntimeOrigin::root(),
				Box::new(MultiLocation::parent().into()),
				CurrencyPayment {
					currency: Currency::AsMultiLocation(Box::new(MultiLocation::parent().into())),
					fee_amount: INITIAL_BALANCE
				},
				remark_encoded,
				OriginKind::Native,
				TransactWeights {
					transact_required_weight_at_most: 1000000000,
					overall_weight: 4000000000
				}
			));
		});

		let hash = <relay_chain::Runtime as frame_system::Config>::Hashing::hash(remark);

		// We expect the tx to be executed from the sov account
		let expected_remark_event: relay_chain::RuntimeEvent =
			frame_system::Event::Remarked { sender: para_account_id(1), hash }.into();
		// OriginKind::Native did not work. Because it does not convert to a signed origin
		Relay::execute_with(|| {
			assert!(!relay_chain::relay_events().contains(&expected_remark_event));
		});
	}

	#[test]
	fn test_xcm_transactor_pallet_from_sovereign_sovereign_kind() {
		MockNet::reset();

		let remark = b"Testing Remark";
		// Get remark encoded
		let remark_encoded = create_remark_relay_call(remark);

		ParaA::execute_with(|| {
			// Transact through sovereig using SOvereign as originKind
			// This will be translated to a signed origin
			// with the sovereign account as caller
			assert_ok!(parachain::XcmTransactor::transact_through_sovereign(
				parachain::RuntimeOrigin::root(),
				Box::new(MultiLocation::parent().into()),
				CurrencyPayment {
					currency: Currency::AsMultiLocation(Box::new(MultiLocation::parent().into())),
					fee_amount: INITIAL_BALANCE
				},
				remark_encoded,
				OriginKind::SovereignAccount,
				TransactWeights {
					transact_required_weight_at_most: 1000000000,
					overall_weight: 4000000000
				}
			));
		});

		// lets get the hash of the remark
		let hash = <relay_chain::Runtime as frame_system::Config>::Hashing::hash(remark);

		// We expect the tx to be executed from the sov account
		let expected_remark_event: relay_chain::RuntimeEvent =
			frame_system::Event::Remarked { sender: para_account_id(1), hash }.into();

		// We can make sure the remark happened by checking the event
		Relay::execute_with(|| {
			assert!(relay_chain::relay_events().contains(&expected_remark_event));
		});
	}

	#[test]
	fn test_xcm_transactor_pallet_from_descended_origin_sovereign_kind() {
		MockNet::reset();

		let remark = b"Testing Remark";
		// Get remark encoded
		let remark_encoded = create_remark_relay_call(remark);

		// We need to fund the descended sovereign to be able to do the
		// remark

		// Let's construct the Junction that we will append with DescendOrigin
		// Its simply the junction containing ALICE
		let signed_origin: Junctions =
			X1(AccountId32 { network: parachain::RelayNetwork::get(), id: ALICE.into() });

		// We start with the parachain multilocation and append the signed origin
		let mut descend_origin_multilocation = MultiLocation::new(1, X1(Parachain(1)));
		descend_origin_multilocation.append_with(signed_origin).unwrap();

		// We need to reanchor because the relay will have its relative view of the
		// multilocation
		descend_origin_multilocation
			.reanchor(&MultiLocation::parent(), &MultiLocation::parent())
			.unwrap();

		// Now lets calculate the derived account. This will be just the hash of
		// the multilcation
		let derived = xcm_builder::Account32Hash::<
			relay_chain::KusamaNetwork,
			relay_chain::AccountId,
		>::convert_ref(descend_origin_multilocation)
		.unwrap();

		// Fund descended sovereign first
		Relay::execute_with(|| {
			assert_ok!(RelayBalances::transfer(
				relay_chain::RuntimeOrigin::signed(ALICE),
				derived.clone(),
				INITIAL_BALANCE
			));
		});

		ParaA::execute_with(|| {
			// Transact through descended sovereign using SOvereign as originKind
			// This will be translated to a signed origin
			// with the descended sovereign account as caller
			assert_ok!(parachain::XcmTransactor::transact_through_descended_sovereign(
				parachain::RuntimeOrigin::signed(ALICE),
				Box::new(MultiLocation::parent().into()),
				CurrencyPayment {
					currency: Currency::AsMultiLocation(Box::new(MultiLocation::parent().into())),
					fee_amount: INITIAL_BALANCE
				},
				remark_encoded,
				TransactWeights {
					transact_required_weight_at_most: 1000000000,
					overall_weight: 4000000000
				}
			));
		});

		let hash = <relay_chain::Runtime as frame_system::Config>::Hashing::hash(remark);

		// We expect the tx to be executed from the derived account
		let expected_remark_event: relay_chain::RuntimeEvent =
			frame_system::Event::Remarked { sender: derived, hash }.into();

		// We make sure the event has been thrown
		Relay::execute_with(|| {
			assert!(relay_chain::relay_events().contains(&expected_remark_event));
		});
	}

	fn create_remark_relay_call(remark: &[u8]) -> Vec<u8> {
		// Encode the call. Balances transact to para_a_account
		// First index
		let mut encoded: Vec<u8> = Vec::new();
		let index = <relay_chain::Runtime as frame_system::Config>::PalletInfo::index::<
			relay_chain::System,
		>()
		.unwrap() as u8;

		encoded.push(index);

		// Then call bytes
		let mut call_bytes = frame_system::Call::<relay_chain::Runtime>::remark_with_event {
			remark: remark.to_vec(),
		}
		.encode();
		encoded.append(&mut call_bytes);
		encoded.to_vec()
	}
}
