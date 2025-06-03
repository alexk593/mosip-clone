use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

// Logout is primarily a client-side action (discarding the token).
// This endpoint is provided for completeness and future extension (e.g., token blocklist).
// For MVP, it doesn't need to do much on the server side with JWTs if not using a blocklist.
pub async fn logout() -> impl IntoResponse {
    // In a stateful session system, you would invalidate the session here.
    // With JWTs, if you had a refresh token system or a blocklist, you'd handle that.
    // For this MVP, we assume the client handles token removal.
    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Logged out successfully. Please discard your token."
        })),
    )
}
