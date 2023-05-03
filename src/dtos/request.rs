#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerifyRequest {
    pub verify_secret: String,
    pub auth_token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
