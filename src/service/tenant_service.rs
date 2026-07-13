use sqlx::SqlitePool;
use bcrypt::{hash, DEFAULT_COST};
use crate::model::Tenant;
use crate::repository::{TenantRepository, UserRepository};
use crate::config::AppState;
use crate::util::AppError;

pub struct TenantService;

impl TenantService {
    pub async fn find_by_slug(state: &AppState, slug: &str) -> Result<Option<Tenant>, AppError> {
        Ok(TenantRepository::find_by_slug(&state.db.master_pool, slug).await?)
    }

    pub async fn get_tenant_for_session(
        state: &AppState,
        session_id: &Option<String>,
        tenant_hint: &Option<String>,
    ) -> Option<Tenant> {
        if let (Some(sid), Some(slug)) = (session_id, tenant_hint) {
            if let Ok(Some(tenant)) = TenantRepository::find_by_slug(&state.db.master_pool, &slug).await {
                if let Ok(tenant_pool) = state.db.get_tenant_pool(&tenant.database_name).await {
                    if let Ok(Some(_)) = UserRepository::find_session(&tenant_pool, &sid).await {
                        return Some(tenant);
                    }
                }
            }
        }
        None
    }

    pub async fn create_tenant(
        state: &AppState,
        name: &str,
        slug: &str,
        admin_username: &str,
        admin_password: &str,
    ) -> Result<Tenant, AppError> {
        let slug = slug.trim().to_lowercase();
        
        // 1. Logic Validation (Outside Transaction for speed)
        if !slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(AppError::Internal("Slug must only contain letters, numbers, and hyphens".to_string()));
        }
        let reserved = vec!["saas", "api", "web", "health", "contact", "login", "registration"];
        if reserved.contains(&slug.as_str()) || slug.is_empty() || name.trim().is_empty() {
            return Err(AppError::Internal("Invalid name or reserved slug".to_string()));
        }

        // 2. Start Transaction on Master Pool
        let mut tx = state.db.master_pool.begin().await?;

        // 3. Check for existence inside TX
        if let Some(_) = TenantRepository::find_by_slug(&mut *tx, &slug).await? {
             return Err(AppError::Internal("Slug is already taken".to_string()));
        }

        let db_name = slug.clone();
        
        // 4. Save Tenant to Master DB (Pass the Transaction)
        let tenant = TenantRepository::save(&mut tx, &slug, name, &db_name).await?;

        // 5. Commit Master Changes
        tx.commit().await?;

        // 6. Provision Tenant DB (This is separate as it creates a file/pool)
        let tenant_pool = state.db.get_tenant_pool(&tenant.database_name).await?;

        // 7. Start Transaction on Tenant Pool
        let mut tenant_tx = tenant_pool.begin().await?;

        // 8. Create Admin User inside Tenant TX
        let hashed_pw = hash(admin_password, DEFAULT_COST).unwrap();
        sqlx::query("INSERT INTO users (username, password_hash, role) VALUES (?, ?, 'admin')")
            .bind(admin_username)
            .bind(hashed_pw)
            .execute(&mut *tenant_tx)
            .await?;

        // 9. Final Commit
        tenant_tx.commit().await?;

        Ok(tenant)
    }

    pub async fn update_tenant_settings(
        state: &AppState,
        slug: &str,
        primary_color: &str,
        dark_mode: bool,
    ) -> Result<(), AppError> {
        Ok(TenantRepository::update_settings(&state.db.master_pool, slug, primary_color, dark_mode).await?)
    }
}
