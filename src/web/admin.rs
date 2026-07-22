//! Admin (control-plane) web UI at `/admin/…`.
//!
//! Deliberately unauthenticated for now — the user asked for a working
//! admin surface without an auth requirement. Once tenant auth stabilises
//! we can add an operator sign-in (probably against a separate operator
//! table in the system DB) and gate this router behind it.
//!
//! Routing lives in [`crate::http::routes`]. This module owns the handlers
//! plus their small askama templates.

use askama::Template;
use axum::{
    body::Body,
    extract::{Form, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::http::AppState;
use crate::system::{NewTenant, Tenant, UpdateTenant};
use crate::tenancy::TenantId;
use crate::web::error::{render, WebError};

// ---------------------------------------------------------------- templates

#[derive(Template)]
#[template(path = "admin/tenants.html")]
struct TenantsPage<'a> {
    tenants: &'a [Tenant],
    flash:   Option<&'a str>,
    error:   Option<&'a str>,
}

#[derive(Template)]
#[template(path = "admin/new_tenant.html")]
struct NewTenantPage<'a> {
    error: Option<&'a str>,
    /// Sticky form values so a validation error doesn't wipe user input.
    form:  &'a NewTenantForm,
}

// ---------------------------------------------------------------- forms

#[derive(Deserialize, Default, Clone)]
pub struct NewTenantForm {
    pub tenant_id: String,
    pub name:      String,
    pub plan:      String,
    pub notes:     String,
}

// ---------------------------------------------------------------- handlers

/// Redirect `/admin` and `/admin/` to the tenants list.
pub async fn index() -> Response {
    (
        StatusCode::SEE_OTHER,
        [(header::LOCATION, "/admin/tenants")],
        Body::empty(),
    ).into_response()
}

pub async fn list_tenants(State(s): State<AppState>) -> Result<Response, WebError> {
    let tenants = s.system.list().await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    render(&TenantsPage { tenants: &tenants, flash: None, error: None })
}

pub async fn new_tenant_form() -> Result<Response, WebError> {
    let form = NewTenantForm::default();
    render(&NewTenantPage { error: None, form: &form })
}

pub async fn create_tenant(
    State(s): State<AppState>,
    Form(f): Form<NewTenantForm>,
) -> Result<Response, WebError> {
    // Validate the tenant id up-front so we can render the form with an
    // inline error instead of a naked 400.
    if let Err(e) = TenantId::new(f.tenant_id.clone()) {
        return render(&NewTenantPage { error: Some(&e.to_string()), form: &f });
    }
    if f.name.trim().is_empty() {
        return render(&NewTenantPage { error: Some("Name is required"), form: &f });
    }

    let body = NewTenant {
        tenant_id: f.tenant_id.trim().to_string(),
        name:      f.name.trim().to_string(),
        plan:      Some(f.plan.trim().to_string()).filter(|s| !s.is_empty()),
        notes:     Some(f.notes.trim().to_string()).filter(|s| !s.is_empty()),
    };

    // Insert into system DB.
    let tenant = match s.system.create(&body).await {
        Ok(t) => t,
        Err(e) => {
            return render(&NewTenantPage { error: Some(&e.to_string()), form: &f });
        }
    };

    // Provision the per-tenant DB (create + migrate). If provisioning
    // fails we roll the system row back to `disabled` so the operator
    // can retry after fixing the underlying issue.
    let tid = TenantId::new(&tenant.tenant_id)
        .map_err(|e| WebError(StatusCode::BAD_REQUEST, e.to_string()))?;
    if let Err(e) = s.tenants.provision(tid).await {
        let _ = s.system.set_status(tenant.id, "disabled").await;
        return render(&NewTenantPage {
            error: Some(&format!("Tenant row created but provisioning failed: {e}")),
            form: &f,
        });
    }

    Ok(redirect("/admin/tenants"))
}

pub async fn enable_tenant(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Response, WebError> {
    let existing = s.system.find_by_tenant_id(&tid).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    s.system.set_status(existing.id, "active").await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(redirect("/admin/tenants"))
}

pub async fn disable_tenant(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Response, WebError> {
    let existing = s.system.find_by_tenant_id(&tid).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    s.system.set_status(existing.id, "disabled").await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if let Ok(t) = TenantId::new(&existing.tenant_id) {
        s.tenants.evict(&t).await;
    }
    Ok(redirect("/admin/tenants"))
}

pub async fn delete_tenant(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Response, WebError> {
    let existing = s.system.find_by_tenant_id(&tid).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    s.system.delete(existing.id).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if let Ok(t) = TenantId::new(&existing.tenant_id) {
        s.tenants.evict(&t).await;
    }
    Ok(redirect("/admin/tenants"))
}

#[derive(Deserialize)]
pub struct RenameForm { pub name: String }

pub async fn rename_tenant(
    State(s): State<AppState>,
    Path(tid): Path<String>,
    Form(f): Form<RenameForm>,
) -> Result<Response, WebError> {
    let existing = s.system.find_by_tenant_id(&tid).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    let update = UpdateTenant { name: Some(f.name), ..Default::default() };
    s.system.update(existing.id, &update).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(redirect("/admin/tenants"))
}

// ---------------------------------------------------------------- helpers

fn redirect(to: &str) -> Response {
    (
        StatusCode::SEE_OTHER,
        [(header::LOCATION, to.to_string())],
        Body::empty(),
    ).into_response()
}
