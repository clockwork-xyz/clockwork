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
    #[serde(default)]
    pub program_includes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            program_includes: Vec::new(),
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
