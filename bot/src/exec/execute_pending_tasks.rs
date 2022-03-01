use solana_client_helpers::Client;
use std::sync::{Arc, RwLock};

use crate::execute_task;
use crate::store::TaskStore;

pub fn execute_pending_tasks(client: Arc<Client>, store: Arc<RwLock<TaskStore>>, blocktime: i64) {
    for t in (blocktime - 10)..blocktime {
        let r_store = store.read().unwrap();
        r_store.index.get(&t).and_then(|keys| {
            for key in keys.iter() {
                r_store.data.get(key).and_then(|task| {
                    execute_task(client.clone(), store.clone(), *key, task.clone());
                    Some(())
                });
            }
            Some(())
        });
    }
}
