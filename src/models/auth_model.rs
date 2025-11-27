// src/models/auth_model.rs
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, Set};
use serde::{Deserialize, Serialize};

//===============================
// ORM Entity Definition
//===============================
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "auth_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//================================
// Data Transfer Objects (DTOs)
//================================
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
}

//===============================
// From Trait Implementation
//===============================
impl From<(RegisterRequest, String)> for ActiveModel {
    fn from((data, hashed_password): (RegisterRequest, String)) -> Self {
        let time_now = chrono::Utc::now().naive_utc();
        Self {
            username: Set(data.username),
            password: Set(hashed_password),
            email: Set(data.email),
            phone: Set(data.phone),
            active: Set(true),
            created_at: Set(time_now),
            updated_at: Set(time_now),
            ..Default::default()
        }
    }
}
