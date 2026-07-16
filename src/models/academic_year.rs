//! Academic Year (per tenant).
//!
//! Almost every other module (enrollments, attendance, exams, fees, timetables)
//! references an academic year. Only ONE AY can be `is_current = 1` at a time,
//! enforced by a partial unique index in the migration.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct AcademicYear {
    pub id: String,
    pub name: String,        // "2025-26"
    pub start_date: String,  // ISO date
    pub end_date: String,    // ISO date
    pub is_current: i64,     // SQLite boolean (0/1)
    pub status: String,      // "active" | "archived"
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl AcademicYear {
    pub fn is_current_bool(&self) -> bool {
        self.is_current == 1
    }
    pub fn is_archived(&self) -> bool {
        self.status == "archived"
    }

    pub fn status_badge_class(&self) -> &'static str {
        if self.is_current_bool() {
            "badge-success-soft"
        } else if self.is_archived() {
            "badge-danger-soft"
        } else {
            "badge-info-soft"
        }
    }
}

/// Form params for create/update.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct AcademicYearForm {
    pub csrf_token: Option<String>,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    #[serde(default)]
    pub status: String,
}

impl AcademicYearForm {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name is required (e.g. '2025-26').".into());
        }
        if self.start_date.trim().is_empty() {
            return Err("Start date is required.".into());
        }
        if self.end_date.trim().is_empty() {
            return Err("End date is required.".into());
        }
        if self.start_date >= self.end_date {
            return Err("Start date must be before end date.".into());
        }
        Ok(())
    }

    pub fn effective_status(&self) -> &str {
        let s = self.status.trim();
        if s.is_empty() { "active" } else { s }
    }
}

// -------------------------------------------------------------------
// Simple key/value settings helper.
// -------------------------------------------------------------------
pub struct TenantSetting;

impl TenantSetting {
    pub async fn get(pool: &SqlitePool, key: &str) -> Result<Option<String>, AppError> {
        Ok(sqlx::query_scalar::<_, String>(
            "SELECT value FROM tenant_settings WHERE key = ?",
        )
        .bind(key)
        .fetch_optional(pool)
        .await?)
    }

    pub async fn get_or(pool: &SqlitePool, key: &str, default: &str) -> Result<String, AppError> {
        Ok(Self::get(pool, key).await?.unwrap_or_else(|| default.to_string()))
    }

    pub async fn set(pool: &SqlitePool, key: &str, value: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"INSERT INTO tenant_settings (key, value, updated_at)
               VALUES (?, ?, CURRENT_TIMESTAMP)
               ON CONFLICT(key) DO UPDATE
                   SET value = excluded.value, updated_at = CURRENT_TIMESTAMP"#,
        )
        .bind(key)
        .bind(value)
        .execute(pool)
        .await?;
        Ok(())
    }
}
