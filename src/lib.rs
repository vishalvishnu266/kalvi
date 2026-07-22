//! School ERP core library.

pub mod api;
pub mod db;
pub mod error;
pub mod health_probes;
pub mod http;
pub mod middleware;
pub mod repositories;
pub mod services;
pub mod shutdown;
pub mod system;
pub mod tenancy;
pub mod web;

pub use error::RepoError;
pub use http::{build_router, AppState};
pub use services::{Actor, AppServices, RequestCtx, ServiceError, ServiceResult};
pub use system::{connect_system, migrate_system, DbTenantGuard, SystemRegistry};
pub use tenancy::{TenantId, TenantRegistry, TenantRegistryConfig};
