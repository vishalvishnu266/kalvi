use axum::http::{header, HeaderMap};
use std::collections::HashSet;

pub const COOKIE_TENANT: &str = "erp_tenant";
pub const COOKIE_USER: &str = "erp_user";
pub const COOKIE_SESSION: &str = "erp_session";

#[derive(Clone, Debug)]
pub struct SessionUser {
    pub user_id: i64,
    pub username: String,
    pub display: String,
    pub session_id: i64,
    pub roles: Vec<String>,
    pub permissions: HashSet<String>,
}

impl SessionUser {
    pub fn has(&self, code: &str) -> bool {
        self.permissions.contains(code)
    }

    pub fn any_of(&self, codes: &[&str]) -> bool {
        codes.iter().any(|c| self.has(c))
    }

    pub fn is_role(&self, r: &str) -> bool {
        self.roles.iter().any(|x| x == r)
    }
}

pub fn read_cookie_from_headers(headers: &HeaderMap, name: &str) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    for kv in raw.split(';') {
        let kv = kv.trim();
        if let Some((k, v)) = kv.split_once('=') {
            if k == name {
                return Some(urlencoding::decode(v).ok()?.into_owned());
            }
        }
    }
    None
}
