use {
    solana_sdk::pubkey::Pubkey,
    std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    },
};

pub struct Bucket {
    locks: HashMap<(Pubkey, i64), Arc<Mutex<()>>>,
}

impl Bucket {
    pub fn get_mutex(&mut self, key: impl Into<(Pubkey, i64)>) -> Arc<Mutex<()>> {
        let mutex = self.locks.entry(key.into()).or_default();
        mutex.clone()
    }
}

impl Default for Bucket {
    fn default() -> Self {
        Self {
            locks: HashMap::new(),
        }
    }
}
