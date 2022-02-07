pub enum Event {
    UpdateAccount(UpdateAccountEvent)
}

include!(concat!(env!("OUT_DIR"), "/blockdaemon.solana.accountsdb_plugin_kafka.types.rs"));
