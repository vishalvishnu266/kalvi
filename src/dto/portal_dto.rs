use serde::Deserialize;

#[derive(Deserialize)]
pub struct PortalLoginForm {
    pub identifier: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct PortalRegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LinkTenantForm {
    pub tenant: String,
    pub identifier: String,
    pub password: String,
}
