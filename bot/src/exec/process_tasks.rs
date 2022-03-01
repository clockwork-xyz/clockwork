use std::sync::{Arc, RwLock};

use crate::{store::TaskStore, utils::new_rpc_client};

use {
    crate::{execute_pending_tasks, monitor_blocktime},
    std::thread,
};

pub fn process_tasks(store: Arc<RwLock<TaskStore>>) {
    let client = Arc::new(new_rpc_client());
    let blocktime_receiver = monitor_blocktime();
    for blocktime in blocktime_receiver {
        println!("⏳ Blocktime: {}", blocktime);
        let tstore = store.clone();
        let tclient = client.clone();
        thread::spawn(move || execute_pending_tasks(tstore, tclient, blocktime));
    }
    process_tasks(store)
}
