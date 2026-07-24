use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Announcement {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub audience: String,
    pub class_section_id: Option<i64>,
    pub published_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
    pub created_by_user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnnouncement {
    pub title: String,
    pub body: String,
    pub audience: String,
    pub class_section_id: Option<i64>,
    pub expires_at: Option<NaiveDateTime>,
    pub created_by_user_id: Option<i64>,
}

#[derive(Clone)]
pub struct AnnouncementRepo { pool: SqlitePool }

impl AnnouncementRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn publish(&self, a: &NewAnnouncement) -> RepoResult<Announcement> {
        if a.audience == "class" && a.class_section_id.is_none() {
            return Err(RepoError::validation("audience=class requires class_section_id"));
        }
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO announcement
                 (title, body, audience, class_section_id, expires_at, created_by_user_id)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&a.title).bind(&a.body).bind(&a.audience)
        .bind(a.class_section_id).bind(a.expires_at).bind(a.created_by_user_id)
        .fetch_one(&self.pool).await?;

        sqlx::query_as::<_, Announcement>("SELECT * FROM announcement WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn active(&self, limit: i64) -> RepoResult<Vec<Announcement>> {
        Ok(sqlx::query_as::<_, Announcement>(
            r#"SELECT * FROM announcement
               WHERE published_at <= datetime('now')
                 AND (expires_at IS NULL OR expires_at > datetime('now'))
               ORDER BY published_at DESC LIMIT ?"#,
        ).bind(limit).fetch_all(&self.pool).await?)
    }

    pub async fn for_class(&self, class_section_id: i64) -> RepoResult<Vec<Announcement>> {
        Ok(sqlx::query_as::<_, Announcement>(
            r#"SELECT * FROM announcement
               WHERE class_section_id = ?
               ORDER BY published_at DESC"#,
        ).bind(class_section_id).fetch_all(&self.pool).await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM announcement WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub from_user_id: Option<i64>,
    pub to_user_id: Option<i64>,
    pub subject: Option<String>,
    pub body: String,
    pub sent_at: NaiveDateTime,
    pub read_at: Option<NaiveDateTime>,
}

#[derive(Clone)]
pub struct MessageRepo { pool: SqlitePool }

impl MessageRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn send(
        &self, from_user_id: Option<i64>, to_user_id: Option<i64>,
        subject: Option<&str>, body: &str,
    ) -> RepoResult<Message> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO message (from_user_id, to_user_id, subject, body)
               VALUES (?, ?, ?, ?) RETURNING id"#,
        )
        .bind(from_user_id).bind(to_user_id).bind(subject).bind(body)
        .fetch_one(&self.pool).await?;
        sqlx::query_as::<_, Message>("SELECT * FROM message WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn inbox(&self, user_id: i64, limit: i64) -> RepoResult<Vec<Message>> {
        Ok(sqlx::query_as::<_, Message>(
            "SELECT * FROM message WHERE to_user_id = ? ORDER BY sent_at DESC LIMIT ?",
        ).bind(user_id).bind(limit).fetch_all(&self.pool).await?)
    }

    pub async fn unread_count(&self, user_id: i64) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM message WHERE to_user_id = ? AND read_at IS NULL",
        ).bind(user_id).fetch_one(&self.pool).await?)
    }

    pub async fn mark_read(&self, id: i64) -> RepoResult<()> {
        sqlx::query("UPDATE message SET read_at = datetime('now') WHERE id = ? AND read_at IS NULL")
            .bind(id).execute(&self.pool).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub body: Option<String>,
    pub kind: Option<String>,
    pub ref_type: Option<String>,
    pub ref_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub read_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewNotification {
    pub user_id: i64,
    pub title: String,
    pub body: Option<String>,
    pub kind: Option<String>,
    pub ref_type: Option<String>,
    pub ref_id: Option<i64>,
}

#[derive(Clone)]
pub struct NotificationRepo { pool: SqlitePool }

impl NotificationRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn push(&self, n: &NewNotification) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO notification (user_id, title, body, kind, ref_type, ref_id)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(n.user_id).bind(&n.title).bind(&n.body).bind(&n.kind)
        .bind(&n.ref_type).bind(n.ref_id)
        .fetch_one(&self.pool).await?)
    }

    pub async fn for_user(&self, user_id: i64, unread_only: bool, limit: i64)
        -> RepoResult<Vec<Notification>>
    {
        let sql = if unread_only {
            "SELECT * FROM notification WHERE user_id = ? AND read_at IS NULL ORDER BY created_at DESC LIMIT ?"
        } else {
            "SELECT * FROM notification WHERE user_id = ? ORDER BY created_at DESC LIMIT ?"
        };
        Ok(sqlx::query_as::<_, Notification>(sql)
            .bind(user_id).bind(limit).fetch_all(&self.pool).await?)
    }

    pub async fn mark_read(&self, id: i64) -> RepoResult<()> {
        sqlx::query("UPDATE notification SET read_at = datetime('now') WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn mark_all_read(&self, user_id: i64) -> RepoResult<()> {
        sqlx::query(
            "UPDATE notification SET read_at = datetime('now') WHERE user_id = ? AND read_at IS NULL",
        ).bind(user_id).execute(&self.pool).await?;
        Ok(())
    }
}
