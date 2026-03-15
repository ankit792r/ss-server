use std::time::{Duration, SystemTime};

use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

use crate::defs::{KeyUtils, StringObject};

#[derive(Clone)]
pub struct StringObjectService {
    key_utils: KeyUtils,
    collection: Collection<StringObject>,
}

impl StringObjectService {
    /// Creates a new [`StringObjectService`].
    pub fn new(collection: Collection<StringObject>, key_utils: KeyUtils) -> Self {
        Self {
            collection,
            key_utils,
        }
    }

    /**
    Create new string object from raw content string

    this will hash the raw content
    returns key of new string object
    */
    pub async fn create_new_object(&self, raw_content: String) -> Result<String, String> {
        let now = SystemTime::now();
        let ten_min = Duration::from_mins(10);
        let future_time = now + ten_min;

        let new_str_obj = StringObject {
            id: ObjectId::new(),
            key: self.key_utils.generate_uniq_key(),
            content: raw_content,
            expires_at: future_time,
            created_at: SystemTime::now(),
            is_deleted: false,
        };

        let res = self.collection.insert_one(&new_str_obj).await;

        println!("{:?}", res);

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

    pub async fn check_object_exists_by_key(&self, key: String) -> Result<bool, String> {
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
}
