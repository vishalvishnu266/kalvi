use axum::{
    body::Body,
    http::{Request as AxumRequest, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::security::session_user::SessionUser;

pub async fn check_perm(
    codes: &'static [&'static str],
    req: AxumRequest<Body>,
    next: Next,
) -> Response {
    let allowed = req
        .extensions()
        .get::<SessionUser>()
        .map(|s| s.any_of(codes))
        .unwrap_or(false);
    if allowed {
        next.run(req).await
    } else {
        let path = req.uri().path().to_string();
        forbidden_response(&path, codes)
    }
}

fn forbidden_response(path: &str, missing: &[&str]) -> Response {
    if path.starts_with("/api/") {
        let body = serde_json::json!({
            "error": "forbidden",
            "missing_any_of": missing,
        });
        (StatusCode::FORBIDDEN, axum::Json(body)).into_response()
    } else {
        let missing_html = missing.join(", ");
        let html = format!(
            r#"<!doctype html>
<html lang="en"><head><meta charset="utf-8">
<title>403 · Access denied</title>
<style>
  body {{ font-family: system-ui, sans-serif; background:#fafafa; color:#111; margin:0;
         display:flex; align-items:center; justify-content:center; min-height:100vh; }}
  main {{ max-width: 32rem; padding: 2rem; text-align:center; }}
  h1   {{ font-size: 2.25rem; margin: 0 0 .5rem; }}
  p    {{ color:#555; }}
  code {{ background:#eee; padding: .1rem .35rem; border-radius: .25rem; font-size: .9em; }}
  a    {{ color:#4f46e5; text-decoration:none; }}
</style></head>
<body><main>
  <h1>403 · Access denied</h1>
  <p>Your account doesn't have permission to open this page.</p>
  <p><small>Required (any of): <code>{missing_html}</code></small></p>
  <p><a href="/">← Back to home</a></p>
</main></body></html>"#
        );
        (
            StatusCode::FORBIDDEN,
            [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
            html,
        )
            .into_response()
    }
}
