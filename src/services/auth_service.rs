// // src/services/auth_service.rs
//
// use sea_orm::DatabaseConnection;
// use crate::handlers::auth_handler::login;
// use crate::models::auth_model::{Registerdto, Logindto};
//
// pub struct AuthService;
//
// impl AuthService {
//     pub async fn login(db: &DatabaseConnection, dto:Logindto) -> Result<String, String> {
//         // Call the login handler logic here
//         // For simplicity, we will just return a success message
//         Ok("Login successful".to_string())
//     }
//     pub async fn register(db: &DatabaseConnection, dto:Registerdto) -> Result<String, String> {
//         // Call the register handler logic here
//         // For simplicity, we will just return a success message
//         Ok("Registration successful".to_string())
//     }
// }
