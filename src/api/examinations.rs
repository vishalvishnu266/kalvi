//! `/api/{tenant}/examinations/*` handlers.

use axum::{extract::Path, Json};

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::examinations::{
    EnterResult, Exam, ExamResult, ExamSchedule, GradeBand, NewExam, NewSchedule,
};
use crate::services::examinations::ReportCard;

#[derive(serde::Deserialize)] pub struct CreateScale { name: String }

pub async fn create_scale(scope: TenantScope, Json(b): Json<CreateScale>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let s = scope.services.repos.grading_scales.create_scale(&b.name).await?;
    Ok(Json(serde_json::json!({ "id": s.id, "name": s.name })))
}

pub async fn add_band(scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(mut b): Json<GradeBand>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    b.grading_scale_id = id;
    let bid = scope.services.repos.grading_scales.add_band(&b).await?;
    Ok(Json(serde_json::json!({ "id": bid })))
}

pub async fn list_bands(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<GradeBand>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.grading_scales.bands(id).await?)) }

pub async fn create_exam(scope: TenantScope, Json(b): Json<NewExam>)
    -> Result<Json<Exam>, ServiceHttpError>
{ Ok(Json(scope.services.examinations.create_exam(b).await?)) }

pub async fn list_by_term(scope: TenantScope, Path((_t, tid)): Path<(String, i64)>)
    -> Result<Json<Vec<Exam>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.exams.list_for_term(tid).await?)) }

pub async fn schedule(scope: TenantScope, Path((_t, exam_id)): Path<(String, i64)>, Json(mut b): Json<NewSchedule>)
    -> Result<Json<ExamSchedule>, ServiceHttpError>
{ b.exam_id = exam_id; Ok(Json(scope.services.examinations.schedule(b).await?)) }

pub async fn list_schedules(scope: TenantScope, Path((_t, exam_id)): Path<(String, i64)>)
    -> Result<Json<Vec<ExamSchedule>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.exam_schedules.for_exam(exam_id).await?)) }

pub async fn enter_result(scope: TenantScope, Json(b): Json<EnterResult>)
    -> Result<Json<ExamResult>, ServiceHttpError>
{ Ok(Json(scope.services.examinations.enter_result(b).await?)) }

pub async fn for_student(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<Vec<ExamResult>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.exam_results.for_student(sid).await?)) }

pub async fn report_card(
    scope: TenantScope, Path((_t, sid, eid)): Path<(String, i64, i64)>,
) -> Result<Json<ReportCardOut>, ServiceHttpError> {
    Ok(Json(ReportCardOut::from(scope.services.examinations.report_card(sid, eid).await?)))
}

#[derive(serde::Serialize)]
pub struct ReportCardOut {
    student_id: i64,
    exam_id: i64,
    rows: Vec<serde_json::Value>,
    total_max: f64,
    total_obtained: f64,
    overall_percent: f64,
    overall_letter: Option<String>,
}
impl From<ReportCard> for ReportCardOut {
    fn from(r: ReportCard) -> Self {
        Self {
            student_id: r.student_id, exam_id: r.exam_id,
            rows: r.rows.into_iter().map(|row| serde_json::json!({
                "subject_id": row.subject_id, "max_marks": row.max_marks,
                "marks_obtained": row.marks_obtained,
                "percent": row.percent, "letter": row.letter,
            })).collect(),
            total_max: r.total_max, total_obtained: r.total_obtained,
            overall_percent: r.overall_percent, overall_letter: r.overall_letter,
        }
    }
}
