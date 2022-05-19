use {
    cronos_sdk::scheduler::state::Queue,
    dashmap::{DashMap, DashSet},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Default)]
pub struct QueueCache {
    pub data: DashMap<Pubkey, Queue>,
    pub index: DashMap<i64, DashSet<Pubkey>>,
}

impl QueueCache {
    pub fn new() -> QueueCache {
        QueueCache {
            data: DashMap::new(),
            index: DashMap::new(),
        }
    }

    pub fn insert(&mut self, key: Pubkey, queue: Queue) {
        self.delete(key);
        self.data.insert(key, queue.clone());

        match self.index.get_mut(&queue.exec_at.unwrap()) {
            Some(cached_set) => {
                cached_set.insert(key);
            }
            None => {
                let set = DashSet::new();
                set.insert(key);
                self.index.insert(queue.exec_at.unwrap(), set);
            }
        }
    }

    pub fn delete(&mut self, key: Pubkey) {
        self.data.clone().get(&key).and_then(|queue| {
            self.data.remove(&key);
            self.index
                .clone()
                .get_mut(&queue.exec_at.unwrap())
                .and_then(|cached_set| {
                    cached_set.remove(&key);
                    if cached_set.is_empty() {
                        self.index.remove(&queue.exec_at.unwrap());
                    }
                    Some(())
                })
        });
    }
}
