#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerifyResponse {
    pub is_verified: bool,
    pub owner: String,
}
