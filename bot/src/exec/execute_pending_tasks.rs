use std::sync::{RwLock, Arc};

use crate::store::TaskStore;
use crate::execute_task;


pub fn execute_pending_tasks(store: Arc<RwLock<TaskStore>>, blocktime: i64) {
    for t in (blocktime - 10)..blocktime {
        let r_store = store.read().unwrap();
        r_store.index.get(&t).and_then(|keys| {
            for key in keys.iter() {
                r_store.data.get(key).and_then(|task| {
                    execute_task(store.clone(), *key, task.clone());
                    Some(())
                });
            }
            Some(())
        });
    }

    
    

    // let mut psql = postgres::Client::connect(env::psql_params().as_str(), postgres::NoTls).unwrap();
    // let query = "SELECT * FROM tasks WHERE status = $1 AND exec_at <= $2";
    // for row in psql
    //     .query(query, &[&TaskStatus::Queued.to_string(), &blocktime])
    //     .unwrap()
    // {
    //     let task = Pubkey::from_str(row.get(0)).unwrap();
    //     let daemon = Pubkey::from_str(row.get(1)).unwrap();
    //     let ix_bytes: Vec<u8> = row.get(4);
    //     let ix = borsh::try_from_slice_with_schema(ix_bytes.as_slice()).unwrap();
    //     execute_task(task, daemon, ix);
    // }
}
