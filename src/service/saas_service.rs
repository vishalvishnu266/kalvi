use sqlx::SqlitePool;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::model::SaasOwner;
use crate::repository::SaasOwnerRepository;
use crate::util::id_util;

pub struct SaasService;

impl SaasService {
    pub async fn has_owners(pool: &SqlitePool) -> bool {
        SaasOwnerRepository::count(pool).await.unwrap_or(0) > 0
    }

    pub async fn onboard_owner(
        pool: &SqlitePool,
        username: &str,
        password: &str,
        full_name: &str,
    ) -> Result<(), crate::util::AppError> {
        let hashed_pw = hash(password, DEFAULT_COST).unwrap();
        Ok(SaasOwnerRepository::save(pool, username, &hashed_pw, full_name).await?)
    }

    pub async fn authenticate(
        pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> Result<Option<SaasOwner>, crate::util::AppError> {
        let owner = SaasOwnerRepository::find_by_username(pool, username).await?;

        if let Some(o) = owner {
            if verify(password, &o.password_hash).unwrap_or(false) {
                return Ok(Some(o));
            }
        }
        Ok(None)
    }

    pub fn generate_session_id() -> String {
        id_util::generate_random_id("saas")
    }
}
