//! JSON API handlers, one submodule per domain. Business modules
//! have been stripped — keep `auth` (register/login/whoami) and
//! `demo` (ping/list/echo) as reference wiring, plus `admin` for
//! the control-plane tenant registry.

pub mod admin;
pub mod auth;
pub mod demo;
