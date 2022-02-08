use solana_accountsdb_plugin_interface::accountsdb_plugin_interface::SlotStatus as PluginSlotStatus;

include!(concat!(
    env!("OUT_DIR"),
    "/blockdaemon.solana.accountsdb_plugin_kafka.types.rs"
));

impl From<PluginSlotStatus> for SlotStatus {
    fn from(other: PluginSlotStatus) -> Self {
        match other {
            PluginSlotStatus::Processed => SlotStatus::Processed,
            PluginSlotStatus::Rooted => SlotStatus::Rooted,
            PluginSlotStatus::Confirmed => SlotStatus::Confirmed,
        }
    }
}
