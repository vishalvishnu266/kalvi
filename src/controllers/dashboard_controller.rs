//! Landing page after login — per-tenant summary stats + recent students.
use askama::Template;
use axum::{extract::Path, response::Html, Extension};
use sqlx::SqlitePool;

use crate::auth_middleware::CurrentUser;
use crate::errors::AppError;
use crate::models::student::Student;
use crate::utils::page::PageChrome;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTpl {
    chrome: PageChrome,

    total_students: i64,
    active_students: i64,
    pending_students: i64,
    inactive_students: i64,

    recent_students: Vec<Student>,
}

pub async fn tenant_dashboard_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
) -> Result<Html<String>, AppError> {
    let chrome = PageChrome::load(&pool, &tenant_id, "dashboard", &cu).await?;

    let total_students: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM students").fetch_one(&pool).await?;
    let active_students: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE status = 'Active'")
            .fetch_one(&pool)
            .await?;
    let pending_students: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE status = 'Pending'")
            .fetch_one(&pool)
            .await?;
    let inactive_students: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE status = 'Inactive'")
            .fetch_one(&pool)
            .await?;

    let recent_students = sqlx::query_as::<_, Student>(
        "SELECT * FROM students ORDER BY created_at DESC LIMIT 5",
    )
    .fetch_all(&pool)
    .await?;

    let tpl = DashboardTpl {
        chrome,
        total_students,
        active_students,
        pending_students,
        inactive_students,
        recent_students,
    };
    Ok(Html(tpl.render()?))
}
