//! Guardians CRUD screens at `/web/{tenant}/guardians[/...]`.
//!
//! This is the reference implementation for a "full CRUD" web module in
//! this codebase — every new module should follow the same shape:
//!
//!   * `list`             — GET  `/guardians`
//!   * `new_form`         — GET  `/guardians/new`
//!   * `create`           — POST `/guardians`
//!   * `show`             — GET  `/guardians/{id}`
//!   * `edit_form`        — GET  `/guardians/{id}/edit`
//!   * `update`           — POST `/guardians/{id}` (form uses `_method=put`)
//!   * `delete`           — POST `/guardians/{id}/delete`
//!
//! RBAC:
//!   * `guardians.view`   — required for list / show.
//!   * `guardians.manage` — required for new / create / edit / update / delete.
//!     Route-level `require_perm!` gates enforce this; templates also
//!     hide the CTAs via `can_manage` for a clean UI.
//!
//! Data flow:
//!   * All reads go through `GuardianRepo` directly (no row scoping for the
//!     directory view — see `docs/RBAC.md` for the guardian *portal* case,
//!     which is a separate `Scope::GuardianOfUser` flow on the students
//!     module).
//!   * Writes go through `PeopleService` where present, otherwise the repo,
//!     so validation and audit can be added later without touching the
//!     handler shape.

use askama::Template;
use axum::{
    body::Body,
    extract::{Form, Path},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};
use serde::Deserialize;

use crate::http::TenantScope;
use crate::middleware::auth::SessionUser;
use crate::repositories::guardians::{Guardian, NewGuardian, UpdateGuardian};
use crate::services::perm;
use crate::web::error::{render, WebError};
use crate::web::layout::{visible_nav_items, NavContext, NavItem};

// =============================================================================
// Templates
// =============================================================================

#[derive(Template)]
#[template(path = "guardians/list.html")]
struct ListPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    q: &'a str,
    rows: Vec<Guardian>,
    can_manage: bool,
    flash: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "guardians/show.html")]
struct ShowPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    g: &'a Guardian,
    /// Students currently linked to this guardian, for the "Linked students"
    /// panel. Empty when the guardian has no children on file.
    linked: Vec<crate::repositories::students::Student>,
    can_manage: bool,
}

/// Single template shared by the "new" and "edit" screens.
///
/// `is_edit = true` means we're editing an existing guardian; the template
/// uses `action_url` (pre-computed in Rust) for the form target and swaps
/// its heading / submit-button label accordingly.
#[derive(Template)]
#[template(path = "guardians/form.html")]
struct FormPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    /// Sticky form values so validation errors don't wipe user input.
    form: &'a GuardianForm,
    /// Where the form posts to (list URL for create, item URL for update).
    action_url: String,
    /// True when editing; drives copy on the page.
    is_edit: bool,
    error: Option<&'a str>,
}

// =============================================================================
// Form model
// =============================================================================

#[derive(Deserialize, Default, Clone)]
pub struct GuardianForm {
    pub first_name: String,
    pub last_name:  String,
    pub phone:      String,
    pub email:      String,
    pub occupation: String,
    pub address:    String,
}

impl GuardianForm {
    fn from_guardian(g: &Guardian) -> Self {
        Self {
            first_name: g.first_name.clone(),
            last_name:  g.last_name.clone(),
            phone:      g.phone.clone().unwrap_or_default(),
            email:      g.email.clone().unwrap_or_default(),
            occupation: g.occupation.clone().unwrap_or_default(),
            address:    g.address.clone().unwrap_or_default(),
        }
    }

    /// Basic validation — returns the first problem (if any) as a message
    /// that can be shown inline above the form.
    fn validate(&self) -> Option<&'static str> {
        if self.first_name.trim().is_empty() { return Some("First name is required"); }
        if self.last_name.trim().is_empty()  { return Some("Last name is required"); }
        None
    }

    fn to_new(&self) -> NewGuardian {
        NewGuardian {
            user_id: None,
            first_name: self.first_name.trim().to_string(),
            last_name:  self.last_name.trim().to_string(),
            phone:      opt(&self.phone),
            email:      opt(&self.email),
            occupation: opt(&self.occupation),
            address:    opt(&self.address),
        }
    }

    fn to_update(&self) -> UpdateGuardian {
        // Every field is `Some(_)` because the form always sends every input.
        // Empty strings for nullable columns are normalized to NULL by the
        // repo, so submitting "" clears the value.
        UpdateGuardian {
            first_name: Some(self.first_name.trim().to_string()),
            last_name:  Some(self.last_name.trim().to_string()),
            phone:      Some(self.phone.trim().to_string()),
            email:      Some(self.email.trim().to_string()),
            occupation: Some(self.occupation.trim().to_string()),
            address:    Some(self.address.trim().to_string()),
        }
    }
}

fn opt(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() { None } else { Some(t.to_string()) }
}

// =============================================================================
// Handlers
// =============================================================================

#[derive(Deserialize)]
pub struct ListQuery { pub q: Option<String>, pub flash: Option<String> }

/// GET `/guardians` — directory view.
pub async fn list(
    ts: TenantScope,
    axum::extract::Query(qp): axum::extract::Query<ListQuery>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    let all = ts.services.repos.guardians.list(200, 0).await.unwrap_or_default();
    let q = qp.q.unwrap_or_default();
    let ql = q.to_lowercase();
    let rows: Vec<Guardian> = all.into_iter().filter(|g| {
        if ql.is_empty() { return true; }
        let name = format!("{} {}", g.first_name, g.last_name).to_lowercase();
        name.contains(&ql)
            || g.email.as_deref().unwrap_or("").to_lowercase().contains(&ql)
            || g.phone.as_deref().unwrap_or("").to_lowercase().contains(&ql)
    }).collect();

    let nav = NavContext::new(
        session.display.clone(),
        ts.tenant.as_str().to_string(),
        "guardians", "Guardians",
    );
    let nav_items  = visible_nav_items(&session);
    let can_manage = session.has(perm::GUARDIANS_MANAGE);

    render(&ListPage {
        nav: &nav, nav_items, q: &q, rows, can_manage,
        flash: qp.flash.as_deref(),
    })
}

/// GET `/guardians/{id}` — profile view.
pub async fn show(
    ts: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    let g = ts.services.repos.guardians.get(id).await?;
    let student_ids = ts.services.repos.guardians.students_of_guardian(id).await
        .unwrap_or_default();
    let linked = ts.services.repos.students.list_by_ids(&student_ids).await
        .unwrap_or_default();

    let title = format!("Guardians · {} {}", g.first_name, g.last_name);
    let nav = NavContext::new(
        session.display.clone(),
        ts.tenant.as_str().to_string(),
        "guardians", title,
    );
    let nav_items  = visible_nav_items(&session);
    let can_manage = session.has(perm::GUARDIANS_MANAGE);

    render(&ShowPage { nav: &nav, nav_items, g: &g, linked, can_manage })
}

/// GET `/guardians/new` — empty form.
pub async fn new_form(
    ts: TenantScope,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    let form = GuardianForm::default();
    render_form(&ts, &session, &form, None, None)
}

/// POST `/guardians` — create.
pub async fn create(
    ts: TenantScope,
    Extension(session): Extension<SessionUser>,
    Form(f): Form<GuardianForm>,
) -> Result<Response, WebError> {
    if let Some(err) = f.validate() {
        return render_form(&ts, &session, &f, None, Some(err));
    }
    match ts.services.repos.guardians.create(&f.to_new()).await {
        Ok(g) => Ok(redirect(&format!(
            "/web/{}/guardians/{}?flash=Guardian+created",
            ts.tenant.as_str(), g.id
        ))),
        Err(e) => render_form(&ts, &session, &f, None, Some(&e.to_string())),
    }
}

/// GET `/guardians/{id}/edit` — form pre-filled with existing data.
pub async fn edit_form(
    ts: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    let g = ts.services.repos.guardians.get(id).await?;
    let form = GuardianForm::from_guardian(&g);
    render_form(&ts, &session, &form, Some(id), None)
}

/// POST `/guardians/{id}` — update.
pub async fn update(
    ts: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Extension(session): Extension<SessionUser>,
    Form(f): Form<GuardianForm>,
) -> Result<Response, WebError> {
    if let Some(err) = f.validate() {
        return render_form(&ts, &session, &f, Some(id), Some(err));
    }
    match ts.services.repos.guardians.update(id, &f.to_update()).await {
        Ok(_) => Ok(redirect(&format!(
            "/web/{}/guardians/{}?flash=Guardian+updated",
            ts.tenant.as_str(), id
        ))),
        Err(e) => render_form(&ts, &session, &f, Some(id), Some(&e.to_string())),
    }
}

/// POST `/guardians/{id}/delete` — remove.
///
/// A POST endpoint (not DELETE) so it can be triggered by a standard HTML
/// form without JavaScript. Ownership is enforced by RBAC
/// (`guardians.manage`); business rules like "can't delete a primary
/// guardian while children still enrolled" would go in `PeopleService`
/// when we add them.
pub async fn delete(
    ts: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<Response, WebError> {
    ts.services.repos.guardians.delete(id).await?;
    Ok(redirect(&format!(
        "/web/{}/guardians?flash=Guardian+deleted",
        ts.tenant.as_str()
    )))
}

// =============================================================================
// Helpers
// =============================================================================

fn render_form(
    ts: &TenantScope,
    session: &SessionUser,
    form: &GuardianForm,
    id: Option<i64>,
    error: Option<&str>,
) -> Result<Response, WebError> {
    let is_edit = id.is_some();
    let title = if is_edit { "Edit guardian" } else { "New guardian" };
    let action_url = match id {
        Some(gid) => format!("/web/{}/guardians/{}", ts.tenant.as_str(), gid),
        None      => format!("/web/{}/guardians",     ts.tenant.as_str()),
    };
    let nav = NavContext::new(
        session.display.clone(),
        ts.tenant.as_str().to_string(),
        "guardians", title,
    );
    let nav_items = visible_nav_items(session);
    render(&FormPage { nav: &nav, nav_items, form, action_url, is_edit, error })
}

fn redirect(to: &str) -> Response {
    (
        StatusCode::SEE_OTHER,
        [(header::LOCATION, to.to_string())],
        Body::empty(),
    ).into_response()
}
