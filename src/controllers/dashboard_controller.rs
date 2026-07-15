use askama::Template;
use axum::{extract::Path, response::Html, Extension};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::student::Student;

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
    let total_students: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM students").fetch_one(&pool).await?;
    let active_students: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE status = 'Active'")
            .fetch_one(&pool).await?;
    let pending_students: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE status = 'Pending'")
            .fetch_one(&pool).await?;
    let inactive_students: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE status IN ('Inactive','Suspended')")
            .fetch_one(&pool).await?;

    let recent_students =
        sqlx::query_as::<_, Student>(
            "SELECT * FROM students ORDER BY created_at DESC, admission_date DESC LIMIT 5",
        )
        .fetch_all(&pool)
        .await?;

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
