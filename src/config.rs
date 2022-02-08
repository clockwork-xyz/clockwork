use {
    rdkafka::{
        config::FromClientConfig,
        error::KafkaResult,
        producer::{DefaultProducerContext, ThreadedProducer},
        ClientConfig,
    },
    serde::Deserialize,
    solana_accountsdb_plugin_interface::accountsdb_plugin_interface::{
        AccountsDbPluginError, Result as PluginResult,
    },
    std::{collections::HashMap, fs::File, path::Path},
};

/// Plugin config.
#[derive(Deserialize)]
pub struct Config {
    /// Kafka config.
    pub kafka: HashMap<String, String>,
    /// Graceful shutdown timeout.
    #[serde(default)]
    pub shutdown_timeout_ms: u64,
    /// Kafka topic to send account updates to.
    #[serde(default)]
    pub update_account_topic: String,
    /// Kafka topic to send slot status updates to.
    #[serde(default)]
    pub slot_status_topic: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            kafka: HashMap::new(),
            shutdown_timeout_ms: 30_000,
            update_account_topic: "".to_owned(),
            slot_status_topic: "".to_owned(),
        }
    }
}

impl Config {
    /// Read plugin from JSON file.
    pub fn read_from<P: AsRef<Path>>(config_path: P) -> PluginResult<Self> {
        let file = File::open(config_path)?;
        let mut this: Self = serde_json::from_reader(file)
            .map_err(|e| AccountsDbPluginError::ConfigFileReadError { msg: e.to_string() })?;
        this.fill_defaults();
        Ok(this)
    }

    /// Create rdkafka::FutureProducer from config.
    pub fn producer(&self) -> KafkaResult<Producer> {
        let mut config = ClientConfig::new();
        for (k, v) in self.kafka.iter() {
            config.set(k, v);
        }
        ThreadedProducer::from_config(&config)
    }

    fn set_default(&mut self, k: &'static str, v: &'static str) {
        if !self.kafka.contains_key(k) {
            self.kafka.insert(k.to_owned(), v.to_owned());
        }
    }

    fn fill_defaults(&mut self) {
        self.set_default("request.required.acks", "1");
        self.set_default("message.timeout.ms", "30000");
        self.set_default("compression.type", "lz4");
        self.set_default("partitioner", "murmur2_random");
    }
}

pub type Producer = ThreadedProducer<DefaultProducerContext>;
