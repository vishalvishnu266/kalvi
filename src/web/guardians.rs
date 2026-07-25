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
use crate::models::guardians::{Guardian, NewGuardian, UpdateGuardian};
use crate::models::people::Student;
use crate::services::{guardians as g_svc, people as people_svc, perm};
use crate::web::error::{render, WebError};
use crate::web::layout::{visible_nav_items, NavContext, NavItem};

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
    linked: Vec<Student>,
    can_manage: bool,
}

#[derive(Template)]
#[template(path = "guardians/form.html")]
struct FormPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    form: &'a GuardianForm,
    action_url: String,
    is_edit: bool,
    error: Option<&'a str>,
}

#[derive(Deserialize, Default, Clone)]
pub struct GuardianForm {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub occupation: String,
    pub address: String,
}

impl GuardianForm {
    fn from_guardian(g: &Guardian) -> Self {
        Self {
            first_name: g.first_name.clone(),
            last_name: g.last_name.clone(),
            phone: g.phone.clone().unwrap_or_default(),
            email: g.email.clone().unwrap_or_default(),
            occupation: g.occupation.clone().unwrap_or_default(),
            address: g.address.clone().unwrap_or_default(),
        }
    }

    fn validate(&self) -> Option<&'static str> {
        if self.first_name.trim().is_empty() {
            return Some("First name is required");
        }
        if self.last_name.trim().is_empty() {
            return Some("Last name is required");
        }
        None
    }

    fn to_new(&self) -> NewGuardian {
        NewGuardian {
            user_id: None,
            first_name: self.first_name.trim().to_string(),
            last_name: self.last_name.trim().to_string(),
            phone: opt(&self.phone),
            email: opt(&self.email),
            occupation: opt(&self.occupation),
            address: opt(&self.address),
        }
    }

    fn to_update(&self) -> UpdateGuardian {
        UpdateGuardian {
            first_name: Some(self.first_name.trim().to_string()),
            last_name: Some(self.last_name.trim().to_string()),
            phone: Some(self.phone.trim().to_string()),
            email: Some(self.email.trim().to_string()),
            occupation: Some(self.occupation.trim().to_string()),
            address: Some(self.address.trim().to_string()),
        }
    }
}

fn opt(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub q: Option<String>,
    pub flash: Option<String>,
}

pub async fn list(
    ts: TenantScope,
    axum::extract::Query(qp): axum::extract::Query<ListQuery>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    ts.ctx
        .require_any(&[perm::GUARDIANS_VIEW, perm::GUARDIANS_MANAGE])?;
    let all = g_svc::list(&ts.pool, 200, 0).await.unwrap_or_default();
    let q = qp.q.unwrap_or_default();
    let ql = q.to_lowercase();
    let rows: Vec<Guardian> = all
        .into_iter()
        .filter(|g| {
            if ql.is_empty() {
                return true;
            }
            let name = format!("{} {}", g.first_name, g.last_name).to_lowercase();
            name.contains(&ql)
                || g.email
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&ql)
                || g.phone
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&ql)
        })
        .collect();

    let nav = NavContext::new(
        session.display.clone(),
        ts.tenant.as_str().to_string(),
        "guardians",
        "Guardians",
    );
    let nav_items = visible_nav_items(&session);
    let can_manage = session.has(perm::GUARDIANS_MANAGE);

    render(&ListPage {
        nav: &nav,
        nav_items,
        q: &q,
        rows,
        can_manage,
        flash: qp.flash.as_deref(),
    })
}

pub async fn show(
    ts: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    ts.ctx
        .require_any(&[perm::GUARDIANS_VIEW, perm::GUARDIANS_MANAGE])?;
    let g = g_svc::get(&ts.pool, id).await?;
    let student_ids = g_svc::students_of_guardian(&ts.pool, id)
        .await
        .unwrap_or_default();
    let linked = people_svc::list_students_by_ids(&ts.pool, &student_ids)
        .await
        .unwrap_or_default();

    let title = format!("Guardians · {} {}", g.first_name, g.last_name);
    let nav = NavContext::new(
        session.display.clone(),
        ts.tenant.as_str().to_string(),
        "guardians",
        title,
    );
    let nav_items = visible_nav_items(&session);
    let can_manage = session.has(perm::GUARDIANS_MANAGE);

    render(&ShowPage {
        nav: &nav,
        nav_items,
        g: &g,
        linked,
        can_manage,
    })
}

pub async fn new_form(
    ts: TenantScope,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    ts.ctx.require(perm::GUARDIANS_MANAGE)?;
    let form = GuardianForm::default();
    render_form(&ts, &session, &form, None, None)
}

pub async fn create(
    ts: TenantScope,
    Extension(session): Extension<SessionUser>,
    Form(f): Form<GuardianForm>,
) -> Result<Response, WebError> {
    ts.ctx.require(perm::GUARDIANS_MANAGE)?;
    if let Some(err) = f.validate() {
        return render_form(&ts, &session, &f, None, Some(err));
    }
    match g_svc::create(&ts.pool, &f.to_new()).await {
        Ok(g) => Ok(redirect(&format!(
            "/web/{}/guardians/{}?flash=Guardian+created",
            ts.tenant.as_str(),
            g.id
        ))),
        Err(e) => render_form(&ts, &session, &f, None, Some(&e.to_string())),
    }
}

pub async fn edit_form(
    ts: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    ts.ctx.require(perm::GUARDIANS_MANAGE)?;
    let g = g_svc::get(&ts.pool, id).await?;
    let form = GuardianForm::from_guardian(&g);
    render_form(&ts, &session, &form, Some(id), None)
}

pub async fn update(
    ts: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Extension(session): Extension<SessionUser>,
    Form(f): Form<GuardianForm>,
) -> Result<Response, WebError> {
    ts.ctx.require(perm::GUARDIANS_MANAGE)?;
    if let Some(err) = f.validate() {
        return render_form(&ts, &session, &f, Some(id), Some(err));
    }
    match g_svc::update(&ts.pool, id, &f.to_update()).await {
        Ok(_) => Ok(redirect(&format!(
            "/web/{}/guardians/{}?flash=Guardian+updated",
            ts.tenant.as_str(),
            id
        ))),
        Err(e) => render_form(&ts, &session, &f, Some(id), Some(&e.to_string())),
    }
}

pub async fn delete(
    ts: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<Response, WebError> {
    ts.ctx.require(perm::GUARDIANS_MANAGE)?;
    g_svc::delete(&ts.pool, id).await?;
    Ok(redirect(&format!(
        "/web/{}/guardians?flash=Guardian+deleted",
        ts.tenant.as_str()
    )))
}

fn render_form(
    ts: &TenantScope,
    session: &SessionUser,
    form: &GuardianForm,
    id: Option<i64>,
    error: Option<&str>,
) -> Result<Response, WebError> {
    let is_edit = id.is_some();
    let title = if is_edit {
        "Edit guardian"
    } else {
        "New guardian"
    };
    let action_url = match id {
        Some(gid) => format!("/web/{}/guardians/{}", ts.tenant.as_str(), gid),
        None => format!("/web/{}/guardians", ts.tenant.as_str()),
    };
    let nav = NavContext::new(
        session.display.clone(),
        ts.tenant.as_str().to_string(),
        "guardians",
        title,
    );
    let nav_items = visible_nav_items(session);
    render(&FormPage {
        nav: &nav,
        nav_items,
        form,
        action_url,
        is_edit,
        error,
    })
}

fn redirect(to: &str) -> Response {
    (
        StatusCode::SEE_OTHER,
        [(header::LOCATION, to.to_string())],
        Body::empty(),
    )
        .into_response()
}
