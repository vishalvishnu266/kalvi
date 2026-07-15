use axum::{response::Html, Extension, extract::Path};
use crate::views::school_dashboard;
use sqlx::SqlitePool;
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub grade: String,
    pub section: String,
    pub status: String,
    pub attendance_pct: f64,
}

pub async fn tenant_dashboard_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>
) -> Html<String> {
    // Fetch students from tenant DB
    let students = sqlx::query_as::<_, Student>("SELECT * FROM students LIMIT 10")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Html(school_dashboard::render(&tenant_id, students))
}
