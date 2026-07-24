use axum::{extract::Path, http::StatusCode, Json};

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::documents::{Document, NewDocument};

pub async fn attach(scope: TenantScope, Json(b): Json<NewDocument>)
    -> Result<Json<Document>, ServiceHttpError>
{ Ok(Json(scope.services.documents.attach(&scope.ctx, b).await?)) }

pub async fn remove(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<StatusCode, ServiceHttpError>
{ scope.services.documents.remove(&scope.ctx, id).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn for_owner(
    scope: TenantScope,
    Path((_t, owner_type, owner_id)): Path<(String, String, i64)>,
) -> Result<Json<Vec<Document>>, ServiceHttpError>
{ Ok(Json(scope.services.documents.list(&scope.ctx, &owner_type, owner_id).await?)) }
