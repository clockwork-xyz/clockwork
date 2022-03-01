use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use solana_sdk::pubkey::Pubkey;

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
