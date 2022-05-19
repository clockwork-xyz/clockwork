use {
    cronos_sdk::scheduler::state::Task,
    dashmap::{DashMap, DashSet},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Default)]
pub struct TaskCache {
    pub data: DashMap<Pubkey, Task>,
    pub index: DashMap<i64, DashSet<Pubkey>>,
}

impl TaskCache {
    pub fn new() -> TaskCache {
        TaskCache {
            data: DashMap::new(),
            index: DashMap::new(),
        }
    }

    pub fn insert(&mut self, key: Pubkey, task: Task) {
        self.delete(key);
        self.data.insert(key, task.clone());

        match self.index.get_mut(&task.exec_at.unwrap()) {
            Some(cached_set) => {
                cached_set.insert(key);
            }
            None => {
                let set = DashSet::new();
                set.insert(key);
                self.index.insert(task.exec_at.unwrap(), set);
            }
        }
    }

    pub fn delete(&mut self, key: Pubkey) {
        self.data.clone().get(&key).and_then(|task| {
            self.data.remove(&key);
            self.index
                .clone()
                .get_mut(&task.exec_at.unwrap())
                .and_then(|cached_set| {
                    cached_set.remove(&key);
                    if cached_set.is_empty() {
                        self.index.remove(&task.exec_at.unwrap());
                    }
                    Some(())
                })
        });
    }
}
