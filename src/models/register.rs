use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserRegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub staff_no: String,
    pub license_no: String,
    pub digi_signature: String,
}
pub enum RegisterResponse {
    UserAlreadyExists,
    UserSuccessfullyRegistered,
    Error(String),
}

#[derive(Serialize)]
pub struct RegisterErrorData {
    pub code: String,
    pub message: String,
}
