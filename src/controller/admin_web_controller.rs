use askama::Template;
use axum::{
    body::Body,
    extract::{Form, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

use crate::application::AppState;
use crate::dto::tenant_dto::{NewTenantForm, RenameTenantForm};
use crate::entity::tenant::{NewTenant, Tenant, UpdateTenant};
use crate::exception::web_error::{render, WebError};
use crate::service::tenant_service;
use crate::tenant::tenant_id::validate_tenant_id;

#[derive(Template)]
#[template(path = "admin/tenants.html")]
struct TenantsPage<'a> {
    tenants: &'a [Tenant],
    flash: Option<&'a str>,
    error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "admin/new_tenant.html")]
struct NewTenantPage<'a> {
    error: Option<&'a str>,
    form: &'a NewTenantForm,
}

pub async fn index() -> Response {
    (
        StatusCode::SEE_OTHER,
        [(header::LOCATION, "/admin/tenants")],
        Body::empty(),
    )
        .into_response()
}

pub async fn list_tenants(State(s): State<AppState>) -> Result<Response, WebError> {
    let tenants = tenant_service::list_all(&s.system)
        .await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    render(&TenantsPage {
        tenants: &tenants,
        flash: None,
        error: None,
    })
}

pub async fn new_tenant_form() -> Result<Response, WebError> {
    let form = NewTenantForm::default();
    render(&NewTenantPage {
        error: None,
        form: &form,
    })
}

pub async fn create_tenant(
    State(s): State<AppState>,
    Form(f): Form<NewTenantForm>,
) -> Result<Response, WebError> {
    if let Err(e) = validate_tenant_id(f.tenant_id.clone()) {
        return render(&NewTenantPage {
            error: Some(&e.to_string()),
            form: &f,
        });
    }
    if f.name.trim().is_empty() {
        return render(&NewTenantPage {
            error: Some("Name is required"),
            form: &f,
        });
    }

    let body = NewTenant {
        tenant_id: f.tenant_id.trim().to_string(),
        name: f.name.trim().to_string(),
        plan: Some(f.plan.trim().to_string()).filter(|x| !x.is_empty()),
        notes: Some(f.notes.trim().to_string()).filter(|x| !x.is_empty()),
    };

    let tenant = match tenant_service::create(&s.system, &body).await {
        Ok(t) => t,
        Err(e) => {
            return render(&NewTenantPage {
                error: Some(&e.to_string()),
                form: &f,
            });
        }
    };

    let tid = validate_tenant_id(&tenant.tenant_id)
        .map_err(|e| WebError(StatusCode::BAD_REQUEST, e.to_string()))?;
    if let Err(e) = s.provision(tid).await {
        let _ = tenant_service::set_status(&s.system, tenant.id, "disabled").await;
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
    let existing = tenant_service::find_by_tenant_id(&s.system, &tid)
        .await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    tenant_service::set_status(&s.system, existing.id, "active")
        .await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(redirect("/admin/tenants"))
}

pub async fn disable_tenant(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Response, WebError> {
    let existing = tenant_service::find_by_tenant_id(&s.system, &tid)
        .await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    tenant_service::set_status(&s.system, existing.id, "disabled")
        .await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if validate_tenant_id(&existing.tenant_id).is_ok() {
        s.evict(&existing.tenant_id).await;
    }
    Ok(redirect("/admin/tenants"))
}

pub async fn delete_tenant(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Response, WebError> {
    let existing = tenant_service::find_by_tenant_id(&s.system, &tid)
        .await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    tenant_service::soft_delete(&s.system, existing.id)
        .await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if validate_tenant_id(&existing.tenant_id).is_ok() {
        s.evict(&existing.tenant_id).await;
    }
    Ok(redirect("/admin/tenants"))
}

pub async fn rename_tenant(
    State(s): State<AppState>,
    Path(tid): Path<String>,
    Form(f): Form<RenameTenantForm>,
) -> Result<Response, WebError> {
    let existing = tenant_service::find_by_tenant_id(&s.system, &tid)
        .await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| WebError(StatusCode::NOT_FOUND, "tenant not found".into()))?;
    let update = UpdateTenant {
        name: Some(f.name),
        ..Default::default()
    };
    tenant_service::update(&s.system, existing.id, &update)
        .await
        .map_err(|e| WebError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(redirect("/admin/tenants"))
}

fn redirect(to: &str) -> Response {
    (
        StatusCode::SEE_OTHER,
        [(header::LOCATION, to.to_string())],
        Body::empty(),
    )
        .into_response()
}
