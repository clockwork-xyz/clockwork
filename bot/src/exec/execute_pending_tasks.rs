use std::sync::{RwLock, Arc};
use solana_client_helpers::Client;

use crate::store::TaskStore;
use crate::execute_task;


pub fn execute_pending_tasks(store: Arc<RwLock<TaskStore>>, client: Arc<Client>, blocktime: i64) {
    for t in (blocktime - 10)..blocktime {
        let r_store = store.read().unwrap();
        r_store.index.get(&t).and_then(|keys| {
            for key in keys.iter() {
                r_store.data.get(key).and_then(|task| {
                    execute_task(store.clone(), client.clone(), *key, task.clone());
                    Some(())
                });
            }
            Some(())
        });
    }
}
