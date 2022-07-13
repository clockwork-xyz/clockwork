use crate::observers::http::{HttpObserver, HttpRequest};

use {
    crate::{
        config::PluginConfig,
        events::AccountUpdateEvent,
        executors::{http::HttpExecutor, tx::TxExecutor, Executors},
        observers::{pool::PoolObserver, queue::QueueObserver, Observers},
        tpu_client::TpuClient,
        utils::read_or_new_keypair,
    },
    cronos_client::Client as CronosClient,
    log::info,
    solana_client::rpc_client::RpcClient,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, ReplicaAccountInfoVersions, Result as PluginResult, SlotStatus,
    },
    solana_program::pubkey::Pubkey,
    solana_sdk::commitment_config::CommitmentConfig,
    std::{fmt::Debug, sync::Arc},
    tokio::runtime::{Builder, Runtime},
};

static LOCAL_RPC_URL: &str = "http://127.0.0.1:8899";
static LOCAL_WEBSOCKET_URL: &str = "ws://127.0.0.1:8900";

#[derive(Debug)]
pub struct CronosPlugin {
    // Plugin config values.
    pub config: PluginConfig,
    pub executors: Option<Arc<Executors>>,
    pub observers: Arc<Observers>,
    // pub delegate: Arc<Delegate>,
    // pub scheduler: Arc<Scheduler>,
    // pub http_executor: Option<Arc<HttpExecutor>>,
    // pub tx_executor: Option<Arc<TxExecutor>>,
    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl GeyserPlugin for CronosPlugin {
    fn name(&self) -> &'static str {
        "cronos-plugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        solana_logger::setup_with_default("info");
        info!("Loading snapshot...");
        *self = CronosPlugin::new_from_config(PluginConfig::read_from(config_file)?);
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        _is_startup: bool,
    ) -> PluginResult<()> {
        // Fetch account info
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(account_info) => account_info.clone(),
        };
        let account_pubkey = Pubkey::new(account_info.clone().pubkey);

        // Parse and process the account update
        match AccountUpdateEvent::try_from(account_info) {
            Ok(event) => match event {
                AccountUpdateEvent::Clock { clock } => {
                    self.observers.queue.clone().handle_updated_clock(clock)
                }
                AccountUpdateEvent::HttpRequest { request } => {
                    self.observers.http.clone().handle_updated_http_request(
                        HttpRequest {
                            pubkey: account_pubkey,
                            request,
                        },
                        slot,
                    )
                }
                AccountUpdateEvent::Pool { pool } => {
                    self.observers.pool.clone().handle_updated_pool(pool, slot)
                }
                AccountUpdateEvent::Queue { queue } => self
                    .observers
                    .queue
                    .clone()
                    .handle_updated_queue(queue, account_pubkey),
                AccountUpdateEvent::Rotator { rotator } => {
                    self.observers.pool.clone().handle_updated_rotator(rotator)
                }
                AccountUpdateEvent::Snapshot { snapshot } => self
                    .observers
                    .pool
                    .clone()
                    .handle_updated_snapshot(snapshot),
            },
            Err(_err) => Ok(()),
        }
    }

    fn notify_end_of_startup(&mut self) -> PluginResult<()> {
        info!("Snapshot loaded");
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        _parent: Option<u64>,
        status: solana_geyser_plugin_interface::geyser_plugin_interface::SlotStatus,
    ) -> PluginResult<()> {
        // Re-check health and attempt to build the executor if we're still not caught up
        if self.executors.is_none() {
            self.try_build_executors()
        }

        // Update the plugin state and execute transactions with the confirmed slot number
        match status {
            SlotStatus::Confirmed => match &self.executors {
                Some(executors) => {
                    executors.clone().handle_confirmed_slot(slot)?;
                }
                None => (),
            },
            _ => (),
        }
        Ok(())
    }

    fn notify_transaction(
        &mut self,
        _transaction: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_block_metadata(
        &mut self,
        _blockinfo: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaBlockInfoVersions,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false
    }
}

impl CronosPlugin {
    fn new_from_config(config: PluginConfig) -> Self {
        let runtime = build_runtime(config.clone());
        let pool_observer = Arc::new(PoolObserver::new(config.clone(), runtime.clone()));
        let queue_observer = Arc::new(QueueObserver::new(
            pool_observer.pool_positions.clone(),
            runtime.clone(),
        ));
        let http_observer = Arc::new(HttpObserver::new(
            pool_observer.pool_positions.clone(),
            runtime.clone(),
        ));
        Self {
            config,
            executors: None,
            observers: Arc::new(Observers {
                http: http_observer,
                pool: pool_observer,
                queue: queue_observer,
            }),
            runtime,
        }
    }

    fn try_build_executors(&mut self) {
        // Return early if not healthy
        if RpcClient::new_with_commitment::<String>(
            LOCAL_RPC_URL.into(),
            CommitmentConfig::confirmed(),
        )
        .get_health()
        .is_err()
        {
            return;
        }

        // Build clients
        let cronos_client = Arc::new(CronosClient::new(
            read_or_new_keypair(self.config.clone().delegate_keypath),
            LOCAL_RPC_URL.into(),
        ));
        let tpu_client = Arc::new(
            TpuClient::new(
                read_or_new_keypair(self.config.clone().delegate_keypath),
                LOCAL_RPC_URL.into(),
                LOCAL_WEBSOCKET_URL.into(),
            )
            .unwrap(),
        );

        // Build executors
        let http_executor = Arc::new(HttpExecutor::new(
            self.config.clone(),
            self.observers.clone(),
            self.runtime.clone(),
            cronos_client.payer_pubkey(),
        ));
        let tx_executor = Arc::new(TxExecutor::new(
            self.config.clone(),
            cronos_client.clone(),
            self.observers.clone(),
            self.runtime.clone(),
            tpu_client.clone(),
        ));
        self.executors = Some(Arc::new(Executors {
            http: http_executor,
            tx: tx_executor,
            observers: self.observers.clone(),
            runtime: self.runtime.clone(),
        }))
    }
}

impl Default for CronosPlugin {
    fn default() -> Self {
        Self::new_from_config(PluginConfig::default())
    }
}

fn build_runtime(config: PluginConfig) -> Arc<Runtime> {
    Arc::new(
        Builder::new_multi_thread()
            .enable_all()
            .thread_name("cronos-plugin")
            .worker_threads(config.worker_threads)
            .max_blocking_threads(config.worker_threads)
            .build()
            .unwrap(),
    )
}
