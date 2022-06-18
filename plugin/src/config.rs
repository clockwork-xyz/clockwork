use {
    serde::Deserialize,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    std::{fs::File, path::Path},
};

/// Plugin config.
#[derive(Clone, Debug, Deserialize)]
pub struct PluginConfig {
    pub bugsnag_api_key: Option<String>,
    pub delegate_keypath: String,
    pub slot_timeout_threshold: u64,
    pub worker_threads: usize,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            bugsnag_api_key: None,
            delegate_keypath: "/home/sol/cronos-delegate-keypair.json".into(),
            slot_timeout_threshold: 150,
            worker_threads: 10,
        }
    }
}

impl PluginConfig {
    /// Read plugin from JSON file.
    pub fn read_from<P: AsRef<Path>>(config_path: P) -> PluginResult<Self> {
        let file = File::open(config_path)?;
        let this: Self = serde_json::from_reader(file)
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: e.to_string() })?;
        Ok(this)
    }
}
