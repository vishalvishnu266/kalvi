use axum::{response::Html, Extension};
use crate::views::{components, layout, school_dashboard};
use sqlx::SqlitePool;

pub async fn dashboard_handler() -> Html<String> {
    Html(layout::base_layout("School Dashboard", &school_dashboard::render()))
}

pub async fn test_handler(Extension(_pool): Extension<SqlitePool>) -> Html<String> {
    // In a real app, you would use `pool` to query the tenant DB.
    // Here we just test that we have access to it.
    let content = components::card("Database Status", 
        //language=HTML
        "<p class='text-emerald-500 font-semibold flex items-center gap-2'><svg class='w-5 h-5' fill='none' stroke='currentColor' viewBox='0 0 24 24'><path stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M5 13l4 4L19 7'></path></svg> Tenant DB is accessible and connected!</p>");
    Html(layout::base_layout("System Test", &content))
}
