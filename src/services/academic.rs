//! Academic domain (was AcademicYearRepo + TermRepo + GradeRepo +
//! SectionRepo + RoomRepo + SubjectRepo + AcademicService).

use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};
use crate::models::academic::{
    AcademicYear, Grade, NewAcademicYear, NewRoom, NewSubject, NewTerm, Room, Section, Subject, Term,
};
use crate::services::{perm, RequestCtx, ServiceResult};

// ── Academic year ───────────────────────────────────────────────────

pub async fn create_year_row(pool: &SqlitePool, y: &NewAcademicYear) -> RepoResult<AcademicYear> {
    if y.start_date >= y.end_date {
        return Err(RepoError::validation("start_date must be before end_date"));
    }
    let mut tx = pool.begin().await?;
    if y.is_current {
        sqlx::query("UPDATE academic_year SET is_current = 0 WHERE is_current = 1")
            .execute(&mut *tx).await?;
    }
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO academic_year (name, start_date, end_date, is_current)
           VALUES (?, ?, ?, ?) RETURNING id"#,
    )
    .bind(&y.name).bind(y.start_date).bind(y.end_date).bind(y.is_current as i64)
    .fetch_one(&mut *tx).await?;
    tx.commit().await?;
    get_year(pool, id).await
}

pub async fn get_year(pool: &SqlitePool, id: i64) -> RepoResult<AcademicYear> {
    sqlx::query_as::<_, AcademicYear>("SELECT * FROM academic_year WHERE id = ?")
        .bind(id).fetch_optional(pool).await?
        .ok_or(RepoError::NotFound)
}

pub async fn current_year_row(pool: &SqlitePool) -> RepoResult<AcademicYear> {
    sqlx::query_as::<_, AcademicYear>("SELECT * FROM academic_year WHERE is_current = 1")
        .fetch_optional(pool).await?
        .ok_or(RepoError::NotFound)
}

pub async fn list_years(pool: &SqlitePool) -> RepoResult<Vec<AcademicYear>> {
    Ok(sqlx::query_as::<_, AcademicYear>(
        "SELECT * FROM academic_year ORDER BY start_date DESC",
    ).fetch_all(pool).await?)
}

pub async fn set_current_year(pool: &SqlitePool, id: i64) -> RepoResult<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("UPDATE academic_year SET is_current = 0 WHERE is_current = 1")
        .execute(&mut *tx).await?;
    let res = sqlx::query("UPDATE academic_year SET is_current = 1 WHERE id = ?")
        .bind(id).execute(&mut *tx).await?;
    if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
    tx.commit().await?;
    Ok(())
}

// ── Terms ───────────────────────────────────────────────────────────

pub async fn create_term_row(pool: &SqlitePool, t: &NewTerm) -> RepoResult<Term> {
    if t.start_date >= t.end_date {
        return Err(RepoError::validation("start_date must be before end_date"));
    }
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO term (academic_year_id, name, start_date, end_date)
           VALUES (?, ?, ?, ?) RETURNING id"#,
    )
    .bind(t.academic_year_id).bind(&t.name).bind(t.start_date).bind(t.end_date)
    .fetch_one(pool).await?;
    get_term(pool, id).await
}

pub async fn get_term(pool: &SqlitePool, id: i64) -> RepoResult<Term> {
    sqlx::query_as::<_, Term>("SELECT * FROM term WHERE id = ?")
        .bind(id).fetch_optional(pool).await?
        .ok_or(RepoError::NotFound)
}

pub async fn list_terms_for_year(pool: &SqlitePool, year_id: i64) -> RepoResult<Vec<Term>> {
    Ok(sqlx::query_as::<_, Term>(
        "SELECT * FROM term WHERE academic_year_id = ? ORDER BY start_date",
    ).bind(year_id).fetch_all(pool).await?)
}

// ── Grades / Sections / Rooms / Subjects ────────────────────────────

pub async fn list_grades(pool: &SqlitePool) -> RepoResult<Vec<Grade>> {
    Ok(sqlx::query_as::<_, Grade>("SELECT * FROM grade ORDER BY level")
        .fetch_all(pool).await?)
}

pub async fn list_sections(pool: &SqlitePool) -> RepoResult<Vec<Section>> {
    Ok(sqlx::query_as::<_, Section>("SELECT * FROM section ORDER BY name")
        .fetch_all(pool).await?)
}

pub async fn list_rooms(pool: &SqlitePool) -> RepoResult<Vec<Room>> {
    Ok(sqlx::query_as::<_, Room>("SELECT * FROM room ORDER BY name")
        .fetch_all(pool).await?)
}

pub async fn create_room(pool: &SqlitePool, r: &NewRoom) -> RepoResult<Room> {
    let id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO room (name, capacity, kind) VALUES (?, ?, ?) RETURNING id",
    )
    .bind(&r.name).bind(r.capacity).bind(&r.kind)
    .fetch_one(pool).await?;
    sqlx::query_as::<_, Room>("SELECT * FROM room WHERE id = ?")
        .bind(id).fetch_optional(pool).await?
        .ok_or(RepoError::NotFound)
}

pub async fn list_subjects(pool: &SqlitePool) -> RepoResult<Vec<Subject>> {
    Ok(sqlx::query_as::<_, Subject>("SELECT * FROM subject ORDER BY code")
        .fetch_all(pool).await?)
}

pub async fn create_subject(pool: &SqlitePool, s: &NewSubject) -> RepoResult<Subject> {
    let id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO subject (code, name, is_elective) VALUES (?, ?, ?) RETURNING id",
    )
    .bind(&s.code).bind(&s.name).bind(s.is_elective as i64)
    .fetch_one(pool).await?;
    sqlx::query_as::<_, Subject>("SELECT * FROM subject WHERE id = ?")
        .bind(id).fetch_optional(pool).await?
        .ok_or(RepoError::NotFound)
}

// ── High-level workflows ────────────────────────────────────────────

pub async fn create_year(
    pool: &SqlitePool, ctx: &RequestCtx, y: NewAcademicYear,
) -> ServiceResult<AcademicYear> {
    ctx.require(perm::ACADEMIC_MANAGE)?;
    Ok(create_year_row(pool, &y).await?)
}

pub async fn current_year(pool: &SqlitePool) -> ServiceResult<AcademicYear> {
    Ok(current_year_row(pool).await?)
}

pub async fn list_terms(
    pool: &SqlitePool, ctx: &RequestCtx, year_id: i64,
) -> ServiceResult<Vec<Term>> {
    ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(list_terms_for_year(pool, year_id).await?)
}
