use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect},
};
use crate::view::{HomeView, contact_view};
use crate::config::AppState;
use crate::util::SessionUtil;
use crate::service::TenantService;

pub async fn show_home(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let session_id = SessionUtil::get_session_id(&headers);
    let tenant_hint = SessionUtil::get_tenant_slug(&headers);

    if let Some(tenant) = TenantService::get_tenant_for_session(&state, &session_id, &tenant_hint).await {
        return Redirect::to(&format!("/web/{}/dashboard", tenant.slug)).into_response();
    }

    Html(HomeView::render()).into_response()
}

pub async fn show_contact() -> impl IntoResponse {
    Html(contact_view::render()).into_response()
}
