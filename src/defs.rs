use std::time::SystemTime;

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
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub is_deleted: bool,
}

/// Key utils

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyUtils {}

impl KeyUtils {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_key(&self) -> String {
        let code: u32 = rand::rng().random_range(0..1_000_000);
        format!("{:06}", code)
    }

    pub fn generate_uniq_key(&self) -> String {
        // TODO: add db check
        self.generate_key()
    }
}
