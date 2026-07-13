use sqlx::SqlitePool;
use bcrypt::{hash, DEFAULT_COST};
use crate::model::Tenant;
use crate::repository::{TenantRepository, UserRepository};
use crate::config::AppState;

pub struct TenantService;

impl TenantService {
    pub async fn find_by_slug(state: &AppState, slug: &str) -> Result<Option<Tenant>, sqlx::Error> {
        TenantRepository::find_by_slug(&state.db.master_pool, slug).await
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
    ) -> Result<Tenant, String> {
        let slug = slug.trim().to_lowercase();
        if slug.is_empty() || name.trim().is_empty() {
            return Err("All fields are required".to_string());
        }

        let reserved_slugs = vec!["saas"];
        if reserved_slugs.contains(&slug.as_str()) {
            return Err("Slug is reserved".to_string());
        }

        match TenantRepository::find_by_slug(&state.db.master_pool, &slug).await {
            Ok(Some(_)) => return Err("Slug is already taken".to_string()),
            Err(_) => return Err("Database error".to_string()),
            Ok(None) => {}
        }

        let db_name = slug.clone();
        
        let tenant = TenantRepository::save(&state.db.master_pool, &slug, name, &db_name)
            .await
            .map_err(|_| "Failed to create tenant".to_string())?;

        let tenant_pool = state.db.get_tenant_pool(&tenant.database_name)
            .await
            .map_err(|_| "Failed to provision tenant database".to_string())?;

        let hashed_pw = hash(admin_password, DEFAULT_COST).unwrap();
        sqlx::query("INSERT INTO users (username, password_hash, role) VALUES (?, ?, 'admin')")
            .bind(admin_username)
            .bind(hashed_pw)
            .execute(&tenant_pool)
            .await
            .map_err(|_| "Failed to create admin user".to_string())?;

        Ok(tenant)
    }

    pub async fn update_tenant_settings(
        state: &AppState,
        slug: &str,
        primary_color: &str,
        dark_mode: bool,
    ) -> Result<(), String> {
        TenantRepository::update_settings(&state.db.master_pool, slug, primary_color, dark_mode)
            .await
            .map_err(|_| "Failed to update settings".to_string())
    }
}
