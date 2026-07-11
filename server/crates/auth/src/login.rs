// Feature: User Login
// Modern, responsive login page with session-based authentication

use axum::{
    Router,
    extract::{Extension, Query},
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Form,
};
use maud::{DOCTYPE, Markup, html, PreEscaped};
use sqlx::SqlitePool;
use validator::Validate;
use serde::Deserialize;

use crate::shared::{User, LoginForm, password, db};
use crate::session::{Session, SessionManager};
use crate::cookie_manager;
use ::shared::middleware::AppState;

#[derive(Deserialize)]
struct LoginQuery {
    redirect: Option<String>,
    error: Option<String>,
}

// ============================================================================
// Business Logic
// ============================================================================

async fn authenticate_user(
    pool: &SqlitePool,
    form: LoginForm,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<(User, Session), String> {
    // Validate form
    form.validate().map_err(|e| format!("Validation error: {}", e))?;

    // Get user from tenant database
    let user = db::get_user_by_username(pool, &form.username)
        .await
        .map_err(|_| "Database error".to_string())?
        .ok_or_else(|| "Invalid username or password".to_string())?;

    // Check if user is active
    if !user.is_active {
        return Err("Your account has been deactivated. Please contact an administrator.".to_string());
    }

    // Verify password
    let password_valid = password::verify_password(&form.password, &user.password_hash)
        .map_err(|_| "Authentication error".to_string())?;

    if !password_valid {
        return Err("Invalid username or password".to_string());
    }

    // Create session in database
    let session_manager = SessionManager::new(pool.clone());
    let session = session_manager
        .create_session(user.id, ip_address, user_agent)
        .await
        .map_err(|_| "Failed to create session".to_string())?;

    Ok((user, session))
}

// ============================================================================
// UI Rendering
// ============================================================================

fn render_login_page(tenant_slug: &str, error: Option<String>, redirect: Option<String>) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Login - School ERP" }

                // Bootstrap CSS
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
                     rel="stylesheet"
                     integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN"
                     crossorigin="anonymous";

                // Bootstrap Icons
                link href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.1/font/bootstrap-icons.css"
                     rel="stylesheet";

                // CSS must be emitted verbatim (braces would otherwise be escaped by maud).
                style {
                    (PreEscaped(r#"
                    body {
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                        min-height: 100vh;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                    }
                    .login-container { width: 100%; max-width: 420px; padding: 15px; }
                    .login-card { background: white; border-radius: 16px; box-shadow: 0 20px 60px rgba(0,0,0,0.3); overflow: hidden; }
                    .login-header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 2rem; text-align: center; }
                    .login-header i { font-size: 3rem; margin-bottom: 0.5rem; }
                    .login-body { padding: 2rem; }
                    .form-floating { margin-bottom: 1rem; }
                    .form-floating .form-control { border-radius: 8px; border: 2px solid #e0e0e0; transition: all 0.3s; }
                    .form-floating .form-control:focus { border-color: #667eea; box-shadow: 0 0 0 0.2rem rgba(102, 126, 234, 0.25); }
                    .btn-login { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border: none; border-radius: 8px; padding: 0.75rem; font-weight: 600; transition: transform 0.2s; }
                    .btn-login:hover { transform: translateY(-2px); box-shadow: 0 5px 15px rgba(102, 126, 234, 0.4); }
                    .alert { border-radius: 8px; border: none; }
                    "#))
                }
            }
            body {
                div class="login-container" {
                    div class="login-card" {
                        div class="login-header" {
                            i class="bi bi-person-circle" {}
                            h3 class="mb-0" { "Welcome Back" }
                            p class="mb-0 mt-2" style="opacity: 0.9;" { "Sign in to your account" }
                        }

                        div class="login-body" {
                            @if let Some(err) = error {
                                div class="alert alert-danger d-flex align-items-center mb-3" role="alert" {
                                    i class="bi bi-exclamation-triangle-fill me-2" {}
                                    div { (err) }
                                }
                            }

                            form method="post" action=(format!("/t/{}/login", tenant_slug)) {
                                @if let Some(r) = redirect {
                                    input type="hidden" name="redirect" value=(r);
                                }

                                div class="form-floating" {
                                    input type="text"
                                           class="form-control"
                                           id="username"
                                           name="username"
                                           placeholder="Username"
                                           required
                                           autofocus;
                                    label for="username" {
                                        i class="bi bi-person me-2" {}
                                        "Username"
                                    }
                                }

                                div class="form-floating" {
                                    input type="password"
                                           class="form-control"
                                           id="password"
                                           name="password"
                                           placeholder="Password"
                                           required;
                                    label for="password" {
                                        i class="bi bi-lock me-2" {}
                                        "Password"
                                    }
                                }

                                div class="d-grid" {
                                    button type="submit" class="btn btn-primary btn-login btn-lg" {
                                        i class="bi bi-box-arrow-in-right me-2" {}
                                        "Sign In"
                                    }
                                }
                            }

                            div class="text-center mt-4 pt-3 border-top" {
                                p class="text-muted small mb-0" {
                                    i class="bi bi-info-circle me-1" {}
                                    "Contact your administrator for access"
                                }
                            }
                        }
                    }

                    div class="text-center mt-3" {
                        a href="/" class="text-white text-decoration-none" {
                            i class="bi bi-arrow-left me-1" {}
                            "Back to Home"
                        }
                    }
                }

                script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js"
                       integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL"
                       crossorigin="anonymous" {}
            }
        }
    }
}

// ============================================================================
// HTTP Handlers
// ============================================================================

async fn show_login_page(
    Extension(tenant_context): Extension<::shared::middleware::TenantContext>,
    Query(query): Query<LoginQuery>,
) -> impl IntoResponse {
    let error = query.error.as_ref().and_then(|e| match e.as_str() {
        "inactive" => Some("Your account is inactive. Please contact an administrator.".to_string()),
        _ => None,
    });

    // Convert Markup to a String so it satisfies IntoResponse via Html<String>.
    Html(render_login_page(&tenant_context.slug, error, query.redirect).into_string())
}

#[derive(Deserialize)]
struct LoginFormWithRedirect {
    #[serde(flatten)]
    login: LoginForm,
    redirect: Option<String>,
}

async fn process_login(
    Extension(pool): Extension<SqlitePool>,
    Extension(tenant_context): Extension<::shared::middleware::TenantContext>,
    headers: HeaderMap,
    Form(form): Form<LoginFormWithRedirect>,
) -> Response {
    // Extract IP and user agent from headers
    let ip_address = cookie_manager::extract_client_ip_from_headers(&headers);
    let user_agent = cookie_manager::extract_user_agent_from_headers(&headers);

    match authenticate_user(&pool, form.login.clone(), ip_address, user_agent).await {
        Ok((_user, session)) => {
            // Create redirect URL
            let default_redirect = format!("/t/{}/dashboard", tenant_context.slug);
            let redirect_to = form
                .redirect
                .as_deref()
                .unwrap_or(&default_redirect);

            // Create response with session cookie
            let mut response = Redirect::to(redirect_to).into_response();
            cookie_manager::set_session_cookie(&mut response, &session.id);

            response
        }
        Err(error) => {
            Html(
                render_login_page(&tenant_context.slug, Some(error), form.redirect).into_string(),
            )
            .into_response()
        }
    }
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/t/{tenant_slug}/login", get(show_login_page))
        .route("/t/{tenant_slug}/login", post(process_login))
}
