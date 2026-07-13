use sqlx::SqlitePool;
use bcrypt::verify;
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::model::{User, Session};
use crate::repository::UserRepository;
use crate::util::AppError;

pub struct UserService;

impl UserService {
    pub async fn authenticate(
        pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> Result<Option<User>, AppError> {
        let user = UserRepository::find_by_username(pool, username).await?;

        if let Some(u) = user {
            if verify(password, &u.password_hash).unwrap_or(false) {
                return Ok(Some(u));
            }
        }
        Ok(None)
    }

    pub async fn create_session(
        pool: &SqlitePool,
        user_id: i64,
    ) -> Result<String, AppError> {
        let session_id = Uuid::new_v4().to_string();
        let session = Session {
            id: session_id.clone(),
            user_id,
            user_agent: None,
            client_ip: None,
            expires_at: (Utc::now() + Duration::days(7)).naive_utc(),
            created_at: None,
        };

        UserRepository::save_session(pool, &session).await?;

        Ok(session_id)
    }
}
