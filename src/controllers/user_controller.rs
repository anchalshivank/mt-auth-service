use std::sync::{Arc, Mutex};
use log::info;
use ntex::web;
use ntex::web::{HttpResponse, Responder};
use ntex::web::types::Json;
use crate::models::
{   register::{RegisterErrorData, RegisterResponse, UserRegisterRequest},
    login::{LoginErrorData, LoginResponse, LoginSuccessData, UserLoginRequest}
};
use crate::services::Services;
use crate::controllers::response::ApiResponse;


#[web::get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[web::post("/login")]
pub async fn handle_login(
    services: web::types::State<Arc<Mutex<Services>>>,
    req: Json<UserLoginRequest>,
) -> impl Responder {
    let services = services.lock().unwrap();
    let service = services.user_service.lock().unwrap();

    match service.login(req).await.unwrap() {
        LoginResponse::Token(token) => {
            let data = LoginSuccessData { token };
            // Explicitly specify both the success data type and the error type
            HttpResponse::Ok().json(&ApiResponse::<LoginSuccessData, ()>::success("Login successful", Some(data)))
        }
        LoginResponse::InvalidPassword => {
            let error_data = LoginErrorData {
                code: "INVALID_PASSWORD".to_string(),
                message: "The provided password is incorrect.".to_string(),
            };
            HttpResponse::Unauthorized().json(&ApiResponse::<(), LoginErrorData>::error("Login failed", error_data))
        }
        LoginResponse::UserNotFound => {
            let error_data = LoginErrorData {
                code: "USER_NOT_FOUND".to_string(),
                message: "The user does not exist.".to_string(),
            };
            HttpResponse::NotFound().json(&ApiResponse::<(), LoginErrorData>::error("Login failed", error_data))
        }
        LoginResponse::Error(err) => {
            let error_data = LoginErrorData {
                code: "INTERNAL_ERROR".to_string(),
                message: format!("An unexpected error occurred: {}", err),
            };
            HttpResponse::InternalServerError().json(&ApiResponse::<(), LoginErrorData>::error("Login failed", error_data))
        }
    }
}

#[web::post("/register")]
pub async fn handle_register(
    services: web::types::State<Arc<Mutex<Services>>>,
    req: Json<UserRegisterRequest>,
) -> impl Responder {
    info!("Registering user");

    let services = services.lock().unwrap();
    let service = services.user_service.lock().unwrap();

    match service.register(req).await.unwrap() {
        RegisterResponse::UserAlreadyExists => {
            info!("User already exists");
            let error_data = RegisterErrorData {
                code: "USER_ALREADY_EXISTS".to_string(),
                message: "The username or email already exists.".to_string(),
            };
            HttpResponse::Conflict().json(&ApiResponse::<(), RegisterErrorData>::error("Registration failed", error_data))
        }
        RegisterResponse::UserSuccessfullyRegistered => {
            info!("User registered successfully");
            HttpResponse::Ok().json(&ApiResponse::<(), ()>::success("User successfully registered", None))
        }
        RegisterResponse::Error(err) => {
            info!("Error: {}", err);
            let error_data = RegisterErrorData {
                code: "INTERNAL_ERROR".to_string(),
                message: format!("An unexpected error occurred: {}", err),
            };
            HttpResponse::InternalServerError().json(&ApiResponse::<(), RegisterErrorData>::error("Registration failed", error_data))
        }
    }
}
