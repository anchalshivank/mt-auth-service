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

#[derive(Serialize)]
pub struct LoginSuccessData {
    pub(crate) token: String,
}

#[derive(Serialize)]
pub struct LoginErrorData {
    pub(crate) code: String,
    pub(crate) message: String,
}
