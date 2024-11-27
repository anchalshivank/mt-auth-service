mod schema;
use diesel::connection::SimpleConnection;
use diesel::r2d2::{ConnectionManager, R2D2Connection};
use diesel::{Connection, Insertable, IntoSql, PgConnection, Queryable};
use dotenv::dotenv;
use ntex::web;
use ntex::web::middleware::Logger;
use ntex::web::types::Json;
use ntex::web::{App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create database pool")
}

#[web::post("/login")]
async fn login(req: Json<User>) -> impl web::Responder {
    HttpResponse::Ok().body("login")
}

#[web::post("/register")]
async fn register(db: web::types::State<DbPool>, req: Json<User>) -> impl web::Responder {
    // let conn = db.get().expect("Failed to get DB connection");

    // Create a dummy user (in a real scenario, use `req` to get the data)
    let username = req.username.clone();
    let password = req.password.clone();

    // Raw SQL to insert the user into the users table
    let query = format!(
        "INSERT INTO users (username, password) VALUES ('{}', '{}') RETURNING id, username, password",
        username,
        password
    );

    // Execute the raw SQL query
    // conn.execute_returning_count(query).unwrap();

    let db = db.get_ref().clone();
    let res = web::block(move || {
        let mut conn = db.get().unwrap();
        conn.batch_execute(&query)
    })
        .await;
    match res {
        Ok(_) => HttpResponse::Ok().body("registerd user"),
        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err))
    }

}

#[web::get("/health_check")]
async fn health_check() -> impl web::Responder {
    HttpResponse::Ok().body("Service is healthy!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = establish_connection();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));

    HttpServer::new(move || {
        App::new()
            .state(pool.clone())
            .service(health_check)
            .service(login)
            .service(register)
            .wrap(Logger::default())
    })
        .bind(addr)?
        .run()
        .await
}
