use crate::http::{api_routes, AppState};
use axum::
Router
;

pub fn routes() -> Router<AppState> {
    let admin_api = api_routes::admin_api();

    Router::new()
}
