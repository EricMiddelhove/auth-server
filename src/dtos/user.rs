use mongodb::bson::{doc, oid::ObjectId};

use std::time::{self, SystemTime};

use crate::password::Password;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    pub auth_token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DtoUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DatabaseUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub auth_grants: Vec<String>,
    pub auth_tokens: Vec<String>,
}
impl DatabaseUser {
    pub fn new(name: &String, email: &String, password: Password) -> DatabaseUser {
        let now = SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        DatabaseUser {
            id: ObjectId::new().to_hex(),
            name: name.clone(),
            password: password.secure_hash,
            email: email.clone(),
            created_at: now,
            updated_at: now,
            auth_grants: vec![],
            auth_tokens: vec![],
        }
    }

    pub async fn insert_to_database(
        &self,
        collection: mongodb::Collection<DatabaseUser>,
    ) -> mongodb::results::InsertOneResult {
        collection.insert_one(self, None).await.unwrap()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Grant {
    pub auth_grant: String,
}
