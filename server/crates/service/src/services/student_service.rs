use sqlx::SqlitePool;
use model::Student;

pub async fn get_student(pool: &SqlitePool, id: i32) -> Result<Option<Student>, sqlx::Error> {
    repository::get_student(pool, id).await
}
