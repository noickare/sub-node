use clueconn_runtime::{
    constants::currency::DOLLARS, AccountId, AuraConfig, BalancesConfig, GenesisConfig,
    GrandpaConfig, Signature, SpacesConfig, SudoConfig, SystemConfig, UtilsConfig, WASM_BINARY,
};
use hex_literal::hex;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use subsocial_primitives::Block;

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "clue";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary =
        WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        move || {
            let endowed_accounts = vec![
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                get_account_id_from_seed::<sr25519::Public>("Dave"),
                get_account_id_from_seed::<sr25519::Public>("Eve"),
            ];

            testnet_genesis(
                wasm_binary,
                vec![authority_keys_from_seed("Alice")],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                endowed_accounts
                    .iter()
                    .cloned()
                    .map(|k| (k, 100_000))
                    .collect(),
                get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                true,
            )
        },
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        Some(clueconn_properties()),
        Default::default(),
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary =
        WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        move || {
            let endowed_accounts = vec![
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                get_account_id_from_seed::<sr25519::Public>("Dave"),
                get_account_id_from_seed::<sr25519::Public>("Eve"),
            ];

            testnet_genesis(
                wasm_binary,
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                endowed_accounts
                    .iter()
                    .cloned()
                    .map(|k| (k, 100_000))
                    .collect(),
                get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                true,
            )
        },
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        Some(clueconn_properties()),
        Default::default(),
    ))
}

pub fn clueconn_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/clueconn.json")[..])
}

pub fn clueconn_staging_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Staging wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "Clueconn Staging",
        "clueconn",
        ChainType::Live,
        move || {
            testnet_genesis(
                wasm_binary,
                vec![(
                    /* AuraId SR25519 */
                    hex!["3ec03f51f26595e34a4139e462c8a9f8e8cc0554fafcb511a638e8b944f14b77"]
                        .unchecked_into(),
                    /* GrandpaId ED25519 */
                    hex!["d1458af9072f340f0699dd34d588d939c6988dd4d4c3fa5cf3009ae1090be5e6"]
                        .unchecked_into(),
                )],
                /* Sudo Account */
                hex!["ca22e5d8cd577003036ef61e58a6f4d37932d3ecb4be0a19cc1b98a0cff66f67"].into(),
                vec![
                    (
                        /* Sudo Account */
                        hex!["ca22e5d8cd577003036ef61e58a6f4d37932d3ecb4be0a19cc1b98a0cff66f67"]
                            .into(),
                        /* Balance */
                        1_000,
                    ),
                    (
                        /* Account X1 */
                        hex!["3e0ecea13eddef0e9b21dce9292f8cdd0c1d837a9060d22c4f4f84e80b215371"]
                            .into(),
                        /* Balance */
                        2_499_000,
                    ),
                    (
                        /* Account X2 */
                        hex!["ee9ba776c9ce8bfa501bce9aa7c57fd0d2959a4baa665bfb96ab865a8281825d"]
                            .into(),
                        /* Balance */
                        2_500_000,
                    ),
                    (
                        /* Account X3 */
                        hex!["98db5cb04f40a5f4fd1db7d4046cacb1610747734680d99f8186a38c2b6dae4d"]
                            .into(),
                        /* Balance */
                        2_500_000,
                    ),
                    (
                        /* Account X4 */
                        hex!["aab402f03007e08560e8e16b563639f4276a069026d17c403a5ec8af15230b29"]
                            .into(),
                        /* Balance */
                        2_500_000,
                    ),
                ],
                // Treasury
                hex!["569531aaa8b014bb10e99af2c6f8b9ced581eaac1714dbc6b86f9c7ac8a59e58"].into(),
                true,
            )
        },
        vec![],
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid; qed"),
        ),
        Some(DEFAULT_PROTOCOL_ID),
        Some(clueconn_properties()),
        Default::default(),
    ))
}

fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<(AccountId, u128)>,
    treasury_account_id: AccountId,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|(k, b)| (k, b * DOLLARS))
                .collect(),
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        sudo: SudoConfig {
            key: root_key.clone(),
        },
        utils: UtilsConfig {
            treasury_account: treasury_account_id,
        },
        spaces: SpacesConfig {
            endowed_account: root_key,
        },
    }
}

pub fn clueconn_properties() -> Properties {
    let mut properties = Properties::new();

    properties.insert("ss58Format".into(), 28.into());
    properties.insert("tokenDecimals".into(), 11.into());
    properties.insert("tokenSymbol".into(), "CLUE".into());

    properties
}
