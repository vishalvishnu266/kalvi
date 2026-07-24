use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::core::{AcademicYear, NewAcademicYear, NewTerm, Term};
use crate::services::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct AcademicService {
    repos: Arc<Repositories>,
}

impl AcademicService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

pub async fn create_year(&self, y: NewAcademicYear) -> ServiceResult<AcademicYear> {
        Ok(self.repos.academic_years.create(&y).await?)
    }

pub async fn rollover(
        &self, new_year: NewAcademicYear, terms: Vec<NewTerm>,
    ) -> ServiceResult<AcademicYear> {
        let year = self.repos.academic_years.create(&new_year).await?;
        self.repos.academic_years.set_current(year.id).await?;
        for mut t in terms {
            t.academic_year_id = year.id;
            self.repos.terms.create(&t).await?;
        }
        Ok(year)
    }

pub async fn current_year(&self) -> ServiceResult<AcademicYear> {
        Ok(self.repos.academic_years.current().await?)
    }

    pub async fn list_terms(&self, year_id: i64) -> ServiceResult<Vec<Term>> {
        Ok(self.repos.terms.list_for_year(year_id).await?)
    }

pub async fn promote_class(
        &self, from_class_id: i64, to_class_id: i64, to_year_id: i64, enrolled_on: NaiveDate,
    ) -> ServiceResult<u64> {
        if from_class_id == to_class_id {
            return Err(ServiceError::validation("from and to class must differ"));
        }
        let n = self.repos.enrollments
            .promote_class(from_class_id, to_class_id, to_year_id, enrolled_on).await?;
        Ok(n)
    }
}
