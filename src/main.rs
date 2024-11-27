mod database;
pub mod models;
mod services;
mod utils;

mod repositories;

use crate::repositories::UserRepository;
use crate::services::UserService;
use auth_service::controllers::{handle_login, handle_register};
use diesel::connection::SimpleConnection;
use diesel::r2d2::R2D2Connection;
use diesel::row::NamedRow;
use diesel::{Connection, Insertable, IntoSql, Queryable, QueryableByName, RunQueryDsl};
use dotenv::dotenv;
use ntex::web::middleware::Logger;
use ntex::web::{App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use ntex::web;
use ntex::web::types::State;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = Arc::new(Mutex::new(database::establish_connection()));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));

    let user_repository = Arc::new(Mutex::new(UserRepository::new(pool.clone())));
    let user_service = UserService::new(user_repository);

    HttpServer::new(move || {
        App::new()
            .state(user_service.clone())
            .service(handle_login)
            .service(handle_register)
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}
