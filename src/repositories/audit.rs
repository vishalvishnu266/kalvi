//! Audit log: append-only trail of user actions.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::RepoResult;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: i64,
    pub user_id: Option<i64>,
    pub entity: String,
    pub entity_id: i64,
    pub action: String,
    pub diff_json: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAudit {
    pub user_id: Option<i64>,
    pub entity: String,
    pub entity_id: i64,
    pub action: String,   // create|update|delete|login|logout|export
    pub diff_json: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Clone)]
pub struct AuditLogRepo { pool: SqlitePool }

impl AuditLogRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn record(&self, a: &NewAudit) -> RepoResult<i64> {
        let json = a.diff_json.as_ref().map(|v| v.to_string());
        Ok(sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO audit_log
                 (user_id, entity, entity_id, action, diff_json, ip_address, user_agent)
               VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(a.user_id).bind(&a.entity).bind(a.entity_id).bind(&a.action)
        .bind(json).bind(&a.ip_address).bind(&a.user_agent)
        .fetch_one(&self.pool).await?)
    }

    pub async fn for_entity(&self, entity: &str, entity_id: i64, limit: i64)
        -> RepoResult<Vec<AuditEntry>>
    {
        Ok(sqlx::query_as::<_, AuditEntry>(
            r#"SELECT * FROM audit_log
               WHERE entity = ? AND entity_id = ?
               ORDER BY created_at DESC LIMIT ?"#,
        ).bind(entity).bind(entity_id).bind(limit).fetch_all(&self.pool).await?)
    }

    pub async fn for_user(&self, user_id: i64, limit: i64) -> RepoResult<Vec<AuditEntry>> {
        Ok(sqlx::query_as::<_, AuditEntry>(
            r#"SELECT * FROM audit_log
               WHERE user_id = ?
               ORDER BY created_at DESC LIMIT ?"#,
        ).bind(user_id).bind(limit).fetch_all(&self.pool).await?)
    }
}
