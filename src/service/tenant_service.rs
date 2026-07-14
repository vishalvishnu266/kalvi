use sqlx::SqlitePool;
use crate::config::database_config::DatabaseConfig;
use crate::repository::tenant_repository::Tenant;
use crate::util::errors::AppError;
use crate::util::id_util::current_timestamp;
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;

pub struct TenantService;

impl TenantService {
    pub async fn register_tenant(
        db_config: Arc<DatabaseConfig>,
        slug: String,
        name: String,
        admin_username: String,
        admin_password: String,
    ) -> Result<Tenant, AppError> {
        // 1. Validate (Simple manual check)
        if slug.is_empty() || name.is_empty() || admin_username.is_empty() || admin_password.is_empty() {
            return Err(AppError::BusinessException {
                errors: std::collections::HashMap::new(),
                message: Some("All fields are required".to_string()),
            });
        }

        // 2. Start Master DB Transaction
        let mut tx = db_config.master_pool.begin().await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        let database_name = format!("tenant_{}.db", slug);
        let now = current_timestamp();

        // 3. Save Tenant record to Master
        sqlx::query("INSERT INTO tenants (slug, name, database_name, created_at) VALUES (?, ?, ?, ?)")
            .bind(&slug)
            .bind(&name)
            .bind(&database_name)
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        // 4. Initialize Tenant Database (Triggers migrations)
        let tenant_pool = db_config.get_tenant_pool(&slug).await?;

        // 5. Create Admin User in Tenant DB
        let password_hash = hash(admin_password, DEFAULT_COST)
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        sqlx::query("INSERT INTO users (username, password_hash, role, full_name, created_at) VALUES (?, ?, 'admin', 'System Administrator', ?)")
            .bind(admin_username)
            .bind(password_hash)
            .bind(now)
            .execute(&tenant_pool)
            .await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        // 6. Commit Master Transaction
        tx.commit().await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        Ok(Tenant {
            id: 0, // Simplified for now
            slug,
            name,
            database_name,
            created_at: now,
        })
    }
}
