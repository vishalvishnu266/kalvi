//! Demo domain model — a single tiny row type used purely to show the
//! shape of a module. Delete this file once real modules replace it.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DemoMessage {
    pub id: i64,
    pub text: String,
    pub created_by: Option<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewDemoMessage {
    pub text: String,
}
