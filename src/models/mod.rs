use diesel::{QueryableByName, Selectable};
use serde::Deserialize;

pub mod login;
pub mod machine;
pub mod notify_machine;
pub mod register;

#[derive(QueryableByName, Deserialize)]
pub struct User {
    #[diesel(sql_type = diesel::sql_types::Integer)] // Correct type for integer ID
    pub(crate) id: i32, // Use `i32` for Postgres integer type
    #[diesel(sql_type = diesel::sql_types::Text)] // Correct type for text in Diesel
    pub(crate) password: String,
}

#[derive(QueryableByName)]
pub struct CountQueryResult {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub(crate) count: i64,
}
