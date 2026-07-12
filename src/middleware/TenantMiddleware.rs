use crate::model::Tenant::Tenant;
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct TenantContext {
    pub tenant: Tenant,
    pub pool: SqlitePool,
}
