use {
    solana_sdk::pubkey::Pubkey,
    std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    },
};

pub struct Bucket {
    _locks: HashMap<(Pubkey, i64), Arc<Mutex<()>>>,
}

impl Bucket {
    pub fn _get_mutex(&mut self, key: impl Into<(Pubkey, i64)>) -> Arc<Mutex<()>> {
        let mutex = self._locks.entry(key.into()).or_default();
        mutex.clone()
    }
}

impl Default for Bucket {
    fn default() -> Self {
        Self {
            _locks: HashMap::new(),
        }
    }
}
