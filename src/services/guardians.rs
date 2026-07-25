//! Guardians domain (was GuardianRepo).

use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};
use crate::models::guardians::{Guardian, NewGuardian, StudentGuardianLink, UpdateGuardian};

pub async fn create(pool: &SqlitePool, g: &NewGuardian) -> RepoResult<Guardian> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO guardian
           (user_id, first_name, last_name, phone, email, occupation, address)
           VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
    )
    .bind(g.user_id).bind(&g.first_name).bind(&g.last_name)
    .bind(&g.phone).bind(&g.email).bind(&g.occupation).bind(&g.address)
    .fetch_one(pool).await?;
    get(pool, id).await
}

pub async fn get(pool: &SqlitePool, id: i64) -> RepoResult<Guardian> {
    sqlx::query_as::<_, Guardian>("SELECT * FROM guardian WHERE id = ?")
        .bind(id).fetch_optional(pool).await?
        .ok_or(RepoError::NotFound)
}

pub async fn list(pool: &SqlitePool, limit: i64, offset: i64) -> RepoResult<Vec<Guardian>> {
    Ok(sqlx::query_as::<_, Guardian>(
        "SELECT * FROM guardian ORDER BY last_name, first_name LIMIT ? OFFSET ?",
    ).bind(limit).bind(offset).fetch_all(pool).await?)
}

pub async fn delete(pool: &SqlitePool, id: i64) -> RepoResult<()> {
    let res = sqlx::query("DELETE FROM guardian WHERE id = ?")
        .bind(id).execute(pool).await?;
    if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
    Ok(())
}

pub async fn update(pool: &SqlitePool, id: i64, u: &UpdateGuardian) -> RepoResult<Guardian> {
    let phone      = u.phone.as_ref()     .map(|v| if v.is_empty() { None } else { Some(v.clone()) });
    let email      = u.email.as_ref()     .map(|v| if v.is_empty() { None } else { Some(v.clone()) });
    let occupation = u.occupation.as_ref().map(|v| if v.is_empty() { None } else { Some(v.clone()) });
    let address    = u.address.as_ref()   .map(|v| if v.is_empty() { None } else { Some(v.clone()) });

    sqlx::query(
        r#"UPDATE guardian SET
             first_name = COALESCE(?, first_name),
             last_name  = COALESCE(?, last_name),
             phone      = COALESCE(?, phone),
             email      = COALESCE(?, email),
             occupation = COALESCE(?, occupation),
             address    = COALESCE(?, address)
           WHERE id = ?"#,
    )
    .bind(&u.first_name).bind(&u.last_name)
    .bind(phone).bind(email).bind(occupation).bind(address).bind(id)
    .execute(pool).await?;
    get(pool, id).await
}

pub async fn link(pool: &SqlitePool, l: &StudentGuardianLink) -> RepoResult<()> {
    sqlx::query(
        r#"INSERT INTO student_guardian
             (student_id, guardian_id, relationship, is_primary, is_emergency, can_pickup)
           VALUES (?, ?, ?, ?, ?, ?)
           ON CONFLICT(student_id, guardian_id) DO UPDATE SET
             relationship = excluded.relationship,
             is_primary   = excluded.is_primary,
             is_emergency = excluded.is_emergency,
             can_pickup   = excluded.can_pickup"#,
    )
    .bind(l.student_id).bind(l.guardian_id).bind(&l.relationship)
    .bind(l.is_primary as i64).bind(l.is_emergency as i64).bind(l.can_pickup as i64)
    .execute(pool).await?;
    Ok(())
}

pub async fn unlink(pool: &SqlitePool, student_id: i64, guardian_id: i64) -> RepoResult<()> {
    sqlx::query("DELETE FROM student_guardian WHERE student_id = ? AND guardian_id = ?")
        .bind(student_id).bind(guardian_id).execute(pool).await?;
    Ok(())
}

pub async fn guardians_of_student(pool: &SqlitePool, student_id: i64) -> RepoResult<Vec<Guardian>> {
    Ok(sqlx::query_as::<_, Guardian>(
        r#"SELECT g.* FROM guardian g
           INNER JOIN student_guardian sg ON sg.guardian_id = g.id
           WHERE sg.student_id = ?
           ORDER BY sg.is_primary DESC, g.last_name"#,
    ).bind(student_id).fetch_all(pool).await?)
}

pub async fn students_of_guardian(pool: &SqlitePool, guardian_id: i64) -> RepoResult<Vec<i64>> {
    Ok(sqlx::query_scalar::<_, i64>(
        "SELECT student_id FROM student_guardian WHERE guardian_id = ?",
    ).bind(guardian_id).fetch_all(pool).await?)
}

pub async fn find_by_user_id(pool: &SqlitePool, user_id: i64) -> RepoResult<Option<Guardian>> {
    Ok(sqlx::query_as::<_, Guardian>(
        "SELECT * FROM guardian WHERE user_id = ? LIMIT 1",
    ).bind(user_id).fetch_optional(pool).await?)
}

pub async fn students_of_user(pool: &SqlitePool, user_id: i64) -> RepoResult<Vec<i64>> {
    match find_by_user_id(pool, user_id).await? {
        Some(g) => students_of_guardian(pool, g.id).await,
        None    => Ok(Vec::new()),
    }
}

pub async fn is_guardian_of(pool: &SqlitePool, user_id: i64, student_id: i64) -> RepoResult<bool> {
    Ok(sqlx::query_scalar::<_, i64>(
        r#"SELECT COUNT(*) FROM student_guardian sg
             INNER JOIN guardian g ON g.id = sg.guardian_id
             WHERE g.user_id = ? AND sg.student_id = ?"#,
    ).bind(user_id).bind(student_id).fetch_one(pool).await? > 0)
}
