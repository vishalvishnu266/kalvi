use askama::Template;
use axum::{extract::Path, response::Html, Extension};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::student::Student;
use crate::services::student_service::StudentService;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTpl {
    tenant_id: String,
    active: &'static str,
    student_count: i64,

    total_students: i64,
    active_students: i64,
    pending_students: i64,
    inactive_students: i64,

    recent_students: Vec<Student>,
}

pub async fn tenant_dashboard_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Html<String>, AppError> {
    let (total_students, active_students, pending_students, inactive_students, recent_students) =
        StudentService::get_dashboard_stats(&pool).await?;

    let tpl = DashboardTpl {
        tenant_id,
        active: "dashboard",
        student_count: total_students,
        total_students,
        active_students,
        pending_students,
        inactive_students,
        recent_students,
    };
    Ok(Html(tpl.render()?))
}
