// src/routes/auth_route.rs
use crate::handlers::auth_handler::{index, login, register};
use actix_web::web;
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(web::scope("/api/v1/auth").service(register).service(login));
}
