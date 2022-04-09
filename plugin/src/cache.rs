use {
    cronos_sdk::scheduler::state::Task,
    solana_sdk::pubkey::Pubkey,
    std::collections::{HashMap, HashSet},
};

#[derive(Default)]
pub struct TaskCache {
    pub data: HashMap<Pubkey, Task>,
    pub index: HashMap<i64, HashSet<Pubkey>>,
}

impl TaskCache {
    pub fn new() -> TaskCache {
        TaskCache {
            data: HashMap::new(),
            index: HashMap::new(),
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
                let mut set = HashSet::new();
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
