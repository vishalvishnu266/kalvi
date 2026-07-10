use sqlx::SqlitePool;

pub async fn get_student(pool: &SqlitePool, id: i32) -> Result<Option<model::Student>, sqlx::Error> {
    let row: Option<(i32, String)> = sqlx::query_as("SELECT id, name FROM students WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|(id, name)| model::Student { id, name }))
}
