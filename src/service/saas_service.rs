use sqlx::SqlitePool;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::repository::SaasOwnerRepository;
use crate::model::SaasOwner;
use crate::util::errors::AppError;

pub struct SaasService;

impl SaasService {
    pub async fn has_owners(pool: &SqlitePool) -> Result<bool, AppError> {
        Ok(SaasOwnerRepository::has_any(pool).await?)
    }

    pub async fn onboard_owner(pool: &SqlitePool, username: &str, password: &str, full_name: &str) -> Result<(), AppError> {
        if username.trim().len() < 3 || password.len() < 8 {
             return Err(AppError::BusinessException("Invalid username or password length".to_string(), std::collections::HashMap::new()));
        }
        
        let hashed = hash(password, DEFAULT_COST).map_err(|e| AppError::RuntimeException(e.to_string()))?;
        SaasOwnerRepository::save(pool, username, &hashed, full_name).await?;
        Ok(())
    }

    pub async fn authenticate(pool: &SqlitePool, username: &str, password: &str) -> Result<Option<SaasOwner>, AppError> {
        if let Some(owner) = SaasOwnerRepository::find_by_username(pool, username).await? {
            if verify(password, &owner.password_hash).unwrap_or(false) {
                return Ok(Some(owner));
            }
        }
        Ok(None)
    }
}
