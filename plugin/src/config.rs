use {
    serde::Deserialize,
    solana_accountsdb_plugin_interface::accountsdb_plugin_interface::{
        AccountsDbPluginError, Result as PluginResult,
    },
    std::{fs::File, path::Path},
};

/// Plugin config.
#[derive(Deserialize)]
pub struct Config {
    pub keypath: String,
    pub program_includes: Vec<String>,
    pub rpc_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keypath: "".to_string(),
            program_includes: Vec::new(),
            rpc_url: "http://127.0.0.1:8899".to_string(),
        }
    }
}

impl Config {
    /// Read plugin from JSON file.
    pub fn read_from<P: AsRef<Path>>(config_path: P) -> PluginResult<Self> {
        let file = File::open(config_path)?;
        let this: Self = serde_json::from_reader(file)
            .map_err(|e| AccountsDbPluginError::ConfigFileReadError { msg: e.to_string() })?;
        Ok(this)
    }
}
