//! Communication workflows: publish an announcement + fan out notifications
//! to the recipient audience.

use std::sync::Arc;

use crate::repositories::Repositories;
use crate::repositories::communication::{Announcement, NewAnnouncement, NewNotification};
use crate::services::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct CommunicationService {
    repos: Arc<Repositories>,
}

impl CommunicationService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    /// Publish an announcement and (best-effort) push notifications to matching
    /// user accounts. Returns the announcement + how many notifications were
    /// pushed.
    pub async fn broadcast(&self, a: NewAnnouncement) -> ServiceResult<(Announcement, u64)> {
        let ann = self.repos.announcements.publish(&a).await?;

        // Fan-out target users depending on audience.
        let user_ids: Vec<i64> = match ann.audience.as_str() {
            "all" => sqlx::query_scalar("SELECT id FROM user_account WHERE is_active = 1")
                .fetch_all(&self.repos.pool).await?,
            "students" => sqlx::query_scalar(
                "SELECT user_id FROM student WHERE user_id IS NOT NULL AND status = 'active'",
            ).fetch_all(&self.repos.pool).await?,
            "staff" => sqlx::query_scalar(
                "SELECT user_id FROM staff WHERE user_id IS NOT NULL AND status = 'active'",
            ).fetch_all(&self.repos.pool).await?,
            "guardians" => sqlx::query_scalar(
                "SELECT user_id FROM guardian WHERE user_id IS NOT NULL",
            ).fetch_all(&self.repos.pool).await?,
            "class" => {
                let cs_id = ann.class_section_id
                    .ok_or_else(|| ServiceError::validation("class_section_id required"))?;
                sqlx::query_scalar(
                    r#"SELECT s.user_id
                         FROM enrollment e
                         JOIN student s ON s.id = e.student_id
                        WHERE e.class_section_id = ? AND e.left_on IS NULL
                          AND s.user_id IS NOT NULL"#,
                ).bind(cs_id).fetch_all(&self.repos.pool).await?
            }
            _ => vec![],
        };

        let mut pushed = 0_u64;
        for uid in user_ids {
            self.repos.notifications.push(&NewNotification {
                user_id: uid,
                title: ann.title.clone(),
                body: Some(ann.body.clone()),
                kind: Some("announcement".into()),
                ref_type: Some("announcement".into()),
                ref_id: Some(ann.id),
            }).await?;
            pushed += 1;
        }

        Ok((ann, pushed))
    }

    pub async fn notify_user(&self, n: NewNotification) -> ServiceResult<i64> {
        Ok(self.repos.notifications.push(&n).await?)
    }
}
