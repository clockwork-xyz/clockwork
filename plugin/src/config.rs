use {
    serde::Deserialize,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    std::{fs::File, path::Path},
};

static DEFAULT_TRANSACTION_TIMEOUT_THRESHOLD: u64 = 150;
static DEFAULT_THREAD_COUNT: usize = 10;

/// Plugin config.
#[derive(Clone, Debug, Deserialize)]
pub struct PluginConfig {
    pub keypath: Option<String>,
    pub sentry_url: Option<String>,
    pub thread_count: usize,
    pub transaction_timeout_threshold: u64,
    pub worker_id: u64,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            keypath: None,
            sentry_url: None,
            transaction_timeout_threshold: DEFAULT_TRANSACTION_TIMEOUT_THRESHOLD,
            thread_count: DEFAULT_THREAD_COUNT,
            worker_id: 0,
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
