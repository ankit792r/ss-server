use std::time::{Duration, SystemTime};

use chrono::Utc;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

use crate::defs::{StringObject, generate_key};

#[derive(Clone)]
pub struct StringObjectService {
    collection: Collection<StringObject>,
}

impl StringObjectService {
    /// Creates a new [`StringObjectService`].
    pub fn new(collection: Collection<StringObject>) -> Self {
        Self { collection }
    }

    async fn generate_key(&self) -> String {
        loop {
            let key = generate_key();

            match self.check_object_exists_by_key(&key).await {
                Ok(false) => return key,
                Ok(true) => continue,
                Err(e) => {
                    eprintln!("Failed to check unique key: {}", e);
                    return String::new();
                }
            }
        }
    }

    /**
    Create new string object from raw content string

    this will hash the raw content
    returns key of new string object
    */
    pub async fn create_new_object(&self, raw_content: String) -> Result<String, String> {
        let now = Utc::now();
        let ten_min = Duration::from_mins(10);
        let future_time = now + ten_min;

        let new_str_obj = StringObject {
            id: ObjectId::new(),
            key: self.generate_key().await,
            content: raw_content,
            expires_at: future_time,
            created_at: now,
            is_deleted: false,
        };

        let res = self.collection.insert_one(&new_str_obj).await;

        match res {
            Err(e) => Err(String::from(format!("Failed to create new Object {:?}", e))),
            Ok(_) => Ok(new_str_obj.key),
        }
    }

    pub async fn get_object_from_id(&self, id: String) -> Result<StringObject, String> {
        let id = ObjectId::parse_str(id).expect("Invalid id provied");
        let filter = doc! { "_id": id };
        let result = self.collection.find_one(filter).await;

        match result {
            Err(e) => Err(String::from(format!("Failed to find object: {:?}", e))),
            Ok(val) => {
                if let Some(obj) = val {
                    Ok(obj)
                } else {
                    Err(String::from("Failed to find object"))
                }
            }
        }
    }

    pub async fn get_object_from_key(&self, key: String) -> Result<StringObject, String> {
        let filter = doc! { "key": key };
        let result = self.collection.find_one(filter).await;

        match result {
            Err(e) => Err(String::from(format!("Failed to find object: {:?}", e))),
            Ok(val) => {
                if let Some(obj) = val {
                    Ok(obj)
                } else {
                    Err(String::from("Failed to find object"))
                }
            }
        }
    }

    pub async fn check_object_exists_by_id(&self, id: String) -> Result<bool, String> {
        let filter = doc! { "_id": id };
        let result = self.collection.count_documents(filter).await;

        match result {
            Ok(ct) => {
                if ct.lt(&(1 as u64)) {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Err(e) => Err(String::from(format!("Failed to check object: {:?}", e))),
        }
    }

    pub async fn check_object_exists_by_key(&self, key: &String) -> Result<bool, String> {
        let filter = doc! { "key": key };
        let result = self.collection.count_documents(filter).await;

        match result {
            Ok(ct) => {
                if ct.lt(&(1 as u64)) {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Err(e) => Err(String::from(format!("Failed to check object: {:?}", e))),
        }
    }

    pub async fn remove_object_by_id(&self, id: String) -> Result<bool, String> {
        let filter = doc! { "_id": id };
        let result = self.collection.delete_one(filter).await;

        match result {
            Ok(dr) => {
                if dr.deleted_count.eq(&(1 as u64)) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(String::from(format!("Failed to delete object: {:?}", e))),
        }
    }

    pub async fn list_objects(&self) -> Result<Vec<StringObject>, String> {
        let filter = doc! {};
        let result_cursor = self.collection.find(filter).await;

        let data: Vec<StringObject> = vec![];

        Ok(data)
    }
}
