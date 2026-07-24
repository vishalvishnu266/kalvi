use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::health::HealthRecord;
use crate::services::ServiceResult;

#[derive(Debug, Clone)]
pub struct BmiSnapshot {
    pub bmi: f64,
    pub category: &'static str,
}

#[derive(Clone)]
pub struct HealthService {
    repos: Arc<Repositories>,
}

impl HealthService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    pub async fn upsert_vitals(
        &self, student_id: i64,
        height_cm: Option<f64>, weight_kg: Option<f64>,
        allergies: Option<&str>, conditions: Option<&str>,
    ) -> ServiceResult<HealthRecord> {
        Ok(self.repos.health_records
            .upsert(student_id, height_cm, weight_kg, allergies, conditions).await?)
    }

    pub async fn bmi_for(&self, student_id: i64) -> ServiceResult<Option<BmiSnapshot>> {
        let rec = self.repos.health_records.for_student(student_id).await?;
        Ok(rec.and_then(|r| {
            match (r.height_cm, r.weight_kg) {
                (Some(h), Some(w)) if h > 0.0 => {
                    let m = h / 100.0;
                    let bmi = w / (m * m);
                    let category = if bmi < 18.5 { "underweight" }
                                   else if bmi < 25.0 { "normal" }
                                   else if bmi < 30.0 { "overweight" }
                                   else { "obese" };
                    Some(BmiSnapshot { bmi, category })
                }
                _ => None,
            }
        }))
    }

    pub async fn record_vaccination(
        &self, student_id: i64, vaccine: &str, dose: Option<&str>, given_on: NaiveDate,
    ) -> ServiceResult<i64> {
        Ok(self.repos.vaccinations.record(student_id, vaccine, dose, given_on).await?)
    }

    pub async fn clinic_visit(
        &self, student_id: i64, complaint: Option<&str>, treatment: Option<&str>,
        attended_by_staff_id: Option<i64>,
    ) -> ServiceResult<i64> {
        Ok(self.repos.clinic_visits.record(student_id, complaint, treatment, attended_by_staff_id).await?)
    }
}
