// src/utils/auth_middleware.rs
use crate::utils::jwt::decode_jwt;
use actix_web::{
    Error, FromRequest, HttpRequest, dev::Payload, error::ErrorInternalServerError,
    error::ErrorUnauthorized, web,
};
use futures::future::LocalBoxFuture;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .map(|header| header.to_owned());
        let redis = req
            .app_data::<web::Data<ConnectionManager>>()
            .cloned();

        Box::pin(async move {
            let auth_header = match auth_header {
                Some(header) => header,
                None => return Err(ErrorUnauthorized("Authorization header missing")),
            };

            let auth_str = match auth_header.to_str() {
                Ok(str) => str,
                Err(_) => return Err(ErrorUnauthorized("Invalid Authorization header")),
            };

            if !auth_str.starts_with("Bearer ") {
                return Err(ErrorUnauthorized("Invalid Authorization scheme"));
            }

            let token = &auth_str[7..]; // Skip "Bearer "
            let claims = match decode_jwt(token) {
                Ok(claims) => claims,
                Err(_) => return Err(ErrorUnauthorized("Invalid or expired token")),
            };

            let redis = match redis {
                Some(redis) => redis,
                None => return Err(ErrorInternalServerError("Redis not configured")),
            };

            let mut conn = redis.get_ref().clone();
            let blacklist_key = format!("bl:{}", token);
            let is_blacklisted: bool = match conn.exists(blacklist_key).await {
                Ok(res) => res,
                Err(_) => return Err(ErrorInternalServerError("Redis query failed")),
            };

            if is_blacklisted {
                return Err(ErrorUnauthorized("Token revoked"));
            }

            Ok(AuthenticatedUser {
                username: claims.sub,
            })
        })
    }
}
