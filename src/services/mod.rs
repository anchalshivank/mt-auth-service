use crate::models::login::request::LoginResponse::{Error, InvalidPassword, Token, UserNotFound};
use crate::models::login::request::{LoginResponse, UserLoginRequest};
use crate::models::register::request::UserRegisterRequest;
use crate::models::register::response::RegisterResponse;
use crate::models::register::response::RegisterResponse::{
    UserAlreadyExists, UserSuccessfullyRegistered,
};
use crate::repositories::UserRepository;
use crate::utils::crypt::{hash_password, is_valid};
use ntex::web::error::BlockingError;
use ntex::web::types::Json;
use std::sync::{Arc, Mutex};


#[derive(Clone)]
pub struct UserService {
    repository: Arc<Mutex<UserRepository>>,
}

impl UserService {
    pub fn new(repository: Arc<Mutex<UserRepository>>) -> UserService {
        Self { repository }
    }

    pub async fn login(
        &self,
        req: Json<UserLoginRequest>,
    ) -> Result<LoginResponse, BlockingError<std::io::Error>> {

        let password = req.password.clone();

        match &self.repository.lock(){
            Ok(repository) => {

                match repository.login(req.into_inner()).await {
                    Ok(users) => {
                        if let Some(user) = users.first() {
                            // Hash the provided password and compare with the stored password
                            let hashed_password = hash_password(&password).await;
                            if is_valid(&password, &hashed_password.unwrap()).await {
                                // Passwords match, generate a JWT token (mock implementation here)
                                let token = format!("jwt_token_for_{}", user.id);
                                Ok(Token(token))
                            } else {
                                Ok(InvalidPassword)
                            }
                        } else {
                            Ok(UserNotFound)
                        }
                    }
                    Err(err) => Ok(Error(err.to_string())),
                }
            }
            Err(err) => {Ok(Error(err.to_string()))}
        }
    }

    pub async fn register(
        &self,
        req: Json<UserRegisterRequest>,
    ) -> Result<RegisterResponse, BlockingError<std::io::Error>> {
        let username = req.username.clone();
        let password = req.password.clone();
        let license_no = req.license_no.clone();
        let staff_no = req.staff_no.clone();
        let digi_signature = req.digi_signature.clone();
        let email = req.email.clone();

        // Check if user exists
        if self.user_exists(username.clone()).await {
            return Ok(UserAlreadyExists);
        }

        // Hash the password
        let hashed_password = hash_password(&password).await.unwrap();
        let user_req = UserRegisterRequest {
            username,
            email,
            password: hashed_password,
            staff_no,
            license_no,
            digi_signature,
        };

        match self.repository.lock(){

            Ok(repository) => {
                match repository.register(user_req).await {
                    Ok(_) => Ok(UserSuccessfullyRegistered),
                    Err(err) => Ok(RegisterResponse::Error(err.to_string())),
                }
            }
            Err(err) => {
                Ok(RegisterResponse::Error(err.to_string()))
            }
        }

    }

    async fn user_exists(&self, user_id: String) -> bool {
        match self.repository.lock(){
            Ok(repository) => {
                match repository.user_exists(user_id).await {
                    Ok(results) => results.len() > 0,
                    Err(_) => false,
                }
            }
            Err(_err) => {
               false
            }
        }

    }
}
