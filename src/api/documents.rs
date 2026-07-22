//! `/api/{tenant}/documents/*` handlers.

use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::documents::{Document, NewDocument};

#[derive(Deserialize)]
pub struct AttachBody { #[serde(flatten)] doc: NewDocument, actor_user_id: Option<i64> }

pub async fn attach(scope: TenantScope, Json(b): Json<AttachBody>)
    -> Result<Json<Document>, ServiceHttpError>
{ Ok(Json(scope.services.documents.attach(b.doc, b.actor_user_id).await?)) }

#[derive(Deserialize)] pub struct RemoveQ { actor_user_id: Option<i64> }

pub async fn remove(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Query(q): Query<RemoveQ>,
) -> Result<StatusCode, ServiceHttpError>
{ scope.services.documents.remove(id, q.actor_user_id).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn for_owner(
    scope: TenantScope,
    Path((_t, owner_type, owner_id)): Path<(String, String, i64)>,
) -> Result<Json<Vec<Document>>, ServiceHttpError>
{ Ok(Json(scope.services.documents.list(&owner_type, owner_id).await?)) }
