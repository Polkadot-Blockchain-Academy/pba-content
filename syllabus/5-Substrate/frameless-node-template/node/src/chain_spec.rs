use runtime::RuntimeGenesisConfig;
use sc_service::{ChainType, Properties};

pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

fn props() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".to_string(), 0.into());
	properties.insert("tokenSymbol".to_string(), "TEST".into());
	properties
}

pub fn development_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || Default::default(),
		vec![],
		None,
		None,
		None,
		Some(props()),
		None,
	))
}
