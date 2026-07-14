use sqlx::{Sqlite, Executor};
use crate::model::{User, Session};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username(executor: impl Executor<'_, Database = Sqlite>, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(executor)
            .await
    }

    pub async fn create(executor: impl Executor<'_, Database = Sqlite>, username: &str, password_hash: &str, role: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (username, password_hash, role) VALUES (?, ?, ?)")
            .bind(username)
            .bind(password_hash)
            .bind(role)
            .execute(executor)
            .await?;
        Ok(())
    }

    pub async fn save_session(executor: impl Executor<'_, Database = Sqlite>, session: &Session) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO sessions (id, user_id, user_agent, client_ip, expires_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&session.id)
            .bind(session.user_id)
            .bind(&session.user_agent)
            .bind(&session.client_ip)
            .bind(session.expires_at)
            .execute(executor)
            .await?;
        Ok(())
    }

    pub async fn find_session(executor: impl Executor<'_, Database = Sqlite> + Copy, session_id: &str) -> Result<Option<(Session, User)>, sqlx::Error> {
        let session = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = ? AND expires_at > CURRENT_TIMESTAMP")
            .bind(session_id)
            .fetch_optional(executor)
            .await?;
            
        match session {
            Some(s) => {
                let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                    .bind(s.user_id)
                    .fetch_one(executor)
                    .await?;
                Ok(Some((s, user)))
            },
            None => Ok(None),
        }
    }

    pub async fn delete_session(executor: impl Executor<'_, Database = Sqlite>, session_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(session_id)
            .execute(executor)
            .await?;
        Ok(())
    }
}
