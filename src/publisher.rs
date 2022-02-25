// Copyright 2022 Blockdaemon Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
    slot_status_topic: String,
}

impl Publisher {
    pub fn new(producer: Producer, config: &Config) -> Self {
        Self {
            producer,
            shutdown_timeout: Duration::from_millis(config.shutdown_timeout_ms),
            update_account_topic: config.update_account_topic.clone(),
            slot_status_topic: config.slot_status_topic.clone(),
        }
    }

    pub fn update_account(&self, ev: UpdateAccountEvent) -> Result<(), KafkaError> {
        let buf = ev.encode_to_vec();
        let record = BaseRecord::<Vec<u8>, _>::to(&self.update_account_topic)
            .key(&ev.pubkey)
            .payload(&buf);
        self.producer.send(record).map(|_| ()).map_err(|(e, _)| e)
    }

    pub fn update_slot_status(&self, ev: SlotStatusEvent) -> Result<(), KafkaError> {
        let buf = ev.encode_to_vec();
        let record = BaseRecord::<(), _>::to(&self.slot_status_topic).payload(&buf);
        self.producer.send(record).map(|_| ()).map_err(|(e, _)| e)
    }

    pub fn wants_update_account(&self) -> bool {
        !self.update_account_topic.is_empty()
    }

    pub fn wants_slot_status(&self) -> bool {
        !self.slot_status_topic.is_empty()
    }
}

impl Drop for Publisher {
    fn drop(&mut self) {
        self.producer.flush(self.shutdown_timeout);
    }
}
