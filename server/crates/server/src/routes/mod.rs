pub mod IdentityRoutes;
pub mod InstitutionRoutes;

use axum::Router;
use crate::config::AppState::AppState;

pub fn app_routes() -> Router<AppState> {
    Router::new()
        .merge(IdentityRoutes::routes())
        .merge(InstitutionRoutes::routes())
}
