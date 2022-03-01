use std::{sync::{Arc, RwLock}};

use dotenv::dotenv;
use solana_client_helpers::ClientResult;
use store::{TaskStore, MutableTaskStore};

mod env;
mod exec;
mod replicate;
mod store;
mod utils;

use {exec::*, replicate::*};

fn main() -> ClientResult<()> {
    // Load env file
    dotenv().ok();

    // Create task store
    let store = Arc::new(RwLock::new(TaskStore::new()));

    // Replicate Cronos tasks to local store
    replicate_cronos_tasks(store.clone());

    // Process pending tasks when Solana blocktime updates
    process_tasks(store);

    Ok(())
}
