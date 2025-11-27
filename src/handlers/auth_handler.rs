// src/handler/auth_handler.rs
use crate::models::auth_model::{ActiveModel, Column, Entity, LoginRequest, RegisterRequest};
use actix_web::{HttpResponse, Responder, get, post, web, HttpRequest};
use argon2::password_hash::SaltString;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, rand_core::OsRng},
};
use log::*;
use sea_orm::DatabaseConnection;
use sea_orm::entity::prelude::*;
use sea_orm::Condition;

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
    req: HttpRequest,
    form: web::Json<LoginRequest>,

) -> impl Responder {
    let client_ip = req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();
    let user = match Entity::find()
        .filter(Column::Username.eq(&form.username))
        .filter(Column::Active.eq(true))
        .one(db.get_ref())
        .await {
        Ok(Some(res)) => {
            debug!("User found: {}", res.username);
            res
        }
        Ok(None) => {
            warn!("Login failed: user {} not found from IP {}", form.username, client_ip);
            return HttpResponse::Unauthorized().body("invalid credentials");
        }
        Err(e) => {
            error!("Database error: {}", e);
            return HttpResponse::InternalServerError().body("Internal server error");
        }
    };
    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(hash) => hash,
        Err(e) => {
            error!("Password hash parsing error: {}", e);
            return HttpResponse::InternalServerError().body("Internal server error");
        }
    };
    match Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash) {
        Ok(()) => {
            info!("User {} logged in successfully from IP {}", form.username, client_ip);
            HttpResponse::Ok().body("Login successful")
        }
        Err(_e) => {
            warn!("Login failed: invalid password for user {} from IP {}", form.username, client_ip);
            HttpResponse::Unauthorized().body("invalid credentials")
        }
    }
}

#[post("/register")]
pub async fn register(
    db: web::Data<DatabaseConnection>,
    form: web::Json<RegisterRequest>,
) -> impl Responder {
    let user = Entity::find()
        .filter(Condition::any()
            .add(Column::Username.eq(&form.username))
            .add(Column::Email.eq(&form.email))
        )
        .one(db.get_ref())
        .await;
    match user {
        Ok(Some(res)) => {
            if res.username == form.username {
                warn!("Registration failed: username {} already exists", form.username);
                return HttpResponse::BadRequest().body("username already exists");
            }
            if res.email == form.email {
                warn!("Registration failed: email {} already exists", form.email);
                return HttpResponse::BadRequest().body("email already exists");
            }
            else {
                warn!("Registration failed: user with username {} or email {} already exists", form.username, form.email);
                return HttpResponse::BadRequest().body("username or email already exists");
            }},
            Ok(None) => (),
            Err(e) => {
                error!("Database error: {}", e);
                return HttpResponse::InternalServerError().body("Internal server error");
            }
        };

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = match argon2.hash_password(form.password.as_bytes(), &salt) {
            Ok(hash) => hash.to_string(),
            Err(e) => {
                error!("Password hashing error: {}", e);
                return HttpResponse::InternalServerError().body("Internal server error");
            }
        };

        let form_data = form.into_inner();
        let create_user: ActiveModel = (form_data, password_hash).into();

        match Entity::insert(create_user).exec(db.get_ref()).await {
            Ok(res) => {
            info!("New user registered with ID: {}", res.last_insert_id);
            HttpResponse::Ok().body("User registered successfully")
            },
            Err(e) => {
            error!("Database insertion error: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
            }
        }
    }
