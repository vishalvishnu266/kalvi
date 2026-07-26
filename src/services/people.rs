//! Staff domain.
//!
//! Student and guardian logic used to live here as well but has been
//! removed — those two domains will be re-implemented as portal-only
//! services in a follow-up. This module now only exposes staff CRUD
//! + workflows.

use chrono::NaiveDate;
use sqlx::SqlitePool;

use crate::error::RepoResult;
use crate::models::people::{NewStaff, Staff, UpdateStaff};
use crate::services::{perm, RequestCtx, ServiceResult};

// ── Staff SQL (was StaffRepo) ───────────────────────────────────────

pub async fn create_staff(pool: &SqlitePool, s: &NewStaff) -> RepoResult<Staff> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO staff
             (employee_no, user_id, department_id, first_name, last_name,
              date_of_birth, gender, phone, email, designation,
              employment_type, date_of_joining, photo_path)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
    )
    .bind(&s.employee_no)
    .bind(s.user_id)
    .bind(s.department_id)
    .bind(&s.first_name)
    .bind(&s.last_name)
    .bind(s.date_of_birth)
    .bind(&s.gender)
    .bind(&s.phone)
    .bind(&s.email)
    .bind(&s.designation)
    .bind(&s.employment_type)
    .bind(s.date_of_joining)
    .bind(&s.photo_path)
    .fetch_one(pool)
    .await?;
    get_staff(pool, id).await
}

pub async fn get_staff(pool: &SqlitePool, id: i64) -> RepoResult<Staff> {
    sqlx::query_as::<_, Staff>("SELECT * FROM staff WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(crate::error::RepoError::NotFound)
}

pub async fn find_staff_by_employee_no(pool: &SqlitePool, no: &str) -> RepoResult<Option<Staff>> {
    Ok(
        sqlx::query_as::<_, Staff>("SELECT * FROM staff WHERE employee_no = ?")
            .bind(no)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn list_staff(pool: &SqlitePool, limit: i64, offset: i64) -> RepoResult<Vec<Staff>> {
    Ok(sqlx::query_as::<_, Staff>(
        "SELECT * FROM staff ORDER BY last_name, first_name LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?)
}

pub async fn update_staff(pool: &SqlitePool, id: i64, u: &UpdateStaff) -> RepoResult<Staff> {
    sqlx::query(
        r#"UPDATE staff SET
             department_id    = COALESCE(?, department_id),
             first_name       = COALESCE(?, first_name),
             last_name        = COALESCE(?, last_name),
             phone            = COALESCE(?, phone),
             email            = COALESCE(?, email),
             designation      = COALESCE(?, designation),
             employment_type  = COALESCE(?, employment_type),
             date_of_leaving  = COALESCE(?, date_of_leaving),
             status           = COALESCE(?, status),
             photo_path       = COALESCE(?, photo_path),
             updated_at       = CURRENT_TIMESTAMP
           WHERE id = ?"#,
    )
    .bind(u.department_id.unwrap_or(None))
    .bind(&u.first_name)
    .bind(&u.last_name)
    .bind(&u.phone)
    .bind(&u.email)
    .bind(&u.designation)
    .bind(&u.employment_type)
    .bind(u.date_of_leaving.unwrap_or(None))
    .bind(&u.status)
    .bind(&u.photo_path)
    .bind(id)
    .execute(pool)
    .await?;
    get_staff(pool, id).await
}

// ── High-level workflows ────────────────────────────────────────────

pub async fn hire_staff(pool: &SqlitePool, ctx: &RequestCtx, s: NewStaff) -> ServiceResult<Staff> {
    ctx.require(perm::STAFF_HIRE)?;
    Ok(create_staff(pool, &s).await?)
}

pub async fn terminate_staff(
    pool: &SqlitePool,
    ctx: &RequestCtx,
    staff_id: i64,
    on: NaiveDate,
) -> ServiceResult<()> {
    ctx.require(perm::STAFF_EDIT)?;
    sqlx::query(
        r#"UPDATE staff
             SET status = 'terminated',
                 date_of_leaving = ?,
                 updated_at = CURRENT_TIMESTAMP
           WHERE id = ?"#,
    )
    .bind(on)
    .bind(staff_id)
    .execute(pool)
    .await?;
    Ok(())
}
