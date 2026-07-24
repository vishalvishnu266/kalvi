use axum::{extract::Path, http::StatusCode, Json};

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::academic_structure::{Grade, NewRoom, NewSubject, Room, Section, Subject};
use crate::repositories::class_enrollment::{ClassSection, ClassSubject, NewClassSection};
use crate::repositories::core::{AcademicYear, NewAcademicYear, NewTerm, Term};
use crate::services::perm;

pub async fn list_years(scope: TenantScope)
    -> Result<Json<Vec<AcademicYear>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(scope.services.repos.academic_years.list().await?))
}

pub async fn create_year(scope: TenantScope, Json(b): Json<NewAcademicYear>)
    -> Result<Json<AcademicYear>, ServiceHttpError>
{ Ok(Json(scope.services.academic.create_year(&scope.ctx, b).await?)) }

pub async fn current_year(scope: TenantScope)
    -> Result<Json<AcademicYear>, ServiceHttpError>
{ Ok(Json(scope.services.academic.current_year().await?)) }

pub async fn activate_year(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    scope.services.repos.academic_years.set_current(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_terms(scope: TenantScope, Path((_t, year_id)): Path<(String, i64)>)
    -> Result<Json<Vec<Term>>, ServiceHttpError>
{ Ok(Json(scope.services.academic.list_terms(&scope.ctx, year_id).await?)) }

pub async fn create_term(
    scope: TenantScope,
    Path((_t, year_id)): Path<(String, i64)>,
    Json(mut b): Json<NewTerm>,
) -> Result<Json<Term>, ServiceHttpError> {
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    b.academic_year_id = year_id;
    Ok(Json(scope.services.repos.terms.create(&b).await?))
}

pub async fn list_grades(scope: TenantScope)
    -> Result<Json<Vec<Grade>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(scope.services.repos.grades.list().await?))
}

pub async fn list_sections(scope: TenantScope)
    -> Result<Json<Vec<Section>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(scope.services.repos.sections.list().await?))
}

pub async fn list_rooms(scope: TenantScope)
    -> Result<Json<Vec<Room>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(scope.services.repos.rooms.list().await?))
}

pub async fn create_room(scope: TenantScope, Json(b): Json<NewRoom>)
    -> Result<Json<Room>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    Ok(Json(scope.services.repos.rooms.create(&b).await?))
}

pub async fn list_subjects(scope: TenantScope)
    -> Result<Json<Vec<Subject>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(scope.services.repos.subjects.list().await?))
}

pub async fn create_subject(scope: TenantScope, Json(b): Json<NewSubject>)
    -> Result<Json<Subject>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    Ok(Json(scope.services.repos.subjects.create(&b).await?))
}

pub async fn create_class_section(scope: TenantScope, Json(b): Json<NewClassSection>)
    -> Result<Json<ClassSection>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    Ok(Json(scope.services.repos.class_sections.create(&b).await?))
}

pub async fn list_class_sections(scope: TenantScope, Path((_t, year_id)): Path<(String, i64)>)
    -> Result<Json<Vec<ClassSection>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(scope.services.repos.class_sections.list_for_year(year_id).await?))
}

pub async fn list_class_subjects(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<ClassSubject>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(scope.services.repos.class_subjects.list_for_class(id).await?))
}

#[derive(serde::Deserialize)]
pub struct AssignSubject { subject_id: i64, teacher_id: Option<i64> }

pub async fn assign_class_subject(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Json(b): Json<AssignSubject>,
) -> Result<Json<ClassSubject>, ServiceHttpError> {
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    Ok(Json(scope.services.repos.class_subjects
        .assign(id, b.subject_id, b.teacher_id).await?))
}
