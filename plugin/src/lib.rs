use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

mod config;
mod delegate;
mod events;
mod plugin;
mod scheduler;
mod tpu_client;
mod utils;

pub use plugin::CronosPlugin;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// The Solana validator and this plugin must be compiled with the same Rust compiler version and Solana core version.
/// Loading this plugin with mismatching versions is undefined behavior and will likely cause memory corruption.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin: Box<dyn GeyserPlugin> = Box::new(CronosPlugin::default());
    Box::into_raw(plugin)
}

// pub fn monitor_delegate_status_updates(self: Arc<Self>) -> PluginResult<()> {
//     thread::spawn(move || {
//         loop {
//             match self.status_receiver.recv() {
//                 Ok(delegate_status) => {
//                     // Update the delegate status
//                     let mut w = self.delegate_status.try_write().unwrap();
//                     w.executor_pool_position = delegate_status.executor_pool_position;
//                 }
//                 Err(err) => {
//                     info!("Error receiving data: {}", err);
//                 }
//             }
//         }
//     });

//     Ok(())
// }
