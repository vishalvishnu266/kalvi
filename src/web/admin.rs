use askama::Template;
use axum::{
    body::Body,
    extract::{Form, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::http::AppState;
use crate::services::system as sys_svc;
use crate::system::{NewTenant, Tenant, UpdateTenant};
use crate::tenancy::TenantId;
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
    let tenants = sys_svc::list_tenants(&s.system).await
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
        plan:      Some(f.plan.trim().to_string()).filter(|x| !x.is_empty()),
        notes:     Some(f.notes.trim().to_string()).filter(|x| !x.is_empty()),
    };

    let tenant = match sys_svc::create_tenant(&s.system, &body).await {
        Ok(t) => t,
        Err(e) => {
            return render(&NewTenantPage { error: Some(&e.to_string()), form: &f });
        }
    };

    let tid = TenantId::new(&tenant.tenant_id)
        .map_err(|e| WebError(StatusCode::BAD_REQUEST, e.to_string()))?;
    if let Err(e) = s.provision(tid).await {
        let _ = sys_svc::set_tenant_status(&s.system, tenant.id, "disabled").await;
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
    let existing = sys_svc::find_tenant_by_tenant_id(&s.system, &tid).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    sys_svc::set_tenant_status(&s.system, existing.id, "active").await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(redirect("/admin/tenants"))
}

pub async fn disable_tenant(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Response, WebError> {
    let existing = sys_svc::find_tenant_by_tenant_id(&s.system, &tid).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    sys_svc::set_tenant_status(&s.system, existing.id, "disabled").await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if let Ok(t) = TenantId::new(&existing.tenant_id) {
        s.evict(&t).await;
    }
    Ok(redirect("/admin/tenants"))
}

pub async fn delete_tenant(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Response, WebError> {
    let existing = sys_svc::find_tenant_by_tenant_id(&s.system, &tid).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    sys_svc::soft_delete_tenant(&s.system, existing.id).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if let Ok(t) = TenantId::new(&existing.tenant_id) {
        s.evict(&t).await;
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
    let existing = sys_svc::find_tenant_by_tenant_id(&s.system, &tid).await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    let update = UpdateTenant { name: Some(f.name), ..Default::default() };
    sys_svc::update_tenant(&s.system, existing.id, &update).await
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
