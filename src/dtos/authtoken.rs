use mongodb::bson::{Bson, Document};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub expire_date: i64,
}

impl From<AuthToken> for Bson {
    fn from(auth_token: AuthToken) -> Self {
        let mut doc = Document::new();
        doc.insert("token", auth_token.token);
        doc.insert("expire_date", auth_token.expire_date);

        Bson::Document(doc)
    }
}
