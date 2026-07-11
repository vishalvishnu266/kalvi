use askama::Template;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use shared::{AppState, TenantContext};

use crate::extractors::RequireAuth;

#[derive(Template)]
#[template(path = "auth/dashboard.html")]
struct DashboardTemplate<'a> {
    tenant_slug: &'a str,
    username: &'a str,
}

async fn show_dashboard(
    RequireAuth(user): RequireAuth,
    Extension(ctx): Extension<TenantContext>,
) -> Response {
    let tmpl = DashboardTemplate {
        tenant_slug: &ctx.slug,
        username: &user.username,
    };
    match tmpl.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            eprintln!("template error: {err:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/t/{tenant_slug}/dashboard", get(show_dashboard))
}
