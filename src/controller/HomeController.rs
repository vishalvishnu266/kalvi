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
    // Check if user has a session
    if let Some(session_id) = SessionUtil::get_session_id(&headers) {
        // Find which tenant this session belongs to
        if let Ok(tenants) = TenantRepository::list_all(&state.db.master_pool).await {
            for tenant in tenants {
                if let Ok(tenant_pool) = state.db.get_tenant_pool(&tenant.database_name).await {
                    if let Ok(Some(_)) = UserRepository::find_session(&tenant_pool, &session_id).await {
                        // Found the session, redirect to tenant dashboard
                        return Redirect::to(&format!("/t/{}/dashboard", tenant.slug)).into_response();
                    }
                }
            }
        }
    }

    Html(HomeView::render()).into_response()
}
