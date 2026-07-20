//! `/api/tenant/examinations/*`

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::examinations::{
    EnterResult, Exam, ExamResult, ExamSchedule, GradeBand, NewExam, NewSchedule,
};
use crate::services::examinations::ReportCard;

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/grading-scales",        post(create_scale))
        .route("/grading-scales/{id}/bands", post(add_band).get(list_bands))
        .route("/exams",                  post(create_exam))
        .route("/exams/term/{tid}",       get(list_by_term))
        .route("/exams/{id}/schedules",   post(schedule).get(list_schedules))
        .route("/results",                post(enter_result))
        .route("/results/student/{sid}",  get(for_student))
        .route("/report-cards/{sid}/{eid}", get(report_card))
}

#[derive(serde::Deserialize)] struct CreateScale { name: String }

async fn create_scale(ExtractServices(a): ExtractServices, Json(b): Json<CreateScale>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let s = a.repos.grading_scales.create_scale(&b.name).await.map_err(|e| ServiceHttpError(e.into()))?;
    Ok(Json(serde_json::json!({ "id": s.id, "name": s.name })))
}

async fn add_band(ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(mut b): Json<GradeBand>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    b.grading_scale_id = id;
    let bid = a.repos.grading_scales.add_band(&b).await.map_err(|e| ServiceHttpError(e.into()))?;
    Ok(Json(serde_json::json!({ "id": bid })))
}

async fn list_bands(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<GradeBand>>, ServiceHttpError>
{ Ok(Json(a.repos.grading_scales.bands(id).await.map_err(|e| ServiceHttpError(e.into()))?)) }

async fn create_exam(ExtractServices(a): ExtractServices, Json(b): Json<NewExam>)
    -> Result<Json<Exam>, ServiceHttpError>
{ Ok(Json(a.examinations.create_exam(b).await?)) }

async fn list_by_term(ExtractServices(a): ExtractServices, Path(tid): Path<i64>)
    -> Result<Json<Vec<Exam>>, ServiceHttpError>
{ Ok(Json(a.repos.exams.list_for_term(tid).await.map_err(|e| ServiceHttpError(e.into()))?)) }

async fn schedule(ExtractServices(a): ExtractServices, Path(exam_id): Path<i64>, Json(mut b): Json<NewSchedule>)
    -> Result<Json<ExamSchedule>, ServiceHttpError>
{ b.exam_id = exam_id; Ok(Json(a.examinations.schedule(b).await?)) }

async fn list_schedules(ExtractServices(a): ExtractServices, Path(exam_id): Path<i64>)
    -> Result<Json<Vec<ExamSchedule>>, ServiceHttpError>
{ Ok(Json(a.repos.exam_schedules.for_exam(exam_id).await.map_err(|e| ServiceHttpError(e.into()))?)) }

async fn enter_result(ExtractServices(a): ExtractServices, Json(b): Json<EnterResult>)
    -> Result<Json<ExamResult>, ServiceHttpError>
{ Ok(Json(a.examinations.enter_result(b).await?)) }

async fn for_student(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<Vec<ExamResult>>, ServiceHttpError>
{ Ok(Json(a.repos.exam_results.for_student(sid).await.map_err(|e| ServiceHttpError(e.into()))?)) }

async fn report_card(
    ExtractServices(a): ExtractServices, Path((sid, eid)): Path<(i64, i64)>,
) -> Result<Json<ReportCardOut>, ServiceHttpError> {
    Ok(Json(ReportCardOut::from(a.examinations.report_card(sid, eid).await?)))
}

#[derive(serde::Serialize)]
struct ReportCardOut {
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
