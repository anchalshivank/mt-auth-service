use std::sync::{Arc, Mutex};
use crate::models::login::request::{LoginResponse, UserLoginRequest};
use crate::models::register::request::UserRegisterRequest;
use crate::models::register::response::RegisterResponse;
use crate::services::UserService;
use ntex::web::types::Json;
use ntex::web::{self, HttpResponse, Responder};

pub struct UserController {
    service: UserService,
}

impl UserController {
    pub fn new(service: UserService) -> Self {
        UserController { service }
    }
}

#[web::post("/login")]
pub async fn handle_login(
    service: web::types::State<Arc<Mutex<UserService>>>,
    req: Json<UserLoginRequest>,
) -> impl Responder {

    let service = service.lock().unwrap();

    match service.login(req).await.unwrap() {
        LoginResponse::Token(token) => {
            // Return the token as a response
            HttpResponse::Ok().json(&token)
        }
        LoginResponse::InvalidPassword => {
            // Respond with unauthorized if the password is incorrect
            HttpResponse::Unauthorized().body("Invalid password")
        }
        LoginResponse::UserNotFound => {
            // Respond with not found if the user does not exist
            HttpResponse::NotFound().body("User not found")
        }
        LoginResponse::Error(err) => {
            // Handle errors (internal server error or unknown errors)
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        }
    }
}

#[web::post("/register")]
pub async fn handle_register(
    service: web::types::State<Arc<Mutex<UserService>>>,
    req: Json<UserRegisterRequest>,
) -> impl Responder {
    let service = service.lock().unwrap();
    // Pass request to service layer
    match service.register(req).await.unwrap() {
        RegisterResponse::UserAlreadyExists => {
            // Respond with conflict if the user already exists
            HttpResponse::Conflict().body("User already exists")
        }
        RegisterResponse::UserSuccessfullyRegistered => {
            // Respond with success if registration is successful
            HttpResponse::Ok().body("User successfully registered")
        }
        RegisterResponse::Error(err) => {
            // Handle errors (internal server error or unknown errors)
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        }
    }
}
