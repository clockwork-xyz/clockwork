use {
    rdkafka::{
        error::KafkaResult,
        producer::FutureProducer,
        ClientConfig,
    },
    serde::Deserialize,
    solana_accountsdb_plugin_interface::accountsdb_plugin_interface::{
        AccountsDbPluginError,
        Result as PluginResult,
    },
    std::{
        collections::HashMap,
        fs::File,
        path::Path,
    },
};

/// Plugin config.
#[derive(Deserialize)]
pub struct Config {
    /// Kafka config.
    pub kafka: HashMap<String, String>,
    /// Kafka topic to send account updates to.
    #[serde(default)]
    pub update_account_topic: String,
    /// Channel buffer size between plugin and librdkafka.
    #[serde(default)]
    pub buffer_size: usize,
    /// Max time to buffer updates before failing.
    #[serde(default)]
    pub produce_timeout_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            kafka: HashMap::new(),
            update_account_topic: "solana.account_updates".to_owned(),
            buffer_size: 128,
            produce_timeout_ms: 30_000,
        }
    }
}

impl Config {
    /// Read plugin from JSON file.
    pub fn read_from<P: AsRef<Path>>(config_path: P) -> PluginResult<Self> {
        let file = File::open(config_path)?;
        serde_json::from_reader(file)
            .map_err(|e| AccountsDbPluginError::ConfigFileReadError { msg: e.to_string() })
    }

    /// Create rdkafka::FutureProducer from config.
    pub fn producer(&self) -> KafkaResult<FutureProducer> {
        let mut config = ClientConfig::new();
        for (k, v) in self.kafka.iter() {
            config.set(k, v);
        }
        config.create()
    }
}
