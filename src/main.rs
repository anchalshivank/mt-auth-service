mod database;
pub mod models;
mod services;
mod utils;

mod repositories;

mod controllers;

use crate::controllers::{handle_login, handle_register, health};
use crate::repositories::UserRepository;
use crate::services::UserService;
use dotenv::dotenv;
use log::info;
use ntex::web::middleware::Logger;
use ntex::web::{App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();
    info!("Logger initialized!");

    let pool = Arc::new(Mutex::new(database::establish_connection()));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let user_repository = Arc::new(Mutex::new(UserRepository::new(pool.clone())));
    let user_service = Arc::new(Mutex::new(UserService::new(user_repository)));

    HttpServer::new(move || {
        App::new()
            .state(user_service.clone())
            .service(health)
            .service(handle_login)
            .service(handle_register)
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}
