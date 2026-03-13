use std::time::SystemTime;

use rand::RngExt;
use uuid::Uuid;

/// Basic String Object type
#[derive(Debug, Clone)]
pub struct StringObject {
    id: Uuid,
    key: String,
    content: String,
    created_at: SystemTime,
    expires_at: SystemTime,
    is_deleted: bool,
}

impl StringObject {
    pub fn new_string_object(
        self,
        content: String,
        expires_at: SystemTime,
        key: String,
    ) -> Result<Self, String> {
        let uuid = Uuid::new_v4();

        Ok(StringObject {
            id: uuid,
            key,
            content,
            expires_at,
            created_at: SystemTime::now(),
            is_deleted: false,
        })
    }
}

/// Key utils
pub struct KeyUtils {
    rng: rand::rngs::ThreadRng,
}

impl KeyUtils {
    pub fn new() -> Self {
        KeyUtils { rng: rand::rng() }
    }

    pub fn generate_key(&mut self) -> String {
        let code: u32 = self.rng.random_range(0..1_000_000);
        format!("{:06}", code)
    }

    pub fn generate_uniq_key(&mut self) -> String {
        // TODO: add db check
        self.generate_key()
    }
}
