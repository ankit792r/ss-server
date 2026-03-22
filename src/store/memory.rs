use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::store::DataStore;

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
    async fn set(&self, key: String, value: String, _ttl: Option<u64>) -> Result<(), String> {
        let mut map = self.map.lock().unwrap();
        map.insert(key, value);
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<String>, String> {
        let map = self.map.lock().unwrap();
        Ok(map.get(key).cloned())
    }

    async fn exists(&self, key: &str) -> Result<bool, String> {
        let map = self.map.lock().unwrap();
        Ok(map.contains_key(key))
    }

    async fn delete(&self, key: &str) -> Result<(), String> {
        let mut map = self.map.lock().unwrap();
        map.remove(key);
        Ok(())
    }

    async fn count(&self) -> Result<usize, String> {
        let map = self.map.lock().unwrap();
        Ok(map.len())
    }
}
