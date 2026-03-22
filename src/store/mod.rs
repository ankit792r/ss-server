pub mod memory;

use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait DataStore: Send + Sync {
    async fn set(&self, key: String, value: String, ttl_secs: Option<u64>) -> Result<(), String>;

    async fn get(&self, key: &str) -> Result<Option<String>, String>;

    async fn exists(&self, key: &str) -> Result<bool, String>;

    async fn delete(&self, key: &str) -> Result<(), String>;

    async fn count(&self) -> Result<usize, String>;
}

pub type DynStore = Arc<dyn DataStore>;
