use mongodb::bson::{doc, oid::ObjectId, Bson};

use std::time::{self, SystemTime};

use crate::password::Password;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerifyRequest {
    pub verify_secret: String,
    pub auth_token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerifyResponse {
    pub is_verified: bool,
    pub owner: String,
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
    pub auth_tokens: Vec<AuthToken>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AuthToken {
    pub auth_token: String,
    pub expire_date: i64,
}
impl From<Bson> for AuthToken {
    fn from(value: Bson) -> Self {
        let document = value.as_document().unwrap();
        let auth_token = document.get("auth_token").unwrap().as_str().unwrap();
        let expire_date = document.get("expire_date").unwrap().as_i64().unwrap();

        AuthToken {
            auth_token: auth_token.to_string(),
            expire_date,
        }
    }
}
impl From<AuthToken> for Bson {
    fn from(value: AuthToken) -> Bson {
        let document = doc! {
            "auth_token": value.auth_token.clone(),
            "expire_date": value.expire_date.clone(),
        };

        Bson::Document(document)
    }
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
