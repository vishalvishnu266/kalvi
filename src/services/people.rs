//! Students + staff domain (was PeopleService + StudentRepo + StaffRepo).

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};
use crate::models::guardians::{NewGuardian, StudentGuardianLink};
use crate::models::people::{NewStaff, NewStudent, Staff, Student, UpdateStaff, UpdateStudent};
use crate::services::{guardians as guardian_svc, perm, RequestCtx, ServiceError, ServiceResult};

// ── Visibility scope ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Scope {
    Global,
    GuardianOfUser(i64),
    SelfStudent(i64),
}

impl Scope {
    pub fn from_ctx(ctx: &RequestCtx) -> Self {
        if ctx.has_permission(perm::STUDENTS_VIEW) {
            Scope::Global
        } else if let Some(uid) = ctx.user_id() {
            Scope::GuardianOfUser(uid)
        } else {
            Scope::GuardianOfUser(0)
        }
    }
}

// ── Admission DTO ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admission {
    pub student: NewStudent,
    pub guardian: Option<(NewGuardian, String, bool)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdmissionResult {
    pub student: Student,
    pub guardian_id: Option<i64>,
}

// ── Student SQL (was StudentRepo) ───────────────────────────────────

pub async fn create_student(pool: &SqlitePool, s: &NewStudent) -> RepoResult<Student> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO student (
            admission_no, user_id, first_name, middle_name, last_name,
            date_of_birth, gender, blood_group, nationality, religion, photo_path,
            admission_date, address_line1, address_line2, city, state, postal_code, country
          ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
    )
    .bind(&s.admission_no).bind(s.user_id)
    .bind(&s.first_name).bind(&s.middle_name).bind(&s.last_name)
    .bind(s.date_of_birth).bind(&s.gender).bind(&s.blood_group)
    .bind(&s.nationality).bind(&s.religion).bind(&s.photo_path)
    .bind(s.admission_date)
    .bind(&s.address_line1).bind(&s.address_line2)
    .bind(&s.city).bind(&s.state).bind(&s.postal_code).bind(&s.country)
    .fetch_one(pool).await?;
    get_student(pool, id).await
}

pub async fn get_student(pool: &SqlitePool, id: i64) -> RepoResult<Student> {
    sqlx::query_as::<_, Student>("SELECT * FROM student WHERE id = ?")
        .bind(id).fetch_optional(pool).await?
        .ok_or(RepoError::NotFound)
}

pub async fn find_student_by_admission_no(pool: &SqlitePool, no: &str) -> RepoResult<Option<Student>> {
    Ok(sqlx::query_as::<_, Student>("SELECT * FROM student WHERE admission_no = ?")
        .bind(no).fetch_optional(pool).await?)
}

pub async fn list_all_students(pool: &SqlitePool, limit: i64, offset: i64) -> RepoResult<Vec<Student>> {
    Ok(sqlx::query_as::<_, Student>(
        "SELECT * FROM student ORDER BY last_name, first_name LIMIT ? OFFSET ?",
    ).bind(limit).bind(offset).fetch_all(pool).await?)
}

pub async fn list_students_by_ids(pool: &SqlitePool, ids: &[i64]) -> RepoResult<Vec<Student>> {
    if ids.is_empty() { return Ok(Vec::new()); }
    let placeholders = std::iter::repeat("?").take(ids.len()).collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT * FROM student WHERE id IN ({placeholders}) ORDER BY last_name, first_name",
    );
    let mut q = sqlx::query_as::<_, Student>(&sql);
    for id in ids { q = q.bind(id); }
    Ok(q.fetch_all(pool).await?)
}

pub async fn find_student_by_user_id(pool: &SqlitePool, user_id: i64) -> RepoResult<Option<Student>> {
    Ok(sqlx::query_as::<_, Student>(
        "SELECT * FROM student WHERE user_id = ? LIMIT 1",
    ).bind(user_id).fetch_optional(pool).await?)
}

pub async fn search_students(pool: &SqlitePool, q: &str, limit: i64) -> RepoResult<Vec<Student>> {
    let like = format!("%{}%", q);
    Ok(sqlx::query_as::<_, Student>(
        r#"SELECT * FROM student
           WHERE first_name LIKE ? OR last_name LIKE ? OR admission_no LIKE ?
           ORDER BY last_name, first_name LIMIT ?"#,
    ).bind(&like).bind(&like).bind(&like).bind(limit).fetch_all(pool).await?)
}

pub async fn update_student(pool: &SqlitePool, id: i64, u: &UpdateStudent) -> RepoResult<Student> {
    sqlx::query(
        r#"UPDATE student SET
             first_name    = COALESCE(?, first_name),
             middle_name   = COALESCE(?, middle_name),
             last_name     = COALESCE(?, last_name),
             gender        = COALESCE(?, gender),
             blood_group   = COALESCE(?, blood_group),
             nationality   = COALESCE(?, nationality),
             religion      = COALESCE(?, religion),
             photo_path    = COALESCE(?, photo_path),
             status        = COALESCE(?, status),
             address_line1 = COALESCE(?, address_line1),
             address_line2 = COALESCE(?, address_line2),
             city          = COALESCE(?, city),
             state         = COALESCE(?, state),
             postal_code   = COALESCE(?, postal_code),
             country       = COALESCE(?, country),
             updated_at    = datetime('now')
           WHERE id = ?"#,
    )
    .bind(&u.first_name).bind(&u.middle_name).bind(&u.last_name)
    .bind(&u.gender).bind(&u.blood_group)
    .bind(&u.nationality).bind(&u.religion).bind(&u.photo_path)
    .bind(&u.status)
    .bind(&u.address_line1).bind(&u.address_line2)
    .bind(&u.city).bind(&u.state).bind(&u.postal_code).bind(&u.country)
    .bind(id).execute(pool).await?;
    get_student(pool, id).await
}

pub async fn set_student_status(pool: &SqlitePool, id: i64, status: &str) -> RepoResult<()> {
    sqlx::query("UPDATE student SET status = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(status).bind(id).execute(pool).await?;
    Ok(())
}

pub async fn delete_student(pool: &SqlitePool, id: i64) -> RepoResult<()> {
    let res = sqlx::query("DELETE FROM student WHERE id = ?")
        .bind(id).execute(pool).await?;
    if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
    Ok(())
}

// ── Staff SQL (was StaffRepo) ───────────────────────────────────────

pub async fn create_staff(pool: &SqlitePool, s: &NewStaff) -> RepoResult<Staff> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO staff (
            employee_no, user_id, department_id, first_name, last_name,
            date_of_birth, gender, phone, email, designation,
            employment_type, date_of_joining, photo_path
          ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
    )
    .bind(&s.employee_no).bind(s.user_id).bind(s.department_id)
    .bind(&s.first_name).bind(&s.last_name)
    .bind(s.date_of_birth).bind(&s.gender)
    .bind(&s.phone).bind(&s.email).bind(&s.designation)
    .bind(&s.employment_type).bind(s.date_of_joining).bind(&s.photo_path)
    .fetch_one(pool).await?;
    get_staff(pool, id).await
}

pub async fn get_staff(pool: &SqlitePool, id: i64) -> RepoResult<Staff> {
    sqlx::query_as::<_, Staff>("SELECT * FROM staff WHERE id = ?")
        .bind(id).fetch_optional(pool).await?
        .ok_or(RepoError::NotFound)
}

pub async fn find_staff_by_employee_no(pool: &SqlitePool, no: &str) -> RepoResult<Option<Staff>> {
    Ok(sqlx::query_as::<_, Staff>("SELECT * FROM staff WHERE employee_no = ?")
        .bind(no).fetch_optional(pool).await?)
}

pub async fn list_staff(pool: &SqlitePool, limit: i64, offset: i64) -> RepoResult<Vec<Staff>> {
    Ok(sqlx::query_as::<_, Staff>(
        "SELECT * FROM staff ORDER BY last_name, first_name LIMIT ? OFFSET ?",
    ).bind(limit).bind(offset).fetch_all(pool).await?)
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
             updated_at       = datetime('now')
           WHERE id = ?"#,
    )
    .bind(u.department_id.unwrap_or(None))
    .bind(&u.first_name).bind(&u.last_name)
    .bind(&u.phone).bind(&u.email).bind(&u.designation)
    .bind(&u.employment_type)
    .bind(u.date_of_leaving.unwrap_or(None))
    .bind(&u.status).bind(&u.photo_path).bind(id)
    .execute(pool).await?;
    get_staff(pool, id).await
}

// ── High-level workflows ────────────────────────────────────────────

pub async fn admit(pool: &SqlitePool, ctx: &RequestCtx, a: Admission) -> ServiceResult<AdmissionResult> {
    ctx.require(perm::STUDENTS_ADMIT)?;

    if find_student_by_admission_no(pool, &a.student.admission_no).await?.is_some() {
        return Err(ServiceError::conflict("admission_no already exists"));
    }
    let student = create_student(pool, &a.student).await?;

    let mut guardian_id = None;
    if let Some((g, relationship, is_primary)) = a.guardian {
        let guardian = guardian_svc::create(pool, &g).await?;
        guardian_svc::link(pool, &StudentGuardianLink {
            student_id: student.id,
            guardian_id: guardian.id,
            relationship,
            is_primary,
            is_emergency: is_primary,
            can_pickup: true,
        }).await?;
        guardian_id = Some(guardian.id);
    }
    Ok(AdmissionResult { student, guardian_id })
}

pub async fn withdraw(pool: &SqlitePool, ctx: &RequestCtx, student_id: i64) -> ServiceResult<()> {
    ctx.require(perm::STUDENTS_EDIT)?;
    set_student_status(pool, student_id, "withdrawn").await?;
    Ok(())
}

pub async fn graduate(pool: &SqlitePool, ctx: &RequestCtx, student_id: i64) -> ServiceResult<()> {
    ctx.require(perm::STUDENTS_EDIT)?;
    set_student_status(pool, student_id, "graduated").await?;
    Ok(())
}

pub async fn hire_staff(pool: &SqlitePool, ctx: &RequestCtx, s: NewStaff) -> ServiceResult<Staff> {
    ctx.require(perm::STAFF_HIRE)?;
    if find_staff_by_employee_no(pool, &s.employee_no).await?.is_some() {
        return Err(ServiceError::conflict("employee_no already exists"));
    }
    Ok(create_staff(pool, &s).await?)
}

pub async fn terminate_staff(
    pool: &SqlitePool, ctx: &RequestCtx, staff_id: i64, on: NaiveDate,
) -> ServiceResult<()> {
    ctx.require(perm::STAFF_EDIT)?;
    update_staff(pool, staff_id, &UpdateStaff {
        status: Some("terminated".into()),
        date_of_leaving: Some(Some(on)),
        ..Default::default()
    }).await?;
    Ok(())
}

pub async fn list_students_for(
    pool: &SqlitePool, ctx: &RequestCtx, limit: i64, offset: i64,
) -> ServiceResult<Vec<Student>> {
    ctx.require_any(&[perm::STUDENTS_VIEW, perm::STUDENTS_VIEW_OWN])?;
    match Scope::from_ctx(ctx) {
        Scope::Global => Ok(list_all_students(pool, limit, offset).await?),
        Scope::GuardianOfUser(uid) => {
            let ids = guardian_svc::students_of_user(pool, uid).await?;
            Ok(list_students_by_ids(pool, &ids).await?)
        }
        Scope::SelfStudent(uid) => {
            Ok(match find_student_by_user_id(pool, uid).await? {
                Some(s) => vec![s],
                None    => Vec::new(),
            })
        }
    }
}

pub async fn can_view_student(
    pool: &SqlitePool, ctx: &RequestCtx, student_id: i64,
) -> ServiceResult<bool> {
    Ok(match Scope::from_ctx(ctx) {
        Scope::Global => true,
        Scope::GuardianOfUser(uid) => guardian_svc::is_guardian_of(pool, uid, student_id).await?,
        Scope::SelfStudent(uid) => find_student_by_user_id(pool, uid).await?
            .map(|s| s.id == student_id).unwrap_or(false),
    })
}
