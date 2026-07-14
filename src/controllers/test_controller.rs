use axum::{response::Html, Extension};
use crate::views::{components, layout};
use sqlx::SqlitePool;

pub async fn test_handler(Extension(pool): Extension<SqlitePool>) -> Html<String> {
    // In a real app, you would use `pool` to query the tenant DB.
    // Here we just test that we have access to it.
    Html("Tenant DB is accessible!".to_string())
}
