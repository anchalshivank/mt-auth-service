use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub user_id: i32,
    pub password: String,
}

pub enum LoginResponse {
    Token(String),
    InvalidPassword,
    UserNotFound,
    Error(String),
}
