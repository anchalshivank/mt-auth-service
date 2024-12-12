use diesel::QueryableByName;
use serde::Deserialize;

pub mod login;
pub mod register;
pub mod notify_machine;

#[derive(QueryableByName, Deserialize)]
pub struct User {
    #[diesel(sql_type = diesel::sql_types::Integer)] // Correct type for integer ID
    pub(crate) id: i32, // Use `i32` for Postgres integer type
    #[diesel(sql_type = diesel::sql_types::Text)] // Correct type for text in Diesel
    pub(crate) password: String,
}
