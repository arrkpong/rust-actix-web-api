// src/models/auth_model.rs
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DeriveEntityModel, DeriveRelation, EnumIter, Set,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

//===============================
// ORM Entity Definition
//===============================
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "auth_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[serde(skip)]
    pub password: String,
    #[sea_orm(unique)]
    pub email: String,
    pub phone: String,
    #[sea_orm(default_value = true)]
    pub active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

//===============================
// Relations
//===============================
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

//===============================
// Active Model Behavior
//===============================
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.created_at = Set(chrono::Utc::now().into());
            self.active = Set(true);
        }
        self.updated_at = Set(chrono::Utc::now().into());
        Ok(self)
    }
}

//================================
// Data Transfer Objects (DTOs)
//================================
#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(
        min = 3,
        max = 30,
        message = "Username must be between 3 and 30 characters"
    ))]
    pub username: String,
    #[validate(custom(function = "validate_password"))]
    pub password: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,
}

//===============================
// From Trait Implementation
//===============================
impl From<(RegisterRequest, String)> for ActiveModel {
    fn from((data, hashed_password): (RegisterRequest, String)) -> Self {
        Self {
            username: Set(data.username),
            password: Set(hashed_password),
            email: Set(data.email),
            phone: Set(data.phone),
            ..Default::default()
        }
    }
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let long_enough = password.chars().count() >= 8;

    if has_upper && has_lower && has_digit && long_enough {
        Ok(())
    } else {
        let mut err = ValidationError::new("password_complexity");
        err.message =
            Some("Password must verify at least 1 Uppercase, 1 Lowercase and 1 Number".into());
        Err(err)
    }
}

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let len = phone.chars().count();
    let all_digits = phone.chars().all(|c| c.is_ascii_digit());

    if all_digits && (10..=15).contains(&len) {
        Ok(())
    } else {
        let mut err = ValidationError::new("phone_format");
        err.message = Some("Phone number must be digits only".into());
        Err(err)
    }
}
