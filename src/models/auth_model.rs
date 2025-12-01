// src/models/auth_model.rs
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DeriveEntityModel, DeriveRelation, EnumIter, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

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
    pub created_at: DateTime,
    pub updated_at: DateTime,
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
            self.created_at = Set(chrono::Utc::now().naive_utc());
            self.active = Set(true);
        }
        self.updated_at = Set(chrono::Utc::now().naive_utc());
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
    #[validate(length(
        min = 8,
        max = 100,
        message = "Password must be between 8 and 100 characters"
    ))]
    pub password: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(
        min = 10,
        max = 15,
        message = "Phone number must be between 10 and 15 digits"
    ))]
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
