//! Users, roles, and permissions.
//!
//! Note: this repo takes an already-hashed `password_hash` — hashing (Argon2)
//! belongs in the service/application layer.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- User ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_active: bool,
    pub last_login_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub is_active: bool,
}

#[derive(Clone)]
pub struct UserRepo { pool: SqlitePool }

impl UserRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, u: &NewUser) -> RepoResult<User> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO user_account (username, email, password_hash, is_active)
               VALUES (?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&u.username).bind(&u.email).bind(&u.password_hash).bind(u.is_active as i64)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<User> {
        sqlx::query_as::<_, User>("SELECT * FROM user_account WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn find_by_username(&self, username: &str) -> RepoResult<Option<User>> {
        Ok(sqlx::query_as::<_, User>("SELECT * FROM user_account WHERE username = ?")
            .bind(username).fetch_optional(&self.pool).await?)
    }

    pub async fn find_by_email(&self, email: &str) -> RepoResult<Option<User>> {
        Ok(sqlx::query_as::<_, User>("SELECT * FROM user_account WHERE email = ?")
            .bind(email).fetch_optional(&self.pool).await?)
    }

    pub async fn set_active(&self, id: i64, is_active: bool) -> RepoResult<()> {
        sqlx::query("UPDATE user_account SET is_active = ? WHERE id = ?")
            .bind(is_active as i64).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_password_hash(&self, id: i64, hash: &str) -> RepoResult<()> {
        sqlx::query("UPDATE user_account SET password_hash = ? WHERE id = ?")
            .bind(hash).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn touch_last_login(&self, id: i64) -> RepoResult<()> {
        sqlx::query("UPDATE user_account SET last_login_at = datetime('now') WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM user_account WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }

    // ------- role assignments -------

    pub async fn assign_role(&self, user_id: i64, role_id: i64) -> RepoResult<()> {
        sqlx::query("INSERT OR IGNORE INTO user_role (user_id, role_id) VALUES (?, ?)")
            .bind(user_id).bind(role_id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn revoke_role(&self, user_id: i64, role_id: i64) -> RepoResult<()> {
        sqlx::query("DELETE FROM user_role WHERE user_id = ? AND role_id = ?")
            .bind(user_id).bind(role_id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn roles_of(&self, user_id: i64) -> RepoResult<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>(
            r#"SELECT r.* FROM role r
               INNER JOIN user_role ur ON ur.role_id = r.id
               WHERE ur.user_id = ? ORDER BY r.name"#,
        )
        .bind(user_id).fetch_all(&self.pool).await?)
    }

    pub async fn permissions_of(&self, user_id: i64) -> RepoResult<Vec<Permission>> {
        Ok(sqlx::query_as::<_, Permission>(
            r#"SELECT DISTINCT p.* FROM permission p
               INNER JOIN role_permission rp ON rp.permission_id = p.id
               INNER JOIN user_role ur       ON ur.role_id = rp.role_id
               WHERE ur.user_id = ? ORDER BY p.code"#,
        )
        .bind(user_id).fetch_all(&self.pool).await?)
    }
}

// ---------- Role ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct RoleRepo { pool: SqlitePool }

impl RoleRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, name: &str) -> RepoResult<Role> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO role (name) VALUES (?) RETURNING id",
        ).bind(name).fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Role> {
        sqlx::query_as::<_, Role>("SELECT * FROM role WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn find_by_name(&self, name: &str) -> RepoResult<Option<Role>> {
        Ok(sqlx::query_as::<_, Role>("SELECT * FROM role WHERE name = ?")
            .bind(name).fetch_optional(&self.pool).await?)
    }

    pub async fn list(&self) -> RepoResult<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>("SELECT * FROM role ORDER BY name")
            .fetch_all(&self.pool).await?)
    }

    pub async fn grant_permission(&self, role_id: i64, permission_id: i64) -> RepoResult<()> {
        sqlx::query("INSERT OR IGNORE INTO role_permission (role_id, permission_id) VALUES (?, ?)")
            .bind(role_id).bind(permission_id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn revoke_permission(&self, role_id: i64, permission_id: i64) -> RepoResult<()> {
        sqlx::query("DELETE FROM role_permission WHERE role_id = ? AND permission_id = ?")
            .bind(role_id).bind(permission_id).execute(&self.pool).await?;
        Ok(())
    }
}

// ---------- Permission ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Permission {
    pub id: i64,
    pub code: String,
}

#[derive(Clone)]
pub struct PermissionRepo { pool: SqlitePool }

impl PermissionRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, code: &str) -> RepoResult<Permission> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO permission (code) VALUES (?) RETURNING id",
        ).bind(code).fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Permission> {
        sqlx::query_as::<_, Permission>("SELECT * FROM permission WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Permission>> {
        Ok(sqlx::query_as::<_, Permission>("SELECT * FROM permission ORDER BY code")
            .fetch_all(&self.pool).await?)
    }
}
