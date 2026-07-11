use sqlx::SqlitePool;
use crate::models::TenantModel::{Tenant, NewTenant};

pub async fn slug_exists(pool: &SqlitePool, slug: &str) -> Result<bool, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tenants WHERE slug = ?")
        .bind(slug)
        .fetch_one(pool)
        .await?;
    Ok(row.0 > 0)
}

pub async fn insert_tenant(pool: &SqlitePool, t: NewTenant<'_>) -> Result<Tenant, sqlx::Error> {
    let database_name = format!("{}", t.slug);

    sqlx::query(
        "INSERT INTO tenants (slug, name, contact_email, contact_phone, address, database_name)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(t.slug)
    .bind(t.name)
    .bind(t.contact_email)
    .bind(t.contact_phone)
    .bind(t.address)
    .bind(&database_name)
    .execute(pool)
    .await?;

    sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE slug = ?")
        .bind(t.slug)
        .fetch_one(pool)
        .await
}

pub async fn update_theme(
    pool: &SqlitePool,
    slug: &str,
    primary_color: &str,
    dark_mode: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE tenants SET primary_color = ?, dark_mode = ? WHERE slug = ?")
        .bind(primary_color)
        .bind(dark_mode)
        .bind(slug)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_tenant_by_slug(
    pool: &SqlitePool,
    slug: &str,
) -> Result<Option<Tenant>, sqlx::Error> {
    sqlx::query_as::<_, Tenant>(
        "SELECT * FROM tenants WHERE slug = ?",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await
}
