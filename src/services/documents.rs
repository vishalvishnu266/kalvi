use std::sync::Arc;

use crate::repositories::Repositories;
use crate::repositories::audit::NewAudit;
use crate::repositories::documents::{Document, NewDocument};
use crate::services::{RequestCtx, ServiceResult};
use crate::services::perm;

#[derive(Clone)]
pub struct DocumentService {
    repos: Arc<Repositories>,
}

impl DocumentService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    pub async fn attach(&self, ctx: &RequestCtx, d: NewDocument)
        -> ServiceResult<Document>
    {
        ctx.require(perm::DOCUMENTS_MANAGE)?;
        let actor_user_id = ctx.user_id();
        let doc = self.repos.documents.attach(&d).await?;
        self.repos.audit_log.record(&NewAudit {
            user_id: actor_user_id,
            entity: "document".into(),
            entity_id: doc.id,
            action: "create".into(),
            diff_json: Some(serde_json::json!({
                "owner_type": doc.owner_type,
                "owner_id": doc.owner_id,
                "kind": doc.kind,
            })),
            ip_address: None,
            user_agent: None,
        }).await?;
        Ok(doc)
    }

    pub async fn list(&self, ctx: &RequestCtx, owner_type: &str, owner_id: i64) -> ServiceResult<Vec<Document>> {
        ctx.require_any(&[perm::DOCUMENTS_VIEW, perm::DOCUMENTS_MANAGE])?;
        Ok(self.repos.documents.for_owner(owner_type, owner_id).await?)
    }

    pub async fn remove(&self, ctx: &RequestCtx, id: i64) -> ServiceResult<()> {
        ctx.require(perm::DOCUMENTS_MANAGE)?;
        let actor_user_id = ctx.user_id();
        self.repos.documents.delete(id).await?;
        self.repos.audit_log.record(&NewAudit {
            user_id: actor_user_id,
            entity: "document".into(),
            entity_id: id,
            action: "delete".into(),
            diff_json: None,
            ip_address: None,
            user_agent: None,
        }).await?;
        Ok(())
    }
}
