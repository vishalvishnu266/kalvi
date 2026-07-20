//! Transport: vehicles, routes, stops, and student assignments.

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- Vehicle ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: i64,
    pub reg_number: String,
    pub model: Option<String>,
    pub capacity: Option<i64>,
    pub driver_staff_id: Option<i64>,
}

#[derive(Clone)]
pub struct VehicleRepo { pool: SqlitePool }

impl VehicleRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, v: &Vehicle) -> RepoResult<Vehicle> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO vehicle (reg_number, model, capacity, driver_staff_id)
               VALUES (?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&v.reg_number).bind(&v.model).bind(v.capacity).bind(v.driver_staff_id)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Vehicle> {
        sqlx::query_as::<_, Vehicle>("SELECT * FROM vehicle WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Vehicle>> {
        Ok(sqlx::query_as::<_, Vehicle>("SELECT * FROM vehicle ORDER BY reg_number")
            .fetch_all(&self.pool).await?)
    }

    pub async fn set_driver(&self, id: i64, driver_staff_id: Option<i64>) -> RepoResult<()> {
        sqlx::query("UPDATE vehicle SET driver_staff_id = ? WHERE id = ?")
            .bind(driver_staff_id).bind(id).execute(&self.pool).await?;
        Ok(())
    }
}

// ---------- Route + stops ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Route {
    pub id: i64,
    pub name: String,
    pub vehicle_id: Option<i64>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RouteStop {
    pub id: i64,
    pub route_id: i64,
    pub name: String,
    pub stop_order: i64,
    pub pickup_time: Option<NaiveTime>,
    pub drop_time: Option<NaiveTime>,
    pub fare_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStop {
    pub name: String,
    pub stop_order: i64,
    pub pickup_time: Option<NaiveTime>,
    pub drop_time: Option<NaiveTime>,
    pub fare_cents: i64,
}

#[derive(Clone)]
pub struct RouteRepo { pool: SqlitePool }

impl RouteRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, name: &str, vehicle_id: Option<i64>) -> RepoResult<Route> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO route (name, vehicle_id) VALUES (?, ?) RETURNING id",
        ).bind(name).bind(vehicle_id).fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Route> {
        sqlx::query_as::<_, Route>("SELECT * FROM route WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Route>> {
        Ok(sqlx::query_as::<_, Route>("SELECT * FROM route ORDER BY name")
            .fetch_all(&self.pool).await?)
    }

    pub async fn add_stop(&self, route_id: i64, s: &NewStop) -> RepoResult<RouteStop> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO route_stop
                 (route_id, name, stop_order, pickup_time, drop_time, fare_cents)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(route_id).bind(&s.name).bind(s.stop_order)
        .bind(s.pickup_time).bind(s.drop_time).bind(s.fare_cents)
        .fetch_one(&self.pool).await?;

        sqlx::query_as::<_, RouteStop>("SELECT * FROM route_stop WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn stops(&self, route_id: i64) -> RepoResult<Vec<RouteStop>> {
        Ok(sqlx::query_as::<_, RouteStop>(
            "SELECT * FROM route_stop WHERE route_id = ? ORDER BY stop_order",
        ).bind(route_id).fetch_all(&self.pool).await?)
    }
}

// ---------- Student transport ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StudentTransport {
    pub id: i64,
    pub student_id: i64,
    pub route_stop_id: i64,
    pub academic_year_id: i64,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
}

#[derive(Clone)]
pub struct StudentTransportRepo { pool: SqlitePool }

impl StudentTransportRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn assign(
        &self, student_id: i64, route_stop_id: i64,
        academic_year_id: i64, valid_from: NaiveDate,
    ) -> RepoResult<StudentTransport> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO student_transport
                 (student_id, route_stop_id, academic_year_id, valid_from)
               VALUES (?, ?, ?, ?)
               ON CONFLICT(student_id, academic_year_id) DO UPDATE SET
                 route_stop_id = excluded.route_stop_id,
                 valid_from    = excluded.valid_from,
                 valid_to      = NULL
               RETURNING id"#,
        )
        .bind(student_id).bind(route_stop_id).bind(academic_year_id).bind(valid_from)
        .fetch_one(&self.pool).await?;

        sqlx::query_as::<_, StudentTransport>("SELECT * FROM student_transport WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn end_assignment(&self, id: i64, valid_to: NaiveDate) -> RepoResult<()> {
        sqlx::query("UPDATE student_transport SET valid_to = ? WHERE id = ?")
            .bind(valid_to).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn current_for_student(
        &self, student_id: i64, academic_year_id: i64,
    ) -> RepoResult<Option<StudentTransport>> {
        Ok(sqlx::query_as::<_, StudentTransport>(
            r#"SELECT * FROM student_transport
               WHERE student_id = ? AND academic_year_id = ? AND valid_to IS NULL"#,
        ).bind(student_id).bind(academic_year_id).fetch_optional(&self.pool).await?)
    }

    pub async fn students_on_route(&self, route_id: i64) -> RepoResult<Vec<StudentTransport>> {
        Ok(sqlx::query_as::<_, StudentTransport>(
            r#"SELECT st.* FROM student_transport st
               INNER JOIN route_stop rs ON rs.id = st.route_stop_id
               WHERE rs.route_id = ? AND st.valid_to IS NULL"#,
        ).bind(route_id).fetch_all(&self.pool).await?)
    }
}
