use sqlx::SqlitePool;
use bcrypt::verify;
use crate::model::user::{User, Session};
use crate::repository::UserRepository;
use crate::util::{errors::AppError, id_util};

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
        let session_id = id_util::generate_uuid();
        let now = id_util::current_timestamp();
        let session = Session {
            id: session_id.clone(),
            user_id,
            user_agent: None,
            client_ip: None,
            expires_at: now + (7 * 24 * 60 * 60), // 7 days
            created_at: now,
        };

        UserRepository::save_session(pool, &session).await?;

        Ok(session_id)
    }
}
