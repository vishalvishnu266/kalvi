use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{ToSchema, IntoParams};

/// Row as stored in the DB.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default, ToSchema)]
pub struct Student {
    pub id: String,
    pub admission_no: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: String,
    pub blood_group: Option<String>,

    pub class_name: String,
    pub section: String,
    pub roll_no: String,
    pub admission_date: String,

    pub guardian_name: String,
    pub guardian_phone: String,
    pub guardian_email: Option<String>,
    pub guardian_relation: Option<String>,

    pub address_line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,

    pub status: String,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Student {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn initials(&self) -> String {
        let f = self.first_name.chars().next().unwrap_or('?');
        let l = self.last_name.chars().next().unwrap_or(' ');
        format!("{}{}", f, l).trim().to_uppercase()
    }

    /// Deterministic hue in [0, 360) from full name for avatar coloring.
    pub fn avatar_hue(&self) -> u32 {
        let name = self.full_name();
        let mut acc: u32 = 0;
        for b in name.as_bytes() {
            acc = acc.wrapping_mul(31).wrapping_add(*b as u32);
        }
        acc % 360
    }

    pub fn email_str(&self) -> &str { self.email.as_deref().unwrap_or("") }
    pub fn phone_str(&self) -> &str { self.phone.as_deref().unwrap_or("") }
    pub fn date_of_birth_str(&self) -> &str { self.date_of_birth.as_deref().unwrap_or("") }
    pub fn blood_group_str(&self) -> &str { self.blood_group.as_deref().unwrap_or("") }
    pub fn guardian_email_str(&self) -> &str { self.guardian_email.as_deref().unwrap_or("") }
    pub fn guardian_relation_str(&self) -> &str { self.guardian_relation.as_deref().unwrap_or("") }
    pub fn address_line_str(&self) -> &str { self.address_line.as_deref().unwrap_or("") }
    pub fn city_str(&self) -> &str { self.city.as_deref().unwrap_or("") }
    pub fn state_str(&self) -> &str { self.state.as_deref().unwrap_or("") }
    pub fn postal_code_str(&self) -> &str { self.postal_code.as_deref().unwrap_or("") }

    pub fn status_badge_class(&self) -> &'static str {
        match self.status.as_str() {
            "Active" => "badge-success-soft",
            "Pending" => "badge-warning-soft",
            "Inactive" | "Suspended" => "badge-danger-soft",
            "Graduated" => "badge-info-soft",
            _ => "badge-soft",
        }
    }

    pub fn gender_matches(&self, g: &str) -> bool { self.gender == g }
    pub fn class_name_matches(&self, c: &str) -> bool { self.class_name == c }
    pub fn section_matches(&self, s: &str) -> bool { self.section == s }
    pub fn status_matches(&self, s: &str) -> bool { self.status == s }
}

/// Form params (from HTML form POSTs).
#[derive(Debug, Deserialize, Default, ToSchema)]
pub struct StudentForm {
    pub csrf_token: Option<String>,

    pub admission_no: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub date_of_birth: String,
    pub gender: String,
    #[serde(default)]
    pub blood_group: String,

    pub class_name: String,
    pub section: String,
    pub roll_no: String,
    #[serde(default)]
    pub admission_date: String,

    pub guardian_name: String,
    pub guardian_phone: String,
    #[serde(default)]
    pub guardian_email: String,
    #[serde(default)]
    pub guardian_relation: String,

    #[serde(default)]
    pub address_line: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub postal_code: String,

    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String { "Active".into() }

fn empty_to_none(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() { None } else { Some(t.to_string()) }
}

impl StudentForm {
    /// Merge form values into a `Student` (creating a new record or updating).
    pub fn apply_to(&self, mut base: Student) -> Student {
        base.admission_no = self.admission_no.trim().to_string();
        base.first_name = self.first_name.trim().to_string();
        base.last_name = self.last_name.trim().to_string();
        base.email = empty_to_none(&self.email);
        base.phone = empty_to_none(&self.phone);
        base.date_of_birth = empty_to_none(&self.date_of_birth);
        base.gender = self.gender.trim().to_string();
        base.blood_group = empty_to_none(&self.blood_group);

        base.class_name = self.class_name.trim().to_string();
        base.section = self.section.trim().to_string();
        base.roll_no = self.roll_no.trim().to_string();
        if !self.admission_date.trim().is_empty() {
            base.admission_date = self.admission_date.trim().to_string();
        }

        base.guardian_name = self.guardian_name.trim().to_string();
        base.guardian_phone = self.guardian_phone.trim().to_string();
        base.guardian_email = empty_to_none(&self.guardian_email);
        base.guardian_relation = empty_to_none(&self.guardian_relation);

        base.address_line = empty_to_none(&self.address_line);
        base.city = empty_to_none(&self.city);
        base.state = empty_to_none(&self.state);
        base.postal_code = empty_to_none(&self.postal_code);

        base.status = if self.status.trim().is_empty() {
            "Active".into()
        } else {
            self.status.trim().to_string()
        };

        base
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.first_name.trim().is_empty() { return Err("First name is required".into()); }
        if self.last_name.trim().is_empty()  { return Err("Last name is required".into()); }
        if self.admission_no.trim().is_empty() { return Err("Admission number is required".into()); }
        if self.class_name.trim().is_empty() { return Err("Class is required".into()); }
        if self.section.trim().is_empty() { return Err("Section is required".into()); }
        if self.roll_no.trim().is_empty() { return Err("Roll number is required".into()); }
        if self.guardian_name.trim().is_empty() { return Err("Guardian name is required".into()); }
        if self.guardian_phone.trim().is_empty() { return Err("Guardian phone is required".into()); }
        match self.gender.as_str() {
            "Male" | "Female" | "Other" => {}
            _ => return Err("Gender must be Male, Female or Other".into()),
        }
        Ok(())
    }
}

/// Filters for the list page.
#[derive(Debug, Deserialize, Default, Clone, IntoParams)]
pub struct StudentFilters {
    #[serde(default)]
    pub class_name: String,
    #[serde(default)]
    pub section: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub q: String,
}

impl StudentFilters {
    pub fn class_name_matches(&self, c: &str) -> bool {
        self.class_name == c
    }
    pub fn section_matches(&self, s: &str) -> bool {
        self.section == s
    }
    pub fn status_matches(&self, s: &str) -> bool {
        self.status == s
    }
}
