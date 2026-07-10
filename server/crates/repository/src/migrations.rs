use sqlx::SqlitePool;

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Ensure tables exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    // Insert test data if empty
    sqlx::query(
        "INSERT OR IGNORE INTO students (id, name) VALUES (1, 'vishal Doe')"
    )
    .execute(pool)
    .await?;

    Ok(())
}
