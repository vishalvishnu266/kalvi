use axum::{
    extract::{Path, State},
    http::Request,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::config::database_config::DatabaseConfig;
use crate::repository::tenant_repository::{Tenant, TenantRepository};
use crate::util::errors::AppError;
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct TenantContext {
    pub tenant: Tenant,
    pub pool: SqlitePool,
}

pub async fn tenant_middleware<B>(
    State(db_config): State<Arc<DatabaseConfig>>,
    Path(slug): Path<String>,
    mut req: Request<B>,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Find tenant in master DB
    let tenant = TenantRepository::find_by_slug(&db_config.master_pool, &slug)
        .await?
        .ok_or_else(|| AppError::RuntimeException(format!("Tenant '{}' not found", slug)))?;

    // 2. Resolve tenant pool
    let pool = db_config.get_tenant_pool(&tenant.slug).await?;

    // 3. Inject context
    req.extensions_mut().insert(TenantContext {
        tenant,
        pool,
    });

    Ok(next.run(req).await)
}
