use axum::{
    extract::{Extension, State},
    response::{IntoResponse, Response},
    Form,
};
use serde::Deserialize;
use crate::config::AppState::AppState;
use sqlx::SqlitePool;

use crate::models::TenantModel::NewTenant;
use crate::repositories::TenantRepository;
use crate::views::OnboardingView;
use crate::services::AuthService;

pub async fn show_form() -> Response {
    OnboardingView::render_form(None).into_response()
}

#[derive(Debug, Deserialize)]
pub struct OnboardForm {
    pub slug: String,
    pub name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub address: String,
    pub admin_username: String,
    pub admin_password: String,
}

pub async fn submit_form(
    Extension(master_pool): Extension<SqlitePool>,
    State(state): State<AppState>,
    Form(form): Form<OnboardForm>,
) -> Response {
    let slug = form.slug.trim().to_lowercase();
    let name = form.name.trim();
    let admin_username = form.admin_username.trim();

    if slug.is_empty() || name.is_empty() || admin_username.is_empty() || form.admin_password.is_empty() {
        return OnboardingView::render_form(Some(
            "Slug, name, admin username and admin password are required".into(),
        )).into_response();
    }

    match TenantRepository::slug_exists(&master_pool, &slug).await {
        Ok(true) => return OnboardingView::render_form(Some(format!("Slug '{slug}' is already taken"))).into_response(),
        Err(err) => {
            eprintln!("slug_exists error: {err:?}");
            return OnboardingView::render_form(Some("Database error, please try again".into())).into_response();
        }
        Ok(false) => {}
    }

    let tenant = match TenantRepository::insert_tenant(
        &master_pool,
        NewTenant {
            slug: &slug,
            name,
            contact_email: form.contact_email.trim(),
            contact_phone: form.contact_phone.trim(),
            address: form.address.trim(),
        },
    )
    .await
    {
        Ok(t) => t,
        Err(err) => {
            eprintln!("insert_tenant error: {err:?}");
            return OnboardingView::render_form(Some("Failed to create tenant".into())).into_response();
        }
    };

    let tenant_pool = match state.db_manager.tenant_pool(&tenant.database_name).await {
        Ok(p) => p,
        Err(err) => {
            eprintln!("tenant_pool error: {err:?}");
            return OnboardingView::render_form(Some(
                "Tenant created but database provisioning failed".into(),
            )).into_response();
        }
    };

    if let Err(err) = AuthService::create_admin_user(&tenant_pool, admin_username, &form.admin_password).await {
        eprintln!("create_admin_user error: {err}");
        return OnboardingView::render_form(Some(
            "Tenant created but seeding admin user failed".into(),
        )).into_response();
    }

    OnboardingView::render_success(&tenant.slug, &tenant.name).into_response()
}
