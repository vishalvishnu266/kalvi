use sqlx::{Sqlite, SqlitePool, Executor};
use crate::model::{User, Session};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username<'a, E>(executor: E, username: &str) -> Result<Option<User>, sqlx::Error> 
    where E: Executor<'a, Database = Sqlite>
    {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(executor)
            .await
    }

    pub async fn save_session<'a, E>(executor: E, session: &Session) -> Result<(), sqlx::Error> 
    where E: Executor<'a, Database = Sqlite>
    {
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

    pub async fn find_session<'a, E>(executor: E, session_id: &str) -> Result<Option<(Session, User)>, sqlx::Error> 
    where E: Executor<'a, Database = Sqlite> + Copy
    {
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

    pub async fn delete_session<'a, E>(executor: E, session_id: &str) -> Result<(), sqlx::Error> 
    where E: Executor<'a, Database = Sqlite>
    {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(session_id)
            .execute(executor)
            .await?;
        Ok(())
    }
}
