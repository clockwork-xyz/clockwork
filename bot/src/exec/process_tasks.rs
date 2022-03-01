use std::sync::{Arc, RwLock};

use solana_client_helpers::Client;

use crate::store::TaskStore;

use {
    crate::{execute_pending_tasks, monitor_blocktime},
    std::thread,
};

pub fn process_tasks(client: Arc<Client>, store: Arc<RwLock<TaskStore>>) {
    let blocktime_receiver = monitor_blocktime();
    for blocktime in blocktime_receiver {
        println!("⏳ Blocktime: {}", blocktime);
        let tstore = store.clone();
        let tclient = client.clone();
        thread::spawn(move || execute_pending_tasks(tclient,tstore, blocktime));
    }
    process_tasks(client, store)
}
