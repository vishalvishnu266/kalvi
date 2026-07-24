use std::sync::Arc;

use crate::repositories::Repositories;
use crate::repositories::communication::NewNotification;
use crate::repositories::discipline::{DisciplineIncident, NewIncident};
use crate::services::{Actor, RequestCtx, ServiceError, ServiceResult};
use crate::services::perm;

#[derive(Clone)]
pub struct DisciplineService {
    repos: Arc<Repositories>,
}

impl DisciplineService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

pub async fn report(
        &self, ctx: &RequestCtx, i: NewIncident, notify_guardians: bool,
    ) -> ServiceResult<DisciplineIncident> {
        ctx.require(perm::DISCIPLINE_MANAGE)?;
        let incident = self.repos.discipline.report(&i).await?;

tracing::info!(
            tenant  = %ctx.tenant,
            actor   = ?ctx.actor,
            request = %ctx.request_id,
            trace   = ctx.trace_id.as_deref(),
            incident_id = incident.id,
            student_id  = incident.student_id,
            "discipline incident reported",
        );

        if notify_guardians {

            let reporter = match ctx.actor {
                Actor::User { user_id } => format!("user #{user_id}"),
                Actor::Impersonated { by_user_id, as_user_id } =>
                    format!("user #{as_user_id} (via admin #{by_user_id})"),
                Actor::System { component } => format!("system:{component}"),
                Actor::Anonymous => "system".to_string(),
            };

            let guardians = self.repos.guardians.guardians_of_student(incident.student_id).await?;
            for g in guardians {
                if let Some(uid) = g.user_id {
                    self.repos.notifications.push(&NewNotification {
                        user_id: uid,
                        title: format!(
                            "Discipline notice for student {} (reported by {reporter})",
                            incident.student_id,
                        ),
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

    pub async fn history(&self, ctx: &RequestCtx, student_id: i64) -> ServiceResult<Vec<DisciplineIncident>> {
        ctx.require_any(&[perm::DISCIPLINE_VIEW, perm::DISCIPLINE_MANAGE])?;
        Ok(self.repos.discipline.for_student(student_id).await?)
    }

    pub async fn severity_gate(&self, sev: &str) -> ServiceResult<()> {
        if !matches!(sev, "low"|"medium"|"high") {
            return Err(ServiceError::validation("severity must be low|medium|high"));
        }
        Ok(())
    }
}
