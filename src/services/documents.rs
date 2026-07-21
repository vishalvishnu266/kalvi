//! Document attachments: thin wrapper that also records an audit entry.

use std::sync::Arc;

use crate::repositories::Repositories;
use crate::repositories::audit::NewAudit;
use crate::repositories::documents::{Document, NewDocument};
use crate::services::ServiceResult;

#[derive(Clone)]
pub struct DocumentService {
    repos: Arc<Repositories>,
}

impl DocumentService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    pub async fn attach(&self, d: NewDocument, actor_user_id: Option<i64>)
        -> ServiceResult<Document>
    {
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

    pub async fn list(&self, owner_type: &str, owner_id: i64) -> ServiceResult<Vec<Document>> {
        Ok(self.repos.documents.for_owner(owner_type, owner_id).await?)
    }

    pub async fn remove(&self, id: i64, actor_user_id: Option<i64>) -> ServiceResult<()> {
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
