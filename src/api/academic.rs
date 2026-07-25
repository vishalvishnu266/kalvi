use axum::{extract::Path, http::StatusCode, Json};

use crate::http::{ServiceHttpError, TenantScope};
use crate::models::academic::{
    AcademicYear, Grade, NewAcademicYear, NewRoom, NewSubject, NewTerm, Room, Section, Subject, Term,
};
use crate::services::academic as ac_svc;
use crate::services::perm;

pub async fn list_years(scope: TenantScope)
    -> Result<Json<Vec<AcademicYear>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(ac_svc::list_years(&scope.pool).await?))
}

pub async fn create_year(scope: TenantScope, Json(b): Json<NewAcademicYear>)
    -> Result<Json<AcademicYear>, ServiceHttpError>
{
    Ok(Json(ac_svc::create_year(&scope.pool, &scope.ctx, b).await?))
}

pub async fn current_year(scope: TenantScope)
    -> Result<Json<AcademicYear>, ServiceHttpError>
{
    Ok(Json(ac_svc::current_year(&scope.pool).await?))
}

pub async fn activate_year(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    ac_svc::set_current_year(&scope.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_terms(scope: TenantScope, Path((_t, year_id)): Path<(String, i64)>)
    -> Result<Json<Vec<Term>>, ServiceHttpError>
{
    Ok(Json(ac_svc::list_terms(&scope.pool, &scope.ctx, year_id).await?))
}

pub async fn create_term(
    scope: TenantScope,
    Path((_t, year_id)): Path<(String, i64)>,
    Json(mut b): Json<NewTerm>,
) -> Result<Json<Term>, ServiceHttpError> {
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    b.academic_year_id = year_id;
    Ok(Json(ac_svc::create_term_row(&scope.pool, &b).await?))
}

pub async fn list_grades(scope: TenantScope)
    -> Result<Json<Vec<Grade>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(ac_svc::list_grades(&scope.pool).await?))
}

pub async fn list_sections(scope: TenantScope)
    -> Result<Json<Vec<Section>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(ac_svc::list_sections(&scope.pool).await?))
}

pub async fn list_rooms(scope: TenantScope)
    -> Result<Json<Vec<Room>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(ac_svc::list_rooms(&scope.pool).await?))
}

pub async fn create_room(scope: TenantScope, Json(b): Json<NewRoom>)
    -> Result<Json<Room>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    Ok(Json(ac_svc::create_room(&scope.pool, &b).await?))
}

pub async fn list_subjects(scope: TenantScope)
    -> Result<Json<Vec<Subject>>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_VIEW)?;
    Ok(Json(ac_svc::list_subjects(&scope.pool).await?))
}

pub async fn create_subject(scope: TenantScope, Json(b): Json<NewSubject>)
    -> Result<Json<Subject>, ServiceHttpError>
{
    scope.ctx.require(perm::ACADEMIC_MANAGE)?;
    Ok(Json(ac_svc::create_subject(&scope.pool, &b).await?))
}
