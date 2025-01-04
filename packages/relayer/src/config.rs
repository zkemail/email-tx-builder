use anyhow::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// The port number for the application to listen on.
    pub port: usize,
    /// The URL for the database connection.
    pub database_url: String,
    /// The URL for the SMTP server.
    pub smtp_url: String,
    /// Configuration for the prover service.
    pub prover: ProverConfig,
    /// Configuration for file paths.
    pub path: PathConfig,
    /// Configuration for ICP (Internet Computer Protocol).
    pub icp: IcpConfig,
    /// A map of chain configurations, keyed by chain name.
    pub chains: HashMap<String, ChainConfig>,
    /// Flag to enable JSON logging.
    pub json_logger: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PathConfig {
    /// Path to the PEM file.
    pub pem: String,
    /// Path to the email templates directory.
    pub email_templates: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IcpConfig {
    /// The canister ID for DKIM (DomainKeys Identified Mail).
    pub dkim_canister_id: String,
    /// The canister ID for the wallet.
    pub wallet_canister_id: String,
    /// The URL for the IC (Internet Computer) replica.
    pub ic_replica_url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainConfig {
    /// The private key for the blockchain.
    pub private_key: String,
    /// The RPC (Remote Procedure Call) URL for the blockchain.
    pub rpc_url: String,
    /// The chain ID for the blockchain.
    pub chain_id: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProverConfig {
    /// The URL for the prover service.
    pub url: String,
    /// The blueprint ID for the prover.
    pub blueprint_id: String,
    /// The URL for downloading zkey files.
    pub zkey_download_url: String,
    /// The URL for downloading circuit CPP files.
    pub circuit_cpp_download_url: String,
    /// The API key for the prover.
    pub api_key: String,
}

// Function to load the configuration from a JSON file
pub fn load_config() -> Result<Config, Error> {
    // Open the configuration file
    let mut file = File::open("config.json")
        .map_err(|e| anyhow::anyhow!("Failed to open config file: {}", e))?;

    // Read the file's content into a string
    let mut data = String::new();
    file.read_to_string(&mut data)
        .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;

    // Deserialize the JSON content into a Config struct
    let config: Config = serde_json::from_str(&data)
        .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;

    // Setting Logger ENV
    if config.json_logger {
        env::set_var("JSON_LOGGER", "true");
    }

    Ok(config)
}
