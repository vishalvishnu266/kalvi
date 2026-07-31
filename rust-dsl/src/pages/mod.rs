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
