//! Ready-to-render ERP page builders.
//!
//! Each function here returns a complete [`crate::components::page::Page`]
//! populated with realistic mock data. They're used both by the file-generator
//! `examples/*.rs` and by an Axum app via handlers that simply call the
//! function and return `.render()`.
//!
//! The mock data is defined per-page — the library itself doesn't depend on
//! any service, database, or serde structs, so it stays framework-free.

pub mod students;
pub mod fees;
pub mod attendance;
pub mod dashboard;
pub mod icons;
pub mod layouts;
pub mod components;
pub mod errors;
pub mod errors_combos;
pub mod errors_roundtrip;
pub mod errors_validator;

// Shared infrastructure for the /dsl/layouts, /dsl/components and any
// future doc pages. NOT part of the public DSL — it's docs-only tooling.
pub(crate) mod doc_helpers;
