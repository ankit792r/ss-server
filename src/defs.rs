use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use rand::RngExt;
use serde::{Deserialize, Serialize};

/// Basic String Object type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringObject {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub key: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_deleted: bool,
}

/// Generate new un-uniqe key
pub fn generate_key() -> String {
    let code: u32 = rand::rng().random_range(0..1_000_000);
    format!("{:06}", code)
}

#[derive(Deserialize, Debug)]
pub struct FormData {
    pub object: Option<String>,
    pub key: Option<String>,
}

pub enum Action {
    Send(String),
    Retrieve(String),
}

#[async_trait]
pub trait DataStore: Send + Sync {
    async fn set(&self, key: String, value: String, ttl_secs: Option<u64>) -> anyhow::Result<()>;

    async fn get(&self, key: &str) -> anyhow::Result<Option<String>>;

    async fn exists(&self, key: &str) -> anyhow::Result<bool>;

    async fn delete(&self, key: &str) -> anyhow::Result<()>;

    async fn count(&self) -> anyhow::Result<usize>;
}

pub type DynStore = Arc<dyn DataStore>;
