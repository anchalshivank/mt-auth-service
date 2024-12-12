mod response;

use crate::controllers::response::ApiResponse;
use crate::models::{
    login::{LoginErrorData, LoginResponse, LoginSuccessData, UserLoginRequest},
    register::{RegisterErrorData, RegisterResponse, UserRegisterRequest},
    notify_machine::{NotifyMachineRequest, NotifyMachineResponse, NotifyMachineErrorData}
};
use crate::services::{notify_service::NotifyService, user_service::UserService, Services};
use log::info;
use ntex::web::types::Json;
use ntex::web::{self, service, HttpResponse, Responder};
use serde::Serialize;
use std::sync::{Arc, Mutex};

pub struct UserController {
    services: Services,
}

impl UserController {
    pub fn new(services: Services) -> Self {
        UserController { services }
    }
}


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

#[web::post("/auth-qr")]
pub async fn handle_qr_code(
    services: web::types::State<Arc<Mutex<Services>>>,
    req: Json<NotifyMachineRequest>,
) -> impl Responder {
    info!("Handling QR code authentication");

    let services = services.lock().unwrap();
    let service = services.notify_service.lock().unwrap();

    // Attempt to notify the machine
    match service.notify_machine(req.clone()).await {
        Ok(NotifyMachineResponse::Success) => {
            info!("Machine notification successful");
            HttpResponse::Ok().json(&ApiResponse::<(), ()>::success(
                "Notification sent successfully",
                None,
            ))
        }
        Ok(NotifyMachineResponse::Failure) => {
            let error_data = NotifyMachineErrorData {
                code: "NOTIFICATION_FAILED".to_string(),
                message: "Notification could not be delivered to the machine.".to_string(),
            };
            HttpResponse::BadRequest().json(&ApiResponse::<(), NotifyMachineErrorData>::error(
                "Notification failed",
                error_data,
            ))
        }
        Ok(NotifyMachineResponse::Error(err)) => {
            let error_data = NotifyMachineErrorData {
                code: "NOTIFICATION_ERROR".to_string(),
                message: format!("An unexpected error occurred: {}", err),
            };
            HttpResponse::InternalServerError().json(&ApiResponse::<(), NotifyMachineErrorData>::error(
                "Notification failed",
                error_data,
            ))
        }
        Err(err) => {
            let error_data = NotifyMachineErrorData {
                code: "NOTIFICATION_EXCEPTION".to_string(),
                message: format!("An exception occurred: {}", err),
            };
            HttpResponse::InternalServerError().json(&ApiResponse::<(), NotifyMachineErrorData>::error(
                "Notification failed",
                error_data,
            ))
        }
    }
}
