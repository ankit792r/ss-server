use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::defs::DataStore;

pub struct InMemoryStore {
    map: Mutex<HashMap<String, String>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl DataStore for InMemoryStore {
    async fn set(&self, key: String, value: String, _ttl: Option<u64>) -> anyhow::Result<()> {
        let mut map = self.map.lock().unwrap();
        map.insert(key, value);
        Ok(())
    }

    async fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        let map = self.map.lock().unwrap();
        Ok(map.get(key).cloned())
    }

    async fn exists(&self, key: &str) -> anyhow::Result<bool> {
        let map = self.map.lock().unwrap();
        Ok(map.contains_key(key))
    }

    async fn delete(&self, key: &str) -> anyhow::Result<()> {
        let mut map = self.map.lock().unwrap();
        map.remove(key);
        Ok(())
    }

    async fn count(&self) -> anyhow::Result<usize> {
        let map = self.map.lock().unwrap();
        Ok(map.len())
    }
}
