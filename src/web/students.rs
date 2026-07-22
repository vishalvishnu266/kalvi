//! Students list + detail pages: `/web/{tenant}/students[/{id}]`.

use askama::Template;
use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::Response,
};
use serde::Deserialize;

use crate::http::TenantScope;
use crate::repositories::students::Student;
use crate::web::auth::read_cookie_from_headers;
use crate::web::error::{render, WebError};
use crate::web::layout::{nav_items, NavContext, NavItem};

// ------------- list -------------

#[derive(Template)]
#[template(path = "students/list.html")]
struct StudentsListPage<'a> {
    nav: &'a NavContext,
    nav_items: &'static [NavItem],
    q: &'a str,
    rows: Vec<StudentRow>,
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
    scope: TenantScope,
    Query(qp): Query<ListParams>,
    headers: HeaderMap,
) -> Result<Response, WebError> {
    let students: Vec<Student> = scope.services.repos.students.list(50, 0).await
        .unwrap_or_default();

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

    let rows = if rows.is_empty() && q.is_empty() { sample_students() } else { rows };

    let user = read_cookie_from_headers(&headers, "erp_user")
        .unwrap_or_else(|| "Admin".into());
    let nav = NavContext::new(user, scope.tenant.as_str().to_string(), "students", "Students");

    render(&StudentsListPage { nav: &nav, nav_items: nav_items(), q: &q, rows })
}

// ------------- detail -------------

#[derive(Template)]
#[template(path = "students/show.html")]
struct StudentShowPage<'a> {
    nav: &'a NavContext,
    nav_items: &'static [NavItem],
    student: StudentRow,
    tabs: Vec<Tab>,
}

pub struct Tab {
    pub label: &'static str,
    pub active: bool,
}

pub async fn show(
    scope: TenantScope,
    Path((_tenant, id)): Path<(String, i64)>,
    headers: HeaderMap,
) -> Result<Response, WebError> {
    let s = scope.services.repos.students.get(id).await.ok();
    let student = match s {
        Some(s) => StudentRow {
            id: s.id,
            name: display_name(&s),
            admission_no: s.admission_no,
            grade: "—".into(),
            section: "—".into(),
            status: if s.status == "active" { "active" } else { "inactive" },
        },
        None => sample_students().into_iter().find(|r| r.id == id)
            .unwrap_or_else(|| sample_students().remove(0)),
    };

    let user = read_cookie_from_headers(&headers, "erp_user")
        .unwrap_or_else(|| "Admin".into());
    let title = format!("Students · {}", student.name);
    let nav = NavContext::new(user, scope.tenant.as_str().to_string(), "students", title);

    let current = "overview";
    let tabs = ["overview","attendance","fees","guardians","documents"]
        .into_iter()
        .map(|l| Tab { label: l, active: l == current })
        .collect();

    render(&StudentShowPage { nav: &nav, nav_items: nav_items(), student, tabs })
}

// ------------- helpers -------------

fn display_name(s: &Student) -> String {
    let mid = s.middle_name.as_deref().map(|m| format!(" {m}")).unwrap_or_default();
    format!("{}{} {}", s.first_name, mid, s.last_name)
}

fn sample_students() -> Vec<StudentRow> {
    vec![
        StudentRow { id: 1, name: "Aarav Sharma".into(), admission_no: "ADM-2025-0001".into(), grade: "Grade 8".into(),  section: "A".into(), status: "active"   },
        StudentRow { id: 2, name: "Diya Patel".into(),   admission_no: "ADM-2025-0002".into(), grade: "Grade 10".into(), section: "B".into(), status: "active"   },
        StudentRow { id: 3, name: "Kabir Khan".into(),   admission_no: "ADM-2025-0003".into(), grade: "Grade 6".into(),  section: "C".into(), status: "active"   },
        StudentRow { id: 4, name: "Ananya Rao".into(),   admission_no: "ADM-2025-0004".into(), grade: "Grade 12".into(), section: "A".into(), status: "inactive" },
        StudentRow { id: 5, name: "Vihaan Mehta".into(), admission_no: "ADM-2025-0005".into(), grade: "Grade 9".into(),  section: "B".into(), status: "active"   },
    ]
}
