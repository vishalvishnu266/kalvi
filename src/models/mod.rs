//! Data-transfer / row types shared between the service layer and
//! the HTTP handlers. There is no repository layer anymore; SQL for
//! each domain lives in the matching module under `src/services/`.

pub mod academic;
pub mod auth;
pub mod guardians;
pub mod people;
