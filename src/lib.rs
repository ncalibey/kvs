use std::collections::HashMap;

pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::<String, String>::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.store.get(&key) {
            Some(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
