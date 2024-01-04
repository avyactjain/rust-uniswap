use std::fs::{self};

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct UniswapAPIconfig {
    pub wallet_addrs: Vec<String>,
    pub chain_id: u8,
    pub rpc_url: String,
    pub listen_address: String,
}

const DEFAULT_CONFIG_FILE_PATH: &str = "config/local.json";

impl UniswapAPIconfig {
    pub fn from_default_config_file() -> Self {
        let config_file_string = fs::read_to_string(DEFAULT_CONFIG_FILE_PATH)
            .expect("Unable to open the default config file, panic!");
        let config: UniswapAPIconfig = serde_json::from_str(&config_file_string)
            .expect("JSON Formatting error in the default config file, panic!");
        config
    }
}
