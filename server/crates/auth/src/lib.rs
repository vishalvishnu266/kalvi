mod shared;
mod session;
mod cookie_manager;
mod session_middleware;
mod cleanup;
mod login;
mod logout;
mod dashboard;
mod middleware;
pub mod seed;

pub use middleware::{RequireAuth, OptionalAuth};
pub use shared::{User, UserRole};
pub use session::{Session, SessionManager, SessionConfig};
pub use session_middleware::session_middleware;
pub use cleanup::{start_cleanup_task, init_session_tables};

use axum::Router;
use ::shared::middleware::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(login::routes())
        .merge(logout::routes())
        .merge(dashboard::routes())
}
