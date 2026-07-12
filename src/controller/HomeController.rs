use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect},
};
use crate::view::HomeView;
use crate::config::AppState::AppState;
use crate::util::SessionUtil;
use crate::repository::{TenantRepository::TenantRepository, UserRepository::UserRepository};

pub async fn show_home(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Check if user has a session and a tenant hint
    let session_id = SessionUtil::get_session_id(&headers);
    let tenant_hint = SessionUtil::get_tenant_slug(&headers);

    if let (Some(sid), Some(slug)) = (session_id, tenant_hint) {
        // Efficiency: directly check the hinted tenant
        if let Ok(Some(tenant)) = TenantRepository::find_by_slug(&state.db.master_pool, &slug).await {
            if let Ok(tenant_pool) = state.db.get_tenant_pool(&tenant.database_name).await {
                if let Ok(Some(_)) = UserRepository::find_session(&tenant_pool, &sid).await {
                    return Redirect::to(&format!("/{}/dashboard", tenant.slug)).into_response();
                }
            }
        }
    }

    Html(HomeView::render()).into_response()
}
