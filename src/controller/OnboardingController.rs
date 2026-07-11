use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};
use crate::config::AppState::AppState;
use crate::repository::TenantRepository::TenantRepository;
use crate::view::OnboardingView;

#[derive(Deserialize)]
pub struct OnboardForm {
    pub name: String,
    pub slug: String,
    pub admin_username: String,
    pub admin_password: String,
}

pub async fn show_form() -> Html<String> {
    Html(OnboardingView::render_form(None))
}

pub async fn submit_form(
    State(state): State<AppState>,
    Form(form): Form<OnboardForm>,
) -> impl IntoResponse {
    let slug = form.slug.trim().to_lowercase();
    if slug.is_empty() || form.name.trim().is_empty() {
        return Html(OnboardingView::render_form(Some("All fields are required".to_string()))).into_response();
    }

    // Check if slug exists
    match TenantRepository::find_by_slug(&state.db.master_pool, &slug).await {
        Ok(Some(_)) => return Html(OnboardingView::render_form(Some("Slug is already taken".to_string()))).into_response(),
        Err(_) => return Html(OnboardingView::render_form(Some("Database error".to_string()))).into_response(),
        Ok(None) => {}
    }

    let db_name = slug.clone();
    
    // Create tenant
    match TenantRepository::save(&state.db.master_pool, &slug, &form.name, &db_name).await {
        Ok(tenant) => {
            // Provision tenant DB
            let tenant_pool = match state.db.get_tenant_pool(&tenant.database_name).await {
                Ok(p) => p,
                Err(_) => return Html(OnboardingView::render_form(Some("Failed to provision tenant database".to_string()))).into_response(),
            };

            // Create admin user
            let hashed_pw = hash(&form.admin_password, DEFAULT_COST).unwrap();
            let res = sqlx::query("INSERT INTO users (username, password_hash, role) VALUES (?, ?, 'admin')")
                .bind(&form.admin_username)
                .bind(hashed_pw)
                .execute(&tenant_pool)
                .await;

            if res.is_err() {
                 return Html(OnboardingView::render_form(Some("Failed to create admin user".to_string()))).into_response();
            }

            Html(OnboardingView::render_success(&tenant.slug, &tenant.name)).into_response()
        }
        Err(_) => Html(OnboardingView::render_form(Some("Failed to create tenant".to_string()))).into_response(),
    }
}
