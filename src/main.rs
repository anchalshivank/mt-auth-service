use std::net::SocketAddr;
use ntex::http::RequestHead;
use ntex::web;
use ntex::web::{App, HttpResponse, HttpServer};
use ntex::web::middleware::Logger;
use ntex::web::types::Json;
use serde::{Deserialize, Serialize};

trait Greet {
    fn greet(&self) {
        println!("Hello, there!");  // Default implementation
    }
}

struct Person;

impl Greet for Person {}


#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

#[web::post("/login")]
async fn login(req: Json<User>) -> impl web::Responder {
    HttpResponse::Ok().body("Hello world login!")
}

#[web::post("/register")]
async fn register() -> impl web::Responder {
    HttpResponse::Ok().body("register!")
}

#[web::get("/health_check")]
async fn health_check() -> impl web::Responder {

    HttpResponse::Ok().body("health check!")

}

#[ntex::main]
async fn main() -> std::io::Result<()> {

    let person = Person;
    person.greet();
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    HttpServer::new(||{
        let logger  = Logger::default();

        App::new()
            .service(health_check)
            .service(login)
            .service(register)
            .wrap(logger)
        }
    ).bind(addr)?.run().await

}
