use sqlx::SqlitePool;
use crate::model::User::User;
use crate::model::User::Session;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(pool)
            .await
    }

    pub async fn save_session(pool: &SqlitePool, session: &Session) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO sessions (id, user_id, user_agent, client_ip, expires_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&session.id)
            .bind(session.user_id)
            .bind(&session.user_agent)
            .bind(&session.client_ip)
            .bind(session.expires_at)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn find_session(pool: &SqlitePool, session_id: &str) -> Result<Option<(Session, User)>, sqlx::Error> {
        let session = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = ? AND expires_at > CURRENT_TIMESTAMP")
            .bind(session_id)
            .fetch_optional(pool)
            .await?;
            
        match session {
            Some(s) => {
                let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                    .bind(s.user_id)
                    .fetch_one(pool)
                    .await?;
                Ok(Some((s, user)))
            },
            None => Ok(None),
        }
    }

    pub async fn delete_session(pool: &SqlitePool, session_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(session_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
