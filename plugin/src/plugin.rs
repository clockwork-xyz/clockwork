use std::{fmt::Debug, sync::Arc};

use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfo, ReplicaAccountInfoVersions, Result as PluginResult,
    SlotStatus,
};
use solana_program::pubkey::Pubkey;
use tokio::runtime::{Builder, Runtime};

use crate::{
    config::PluginConfig,
    events::AccountUpdateEvent,
    executors::Executors,
    observers::{webhook::HttpRequest, Observers},
};

pub struct ClockworkPlugin {
    pub inner: Arc<Inner>,
}

impl Debug for ClockworkPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "inner: {:?}", self.inner)
    }
}

#[derive(Debug)]
pub struct Inner {
    pub config: PluginConfig,
    pub executors: Arc<Executors>,
    pub observers: Arc<Observers>,
    pub runtime: Arc<Runtime>,
}

impl GeyserPlugin for ClockworkPlugin {
    fn name(&self) -> &'static str {
        "clockwork-plugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        solana_logger::setup_with_default("info");
        info!(
            "clockwork-plugin crate-info - spec: {}, geyser_interface_version: {}",
            env!("SPEC"),
            env!("GEYSER_INTERFACE_VERSION")
        );
        info!("Loading snapshot...");
        let config = PluginConfig::read_from(config_file)?;
        let _guard = sentry::init((
            config.clone().sentry_url,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));
        *self = ClockworkPlugin::new_from_config(config);
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        // Parse account info.
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(account_info) => ReplicaAccountInfo {
                pubkey: account_info.pubkey,
                lamports: account_info.lamports,
                owner: account_info.owner,
                executable: account_info.executable,
                rent_epoch: account_info.rent_epoch,
                data: account_info.data,
                write_version: account_info.write_version,
            },
            ReplicaAccountInfoVersions::V0_0_2(account_info) => ReplicaAccountInfo {
                pubkey: account_info.pubkey,
                lamports: account_info.lamports,
                owner: account_info.owner,
                executable: account_info.executable,
                rent_epoch: account_info.rent_epoch,
                data: account_info.data,
                write_version: account_info.write_version,
            },
        };
        let account_pubkey = Pubkey::new(account_info.pubkey);
        let event = AccountUpdateEvent::try_from(account_info);

        // Process event on tokio task.
        self.inner.clone().spawn(|inner| async move {
            // Send all account updates to the thread observer for account listeners.
            // Only process account updates if we're past the startup phase.
            if !is_startup {
                inner
                    .observers
                    .thread
                    .clone()
                    .observe_account(account_pubkey, slot)
                    .await?;
            }

            // Parse and process specific update events.
            if let Ok(event) = event {
                match event {
                    AccountUpdateEvent::Clock { clock } => {
                        inner
                            .observers
                            .thread
                            .clone()
                            .observe_clock(clock)
                            .await
                            .ok();
                    }
                    AccountUpdateEvent::HttpRequest { request } => {
                        inner
                            .observers
                            .webhook
                            .clone()
                            .observe_request(HttpRequest {
                                pubkey: account_pubkey,
                                request,
                            })
                            .await
                            .ok();
                    }
                    AccountUpdateEvent::Thread { thread } => {
                        inner
                            .observers
                            .thread
                            .clone()
                            .observe_thread(thread, account_pubkey, slot)
                            .await
                            .ok();
                    }
                }
            }
            Ok(())
        });
        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> PluginResult<()> {
        info!("Snapshot loaded");
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        _parent: Option<u64>,
        status: SlotStatus,
    ) -> PluginResult<()> {
        self.inner.clone().spawn(|inner| async move {
            match status {
                SlotStatus::Processed => {
                    inner
                        .executors
                        .clone()
                        .process_slot(inner.observers.clone(), slot, inner.runtime.clone())
                        .await?;
                }
                _ => (),
            }
            Ok(())
        });
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

impl ClockworkPlugin {
    fn new_from_config(config: PluginConfig) -> Self {
        let runtime = build_runtime(config.clone());
        let observers = Arc::new(Observers::new());
        let executors = Arc::new(Executors::new(config.clone()));
        Self {
            inner: Arc::new(Inner {
                config,
                executors,
                observers,
                runtime,
            }),
        }
    }
}

impl Default for ClockworkPlugin {
    fn default() -> Self {
        Self::new_from_config(PluginConfig::default())
    }
}

impl Inner {
    fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
        self: Arc<Self>,
        f: impl FnOnce(Arc<Self>) -> F,
    ) {
        self.runtime.spawn(f(self.clone()));
    }
}

fn build_runtime(config: PluginConfig) -> Arc<Runtime> {
    Arc::new(
        Builder::new_multi_thread()
            .enable_all()
            .thread_name("clockwork-plugin")
            .worker_threads(config.thread_count)
            .max_blocking_threads(config.thread_count)
            .build()
            .unwrap(),
    )
}
