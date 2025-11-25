// src/handler/auth_handler.rs
use crate::models::auth_model::{ActiveModel, Column, Entity, Logindto, Registerdto};
use actix_web::{HttpResponse, Responder, get, post, web};
use argon2::password_hash::SaltString;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, rand_core::OsRng},
};
use sea_orm::DatabaseConnection;
use sea_orm::entity::prelude::*;

//===============================
// Actix-web Handlers
//===============================
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/login")]
pub async fn login(
    db: web::Data<DatabaseConnection>,
    request: web::Json<Logindto>,
) -> impl Responder {
    let user = match Entity::find()
        .filter(Column::Username.eq(&request.username))
        .one(db.get_ref())
        .await
    {
        Ok(Some(existing_user)) => existing_user,
        Ok(None) => return HttpResponse::Unauthorized().body("Username not found"),
        Err(e) => {
            eprintln!("Database error: {}", e);
            return HttpResponse::InternalServerError().body("Internal server error");
        }
    };
    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Password hash parsing error: {}", e);
            return HttpResponse::InternalServerError().body("Internal server error");
        }
    };
    match Argon2::default().verify_password(request.password.as_bytes(), &parsed_hash) {
        Ok(_) => HttpResponse::Ok().body("Login successful"),
        Err(_) => HttpResponse::Unauthorized().body("Invalid password"),
    }
}

#[post("/register")]
pub async fn register(
    db: web::Data<DatabaseConnection>,
    request: web::Json<Registerdto>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(request.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            eprintln!("Password hashing error: {}", e);
            return HttpResponse::InternalServerError().body("Internal server error");
        }
    };

    let form_data = request.into_inner();

    let new_user: ActiveModel = (form_data, password_hash).into();

    match Entity::insert(new_user).exec(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
