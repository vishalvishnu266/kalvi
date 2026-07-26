//! Data-transfer / row types shared between the service layer and
//! the HTTP handlers. There is no repository layer; SQL for each
//! domain lives in the matching module under `src/services/`.
//!
//! Real business models (students, staff, fees, ...) have been
//! removed — this framework skeleton keeps only `auth` (login state)
//! and a tiny `demo` example. Add one module per domain as you build
//! them out.

pub mod auth;
pub mod demo;
