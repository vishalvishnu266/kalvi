use sqlx::{Sqlite, Executor};
use crate::model::student::Student;

pub struct StudentRepository;

impl StudentRepository {
    pub async fn create(
        executor: impl Executor<'_, Database = Sqlite>,
        first_name: &str,
        last_name: &str,
        email: Option<&str>,
        phone: Option<&str>,
        enrollment_number: Option<&str>,
    ) -> Result<Student, sqlx::Error> {
        let now = crate::util::id_util::current_timestamp();
        
        let id = sqlx::query("INSERT INTO students (first_name, last_name, email, phone, enrollment_number, created_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(first_name)
            .bind(last_name)
            .bind(email)
            .bind(phone)
            .bind(enrollment_number)
            .bind(now)
            .execute(executor)
            .await?
            .last_insert_rowid();

        Ok(Student {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.map(|s| s.to_string()),
            phone: phone.map(|s| s.to_string()),
            enrollment_number: enrollment_number.map(|s| s.to_string()),
            created_at: now,
        })
    }

    pub async fn count(executor: impl Executor<'_, Database = Sqlite>) -> Result<i64, sqlx::Error> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM students")
            .fetch_one(executor)
            .await?;
        Ok(count)
    }
}
