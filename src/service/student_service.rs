use sqlx::SqlitePool;
use crate::repository::student_repository::StudentRepository;
use crate::util::errors::AppError;
use crate::util::id_util::current_timestamp;
use std::collections::HashMap;

pub struct StudentService;

impl StudentService {
    pub async fn add_student(
        pool: &SqlitePool,
        first_name: String,
        last_name: String,
        email: Option<String>,
        phone: Option<String>,
        enrollment_number: Option<String>,
    ) -> Result<i64, AppError> {
        // Validation logic
        let mut errors = HashMap::new();
        if first_name.trim().is_empty() {
            errors.insert("first_name".to_string(), "First name is required".to_string());
        }
        if last_name.trim().is_empty() {
            errors.insert("last_name".to_string(), "Last name is required".to_string());
        }

        if !errors.is_empty() {
            return Err(AppError::BusinessException {
                errors,
                message: Some("Please correct the errors below".to_string()),
            });
        }

        StudentRepository::create(
            pool,
            &first_name,
            &last_name,
            email,
            phone,
            enrollment_number,
            current_timestamp(),
        ).await
    }
}
