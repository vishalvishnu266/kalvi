//! Students list + detail pages: `/web/{tenant}/students[/{id}]`.
//!
//! ### RBAC
//! The route layer already guarantees the caller holds either
//! `students.view` or `students.view_own`. Here we translate the session
//! into a [`Scope`] so the list only returns rows the caller is allowed to
//! see (parents get their own children, students get themselves, admins get
//! everyone), and the detail handler verifies ownership before rendering.

use askama::Template;
use axum::{
    extract::{Path, Query},
    response::Response,
    Extension,
};
use serde::Deserialize;
use tracing::log::info;
use crate::http::TenantScope;
use crate::repositories::students::Student;
use crate::middleware::auth::SessionUser;
use crate::services::people::Scope;
use crate::services::perm;
use crate::web::error::{render, WebError};
use crate::web::layout::{visible_nav_items, NavContext, NavItem};

// ------------- list -------------

#[derive(Template)]
#[template(path = "students/list.html")]
struct StudentsListPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    q: &'a str,
    rows: Vec<StudentRow>,
    /// Passed through so the template can `{% if can_admit %}...{% endif %}`
    /// the "Admit student" CTA. Set from the caller's permissions.
    can_admit: bool,
}

pub struct StudentRow {
    pub id: i64,
    pub name: String,
    pub admission_no: String,
    pub grade: String,
    pub section: String,
    pub status: &'static str,
}

#[derive(Deserialize)]
pub struct ListParams { q: Option<String> }

pub async fn list(
    tscope: TenantScope,
    Query(qp): Query<ListParams>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    // Row-level filter driven by the caller's roles/permissions.
    let scope = Scope::from_session(&session);
    let students: Vec<Student> = tscope.services.people
        .list_students_for(scope, 50).await?;
    info!(rows = students.len(), "student list");

    let q = qp.q.unwrap_or_default();
    let ql = q.to_lowercase();
    let rows: Vec<StudentRow> = students.into_iter().filter_map(|s| {
        let name = display_name(&s);
        if !ql.is_empty()
            && !name.to_lowercase().contains(&ql)
            && !s.admission_no.to_lowercase().contains(&ql)
        { return None; }
        Some(StudentRow {
            id: s.id,
            name,
            admission_no: s.admission_no,
            grade:   "—".into(),
            section: "—".into(),
            status:  if s.status == "active" { "active" } else { "inactive" },
        })
    }).collect();

    let nav = NavContext::new(
        session.display.clone(),
        tscope.tenant.as_str().to_string(),
        "students", "Students",
    );
    let nav_items = visible_nav_items(&session);
    let can_admit = session.has(perm::STUDENTS_ADMIT);

    render(&StudentsListPage { nav: &nav, nav_items, q: &q, rows, can_admit })
}

// ------------- detail -------------

#[derive(Template)]
#[template(path = "students/show.html")]
struct StudentShowPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    student: StudentRow,
    tabs: Vec<Tab>,
    can_edit: bool,
}

pub struct Tab {
    pub label: &'static str,
    pub active: bool,
}

pub async fn show(
    tscope: TenantScope,
    Path((_tenant, id)): Path<(String, i64)>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    // Ownership check: a parent must be linked to this student, a student
    // user must be viewing their own profile, admins/staff-with-view pass
    // straight through. Anything else → 403.
    let scope = Scope::from_session(&session);
    if !tscope.services.people.can_view_student(&scope, id).await? {
        return Err(WebError::forbidden("not permitted to view this student"));
    }

    let s = tscope.services.repos.students.get(id).await?;
    let student = StudentRow {
        id: s.id,
        name: display_name(&s),
        admission_no: s.admission_no,
        grade: "—".into(),
        section: "—".into(),
        status: if s.status == "active" { "active" } else { "inactive" },
    };

    let title = format!("Students · {}", student.name);
    let nav = NavContext::new(
        session.display.clone(),
        tscope.tenant.as_str().to_string(),
        "students", title,
    );

    let current = "overview";
    let tabs = ["overview","attendance","fees","guardians","documents"]
        .into_iter()
        .map(|l| Tab { label: l, active: l == current })
        .collect();

    let nav_items = visible_nav_items(&session);
    let can_edit = session.has(perm::STUDENTS_EDIT);
    render(&StudentShowPage { nav: &nav, nav_items, student, tabs, can_edit })
}

// ------------- helpers -------------

fn display_name(s: &Student) -> String {
    let mid = s.middle_name.as_deref().map(|m| format!(" {m}")).unwrap_or_default();
    format!("{}{} {}", s.first_name, mid, s.last_name)
}
