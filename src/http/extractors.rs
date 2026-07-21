//! Axum extractors for tenant + services.
//!
//! `tenant_scope` middleware puts these into request extensions; extractors
//! just pull them out. If middleware wasn't wired, extractors return 500.

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::services::{AppServices, RequestCtx};
use crate::tenancy::TenantId;

pub struct ExtractTenant(pub TenantId);

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

impl<S: Send + Sync> FromRequestParts<S> for ExtractServices {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<AppServices>()
            .cloned()
            .map(ExtractServices)
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "services middleware missing"))
    }
}

/// Pulls the per-request [`RequestCtx`] out of the request extensions.
///
/// The [`crate::http::tenant_scope`] middleware always inserts a
/// `RequestCtx` (even for unauthenticated calls — as `Actor::Anonymous`),
/// so a missing ctx here indicates the middleware wasn't wired.
pub struct ExtractCtx(pub RequestCtx);

impl<S: Send + Sync> FromRequestParts<S> for ExtractCtx {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<RequestCtx>()
            .cloned()
            .map(ExtractCtx)
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "request context middleware missing"))
    }
}
