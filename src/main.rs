mod database;
pub mod models;
mod services;
mod utils;

mod repositories;

mod controllers;

use crate::controllers::{handle_login, handle_register, health, handle_qr_code};
use crate::repositories::UserRepository;
use crate::services::{
    Services,
    user_service::UserService,
    notify_service::NotifyService
};
use dotenv::dotenv;
use log::info;
use ntex::web::middleware::Logger;
use ntex::web::{App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();
    info!("Logger initialized!");
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().expect("PORT must be a number!");
    let pool = Arc::new(Mutex::new(database::establish_connection()));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let notify_service_addr = std::env::var("NOTIFY_SERVICE_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".to_string());
    let notify_service_addr = SocketAddr::from_str(notify_service_addr.as_str()).expect("Invalid NOTIFY_SERVICE_ADDR");
    let user_repository = Arc::new(Mutex::new(UserRepository::new(pool.clone())));
    let user_service = Arc::new(Mutex::new(UserService::new(user_repository)));
    let notify_service = Arc::new(Mutex::new(NotifyService::new(notify_service_addr)));

    let services = Arc::new(Mutex::new(Services {
        user_service,
        notify_service,
    }));

    info!("Starting server on addr {}", addr);
    HttpServer::new(move || {
        App::new()
            .state(services.clone())
            .service(health)
            .service(handle_login)
            .service(handle_register)
            .service(handle_qr_code)
            .wrap(Logger::default())
    })
        .bind(addr)?
        .run()
        .await
}
