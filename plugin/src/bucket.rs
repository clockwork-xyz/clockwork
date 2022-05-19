use {
    solana_sdk::pubkey::Pubkey,
    std::{collections::HashMap, sync::Arc},
    tokio::sync::Mutex,
};

pub struct Bucket {
    locks: HashMap<(Pubkey, i64), Arc<Mutex<()>>>,
}

impl Bucket {
    pub fn new() -> Bucket {
        Self {
            locks: HashMap::new(),
        }
    }

    pub fn get_mutex(&mut self, key: impl Into<(Pubkey, i64)>) -> Arc<Mutex<()>> {
        let mutex = self.locks.entry(key.into()).or_default();
        mutex.clone()
    }
}
