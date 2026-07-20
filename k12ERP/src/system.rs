//! Control-plane ("system") database + `SystemRegistry`.
//!
//! Holds a global list of tenants (name, status, plan) that gate whether a
//! given `TenantId` is allowed at request time. Kept in its own SQLite file
//! (default `data/system.db`) with its own migration set under `migrations_system/`.

use std::str::FromStr;
use std::time::Duration;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};
use crate::tenancy::{TenantError, TenantGuard, TenantId};

// -------- Pool + migrations --------

pub async fn connect_system(url: &str) -> RepoResult<SqlitePool> {
    let opts = SqliteConnectOptions::from_str(url)
        .map_err(RepoError::from)?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    Ok(SqlitePoolOptions::new()
        .max_connections(4)
        .connect_with(opts)
        .await?)
}

pub async fn migrate_system(pool: &SqlitePool) -> RepoResult<()> {
    sqlx::migrate!("./migrations_system").run(pool).await?;
    Ok(())
}

// -------- Types --------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Tenant {
    pub id: i64,
    pub tenant_id: String,
    pub name: String,
    pub status: String,     // active | disabled | deleted
    pub plan: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewTenant {
    pub tenant_id: String,
    pub name: String,
    pub plan: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UpdateTenant {
    pub name: Option<String>,
    pub plan: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
}

// -------- Registry --------

/// Repository-plus-service for the control-plane tenants table.
#[derive(Clone)]
pub struct SystemRegistry {
    pool: SqlitePool,
}

impl SystemRegistry {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }
    pub fn pool(&self) -> &SqlitePool { &self.pool }

    /// Clone the underlying pool handle. Cheap (`Arc` clone) and needed by
    /// [`crate::shutdown::close_pools`] so the system DB can be closed as
    /// part of graceful shutdown.
    pub fn pool_clone(&self) -> SqlitePool { self.pool.clone() }

    pub async fn create(&self, t: &NewTenant) -> RepoResult<Tenant> {
        // Validate tenant_id format up-front.
        let _ = TenantId::new(&t.tenant_id)
            .map_err(|e| RepoError::validation(e.to_string()))?;

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO tenant (tenant_id, name, plan, notes)
               VALUES (?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&t.tenant_id).bind(&t.name).bind(&t.plan).bind(&t.notes)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Tenant> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenant WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn find_by_tenant_id(&self, tid: &str) -> RepoResult<Option<Tenant>> {
        Ok(sqlx::query_as::<_, Tenant>("SELECT * FROM tenant WHERE tenant_id = ?")
            .bind(tid).fetch_optional(&self.pool).await?)
    }

    pub async fn list(&self) -> RepoResult<Vec<Tenant>> {
        Ok(sqlx::query_as::<_, Tenant>(
            "SELECT * FROM tenant WHERE status <> 'deleted' ORDER BY tenant_id",
        ).fetch_all(&self.pool).await?)
    }

    pub async fn update(&self, id: i64, u: &UpdateTenant) -> RepoResult<Tenant> {
        if let Some(s) = &u.status {
            if !matches!(s.as_str(), "active"|"disabled"|"deleted") {
                return Err(RepoError::validation("status must be active|disabled|deleted"));
            }
        }
        sqlx::query(
            r#"UPDATE tenant SET
                 name       = COALESCE(?, name),
                 plan       = COALESCE(?, plan),
                 notes      = COALESCE(?, notes),
                 status     = COALESCE(?, status),
                 updated_at = datetime('now')
               WHERE id = ?"#,
        )
        .bind(&u.name).bind(&u.plan).bind(&u.notes).bind(&u.status).bind(id)
        .execute(&self.pool).await?;
        self.get(id).await
    }

    pub async fn set_status(&self, id: i64, status: &str) -> RepoResult<()> {
        self.update(id, &UpdateTenant {
            status: Some(status.into()), ..Default::default()
        }).await.map(|_| ())
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        // Soft delete — actual file cleanup is a separate ops task.
        self.set_status(id, "deleted").await
    }
}

// -------- Tenant guard backed by SystemRegistry --------

#[derive(Clone)]
pub struct DbTenantGuard {
    pub system: SystemRegistry,
}

#[async_trait::async_trait]
impl TenantGuard for DbTenantGuard {
    async fn admit(&self, tenant: &TenantId) -> Result<(), TenantError> {
        let row = self.system
            .find_by_tenant_id(tenant.as_str())
            .await
            .map_err(TenantError::from)?;
        match row {
            None => Err(TenantError::NotFound(tenant.clone())),
            Some(t) if t.status != "active" => Err(TenantError::Disabled(tenant.clone())),
            Some(_) => Ok(()),
        }
    }
}
