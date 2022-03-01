use std::sync::{Arc, RwLock};

use crate::store::TaskStore;

use {
    crate::{execute_pending_tasks, monitor_blocktime},
    std::thread,
};

pub fn process_tasks(store: Arc<RwLock<TaskStore>>) {
    let blocktime_receiver = monitor_blocktime();
    for blocktime in blocktime_receiver {
        println!("⏳ Blocktime: {}", blocktime);
        let tstore = store.clone();
        thread::spawn(move || execute_pending_tasks(tstore, blocktime));
    }
    process_tasks(store)
}
