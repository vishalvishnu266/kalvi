use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct NewTenantForm {
    pub tenant_id: String,
    pub name: String,
    pub plan: String,
    pub notes: String,
}

#[derive(Deserialize)]
pub struct RenameTenantForm {
    pub name: String,
}
