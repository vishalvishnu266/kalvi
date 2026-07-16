use sqlx::SqlitePool;
use crate::models::academic_year::TenantSetting;
use crate::errors::AppError;

pub struct GeneralSettings {
    pub institution_type: String,
    pub display_name: String,
    pub locale: String,
    pub timezone: String,
}

pub struct SettingsService;

impl SettingsService {
    pub async fn get_general_settings(pool: &SqlitePool) -> Result<GeneralSettings, AppError> {
        let institution_type = TenantSetting::get_or(pool, "institution_type", "school").await?;
        let display_name = TenantSetting::get_or(pool, "display_name", "Institution").await?;
        let locale = TenantSetting::get_or(pool, "locale", "en-IN").await?;
        let timezone = TenantSetting::get_or(pool, "timezone", "Asia/Kolkata").await?;

        Ok(GeneralSettings {
            institution_type,
            display_name,
            locale,
            timezone,
        })
    }

    pub async fn update_general_settings(
        pool: &SqlitePool,
        institution_type: &str,
        display_name: &str,
        locale: &str,
        timezone: &str,
    ) -> Result<(), AppError> {
        let it = match institution_type {
            "school" | "university" => institution_type,
            _ => "school",
        };
        TenantSetting::set(pool, "institution_type", it).await?;
        TenantSetting::set(pool, "display_name", display_name.trim()).await?;
        
        if !locale.trim().is_empty() {
            TenantSetting::set(pool, "locale", locale.trim()).await?;
        }
        if !timezone.trim().is_empty() {
            TenantSetting::set(pool, "timezone", timezone.trim()).await?;
        }
        
        Ok(())
    }
}
