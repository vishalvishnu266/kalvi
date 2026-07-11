use axum::{
    extract::{Extension, State},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use shared::AppState;
use sqlx::SqlitePool;

use crate::model::NewTenant;
use crate::repository;
use crate::view::onboarding::{form_page, success_page};

fn render_form(error: Option<String>) -> Response {
    Html(form_page(error).into_string()).into_response()
}

fn render_success(slug: &str, name: &str) -> Response {
    Html(success_page(slug, name).into_string()).into_response()
}

pub async fn show_form() -> Response {
    render_form(None)
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
        return render_form(Some(
            "Slug, name, admin username and admin password are required".into(),
        ));
    }

    match repository::slug_exists(&master_pool, &slug).await {
        Ok(true) => return render_form(Some(format!("Slug '{slug}' is already taken"))),
        Err(err) => {
            eprintln!("slug_exists error: {err:?}");
            return render_form(Some("Database error, please try again".into()));
        }
        Ok(false) => {}
    }

    let tenant = match repository::insert_tenant(
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
            return render_form(Some("Failed to create tenant".into()));
        }
    };

    let tenant_pool = match state.db_manager.tenant_pool(&tenant.database_name).await {
        Ok(p) => p,
        Err(err) => {
            eprintln!("tenant_pool error: {err:?}");
            return render_form(Some(
                "Tenant created but database provisioning failed".into(),
            ));
        }
    };

    if let Err(err) = auth::create_admin_user(&tenant_pool, admin_username, &form.admin_password).await {
        eprintln!("create_admin_user error: {err}");
        return render_form(Some(
            "Tenant created but seeding admin user failed".into(),
        ));
    }

    render_success(&tenant.slug, &tenant.name)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/onboard", get(show_form))
        .route("/onboard", post(submit_form))
}
