use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub user_id: i64,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub tenant: String,
    pub identifier: String,
    pub password: String,
}
