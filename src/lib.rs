use solana_accountsdb_plugin_interface::accountsdb_plugin_interface::AccountsDbPlugin;

mod config;
mod event;
mod plugin;
mod publisher;

pub use {
    config::Config,
    event::*,
    plugin::KafkaPlugin,
    publisher::{Publisher, PublisherHandle},
};

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns a pointer to the Kafka Plugin box implementing trait AccountsDbPlugin.
///
/// The Solana validator and this plugin must be compiled with the same Rust compiler version and Solana core version.
/// Loading this plugin with mismatching versions is undefined behavior and will likely cause memory corruption.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn AccountsDbPlugin {
    let plugin = KafkaPlugin::new();
    let plugin: Box<dyn AccountsDbPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}
