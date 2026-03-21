use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait DataStore: Send + Sync {
    async fn set(&self, key: String, value: String, ttl_secs: Option<u64>) -> anyhow::Result<()>;

    async fn get(&self, key: &str) -> anyhow::Result<Option<String>>;

    async fn exists(&self, key: &str) -> anyhow::Result<bool>;

    async fn delete(&self, key: &str) -> anyhow::Result<()>;

    async fn count(&self) -> anyhow::Result<usize>;
}

pub type DynStore = Arc<dyn DataStore>;
