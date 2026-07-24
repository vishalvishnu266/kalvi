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
use crate::tenancy::{tenant_evict, tenant_provision, TenantId};
use crate::web::error::{render, WebError};

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

    form:  &'a NewTenantForm,
}

#[derive(Deserialize, Default, Clone)]
pub struct NewTenantForm {
    pub tenant_id: String,
    pub name:      String,
    pub plan:      String,
    pub notes:     String,
}

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

let tenant = match s.system.create(&body).await {
        Ok(t) => t,
        Err(e) => {
            return render(&NewTenantPage { error: Some(&e.to_string()), form: &f });
        }
    };

let tid = TenantId::new(&tenant.tenant_id)
        .map_err(|e| WebError(StatusCode::BAD_REQUEST, e.to_string()))?;
    if let Err(e) = tenant_provision(&s.tenants, tid).await {
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
        tenant_evict(&s.tenants, &t).await;
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
        tenant_evict(&s.tenants, &t).await;
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

fn redirect(to: &str) -> Response {
    (
        StatusCode::SEE_OTHER,
        [(header::LOCATION, to.to_string())],
        Body::empty(),
    ).into_response()
}
