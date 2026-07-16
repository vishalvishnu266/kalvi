use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::student::{Student, StudentFilters, StudentForm};
use crate::errors::AppError;

pub struct StudentService;

impl StudentService {
    pub async fn list(pool: &SqlitePool, f: &StudentFilters) -> Result<Vec<Student>, AppError> {
        let mut sql = String::from("SELECT * FROM students WHERE 1=1");
        let mut binds: Vec<String> = Vec::new();

        if !f.class_name.is_empty() {
            sql.push_str(" AND class_name = ?");
            binds.push(f.class_name.clone());
        }
        if !f.section.is_empty() {
            sql.push_str(" AND section = ?");
            binds.push(f.section.clone());
        }
        if !f.status.is_empty() {
            sql.push_str(" AND status = ?");
            binds.push(f.status.clone());
        }
        if !f.q.trim().is_empty() {
            sql.push_str(
                " AND (first_name LIKE ? OR last_name LIKE ? OR admission_no LIKE ? OR email LIKE ?)",
            );
            let pat = format!("%{}%", f.q.trim());
            binds.push(pat.clone());
            binds.push(pat.clone());
            binds.push(pat.clone());
            binds.push(pat);
        }
        sql.push_str(" ORDER BY class_name, section, roll_no");

        let mut q = sqlx::query_as::<_, Student>(&sql);
        for b in &binds {
            q = q.bind(b);
        }
        Ok(q.fetch_all(pool).await?)
    }

    pub async fn count(pool: &SqlitePool) -> Result<i64, AppError> {
        Ok(sqlx::query_scalar("SELECT COUNT(*) FROM students").fetch_one(pool).await?)
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Student>, AppError> {
        Ok(sqlx::query_as::<_, Student>("SELECT * FROM students WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?)
    }

    pub async fn create(pool: &SqlitePool, form: StudentForm) -> Result<Student, AppError> {
        let id = Uuid::new_v4().to_string();
        let student = form.apply_to(Student {
            id: id.clone(),
            ..Default::default()
        });

        let res = sqlx::query(
            r#"INSERT INTO students (
                id, admission_no, first_name, last_name, email, phone, date_of_birth, gender, blood_group,
                class_name, section, roll_no, admission_date,
                guardian_name, guardian_phone, guardian_email, guardian_relation,
                address_line, city, state, postal_code, status
            ) VALUES (?,?,?,?,?,?,?,?,?, ?,?,?,?, ?,?,?,?, ?,?,?,?, ?)"#,
        )
        .bind(&student.id)
        .bind(&student.admission_no)
        .bind(&student.first_name)
        .bind(&student.last_name)
        .bind(&student.email)
        .bind(&student.phone)
        .bind(&student.date_of_birth)
        .bind(&student.gender)
        .bind(&student.blood_group)
        .bind(&student.class_name)
        .bind(&student.section)
        .bind(&student.roll_no)
        .bind(&student.admission_date)
        .bind(&student.guardian_name)
        .bind(&student.guardian_phone)
        .bind(&student.guardian_email)
        .bind(&student.guardian_relation)
        .bind(&student.address_line)
        .bind(&student.city)
        .bind(&student.state)
        .bind(&student.postal_code)
        .bind(&student.status)
        .execute(pool)
        .await;

        match res {
            Ok(_) => Ok(student),
            Err(e) => {
                let s = e.to_string();
                if s.contains("UNIQUE") && s.contains("admission_no") {
                    Err(AppError::Conflict("Admission number already exists.".into()))
                } else {
                    Err(AppError::Database(e))
                }
            }
        }
    }

    pub async fn update(pool: &SqlitePool, id: &str, form: StudentForm) -> Result<(), AppError> {
        let res = sqlx::query(
            r#"UPDATE students SET
                admission_no=?, first_name=?, last_name=?, email=?, phone=?, date_of_birth=?, gender=?, blood_group=?,
                class_name=?, section=?, roll_no=?, admission_date=?,
                guardian_name=?, guardian_phone=?, guardian_email=?, guardian_relation=?,
                address_line=?, city=?, state=?, postal_code=?, status=?,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?"#,
        )
        .bind(form.admission_no.trim())
        .bind(form.first_name.trim())
        .bind(form.last_name.trim())
        .bind(crate::utils::crud::opt(&form.email))
        .bind(crate::utils::crud::opt(&form.phone))
        .bind(crate::utils::crud::opt(&form.date_of_birth))
        .bind(form.gender.trim())
        .bind(crate::utils::crud::opt(&form.blood_group))
        .bind(form.class_name.trim())
        .bind(form.section.trim())
        .bind(form.roll_no.trim())
        .bind(if form.admission_date.trim().is_empty() {
            None
        } else {
            Some(form.admission_date.trim().to_string())
        })
        .bind(form.guardian_name.trim())
        .bind(form.guardian_phone.trim())
        .bind(crate::utils::crud::opt(&form.guardian_email))
        .bind(crate::utils::crud::opt(&form.guardian_relation))
        .bind(crate::utils::crud::opt(&form.address_line))
        .bind(crate::utils::crud::opt(&form.city))
        .bind(crate::utils::crud::opt(&form.state))
        .bind(crate::utils::crud::opt(&form.postal_code))
        .bind(form.status.trim())
        .bind(id)
        .execute(pool)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                let s = e.to_string();
                if s.contains("UNIQUE") && s.contains("admission_no") {
                    Err(AppError::Conflict("Admission number already exists.".into()))
                } else {
                    Err(AppError::Database(e))
                }
            }
        }
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM students WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_dashboard_stats(pool: &SqlitePool) -> Result<(i64, i64, i64, i64, Vec<Student>), AppError> {
        let total = Self::count(pool).await?;
        let active: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE status = 'Active'")
            .fetch_one(pool).await?;
        let pending: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE status = 'Pending'")
            .fetch_one(pool).await?;
        let inactive: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE status IN ('Inactive','Suspended')")
            .fetch_one(pool).await?;
        
        let recent = sqlx::query_as::<_, Student>(
            "SELECT * FROM students ORDER BY created_at DESC, admission_date DESC LIMIT 5",
        )
        .fetch_all(pool)
        .await?;

        Ok((total, active, pending, inactive, recent))
    }
}
