use crate::database::DbPool;
use crate::models::login::UserLoginRequest;
use crate::models::register::UserRegisterRequest;
use crate::models::User;
use diesel::result::Error;
use diesel::{sql_query, RunQueryDsl};
use ntex::http::error::BlockingError;
use ntex::web;
use std::sync::{Arc, Mutex};
use log::info;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Arc<Mutex<DbPool>>,
}

impl UserRepository {
    pub fn new(pool: Arc<Mutex<DbPool>>) -> UserRepository {
        UserRepository { pool }
    }

    pub async fn login(&self, req: UserLoginRequest) -> Result<Vec<User>, BlockingError<Error>> {
        match self.pool.lock() {
            Ok(db) => {
                let db = db.clone();
                let res = web::block(move || {
                    let mut conn = db.get().unwrap();
                    sql_query("SELECT id, password FROM users WHERE id = $1")
                        .bind::<diesel::sql_types::Integer, _>(req.user_id)
                        .load::<User>(&mut conn)
                })
                .await;
                res
            }
            Err(_) => Err(BlockingError::Canceled),
        }
    }

    pub async fn register(&self, req: UserRegisterRequest) -> Result<usize, BlockingError<Error>> {
        match self.pool.lock() {
            Ok(db) => {
                let db = db.clone();
                let res = web::block(move || {
                    let mut conn = db.get().unwrap();
                    sql_query(
                        "INSERT INTO users (username, password, email, license_no, staff_no, digi_signature)
             VALUES ($1, $2, $3, $4, $5, $6)",
                    )
                        .bind::<diesel::sql_types::Text, _>(req.username)
                        .bind::<diesel::sql_types::Text, _>(req.password)
                        .bind::<diesel::sql_types::Text, _>(req.email)
                        .bind::<diesel::sql_types::Text, _>(req.license_no)
                        .bind::<diesel::sql_types::Text, _>(req.staff_no)
                        .bind::<diesel::sql_types::Text, _>(req.digi_signature)
                        .execute(&mut conn)
                })
                    .await;
                res
            }
            Err(_) => Err(BlockingError::Canceled),
        }
    }

    pub async fn user_exists(&self, user_id: i32) -> Result<Vec<User>, BlockingError<Error>> {
        // Lock the pool and get a connection before calling `web::block`
        match self.pool.lock() {
            Ok(db) => {
                let db = db.clone();
                // Now pass the connection into the blocking task to perform the query
                let res = web::block(move || {
                    let mut conn = db.get().unwrap(); // Get a connection from the pool
                    sql_query("SELECT id, password FROM users WHERE id = $1")
                        .bind::<diesel::sql_types::Integer, _>(user_id)
                        .load::<User>(&mut conn)
                })
                .await?;

                info!("{}", res.clone().len());
                Ok(res)
            }
            Err(err) => Err(BlockingError::Canceled),
        }
    }
}
