use sqlx::SqlitePool;
use crate::model::student::Student;
use crate::repository::StudentRepository;
use crate::util::AppError;
use std::collections::HashMap;

pub struct StudentService;

impl StudentService {
    pub async fn add_student(
        pool: &SqlitePool,
        first_name: &str,
        last_name: &str,
        email: Option<&str>,
        phone: Option<&str>,
        enrollment_number: Option<&str>,
    ) -> Result<Student, AppError> {
        // Business Validation
        if first_name.trim().is_empty() || last_name.trim().is_empty() {
            let mut fields = HashMap::new();
            if first_name.trim().is_empty() { fields.insert("first_name".to_string(), "First name is required".to_string()); }
            if last_name.trim().is_empty() { fields.insert("last_name".to_string(), "Last name is required".to_string()); }
            return Err(AppError::BusinessException("Missing required student fields".to_string(), fields));
        }

        Ok(StudentRepository::create(pool, first_name, last_name, email, phone, enrollment_number).await?)
    }
}
