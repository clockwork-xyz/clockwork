use {
    serde::Deserialize,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    std::{fs::File, path::Path},
};

static DEFAULT_SLOT_TIMEOUT_THRESHOLD: u64 = 150;
static DEFAULT_WORKER_THREADS: usize = 10;

/// Plugin config.
#[derive(Clone, Debug, Deserialize)]
pub struct PluginConfig {
    pub bugsnag_api_key: Option<String>,
    pub keypath: Option<String>,
    pub slot_timeout_threshold: u64,
    pub worker_threads: usize,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            bugsnag_api_key: None,
            keypath: None,
            slot_timeout_threshold: DEFAULT_SLOT_TIMEOUT_THRESHOLD,
            worker_threads: DEFAULT_WORKER_THREADS,
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
