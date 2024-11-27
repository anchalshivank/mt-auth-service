// use crate::database::DbPool;
// use crate::models::register::request::UserRegisterRequest;
// use crate::repositories;
// use crate::utils::crypt::hash_password;
// use ntex::web;
// use ntex::web::types::Json;
// use ntex::web::{HttpResponse, Responder};
// pub async fn register(
//     db: web::types::State<DbPool>,
//     req: Json<UserRegisterRequest>,
// ) -> impl Responder {
//     let username = req.username.clone();
//     let password = req.password.clone();
//     let license_no = req.license_no.clone();
//     let staff_no = req.staff_no.clone();
//     let digi_signature = req.digi_signature.clone();
//     let email = req.email.clone();
//
//     // Check if user exists
//     if user_exists(username.clone(), db.clone()).await {
//         return HttpResponse::Conflict().body("User already exists");
//     }
//
//     // Hash the password
//     let hashed_password = hash_password(&password).await.unwrap();
//     let user_req = UserRegisterRequest {
//         username,
//         email,
//         password: hashed_password,
//         staff_no,
//         license_no,
//         digi_signature,
//     };
//
//     match repositories::register::register(user_req, db).await {
//         Ok(_) => HttpResponse::Ok().body("User registered successfully"),
//         Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {:?}", err)),
//     }
// }
//
// async fn user_exists(user_id: String, db: web::types::State<DbPool>) -> bool {
//     match repositories::register::user_exists(user_id, db).await {
//         Ok(results) => results.len() > 0,
//         Err(_) => false,
//     }
// }
