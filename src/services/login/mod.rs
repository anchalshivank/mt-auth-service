// use crate::database::DbPool;
// use crate::models::login::request::UserLoginRequest;
// use crate::models::User;
// use crate::utils::crypt::{hash_password, is_valid};
// use diesel::{sql_query, RunQueryDsl};
// use ntex::web;
// use ntex::web::types::Json;
// use ntex::web::HttpResponse;
// use crate::repositories;
//
// pub async fn login(
//     db: web::types::State<DbPool>,
//     req: Json<UserLoginRequest>,
// ) -> impl web::Responder {
//     let password = req.password.clone();
//     match repositories::login::login(db, req.into_inner()).await {
//         Ok(users) => {
//             if let Some(user) = users.first() {
//                 // Hash the provided password and compare with the stored password
//                 let hashed_password = hash_password(&password).await;
//                 if is_valid(&password, &hashed_password.unwrap()).await {
//                     // Passwords match, generate a JWT token (mock implementation here)
//                     let token = format!("jwt_token_for_{}", user.id);
//                     HttpResponse::Ok().body(token)
//                 } else {
//                     HttpResponse::Unauthorized().body("Invalid password")
//                 }
//             } else {
//                 HttpResponse::Unauthorized().body("User not found")
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
//     }
// }
