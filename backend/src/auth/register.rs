use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::json;
use sqlx::MySqlPool; // Assuming MySql, adjust if different
use uuid::Uuid;
use crate::models::{CreateUserPayload, User, UserResponse, UserRole}; // Adjust path as needed
use axum::extract::State; // For accessing shared state like db pool
use std::sync::Arc; // For sharing state

// Define an AppState struct if you don't have one yet
// This would typically be in main.rs or a state.rs module
// For now, defining a simple one here for compilation.
// In a real app, this would be properly defined and initialized in main.
// This AppState is now defined in main.rs, so this local one can be removed
// pub struct AppState {
//     pub db_pool: MySqlPool,
// }
use crate::AppState;


pub async fn register(
    State(app_state): State<Arc<AppState>>, // Access the shared state
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    // Validate input (basic example, add more robust validation)
    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() || payload.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "error", "message": "Missing required fields"})),
        )
            .into_response();
    }

    // Hash the password
    let hashed_password = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": "Failed to hash password"})),
            )
                .into_response();
        }
    };

    let user_id = Uuid::new_v4();

    // Insert user into database
    // Note: sqlx::query! or query_as! macros are preferred for compile-time checks,
    // but require DATABASE_URL at compile time. Using query() for broader compatibility in this subtask.
    let insert_result = sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, name, role, is_active) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(user_id.to_string()) // Store UUID as string
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&hashed_password)
    .bind(&payload.name)
    .bind(UserRole::User.to_string().to_lowercase()) // Store enum as string
    .bind(true) // is_active
    .execute(&app_state.db_pool)
    .await;

    match insert_result {
        Ok(_) => {
            // Retrieve the created user to return its details (optional, or construct from input)
            // For simplicity, we'll construct a UserResponse from the payload and generated ID.
            // In a real app, you might want to fetch the user from DB to get created_at, etc.
            let new_user = User {
                id: user_id,
                username: payload.username,
                email: payload.email,
                password_hash: hashed_password, // Not sent in response
                name: payload.name,
                role: UserRole::User,
                is_active: true,
                created_at: chrono::Utc::now(), // Approximate
                updated_at: chrono::Utc::now(), // Approximate
            };

            (StatusCode::CREATED, Json(json!({"status": "success", "data": UserResponse::from(new_user)}))).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.message().contains("Duplicate entry") => {
            // This is a basic way to check for duplicate username/email.
            // More specific error codes from MySQL could be checked (e.g., 1062 for ER_DUP_ENTRY)
            (
                StatusCode::CONFLICT,
                Json(json!({"status": "error", "message": "Username or email already exists"})),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to insert user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": "Failed to register user"})),
            )
                .into_response()
        }
    }
}
