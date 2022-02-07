use {
    crate::*,
    log::{error, info},
    rdkafka::{
        util::get_rdkafka_version,
    },
    simple_error::simple_error,
    solana_accountsdb_plugin_interface::accountsdb_plugin_interface::{
        AccountsDbPlugin,
        AccountsDbPluginError,
        Result as PluginResult,
        ReplicaAccountInfo,
        ReplicaAccountInfoVersions,
    },
    std::fmt::{Debug, Formatter},
    tokio::runtime::Runtime,
};

#[derive(Default)]
pub struct KafkaPlugin {
    publisher: Option<PublisherHandle>,
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
            // TODO(richard): Panic here instead?
            error!("Plugin already loaded, ignoring on_load");
            return Ok(());
        }

        solana_logger::setup_with_default("info");
        info!(
            "Loading plugin {:?} from config_file {:?}",
            self.name(),
            config_file
        );
        let config = Config::read_from(config_file)?;

        let (version_n, version_s) = get_rdkafka_version();
        info!("rd_kafka_version: {:#08x}, {}", version_n, version_s);

        let producer = config.producer()
            .map_err(|e| AccountsDbPluginError::Custom(Box::new(e)))?;
        info!("Created rdkafka::FutureProducer");

        let (producer_handle, producer_future) = Publisher::spawn(producer, &config);
        self.publisher = Some(producer_handle);

        let runtime = Runtime::new()?;
        runtime.spawn(producer_future);
        info!("Spawned producer event loop");

        Ok(())
    }

    fn on_unload(&mut self) {
        if let Some(publisher) = self.publisher.take() {
            publisher.blocking_close();
        }
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        if is_startup {
            return Ok(())
        }

        let info = Self::unwrap_update_account(account);
        self.send_event_unwrap(Event::UpdateAccount(UpdateAccountEvent {
            slot,
            pubkey: info.pubkey.to_vec(),
            lamports: info.lamports,
            owner: info.owner.to_vec(),
            executable: info.executable,
            rent_epoch: info.rent_epoch,
            data: info.data.to_vec(),
            write_version: info.write_version,
        }))?;

        Ok(())
    }
}

impl KafkaPlugin {
    pub fn new() -> Self {
        Default::default()
    }

    fn unwrap_publisher(&self) -> &PublisherHandle {
        self.publisher.as_ref().expect("publisher is unavailable")
    }

    fn send_event_unwrap(&self, event: Event) -> PluginResult<()> {
        let publisher = self.unwrap_publisher();
        publisher.blocking_send(event)
            .map_err(|e| AccountsDbPluginError::Custom(Box::new(simple_error!("publisher is unavailable: {}", e))))
    }

    fn unwrap_update_account(account: ReplicaAccountInfoVersions) -> &ReplicaAccountInfo {
        match account {
            ReplicaAccountInfoVersions::V0_0_1(info) => info,
        }
    }
}
