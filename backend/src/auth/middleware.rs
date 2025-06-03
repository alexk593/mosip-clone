use axum::{
    extract::State, // State is not used in this version of middleware, but kept for future
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid; // Ensure Uuid is imported

// Re-use Claims struct, assuming it's accessible (e.g., moved to models.rs or auth/mod.rs)
// For now, duplicating here for subtask atomicity.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID (usually UUID)
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

// This struct will be inserted into request extensions
#[derive(Clone, Debug)] // Added Debug for easier inspection
pub struct AuthenticatedUser {
    pub user_id: Uuid, // Store as Uuid type
    pub role: String, // or a UserRole enum if preferred
}

// Placeholder for JWT secret - this should come from AppState or config via .env
// For now, using a fixed string for structure.
// const JWT_SECRET_KEY: &str = "your-super-secret-jwt-key-please-change-me"; // DO NOT USE IN PRODUCTION

pub async fn auth_middleware<B>(
    // If JWT_SECRET comes from AppState, you'd uncomment the State extractor
    // State(_app_state): State<Arc<crate::AppState>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            let token_str = &auth_header[7..]; // Skip "Bearer "

            // Get JWT_SECRET from environment. Ideally, this comes from AppState.
            let jwt_secret = match std::env::var("JWT_SECRET") {
                Ok(secret) => secret,
                Err(_) => {
                    tracing::error!("JWT_SECRET not set in environment for middleware");
                    // Not returning specific error to client here for security, just falling through
                    // to deny access if secret isn't configured on server.
                    return Err(StatusCode::UNAUTHORIZED);
                }
            };

            let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
            let mut validation = Validation::new(Algorithm::HS256); // Ensure this matches encoding algorithm
            // validation.validate_exp = true; // Default
            // validation.leeway = 60; // Add some leeway if needed for clock skew

            match decode::<Claims>(token_str, &decoding_key, &validation) {
                Ok(token_data) => {
                    let claims = token_data.claims;
                    match Uuid::parse_str(&claims.sub) {
                        Ok(user_id_uuid) => {
                            let authenticated_user = AuthenticatedUser {
                                user_id: user_id_uuid,
                                role: claims.role.clone(),
                            };
                            request.extensions_mut().insert(authenticated_user);
                            return Ok(next.run(request).await);
                        }
                        Err(_) => {
                            tracing::warn!("Invalid UUID format in token sub field: {}", claims.sub);
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("JWT validation error: {}. Token: '{}'", e, token_str);
                }
            }
        }
    }

    // Using a custom response structure for 401 to match other error responses
    let error_body = Json(json!({
        "status": "error",
        "message": "Unauthorized: Authentication required or token invalid."
    }));
    let mut response = (StatusCode::UNAUTHORIZED, error_body).into_response();
    response.headers_mut().insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    Err(StatusCode::UNAUTHORIZED) // Axum will use this status, but the response body won't be from here directly.
                                  // A more Axum-idiomatic way is to return a full Response for errors.
                                  // For now, this structure is kept simple. A proper error response should be built.
                                  // However, the default Axum behavior for Err(StatusCode) is often sufficient.
                                  // Let's simplify and let Axum handle the response body for Err(StatusCode).
}

// Example protected handler (for testing middleware setup in main.rs)
pub async fn protected_route_example(
    extensions: axum::extract::Extension<AuthenticatedUser>
) -> impl axum::response::IntoResponse {
    let AuthenticatedUser { user_id, role } = extensions.0;
    Json(json!({
        "status": "success",
        "message": "You are accessing a protected route!",
        "user_id": user_id.to_string(),
        "role": role
    }))
}
