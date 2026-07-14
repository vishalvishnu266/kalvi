use crate::config::AppState;
use crate::model::Tenant;
use crate::repository::TenantRepository;
use crate::util::AppError;

pub struct TenantService;

use bcrypt::{hash, DEFAULT_COST};
use crate::repository::UserRepository;

impl TenantService {
    pub async fn find_by_slug(state: &AppState, slug: &str) -> Result<Option<Tenant>, AppError> {
        Ok(TenantRepository::find_by_slug(&state.db.master_pool, slug).await?)
    }

    pub async fn register_tenant(
        state: &AppState,
        name: &str,
        tenant_slug: &str,
        admin_username: &str,
        admin_password: &str,
    ) -> Result<Tenant, AppError> {
        let slug = tenant_slug.trim().to_lowercase();
        
        // 1. Transactional check & save in Master DB
        let mut tx = state.db.master_pool.begin().await?;
        
        if TenantRepository::find_by_slug(&mut *tx, &slug).await?.is_some() {
             return Err(AppError::BusinessException("This tenant name is already taken".to_string(), std::collections::HashMap::new()));
        }

        let database_name = format!("tenant_{}", slug);
        let tenant = TenantRepository::save(&mut *tx, &slug, name, &database_name).await?;
        
        // 2. Initialize the isolated tenant database and run migrations
        let tenant_pool = state.db.get_tenant_pool(&database_name).await?;
        
        // 3. Create the initial admin user in the NEW tenant DB
        let password_hash = hash(admin_password, DEFAULT_COST).map_err(|e| AppError::RuntimeException(e.to_string()))?;
        UserRepository::create(&tenant_pool, admin_username, &password_hash, "admin").await?;
        
        tx.commit().await?;
        
        Ok(tenant)
    }
}
