use sqlx::{Sqlite, Executor};
use crate::model::user::{User, Session};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username(executor: impl Executor<'_, Database = Sqlite>, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(executor)
            .await
    }

    pub async fn create(executor: impl Executor<'_, Database = Sqlite>, username: &str, password_hash: &str, role: &str) -> Result<(), sqlx::Error> {
        let now = crate::util::id_util::current_timestamp();
        sqlx::query("INSERT INTO users (username, password_hash, role, created_at) VALUES (?, ?, ?, ?)")
            .bind(username)
            .bind(password_hash)
            .bind(role)
            .bind(now)
            .execute(executor)
            .await?;
        Ok(())
    }

    pub async fn save_session(executor: impl Executor<'_, Database = Sqlite>, session: &Session) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO sessions (id, user_id, user_agent, client_ip, expires_at, created_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&session.id)
            .bind(session.user_id)
            .bind(&session.user_agent)
            .bind(&session.client_ip)
            .bind(session.expires_at)
            .bind(session.created_at)
            .execute(executor)
            .await?;
        Ok(())
    }

    pub async fn find_session(executor: impl Executor<'_, Database = Sqlite> + Copy, session_id: &str) -> Result<Option<(Session, User)>, sqlx::Error> {
        let now = crate::util::id_util::current_timestamp();
        
        // SQLite doesn't support returning tuples directly in query_as, so we fetch manually or use a join struct
        let session = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = ? AND expires_at > ?")
            .bind(session_id)
            .bind(now)
            .fetch_optional(executor)
            .await?;

        if let Some(sess) = session {
            let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(sess.user_id)
                .fetch_one(executor)
                .await?;
            Ok(Some((sess, user)))
        } else {
            Ok(None)
        }
    }
}
