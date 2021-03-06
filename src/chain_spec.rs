use sp_core::{Pair, Public, sr25519};
use paracon_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature,
	ContractsConfig, MILLICENTS,
};
use sp_consensus_aura::sr25519::{AuthorityId as AuraId};
use grandpa_primitives::{AuthorityId as GrandpaId};
use sc_service;
use sp_runtime::traits::{Verify, IdentifyAccount};
use hex_literal::hex;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Paracon Test net 1
	TestNet1,
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn get_authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn testnet_authorities() -> Vec<(AuraId, GrandpaId)> {
	use sp_core::crypto::UncheckedInto;
	vec![
		(
			hex!("74608217b1709e1d3a4fe65b132db5c3f321e625026080833189661aa5e20712").unchecked_into(),
			hex!("a5abc21ac95ae63dd6e61e5bec263ab46d1efe16d3dcc085d0de297318cb662d").unchecked_into(),
		),
		(
			hex!("44f3876fe4f653533c65e79461a476b8d6a107fb71b6ec0f3485bb53b4e7b842").unchecked_into(),
			hex!("281be34a71b661b257153e1145522fd0820cfff6a3601b40e7f85d3bc155240d").unchecked_into(),
		),
	]
}

pub fn testnet_root() -> AccountId {
	hex!("baa78c7154c7f82d6d377177e20bcab65d327eca0086513f9964f5a0f6bdad56").into()
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"dev",
				|| testnet_genesis(vec![
					get_authority_keys_from_seed("Alice"),
				],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::TestNet1 => ChainSpec::from_genesis(
				"Paracon Testnet 1",
				"paracon_testnet1",
				|| testnet_genesis(
					testnet_authorities(),
					testnet_root(),
					vec![testnet_root()],
					true,
				),
				vec![
					"/ip4/35.233.19.96/tcp/30333/p2p/QmNvYhAZSBtahCqCXznYiq8e24Yes1GraPFYCc3DyA5f3z".to_string(),
					"/ip4/35.205.110.21/tcp/30333/p2p/QmPKFc9B2oeQFc5oxbNsRENwSYibzzafKmcHs9wBZCJH4U".to_string(),
				],
				None,
				Some("prc"),
				None,
				None
			),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"" | "local" => Some(Alternative::TestNet1),
			_ => None,
		}
	}
}

fn testnet_genesis(
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	enable_println: bool
) -> GenesisConfig {
    let mut contracts_config = ContractsConfig {
        current_schedule: Default::default(),
        gas_price: 1 * MILLICENTS,
    };
    contracts_config.current_schedule.enable_println = enable_println;

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		contracts: Some(contracts_config),
	}
}

pub fn load_spec(id: &str) -> Result<Option<ChainSpec>, String> {
	Ok(match Alternative::from(id) {
		Some(spec) => Some(spec.load()?),
		None => None,
	})
}
