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
    log::info,
    rdkafka::util::get_rdkafka_version,
    simple_error::simple_error,
    solana_accountsdb_plugin_interface::accountsdb_plugin_interface::{
        AccountsDbPlugin, AccountsDbPluginError as PluginError, ReplicaAccountInfo,
        ReplicaAccountInfoVersions, Result as PluginResult, SlotStatus as PluginSlotStatus,
    },
    std::fmt::{Debug, Formatter},
};

#[derive(Default)]
pub struct KafkaPlugin {
    publisher: Option<Publisher>,
    filter: Option<Filter>,
    publish_all_accounts: bool,
}

impl Debug for KafkaPlugin {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl AccountsDbPlugin for KafkaPlugin {
    fn name(&self) -> &'static str {
        "KafkaPlugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        if self.publisher.is_some() {
            let err = simple_error!("plugin already loaded");
            return Err(PluginError::Custom(Box::new(err)));
        }

        solana_logger::setup_with_default("info");
        info!(
            "Loading plugin {:?} from config_file {:?}",
            self.name(),
            config_file
        );
        let config = Config::read_from(config_file)?;
        self.publish_all_accounts = config.publish_all_accounts;

        let (version_n, version_s) = get_rdkafka_version();
        info!("rd_kafka_version: {:#08x}, {}", version_n, version_s);

        let producer = config
            .producer()
            .map_err(|e| PluginError::Custom(Box::new(e)))?;
        info!("Created rdkafka::FutureProducer");

        let publisher = Publisher::new(producer, &config);
        self.publisher = Some(publisher);
        self.filter = Some(Filter::new(&config));
        info!("Spawned producer");

        Ok(())
    }

    fn on_unload(&mut self) {
        self.publisher = None;
        self.filter = None;
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        if is_startup && !self.publish_all_accounts {
            return Ok(());
        }

        let info = Self::unwrap_update_account(account);
        if !self.unwrap_filter().wants_program(info.owner) {
            return Ok(());
        }

        let event = UpdateAccountEvent {
            slot,
            pubkey: info.pubkey.to_vec(),
            lamports: info.lamports,
            owner: info.owner.to_vec(),
            executable: info.executable,
            rent_epoch: info.rent_epoch,
            data: info.data.to_vec(),
            write_version: info.write_version,
        };

        let publisher = self.unwrap_publisher();
        publisher
            .update_account(event)
            .map_err(|e| PluginError::AccountsUpdateError { msg: e.to_string() })
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        parent: Option<u64>,
        status: PluginSlotStatus,
    ) -> PluginResult<()> {
        let publisher = self.unwrap_publisher();
        if !publisher.wants_slot_status() {
            return Ok(());
        }

        let event = SlotStatusEvent {
            slot,
            parent: parent.unwrap_or(0),
            status: SlotStatus::from(status).into(),
        };

        publisher
            .update_slot_status(event)
            .map_err(|e| PluginError::AccountsUpdateError { msg: e.to_string() })
    }

    fn account_data_notifications_enabled(&self) -> bool {
        self.unwrap_publisher().wants_update_account()
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false
    }
}

impl KafkaPlugin {
    pub fn new() -> Self {
        Default::default()
    }

    fn unwrap_publisher(&self) -> &Publisher {
        self.publisher.as_ref().expect("publisher is unavailable")
    }

    fn unwrap_filter(&self) -> &Filter {
        self.filter.as_ref().expect("filter is unavailable")
    }

    fn unwrap_update_account(account: ReplicaAccountInfoVersions) -> &ReplicaAccountInfo {
        match account {
            ReplicaAccountInfoVersions::V0_0_1(info) => info,
        }
    }
}
