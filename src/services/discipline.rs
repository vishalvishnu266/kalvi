//! Discipline workflows: report an incident and optionally notify guardians.

use std::sync::Arc;

use crate::repositories::Repositories;
use crate::repositories::communication::NewNotification;
use crate::repositories::discipline::{DisciplineIncident, NewIncident};
use crate::services::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct DisciplineService {
    repos: Arc<Repositories>,
}

impl DisciplineService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    /// Report an incident. If `notify_guardians` is true, push a notification
    /// to every guardian of the student that has a linked user account.
    pub async fn report(
        &self, i: NewIncident, notify_guardians: bool,
    ) -> ServiceResult<DisciplineIncident> {
        let incident = self.repos.discipline.report(&i).await?;

        if notify_guardians {
            let guardians = self.repos.guardians.guardians_of_student(incident.student_id).await?;
            for g in guardians {
                if let Some(uid) = g.user_id {
                    self.repos.notifications.push(&NewNotification {
                        user_id: uid,
                        title: format!("Discipline notice for student {}", incident.student_id),
                        body: Some(incident.description.clone()),
                        kind: Some("discipline".into()),
                        ref_type: Some("discipline_incident".into()),
                        ref_id: Some(incident.id),
                    }).await?;
                }
            }
        }
        Ok(incident)
    }

    pub async fn history(&self, student_id: i64) -> ServiceResult<Vec<DisciplineIncident>> {
        Ok(self.repos.discipline.for_student(student_id).await?)
    }

    pub async fn severity_gate(&self, sev: &str) -> ServiceResult<()> {
        if !matches!(sev, "low"|"medium"|"high") {
            return Err(ServiceError::validation("severity must be low|medium|high"));
        }
        Ok(())
    }
}
