use chrono::{DateTime, Utc};
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
