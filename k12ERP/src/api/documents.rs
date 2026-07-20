//! `/api/tenant/documents/*`

use axum::{
    extract::Path,
    routing::{get, post, delete},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::documents::{Document, NewDocument};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/",                     post(attach))
        .route("/{id}",                  delete(remove))
        .route("/owner/{owner_type}/{owner_id}", get(for_owner))
}

#[derive(Deserialize)]
struct AttachBody { #[serde(flatten)] doc: NewDocument, actor_user_id: Option<i64> }

async fn attach(ExtractServices(a): ExtractServices, Json(b): Json<AttachBody>)
    -> Result<Json<Document>, ServiceHttpError>
{ Ok(Json(a.documents.attach(b.doc, b.actor_user_id).await?)) }

#[derive(Deserialize)] struct RemoveQ { actor_user_id: Option<i64> }

async fn remove(
    ExtractServices(a): ExtractServices,
    Path(id): Path<i64>,
    axum::extract::Query(q): axum::extract::Query<RemoveQ>,
) -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.documents.remove(id, q.actor_user_id).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn for_owner(
    ExtractServices(a): ExtractServices,
    Path((owner_type, owner_id)): Path<(String, i64)>,
) -> Result<Json<Vec<Document>>, ServiceHttpError>
{ Ok(Json(a.documents.list(&owner_type, owner_id).await?)) }
