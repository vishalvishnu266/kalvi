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
        let sid = session_id.as_ref()?;
        let slug = tenant_hint.as_ref()?;
        
        let tenant = TenantRepository::find_by_slug(&state.db.master_pool, slug).await.ok()??;
        let tenant_pool = state.db.get_tenant_pool(&tenant.database_name).await.ok()?;
        
        UserRepository::find_session(&tenant_pool, sid).await.ok()??;
        Some(tenant)
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
        use crate::util::html_util;
        if !html_util::is_valid_slug(&slug) || name.trim().is_empty() {
            return Err(AppError::Internal("Invalid name or reserved slug".to_string()));
        }

        // 2. Start Transaction on Master Pool
        let mut tx = state.db.master_pool.begin().await?;

        // 3. Check for existence inside TX
        if TenantRepository::find_by_slug(&mut *tx, &slug).await?.is_some() {
             return Err(AppError::Internal("Slug is already taken".to_string()));
        }

        // 4. Save Tenant to Master DB
        let tenant = TenantRepository::save(&mut *tx, &slug, name, &slug).await?;
        tx.commit().await?;

        // 5. Provision and Initialize Tenant DB
        let tenant_pool = state.db.get_tenant_pool(&tenant.database_name).await?;
        let mut tenant_tx = tenant_pool.begin().await?;

        // 6. Create Admin User
        let hashed_pw = hash(admin_password, DEFAULT_COST).unwrap();
        UserRepository::create(&mut *tenant_tx, admin_username, &hashed_pw, "admin").await?;
        
        tenant_tx.commit().await?;

        Ok(tenant)
    }

}
