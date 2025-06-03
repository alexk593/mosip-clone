pub mod register;
pub mod login;
// pub mod middleware; // Will be added later

// Re-export handlers for easier access in main.rs
pub use login::login;
pub mod logout;
pub use logout::logout;
pub mod middleware;
pub use middleware::auth_middleware;
pub use middleware::{AuthenticatedUser, protected_route_example};
pub use register::register;
