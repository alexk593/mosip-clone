use axum::{routing::{get, post}, Router, Extension};
use axum::middleware;
use std::net::SocketAddr;
use std::sync::Arc;
use sqlx::mysql::MySqlPoolOptions; // Specific for MySQL
use dotenvy::dotenv;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import modules
mod auth;
mod models;

// Use the handlers
use auth::register;
use auth::login;
use auth::logout;
use auth::{auth_middleware, protected_route_example};

// Define AppState
// This needs to be public for other modules to use it via State<Arc<AppState>>
pub struct AppState {
    db_pool: sqlx::MySqlPool,
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Initialize tracing (logging)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug,tower_http=debug,sqlx=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("Successfully connected to the database");
            pool
        }
        Err(err) => {
            tracing::error!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Create shared AppState
    let app_state = Arc::new(AppState { db_pool });

    // Define the application routes
    let app = Router::new()
        .route("/health", get(health_check))
pi/v1/auth/logout, post(logout))/a         .route(/api/v1/protected, get(protected_route_example).route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware)))
pi/v1/auth/login, post(login))/a         .route(/api/v1/auth/logout, post(logout))
pi/v1/auth/logout, post(logout))/a         .route(/api/v1/protected, get(protected_route_example).route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware)))
pi/v1/auth/register, post(register))/a         .route(/api/v1/auth/login, post(login))
pi/v1/auth/logout, post(logout))/a         .route(/api/v1/protected, get(protected_route_example).route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware)))
pi/v1/auth/login, post(login))/a         .route(/api/v1/auth/logout, post(logout))
pi/v1/auth/logout, post(logout))/a         .route(/api/v1/protected, get(protected_route_example).route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware)))
        .route("/api/v1/auth/register", post(register)) // Add register route
pi/v1/auth/logout, post(logout))/a         .route(/api/v1/protected, get(protected_route_example).route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware)))
pi/v1/auth/login, post(login))/a         .route(/api/v1/auth/logout, post(logout))
pi/v1/auth/logout, post(logout))/a         .route(/api/v1/protected, get(protected_route_example).route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware)))
pi/v1/auth/register, post(register))/a         .route(/api/v1/auth/login, post(login))
pi/v1/auth/logout, post(logout))/a         .route(/api/v1/protected, get(protected_route_example).route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware)))
pi/v1/auth/login, post(login))/a         .route(/api/v1/auth/logout, post(logout))
pi/v1/auth/logout, post(logout))/a         .route(/api/v1/protected, get(protected_route_example).route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware)))
        .with_state(app_state); // Provide AppState to all routes

    // Define the address to run the server on
    let port_str = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port = port_str.parse::<u16>().expect("PORT must be a valid u16");
    let addr = SocketAddr::from(([0, 0, 0, 0], port)); // Listen on all interfaces
    tracing::info!("Backend server listening on {}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
