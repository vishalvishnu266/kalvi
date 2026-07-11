pub mod controller;
pub mod model;
pub mod repository;
pub mod view;
pub mod session;
pub mod session_middleware;
pub mod extractors;
pub mod seed;

pub use extractors::RequireAuth;
pub use model::{User, Session};
pub use session_middleware::session_middleware;
pub use seed::create_admin_user;

use axum::Router;
use shared::AppState;

pub fn routes() -> Router<AppState> {
    controller::routes()
}
