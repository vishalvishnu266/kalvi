pub mod models;
pub mod session;
pub mod session_middleware;
pub mod extractors;
pub mod seed;

mod login;
mod logout;
mod dashboard;

pub use extractors::RequireAuth;
pub use models::{User, Session};
pub use session_middleware::session_middleware;
pub use seed::create_admin_user;

use axum::Router;
use shared::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(login::routes())
        .merge(logout::routes())
        .merge(dashboard::routes())
}
