//! Discipline workflows: report an incident and optionally notify guardians.
//!
//! ## `RequestCtx` adoption note
//!
//! `DisciplineService::report` is the first-wave demo for the
//! per-request-context pattern documented in
//! [`crate::services::context`]. It takes `&RequestCtx` as its first
//! parameter because it:
//!
//! * writes an audit-worthy row (a discipline incident),
//! * sends notifications that must attribute *who* filed the report, and
//! * is exactly the kind of call an admin might make on behalf of a
//!   teacher (impersonation), which the audit trail must record.
//!
//! Read paths (`history`, `severity_gate`) intentionally do **not** take
//! `&RequestCtx` — they neither audit nor authorize on the caller.

use std::sync::Arc;

use crate::repositories::Repositories;
use crate::repositories::communication::NewNotification;
use crate::repositories::discipline::{DisciplineIncident, NewIncident};
use crate::services::{Actor, RequestCtx, ServiceError, ServiceResult};

#[derive(Clone)]
pub struct DisciplineService {
    repos: Arc<Repositories>,
}

impl DisciplineService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    /// Report an incident. If `notify_guardians` is true, push a notification
    /// to every guardian of the student that has a linked user account.
    ///
    /// `ctx` is used for:
    /// * a `tracing` event that records the acting user / system component,
    ///   the request id and (if any) the trace id — so log tails can
    ///   correlate the incident row to the originating request;
    /// * attributing the notification title so guardians see *who*
    ///   reported the incident (teacher user id vs. a system component).
    pub async fn report(
        &self, ctx: &RequestCtx, i: NewIncident, notify_guardians: bool,
    ) -> ServiceResult<DisciplineIncident> {
        let incident = self.repos.discipline.report(&i).await?;

        // Structured log for the audit tail. Once you add an `audit_log`
        // repo, swap this for a real row insert — the ctx already carries
        // everything you need (`ctx.actor`, `ctx.request_id`, `ctx.trace_id`).
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
            // Short, human-friendly attribution derived from the actor.
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
