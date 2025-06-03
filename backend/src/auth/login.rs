use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
// use sqlx::MySqlPool; // Not directly used, AppState has it
use std::sync::Arc;
use axum::extract::State;
use chrono::{Utc, Duration}; // Corrected: Duration was from chrono, not std::time

use crate::models::{LoginPayload, User, UserResponse}; // UserRole removed as it's part of User
use crate::AppState; // Correctly using AppState from main via crate root

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String, // User ID
    role: String, // User role as string
    exp: usize,  // Expiration timestamp (seconds since epoch)
    iat: usize,  // Issued at timestamp
}

// Placeholder for JWT secret - this should come from AppState or config via .env
// For now, using a fixed string for structure. In main.rs, this would be loaded from .env
// and ideally stored in AppState.
// const JWT_SECRET_KEY: &str = "your-super-secret-jwt-key-please-change-me"; // DO NOT USE IN PRODUCTION

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    if payload.username.is_empty() || payload.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "error", "message": "Username and password are required"})),
        )
            .into_response();
    }

    // Fetch user from database
    let user_result = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password_hash, name, role, is_active, created_at, updated_at FROM users WHERE username = ?",
    )
    .bind(&payload.username)
    .fetch_optional(&app_state.db_pool) // app_state.db_pool is correct
    .await;

    let user = match user_result {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"status": "error", "message": "Invalid username or password"})),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to fetch user: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": "Database error"})),
            )
                .into_response();
        }
    };

    if !user.is_active {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"status": "error", "message": "Account is inactive"})),
        )
            .into_response();
    }

    // Verify password
    let valid_password = match verify(&payload.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(_) => {
            // Log this as a server error, but return generic auth error to client
            tracing::error!("Error during password verification for user: {}", user.username);
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"status": "error", "message": "Invalid username or password"})),
            )
                .into_response();
        }
    };

    if !valid_password {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"status": "error", "message": "Invalid username or password"})),
        )
            .into_response();
    }

    // Generate JWT
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::hours(24)).timestamp() as usize; // Token valid for 24 hours

    let claims = Claims {
        sub: user.id.to_string(),
        role: user.role.to_string().to_lowercase(),
        exp,
        iat,
    };

    // Get JWT_SECRET from environment (via AppState ideally, or directly for now)
    // This should be loaded into AppState in main.rs and accessed here.
    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            tracing::error!("JWT_SECRET not set in environment");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": "Internal server configuration error"})),
            ).into_response();
        }
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref())) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to create JWT: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": "Failed to create authentication token"})),
            )
                .into_response();
        }
    };

    let user_response = UserResponse::from(user);

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "data": {
                "token": token,
                "user": user_response
            }
        })),
    )
        .into_response()
}
