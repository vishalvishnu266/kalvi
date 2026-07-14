use axum::{
    extract::Extension,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use crate::middleware::TenantContext;
use crate::util::AppError;

pub async fn health(Extension(ctx): Extension<TenantContext>) -> Result<Response, AppError> {
    let body = format!(
        r#"{{"status":"up","tenant":"{}","database":"{}","timestamp":{}}}"#,
        ctx.tenant.name,
        ctx.tenant.database_name,
        crate::util::id_util::current_timestamp()
    );
    
    Ok((
        StatusCode::OK,
        [("content-type", "application/json")],
        body
    ).into_response())
}
