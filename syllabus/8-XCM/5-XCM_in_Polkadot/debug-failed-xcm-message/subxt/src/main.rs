// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use subxt::{OnlineClient, PolkadotConfig};

// Generate the API from a static metadata path.
#[subxt::subxt(
	runtime_metadata_path = "./artifacts/polkadot_metadata.scale",
	derive_for_all_types = "Eq, PartialEq"
)]
pub mod polkadot {}

use clap::Parser;
#[derive(Parser, Default, Debug)]
struct Arguments {
	#[clap(short)]
	para_id: u32,
	#[clap(short)]
	block_number: u32,
}

type ParasInherentCall =
	polkadot::runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Call;
type RuntimeCall = polkadot::runtime_types::polkadot_runtime::RuntimeCall;
type VersionedXcm = polkadot::runtime_types::xcm::VersionedXcm;

use crate::polkadot::runtime_types::xcm::{
	v2::{Instruction as InstructionV2, Xcm as XcmV2},
	VersionedXcm::V2,
};

use polkadot::runtime_types::polkadot_parachain::primitives::Id as ParaId;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Arguments::parse();
	tracing_subscriber::fmt::init();

	// Create a client to use:
	let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

	let block_number = args.block_number;

	let block_hash = api
		.rpc()
		.block_hash(Some(block_number.into()))
		.await?
		.expect("block number not found");

	let block = api.clone().rpc().block(Some(block_hash)).await?.expect("block not found");

	for extrinsic in block.block.extrinsics {
		let mut sliced = &extrinsic.0[1..];

		let decoded: Result<RuntimeCall, codec::Error> = codec::Decode::decode(&mut sliced);
		match decoded {
			Ok(RuntimeCall::ParaInherent(ParasInherentCall::enter { data })) => {
				for candidate in data.backed_candidates {
					if candidate.candidate.descriptor.para_id == ParaId(args.para_id) {
						for (index, upward_message) in
							candidate.candidate.commitments.upward_messages.iter().enumerate()
						{
							let xcm: VersionedXcm =
								codec::Decode::decode(&mut upward_message.as_slice())
									.expect("unable to decode");
							let instructions = match xcm {
								V2(XcmV2(instructions)) => instructions,
								_ => panic!("Only v2 supported"),
							};
							pretty_print_xcm(index as u32, instructions);
						}
					}
				}
			},
			_ => continue,
		}
	}
	Ok(())
}

fn pretty_print_xcm(index: u32, instructions: Vec<InstructionV2>) {
	println!("Xcm message number {:?}", index + 1);

	for instruction in instructions {
		match instruction {
			InstructionV2::Transact { origin_type, require_weight_at_most, call } => {
				println!(
					"{{ origin_type {:?}, require_weight_at_most: {:?}, call: {:?} }}",
					origin_type,
					require_weight_at_most,
					hex::encode(call.encoded)
				)
			},
			_ => println!("{:?}", instruction),
		}
	}
}
