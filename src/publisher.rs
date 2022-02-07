use {
    crate::*,
    prost::Message,
    rdkafka::{
        error::KafkaError,
        producer::{BaseRecord, Producer as KafkaProducer},
    },
    std::time::Duration,
};

pub struct Publisher {
    producer: Producer,
    shutdown_timeout: Duration,
    update_account_topic: String,
}

impl Publisher {
    pub fn new(producer: Producer, config: &Config) -> Self {
        Self {
            producer,
            shutdown_timeout: Duration::from_millis(config.shutdown_timeout_ms),
            update_account_topic: config.update_account_topic.clone(),
        }
    }

    pub fn update_account(&self, ev: UpdateAccountEvent) -> Result<(), KafkaError> {
        let buf = ev.encode_to_vec();
        let record = BaseRecord::<Vec<u8>, _>::to(&self.update_account_topic)
            .key(&ev.pubkey)
            .payload(&buf);
        match self.producer.send(record) {
            Ok(_) => Ok(()),
            Err((e, _)) => Err(e),
        }
    }
}

impl Drop for Publisher {
    fn drop(&mut self) {
        self.producer.flush(self.shutdown_timeout);
    }
}
