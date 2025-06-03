use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // Don't send password hash to client
    pub password_hash: String,
    pub name: String,
    #[sqlx(rename = "role")] // Map enum from database
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "ENUM", rename_all = "lowercase")] // For ENUM mapping
pub enum UserRole {
    User,
    Admin,
}

#[derive(Debug, Deserialize)]
#[derive(Debug, Deserialize)]pub struct LoginPayload {    pub username: String,    pub password: String,}
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: String,
}

// For sending user data back, without password hash
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            name: user.name,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
