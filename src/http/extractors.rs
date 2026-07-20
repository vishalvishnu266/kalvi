//! Axum extractors for tenant + services.
//!
//! `tenant_scope` middleware puts these into request extensions; extractors
//! just pull them out. If middleware wasn't wired, extractors return 500.

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::services::AppServices;
use crate::tenancy::TenantId;

pub struct ExtractTenant(pub TenantId);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ExtractTenant {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<TenantId>()
            .cloned()
            .map(ExtractTenant)
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "tenant middleware missing"))
    }
}

pub struct ExtractServices(pub AppServices);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ExtractServices {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<AppServices>()
            .cloned()
            .map(ExtractServices)
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "services middleware missing"))
    }
}
