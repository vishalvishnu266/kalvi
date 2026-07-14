use axum::{response::Html, Extension};
use crate::views::{components, layout};
use sqlx::SqlitePool;

pub async fn test_handler(Extension(pool): Extension<SqlitePool>) -> Html<String> {
    // In a real app, you would use `pool` to query the tenant DB.
    // Here we just test that we have access to it.
    let content = format!(
        "<h1>Test Page</h1><p>DB pool connected: {}</p>{}",
        pool.is_closed() == false,
        components::form_input("Username", "username", Some("This field is required"))
    );
    Html(layout::base_layout("Test Page", &content))
}
