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
                vec![
                (
                    /* AuraId SR25519 */
                    hex!["8eb5da0628336e262bf6b479b67e69abba4f0f34574362a0811a0540dae1dc76"].unchecked_into(),
                    /* GrandpaId ED25519 */
                    hex!["f879f1280531c9fbbf748cb51f6fd8db1dbfc9c6403c71629ccfe67f13edfd07"].unchecked_into()
                ),
                (
                    /* AuraId SR25519 */
                    hex!["4078333251614a09d3e450c8651609fdb8e7886b134e1c892f19cbabf7275c0e"].unchecked_into(),
                    /* GrandpaId ED25519 */
                    hex!["58058e7d56ad185dca7a0d8295066e2707c69f31c1b2065b71221dec62c90017"].unchecked_into()
                ),
            ],
            /*Sudo account */
                hex!["f0f16440ffde4a4da023e1d09767b2a829425b45df601e4b3de9012f4b9ac82b"].into(),
                vec![
                    (
                        /* Sudo Account */
                        hex!["f0f16440ffde4a4da023e1d09767b2a829425b45df601e4b3de9012f4b9ac82b"]
                            .into(),
                        /* Balance */
                        1_000,
                    ),
                    (
                        /* Account X1 */
                        hex!["88ed33779dc098f0a7d9110e180c61cc1e138b45eaab18a97f4bf2f9bebba975"]
                            .into(),
                        /* Balance */
                        2_499_000,
                    ),
                    (
                        /* Account X2 */
                        hex!["1c9ed9230f585b6a933a975307d652c3316394063b844c93ce1eef8f5babc94a"]
                            .into(),
                        /* Balance */
                        2_500_000,
                    ),
                    (
                        /* Account X3 */
                        hex!["88cea449161f633299b8c8da4744ce4939a7bd19318b867f8982c1d039ee3c56"]
                            .into(),
                        /* Balance */
                        2_500_000,
                    ),
                    (
                        /* Account X4 */
                        hex!["e079cfec8049d8819c93398b93f11145f1d3b6b324757d577d063e9842f10002"]
                            .into(),
                        /* Balance */
                        2_500_000,
                    ),
                ],
                // Treasury
                hex!["be3f8f4faf72a1ea70b4a3c0ba57127d82c84b12f75f24bc4614411c4244b230"].into(),
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
