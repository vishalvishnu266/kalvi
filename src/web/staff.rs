use askama::Template;
use axum::{
    extract::{Path, Query},
    response::Response,
    Extension,
};
use serde::Deserialize;
use tracing::log::info;

use crate::http::TenantScope;
use crate::middleware::auth::SessionUser;
use crate::models::people::Staff;
use crate::services::{people as people_svc, perm};
use crate::web::error::{render, WebError};
use crate::web::layout::{visible_nav_items, NavContext, NavItem};

#[derive(Template)]
#[template(path = "staff/list.html")]
struct StaffListPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    q: &'a str,
    rows: Vec<StaffRow>,
}

pub struct StaffRow {
    pub id: i64,
    pub name: String,
    pub employee_no: String,
    pub designation: String,
    pub department: String,
    pub email: String,
    pub phone: String,
    pub employment_type: String,
    pub status: &'static str,
}

#[derive(Deserialize)]
pub struct ListParams {
    q: Option<String>,
}

pub async fn list(
    scope: TenantScope,
    Query(qp): Query<ListParams>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    scope.ctx.require(perm::STAFF_VIEW)?;
    let staff: Vec<Staff> = people_svc::list_staff(&scope.pool, 50, 0)
        .await
        .unwrap_or_default();
    info!("staff list");
    let q = qp.q.unwrap_or_default();
    let ql = q.to_lowercase();

    let rows: Vec<StaffRow> = staff
        .into_iter()
        .filter_map(|s| {
            let name = display_name(&s);
            if !ql.is_empty()
                && !name.to_lowercase().contains(&ql)
                && !s.employee_no.to_lowercase().contains(&ql)
                && !s
                    .designation
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&ql)
            {
                return None;
            }
            Some(row_from(&s, name))
        })
        .collect();

    let nav = NavContext::new(
        session.display.clone(),
        scope.tenant.as_str().to_string(),
        "staff",
        "Staff",
    );
    let nav_items = visible_nav_items(&session);

    render(&StaffListPage {
        nav: &nav,
        nav_items,
        q: &q,
        rows,
    })
}

#[derive(Template)]
#[template(path = "staff/show.html")]
struct StaffShowPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    staff: StaffRow,
    tabs: Vec<Tab>,
}

pub struct Tab {
    pub label: &'static str,
    pub active: bool,
}

pub async fn show(
    scope: TenantScope,
    Path((_tenant, id)): Path<(String, i64)>,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    scope.ctx.require(perm::STAFF_VIEW)?;
    let s = people_svc::get_staff(&scope.pool, id).await?;
    let name = display_name(&s);
    let staff = row_from(&s, name);

    let title = format!("Staff · {}", staff.name);
    let nav = NavContext::new(
        session.display.clone(),
        scope.tenant.as_str().to_string(),
        "staff",
        title,
    );

    let current = "overview";
    let tabs = ["overview", "attendance", "payroll", "classes", "documents"]
        .into_iter()
        .map(|l| Tab {
            label: l,
            active: l == current,
        })
        .collect();

    let nav_items = visible_nav_items(&session);
    render(&StaffShowPage {
        nav: &nav,
        nav_items,
        staff,
        tabs,
    })
}

fn display_name(s: &Staff) -> String {
    format!("{} {}", s.first_name, s.last_name)
}

fn row_from(s: &Staff, name: String) -> StaffRow {
    StaffRow {
        id: s.id,
        name,
        employee_no: s.employee_no.clone(),
        designation: s.designation.clone().unwrap_or_else(|| "—".into()),
        department: "—".into(),
        email: s.email.clone().unwrap_or_else(|| "—".into()),
        phone: s.phone.clone().unwrap_or_else(|| "—".into()),
        employment_type: s.employment_type.clone().unwrap_or_else(|| "—".into()),
        status: if s.status == "active" {
            "active"
        } else {
            "inactive"
        },
    }
}
