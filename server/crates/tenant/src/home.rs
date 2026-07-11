use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use shared::AppState;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

async fn show_home() -> Response {
    match (HomeTemplate {}).render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            eprintln!("template error: {err:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(show_home))
}
