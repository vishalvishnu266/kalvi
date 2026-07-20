//! `/api/tenant/academic/*` — years, terms, grades, sections, rooms, subjects,
//! class sections, class subjects.

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::academic_structure::{Grade, NewRoom, NewSubject, Room, Section, Subject};
use crate::repositories::class_enrollment::{ClassSection, ClassSubject, NewClassSection};
use crate::repositories::core::{AcademicYear, NewAcademicYear, NewTerm, Term};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        // Years / current / rollover
        .route("/years",               get(list_years).post(create_year))
        .route("/years/current",       get(current_year))
        .route("/years/{id}/activate",  post(activate_year))
        .route("/years/{id}/terms",     get(list_terms).post(create_term))
        // Reference data
        .route("/grades",   get(list_grades))
        .route("/sections", get(list_sections))
        .route("/rooms",    get(list_rooms).post(create_room))
        .route("/subjects", get(list_subjects).post(create_subject))
        // Class sections
        .route("/class-sections",              post(create_class_section))
        .route("/class-sections/{year_id}",     get(list_class_sections))
        .route("/class-sections/{id}/subjects", get(list_class_subjects).post(assign_class_subject))
}

// --- Years ---
async fn list_years(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<AcademicYear>>, ServiceHttpError>
{ Ok(Json(a.repos.academic_years.list().await.map_err(school_erp_err)?)) }

async fn create_year(ExtractServices(a): ExtractServices, Json(b): Json<NewAcademicYear>)
    -> Result<Json<AcademicYear>, ServiceHttpError>
{ Ok(Json(a.academic.create_year(b).await?)) }

async fn current_year(ExtractServices(a): ExtractServices)
    -> Result<Json<AcademicYear>, ServiceHttpError>
{ Ok(Json(a.academic.current_year().await?)) }

async fn activate_year(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{
    a.repos.academic_years.set_current(id).await.map_err(school_erp_err)?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn list_terms(ExtractServices(a): ExtractServices, Path(year_id): Path<i64>)
    -> Result<Json<Vec<Term>>, ServiceHttpError>
{ Ok(Json(a.academic.list_terms(year_id).await?)) }

async fn create_term(
    ExtractServices(a): ExtractServices,
    Path(year_id): Path<i64>,
    Json(mut b): Json<NewTerm>,
) -> Result<Json<Term>, ServiceHttpError> {
    b.academic_year_id = year_id;
    Ok(Json(a.repos.terms.create(&b).await.map_err(school_erp_err)?))
}

// --- Reference ---
async fn list_grades(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<Grade>>, ServiceHttpError>
{ Ok(Json(a.repos.grades.list().await.map_err(school_erp_err)?)) }

async fn list_sections(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<Section>>, ServiceHttpError>
{ Ok(Json(a.repos.sections.list().await.map_err(school_erp_err)?)) }

async fn list_rooms(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<Room>>, ServiceHttpError>
{ Ok(Json(a.repos.rooms.list().await.map_err(school_erp_err)?)) }

async fn create_room(ExtractServices(a): ExtractServices, Json(b): Json<NewRoom>)
    -> Result<Json<Room>, ServiceHttpError>
{ Ok(Json(a.repos.rooms.create(&b).await.map_err(school_erp_err)?)) }

async fn list_subjects(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<Subject>>, ServiceHttpError>
{ Ok(Json(a.repos.subjects.list().await.map_err(school_erp_err)?)) }

async fn create_subject(ExtractServices(a): ExtractServices, Json(b): Json<NewSubject>)
    -> Result<Json<Subject>, ServiceHttpError>
{ Ok(Json(a.repos.subjects.create(&b).await.map_err(school_erp_err)?)) }

// --- Class sections ---
async fn create_class_section(
    ExtractServices(a): ExtractServices,
    Json(b): Json<NewClassSection>,
) -> Result<Json<ClassSection>, ServiceHttpError> {
    Ok(Json(a.repos.class_sections.create(&b).await.map_err(school_erp_err)?))
}

async fn list_class_sections(ExtractServices(a): ExtractServices, Path(year_id): Path<i64>)
    -> Result<Json<Vec<ClassSection>>, ServiceHttpError>
{ Ok(Json(a.repos.class_sections.list_for_year(year_id).await.map_err(school_erp_err)?)) }

async fn list_class_subjects(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<ClassSubject>>, ServiceHttpError>
{ Ok(Json(a.repos.class_subjects.list_for_class(id).await.map_err(school_erp_err)?)) }

#[derive(serde::Deserialize)]
struct AssignSubject { subject_id: i64, teacher_id: Option<i64> }

async fn assign_class_subject(
    ExtractServices(a): ExtractServices,
    Path(id): Path<i64>,
    Json(b): Json<AssignSubject>,
) -> Result<Json<ClassSubject>, ServiceHttpError> {
    Ok(Json(a.repos.class_subjects
        .assign(id, b.subject_id, b.teacher_id).await.map_err(school_erp_err)?))
}

fn school_erp_err(e: crate::error::RepoError) -> ServiceHttpError {
    ServiceHttpError(crate::ServiceError::from(e))
}
