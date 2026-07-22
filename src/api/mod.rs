//! REST API handler modules.
//!
//! Every submodule here only contains `pub async fn` handlers. The router
//! that wires URLs to these handlers lives in [`crate::http::routes`].
//!
//! `AppState` and `TenantScope` used to be re-exported from here — they now
//! live in [`crate::http`] to keep this module a pure handler namespace.

pub mod academic;
pub mod admin;
pub mod attendance;
pub mod audit;
pub mod auth;
pub mod communication;
pub mod discipline;
pub mod documents;
pub mod enrollment;
pub mod examinations;
pub mod fees;
pub mod guardians;
pub mod health;
pub mod hostel;
pub mod inventory;
pub mod library;
pub mod payroll;
pub mod people;
pub mod timetable;
pub mod transport;
